use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use dashmap::DashMap;
use tokio::sync::mpsc;

pub mod replay;
pub mod setup;
pub mod oracle;
pub mod env_config;
pub mod trading;
pub mod api_football;
pub mod transfers;
pub mod live_match;
pub mod injuries;
pub mod admin_correction;

/// Approximate real transfer-window calendar for the Big Five leagues:
/// summer (June 1 - Sept 1) and winter (Jan 1 - Feb 3). Deadlines vary
/// slightly year to year and league to league - this is deliberately a
/// close approximation, not scraped from a real calendar API, since the
/// only thing it drives is how strongly `WhistleEnd` weights recent form
/// (see `is_transfer_window`'s usage in `process_event`), not anything
/// requiring day-level precision. Takes plain `(month, day)` rather than a
/// `chrono` type so this crate doesn't need a date-time dependency just for
/// one calendar check - callers that already have `chrono` (e.g. `ssx-node`)
/// extract the two integers from `Utc::now()`.
pub fn is_transfer_window_open(month: u32, day: u32) -> bool {
    match month {
        6 | 7 | 8 => true,
        9 => day <= 1,
        1 => true,
        2 => day <= 3,
        _ => false,
    }
}

/// The multiplier one `Heartbeat` tick applies to a club's `intrinsic_value`,
/// given a symmetric shock drawn from `-magnitude..magnitude`.
///
/// `exp(shock)`, not `1.0 + shock` - the two look interchangeable for a tiny
/// shock but aren't: for any symmetric `X` around zero, `E[ln(1+X)] < 0`
/// (Jensen's inequality applied to the concave `ln`), while `E[ln(exp(X))]
/// = E[X] = 0` exactly, by construction. Compounded across ~1.3M ticks/month
/// at this engine's 2s heartbeat interval, the old `1.0 + shock` formula
/// produced a real, systematic ~-5%/month downward drift for *every* club
/// regardless of actual performance - confirmed both analytically (the
/// closed-form expectation of `ln(1+X)` for `X ~ Uniform(-a,a)`) and via a
/// 200-trial Monte Carlo. This is the same "volatility drag" that makes a
/// stock earning +10% one day and -10% the next end up net negative, not
/// flat - familiar from real finance, easy to miss when a random walk is
/// implemented as "just multiply by 1 plus a small percentage."
fn heartbeat_multiplier(shock: f64) -> Decimal {
    Decimal::from_f64_retain(shock.exp()).unwrap_or(dec!(1.0))
}

/// Converts a target standalone monthly (30-day) volatility for the
/// `Heartbeat` random walk into the per-tick uniform half-width the caller
/// should draw `MatchEvent::Heartbeat`'s shock from. This is what keeps the
/// walk's *magnitude* deliberately calibrated instead of an arbitrarily
/// chosen per-tick constant - the exact mistake that let the (now-fixed)
/// drift bug produce swings of ±50%+ a month on real production data,
/// unrelated to any club's actual performance (see AGENTS.md's "Known
/// gotchas"). "Standalone" matters: this is the volatility the walk would
/// produce with *zero* real match events mixed in - real events (goals,
/// cards, results) are expected to still dominate a club's actual month,
/// this just bounds how much of that month is pure noise.
///
/// Derivation: `Uniform(-a,a)` has variance `a²/3`, so one tick's std is
/// `a/sqrt(3)`. Because `heartbeat_multiplier` applies each shock on the
/// log scale (`exp(shock)`, driftless), and per-tick shocks are small
/// enough that summing them approximates the compounded log-return well,
/// `N` independent ticks have variance `N * a²/3`. Solving for `a` given a
/// target monthly variance:
/// ```text
/// target_monthly_std² = N_ticks_per_month * a² / 3
/// a = target_monthly_std * sqrt(3 / N_ticks_per_month)
/// ```
/// `N_ticks_per_month` is derived from the real heartbeat interval, not a
/// second hand-picked number - if the interval ever changes, the realized
/// volatility stays correctly calibrated to `target_monthly_std` rather
/// than silently drifting.
pub fn heartbeat_tick_half_width(target_monthly_std: f64, heartbeat_interval_secs: f64) -> f64 {
    const SECONDS_PER_MONTH: f64 = 30.0 * 24.0 * 3600.0;
    let ticks_per_month = SECONDS_PER_MONTH / heartbeat_interval_secs;
    target_monthly_std * (3.0 / ticks_per_month).sqrt()
}

// --- ID Newtypes ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ClubId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct PlayerId(pub u32);

// --- Core Valuation Types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C, u8)]
pub enum MatchEvent {
    Goal { team_id: ClubId, opponent_id: ClubId, player_id: PlayerId, minute: u32 },
    YellowCard { team_id: ClubId, opponent_id: ClubId, player_id: PlayerId },
    RedCard { team_id: ClubId, opponent_id: ClubId, player_id: PlayerId },
    Injury { player_id: PlayerId, severity: u32 },
    WhistleEnd { team_a_id: ClubId, team_b_id: ClubId },
    /// `tick_std_pct` is the uniform half-width the shock is drawn from -
    /// caller-supplied (see `heartbeat_tick_half_width`), not hardcoded
    /// here, so the walk's magnitude is a calibrated, product-level
    /// decision rather than an opaque constant nobody can reason about.
    Heartbeat { team_id: ClubId, tick_std_pct: f64 },
    /// A one-time, audited admin correction to `intrinsic_value` - see
    /// `admin_correction::AdminCorrectionFact` for why this exists and the
    /// isolation principle it preserves. `delta_pct` is applied
    /// multiplicatively against whatever the club's value is *right now*,
    /// not a value computed when the correction was requested.
    AdminCorrection { team_id: ClubId, delta_pct: Decimal },
}

// --- Position Modeling ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Position {
    GK = 0,
    CB, LB, RB, LWB, RWB,
    CDM, CM, CAM, LM, RM,
    ST, CF, LW, RW,
}

impl Position {
    pub fn base_weight(&self) -> Decimal {
        match self {
            Position::GK => dec!(1.00),
            Position::CB | Position::LB | Position::RB => dec!(0.90),
            Position::CAM | Position::ST | Position::CF => dec!(0.95),
            Position::LW | Position::RW => dec!(0.98),
            Position::LWB | Position::RWB | Position::LM | Position::RM => dec!(0.92),
            Position::CDM => dec!(0.99),
            Position::CM => dec!(0.94),
        }
    }
}

// --- State Definitions ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerValues {
    pub team_id: ClubId,
    pub intrinsic_value: Decimal,
    pub form_weight: Decimal,
    pub sentiment_score: Decimal,
    pub volatility_factor: Decimal,
    pub performance_history: Vec<Decimal>,
    pub position: Position,
    pub is_captain: bool,
    /// Soft-delete flag: false once a transfer moves this player out of our
    /// tracked leagues entirely. Never hard-deleted - `crate::transfers` can
    /// reactivate a returning player against this same record instead of
    /// creating a duplicate. `WhistleEnd`'s per-club recompute skips inactive
    /// players (see `process_event`), so a departed player's value simply
    /// freezes at whatever it was when they left.
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubState {
    pub id: ClubId,
    pub intrinsic_value: Decimal,
    pub last_match_update: u64,
    /// The value as of the last *real* event (Goal/Card/WhistleEnd) that
    /// touched this club - the anchor `apply_stale_club_reversion` pulls a
    /// long-quiet club's `intrinsic_value` back toward. `Option` (not a bare
    /// `Decimal` defaulting to zero) so that a Redis-persisted `ClubState`
    /// from before this field existed deserializes safely (`#[serde(default)]`
    /// gives `None`) instead of silently anchoring toward zero.
    #[serde(default)]
    pub resting_value: Option<Decimal>,
    pub top_oppositions: BTreeMap<ClubId, Decimal>,
    pub rivals: Vec<(ClubId, Decimal)>,
    pub player_ids: Vec<PlayerId>,
}

impl ClubState {
    pub fn new(id: ClubId) -> Self {
        Self {
            id,
            intrinsic_value: dec!(100.0),
            last_match_update: 0,
            resting_value: Some(dec!(100.0)),
            top_oppositions: BTreeMap::new(),
            rivals: Vec::new(),
            player_ids: Vec::new(),
        }
    }

    pub fn set_rival_factor(&mut self, opponent_id: ClubId, factor: Decimal) {
        if let Some(pos) = self.rivals.iter().position(|(id, _)| *id == opponent_id) {
            self.rivals[pos].1 = factor;
        } else {
            self.rivals.push((opponent_id, factor));
        }
    }

    pub fn set_opposition_factor(&mut self, opponent_id: ClubId, factor: Decimal) {
        self.top_oppositions.insert(opponent_id, factor);
    }
}

// --- Engine Update Notification ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineUpdate {
    Player { id: PlayerId, state: PlayerValues },
    Club { id: ClubId, state: ClubState },
    Event { event: MatchEvent, ts: u64 },
    CandleUpdate { 
        base_id: ClubId, 
        quote_id: ClubId, 
        open: f64, 
        high: f64, 
        low: f64, 
        close: f64, 
        ts: u64 
    },
}

// --- Engine Implementation ---

pub struct ValuationEngine {
    pub player_states: Arc<DashMap<PlayerId, PlayerValues>>,
    pub club_states: Arc<DashMap<ClubId, ClubState>>,
    pub names: Arc<DashMap<u32, String>>, // Keep name lookup as u32 for now, or consider generic
    pub is_transfer_window: std::sync::atomic::AtomicBool,
    /// Next id handed to a player created by `transfers::process_transfer`
    /// for a signing from outside our tracked universe. See
    /// `transfers::DYNAMIC_PLAYER_ID_BASE` for why this starts where it does.
    pub next_dynamic_player_id: AtomicU32,
    pub update_tx: Option<mpsc::UnboundedSender<EngineUpdate>>,
}

impl ValuationEngine {
    pub fn new() -> Self {
        Self {
            player_states: Arc::new(DashMap::with_capacity(2000)),
            club_states: Arc::new(DashMap::with_capacity(200)),
            names: Arc::new(DashMap::with_capacity(2200)),
            is_transfer_window: std::sync::atomic::AtomicBool::new(false),
            next_dynamic_player_id: transfers::new_dynamic_player_id_counter(),
            update_tx: None,
        }
    }

    pub fn set_update_channel(&mut self, tx: mpsc::UnboundedSender<EngineUpdate>) {
        self.update_tx = Some(tx);
    }

    fn notify(&self, update: EngineUpdate) {
        if let Some(tx) = &self.update_tx {
            let _ = tx.send(update);
        }
    }

    pub fn update_financial_metric(&self, is_window: bool) {
        self.is_transfer_window.store(is_window, std::sync::atomic::Ordering::Relaxed);
    }

    /// Gently pulls a long-quiet club's `intrinsic_value` back toward
    /// `resting_value` (its value as of the last *real* event) rather than
    /// leaving it to whatever the heartbeat's random walk has drifted it to.
    /// Deliberately a *reversion*, not a one-directional decay: the original
    /// version of this always multiplied the value down, which - once a club
    /// goes quiet for a week - becomes a free, predictable one-way bet
    /// (short everything in the off-season). Pulling toward the last known
    /// real anchor can move the price either way depending on which side of
    /// it the heartbeat has wandered to, so there's no guaranteed direction
    /// to farm.
    ///
    /// Closes a fixed fraction of the *current* gap each call rather than
    /// computing an elapsed-time decay curve - simpler, self-correcting
    /// (works regardless of how the gap got there, including heartbeat noise
    /// still perturbing it between calls), and only meaningful given the
    /// assumption (documented, not enforced here) that the caller invokes
    /// this on a steady cadence. `PULL_FRACTION` is tuned for an hourly
    /// caller: ~50% of the gap closes over 2 weeks of continued inactivity.
    /// Never touches `last_match_update` itself - only a real event does
    /// that - so this stays idempotent no matter how often it's called.
    pub fn apply_stale_club_reversion(&self, current_ts: u64) {
        const SEVEN_DAYS_SECS: u64 = 7 * 24 * 60 * 60;
        const PULL_FRACTION: Decimal = dec!(0.002);

        for mut club in self.club_states.iter_mut() {
            if current_ts <= club.last_match_update + SEVEN_DAYS_SECS {
                continue;
            }
            let Some(resting_value) = club.resting_value else { continue };
            let gap = resting_value - club.intrinsic_value;
            if gap == Decimal::ZERO {
                continue;
            }
            club.intrinsic_value += gap * PULL_FRACTION;
            self.notify(EngineUpdate::Club { id: club.id, state: club.clone() });
        }
    }

    pub fn get_club_pair_exchange_rate(&self, base_id: ClubId, quote_id: ClubId) -> Decimal {
        let v_base = self.club_states.get(&base_id).map(|c| c.intrinsic_value).unwrap_or(dec!(0));
        let v_quote = self.club_states.get(&quote_id).map(|c| c.intrinsic_value).unwrap_or(dec!(1));
        if v_quote.is_zero() { dec!(0) } else { v_base / v_quote }
    }

    pub fn process_event(&self, event: MatchEvent, current_ts: u64) {
        self.notify(EngineUpdate::Event { event: event.clone(), ts: current_ts });
        
        match event {
            MatchEvent::Goal { team_id, opponent_id, player_id, minute } => {
                let scorer_rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
                let time_multiplier = Decimal::from(minute) / dec!(90.0);
                // Percent of the club's *current* value, not a flat point
                // add - a flat number meant identical events were worth
                // wildly different relative moves depending purely on a
                // club's absolute valuation (confirmed live: ~0.7% for a
                // ~2,250-value club vs. ~5.6% for a ~270-value one, same
                // goal). `value += value * pct` also can't drive a value
                // negative the way repeated flat subtractions could.
                let base_impact_pct = dec!(0.05) + (dec!(0.10) * time_multiplier);

                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    let impact_pct = base_impact_pct * scorer_rivalry_multiplier;
                    let base_value = club.intrinsic_value;
                    club.intrinsic_value += base_value * impact_pct;
                    club.last_match_update = current_ts;
                    club.resting_value = Some(club.intrinsic_value);
                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone() });
                }
                // Conceding is real, negative signal too - a goal used to
                // only ever pump the scorer and leave the conceding club
                // completely untouched, which meant a win had unbounded
                // upside with no symmetric downside for the loss (confirmed
                // live: a single lopsided result pushed one club to ~2x the
                // next-highest valuation in the entire game). Mirrors the
                // scorer's own move exactly - same base formula, opposite
                // sign - scaled by the *conceding* club's own rivalry factor
                // toward the scorer, not the scorer's factor toward them,
                // since each side's reaction is sized by how much *they*
                // care about this fixture.
                let conceder_rivalry_multiplier = self.get_rivalry_multiplier(opponent_id, team_id);
                if let Some(mut opponent_club) = self.club_states.get_mut(&opponent_id) {
                    let impact_pct = base_impact_pct * conceder_rivalry_multiplier;
                    let base_value = opponent_club.intrinsic_value;
                    opponent_club.intrinsic_value -= base_value * impact_pct;
                    opponent_club.last_match_update = current_ts;
                    opponent_club.resting_value = Some(opponent_club.intrinsic_value);
                    self.notify(EngineUpdate::Club { id: opponent_id, state: opponent_club.clone() });
                }
                if let Some(mut player) = self.player_states.get_mut(&player_id) {
                    let pos_multiplier = player.position.base_weight();
                    let player_impact_pct = dec!(0.15) * pos_multiplier * player.form_weight * scorer_rivalry_multiplier;
                    let base_value = player.intrinsic_value;
                    player.intrinsic_value += base_value * player_impact_pct;
                    self.notify(EngineUpdate::Player { id: player_id, state: player.clone() });
                }
            }
            MatchEvent::RedCard { team_id, opponent_id, player_id } => {
                let carded_rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    let base_value = club.intrinsic_value;
                    club.intrinsic_value -= base_value * dec!(0.075) * carded_rivalry_multiplier;
                    club.last_match_update = current_ts;
                    club.resting_value = Some(club.intrinsic_value);
                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone() });
                }
                // A red card against the opponent is real positive signal
                // too - down a player is a genuine disadvantage for them,
                // which reads as good news for the other side. Mirrors the
                // carded club's own move (same magnitude, opposite sign),
                // scaled by the *benefiting* club's own rivalry factor
                // toward the carded team.
                let benefiting_rivalry_multiplier = self.get_rivalry_multiplier(opponent_id, team_id);
                if let Some(mut opponent_club) = self.club_states.get_mut(&opponent_id) {
                    let base_value = opponent_club.intrinsic_value;
                    opponent_club.intrinsic_value += base_value * dec!(0.075) * benefiting_rivalry_multiplier;
                    opponent_club.last_match_update = current_ts;
                    opponent_club.resting_value = Some(opponent_club.intrinsic_value);
                    self.notify(EngineUpdate::Club { id: opponent_id, state: opponent_club.clone() });
                }
                if let Some(mut player) = self.player_states.get_mut(&player_id) {
                    player.form_weight *= dec!(0.5);
                    self.notify(EngineUpdate::Player { id: player_id, state: player.clone() });
                }
            }
            MatchEvent::YellowCard { team_id, opponent_id, player_id } => {
                let carded_rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    let base_value = club.intrinsic_value;
                    club.intrinsic_value -= base_value * dec!(0.025) * carded_rivalry_multiplier;
                    club.last_match_update = current_ts;
                    club.resting_value = Some(club.intrinsic_value);
                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone() });
                }
                // Same mirrored-benefit reasoning as RedCard above, scaled
                // to the yellow-card magnitude.
                let benefiting_rivalry_multiplier = self.get_rivalry_multiplier(opponent_id, team_id);
                if let Some(mut opponent_club) = self.club_states.get_mut(&opponent_id) {
                    let base_value = opponent_club.intrinsic_value;
                    opponent_club.intrinsic_value += base_value * dec!(0.025) * benefiting_rivalry_multiplier;
                    opponent_club.last_match_update = current_ts;
                    opponent_club.resting_value = Some(opponent_club.intrinsic_value);
                    self.notify(EngineUpdate::Club { id: opponent_id, state: opponent_club.clone() });
                }
                if let Some(mut player) = self.player_states.get_mut(&player_id) {
                    player.form_weight *= dec!(0.9);
                    self.notify(EngineUpdate::Player { id: player_id, state: player.clone() });
                }
            }
            MatchEvent::Injury { player_id, severity } => {
                if let Some(mut player) = self.player_states.get_mut(&player_id) {
                    let loss = player.intrinsic_value * (Decimal::from(severity) / dec!(100.0));
                    player.intrinsic_value -= loss;
                    self.notify(EngineUpdate::Player { id: player_id, state: player.clone() });
                }
            }
            MatchEvent::WhistleEnd { team_a_id, team_b_id } => {
                let is_window = self.is_transfer_window.load(std::sync::atomic::Ordering::Relaxed);
                for &team_id in &[team_a_id, team_b_id] {
                    if let Some(mut club) = self.club_states.get_mut(&team_id) {
                        club.last_match_update = current_ts;
                        club.resting_value = Some(club.intrinsic_value);
                        self.notify(EngineUpdate::Club { id: team_id, state: club.clone() });
                    }
                    
                        let pids: Vec<PlayerId> = self.player_states.iter()
                        .filter(|entry| entry.value().team_id == team_id && entry.value().active)
                        .map(|entry| *entry.key())
                        .collect();

                    for pid in pids {
                        if let Some(mut player) = self.player_states.get_mut(&pid) {
                            player.volatility_factor = dec!(1.0);

                            // form_weight/sentiment_score are multipliers everywhere else
                            // in this engine (see the Goal-impact formula above) - they were
                            // never meant to be blended additively with intrinsic_value,
                            // which lives on a completely different, unbounded-growing scale.
                            // Blending them as comparable terms (the old formula) silently
                            // divided a player's value by ~3-10x on every single match.
                            // `is_window` still lets a transfer window make current form
                            // matter more, but now as an amplifier on the multiplier's
                            // distance from neutral (1.0), not a weight on mismatched units.
                            let form_sensitivity = if is_window { dec!(2.0) } else { dec!(0.5) };
                            let form_multiplier = dec!(1.0) + (player.form_weight - dec!(1.0)) * form_sensitivity;
                            let v = player.intrinsic_value * form_multiplier * player.sentiment_score;

                            player.performance_history.push(v);
                            if player.performance_history.len() > 10 {
                                player.performance_history.remove(0);
                            }

                            let sum: Decimal = player.performance_history.iter().sum();
                            player.intrinsic_value = sum / Decimal::from(player.performance_history.len());
                            self.notify(EngineUpdate::Player { id: pid, state: player.clone() });
                        }
                    }
                }
            }
            MatchEvent::Heartbeat { team_id, tick_std_pct } => {
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    club.last_match_update = current_ts;

                    // Live Market Simulation: driftless geometric random walk -
                    // see heartbeat_multiplier's own comment for why this must
                    // be exp(shock), not 1.0 + shock, and
                    // heartbeat_tick_half_width's for where tick_std_pct comes
                    // from (a calibrated target, not a hand-picked constant).
                    // `gen_range` panics on an empty range, so a misconfigured
                    // (or deliberately zero) volatility target skips the draw
                    // entirely rather than crashing this tick for every club -
                    // zero volatility legitimately means "no noise," not an
                    // error.
                    if tick_std_pct > 0.0 {
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        let shock: f64 = rng.gen_range(-tick_std_pct..tick_std_pct);
                        club.intrinsic_value *= heartbeat_multiplier(shock);
                    }

                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone() });
                }
            }
            MatchEvent::AdminCorrection { team_id, delta_pct } => {
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    club.intrinsic_value *= dec!(1.0) + delta_pct / dec!(100.0);
                    club.last_match_update = current_ts;
                    // Same reasoning as every real event handler above -
                    // anchor resting_value to the corrected value so
                    // apply_stale_club_reversion has no stale target to
                    // fight this back toward.
                    club.resting_value = Some(club.intrinsic_value);
                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone() });
                }
            }
        }
    }

    fn get_rivalry_multiplier(&self, team_id: ClubId, opponent_id: ClubId) -> Decimal {
        self.club_states.get(&team_id)
            .and_then(|club| {
                club.rivals.iter()
                    .find(|(rid, _)| *rid == opponent_id)
                    .map(|(_, multiplier)| *multiplier)
            })
            .unwrap_or(dec!(1.0))
    }
}

#[cfg(test)]
mod event_impact_tests {
    use super::*;

    /// Two clubs, one much smaller than the other, each with a single
    /// player - lets a test apply the exact same event to both and compare
    /// *relative* change, which is the one behavior this whole change
    /// exists to fix (a flat point value made the same real event worth
    /// wildly different percentages depending purely on a club's absolute
    /// valuation - confirmed live: ~0.7% for a ~2,250-value club vs. ~5.6%
    /// for a ~270-value one, same goal).
    fn engine_with_two_clubs(small_value: Decimal, big_value: Decimal) -> (ValuationEngine, ClubId, PlayerId, ClubId, PlayerId) {
        let engine = ValuationEngine::new();
        let small_club = ClubId(1);
        let big_club = ClubId(2);
        let opponent = ClubId(3);

        let mut small = ClubState::new(small_club);
        small.intrinsic_value = small_value;
        engine.club_states.insert(small_club, small);

        let mut big = ClubState::new(big_club);
        big.intrinsic_value = big_value;
        engine.club_states.insert(big_club, big);

        engine.club_states.insert(opponent, ClubState::new(opponent));

        let small_player = PlayerId(1);
        let big_player = PlayerId(2);
        for (pid, club, value) in [(small_player, small_club, small_value), (big_player, big_club, big_value)] {
            engine.player_states.insert(
                pid,
                PlayerValues {
                    team_id: club,
                    intrinsic_value: value,
                    form_weight: dec!(1.0),
                    sentiment_score: dec!(1.0),
                    volatility_factor: dec!(1.0),
                    performance_history: vec![value; 5],
                    position: Position::CM,
                    is_captain: false,
                    active: true,
                },
            );
        }
        (engine, small_club, small_player, big_club, big_player)
    }

    #[test]
    fn goal_impact_scales_with_current_club_value_not_flat() {
        let (engine, small_club, small_player, big_club, big_player) = engine_with_two_clubs(dec!(270.0), dec!(2250.0));

        engine.process_event(MatchEvent::Goal { team_id: small_club, opponent_id: ClubId(3), player_id: small_player, minute: 60 }, 0);
        engine.process_event(MatchEvent::Goal { team_id: big_club, opponent_id: ClubId(3), player_id: big_player, minute: 60 }, 0);

        let small_after = engine.club_states.get(&small_club).unwrap().intrinsic_value;
        let big_after = engine.club_states.get(&big_club).unwrap().intrinsic_value;

        let small_pct = (small_after - dec!(270.0)) / dec!(270.0);
        let big_pct = (big_after - dec!(2250.0)) / dec!(2250.0);

        assert_eq!(small_pct, big_pct, "identical goal (same minute, no rivalry) must move both clubs by the same relative percentage, regardless of their absolute value - got {small_pct} vs {big_pct}");
        // Sanity: this is the real fix - the same goal used to be ~5.6% for
        // the small club and ~0.7% for the big one. Now both get the same
        // ~11.7% (0.05 + 0.10 * 60/90).
        assert!(small_pct > dec!(0.11) && small_pct < dec!(0.12), "expected ~11.7% impact, got {small_pct}");
    }

    #[test]
    fn goal_impact_scales_with_current_player_value_not_flat() {
        let (engine, small_club, small_player, big_club, big_player) = engine_with_two_clubs(dec!(270.0), dec!(2250.0));
        // Player intrinsic_value was seeded equal to club value in the
        // helper purely for setup convenience - only the *relative* move
        // matters for this assertion, not any relationship to club value.
        let small_before = engine.player_states.get(&small_player).unwrap().intrinsic_value;
        let big_before = engine.player_states.get(&big_player).unwrap().intrinsic_value;

        engine.process_event(MatchEvent::Goal { team_id: small_club, opponent_id: ClubId(3), player_id: small_player, minute: 60 }, 0);
        engine.process_event(MatchEvent::Goal { team_id: big_club, opponent_id: ClubId(3), player_id: big_player, minute: 60 }, 0);

        let small_after = engine.player_states.get(&small_player).unwrap().intrinsic_value;
        let big_after = engine.player_states.get(&big_player).unwrap().intrinsic_value;

        let small_pct = (small_after - small_before) / small_before;
        let big_pct = (big_after - big_before) / big_before;
        assert_eq!(small_pct, big_pct, "identical goal must move both scorers by the same relative percentage - got {small_pct} vs {big_pct}");
    }

    #[test]
    fn red_card_reduces_club_value_by_the_expected_percentage() {
        let (engine, small_club, small_player, _big_club, _big_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        engine.process_event(MatchEvent::RedCard { team_id: small_club, opponent_id: ClubId(3), player_id: small_player }, 0);
        let after = engine.club_states.get(&small_club).unwrap().intrinsic_value;
        assert_eq!(after, dec!(1000.0) * (dec!(1) - dec!(0.075)), "expected exactly a 7.5% reduction, got {after}");
    }

    #[test]
    fn yellow_card_reduces_club_value_by_the_expected_percentage() {
        let (engine, small_club, small_player, _big_club, _big_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        engine.process_event(MatchEvent::YellowCard { team_id: small_club, opponent_id: ClubId(3), player_id: small_player }, 0);
        let after = engine.club_states.get(&small_club).unwrap().intrinsic_value;
        assert_eq!(after, dec!(1000.0) * (dec!(1) - dec!(0.025)), "expected exactly a 2.5% reduction, got {after}");
    }

    #[test]
    fn goal_also_moves_the_conceding_club_down_by_the_mirrored_percentage() {
        // A goal used to only ever pump the scorer, leaving the conceding
        // club untouched - which meant wins had unbounded upside with no
        // symmetric downside for a loss (confirmed live: a single lopsided
        // result pushed one club to ~2x the next-highest valuation in the
        // entire game). This is the direct fix: the conceding club moves
        // down by the same formula, opposite sign.
        let (engine, scorer, scorer_player, conceder, _conceder_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        engine.process_event(MatchEvent::Goal { team_id: scorer, opponent_id: conceder, player_id: scorer_player, minute: 60 }, 0);

        let scorer_after = engine.club_states.get(&scorer).unwrap().intrinsic_value;
        let conceder_after = engine.club_states.get(&conceder).unwrap().intrinsic_value;

        // No rivalry configured between these two, so both sides use a 1.0x
        // multiplier - the move should be an exact mirror image.
        let expected_pct = dec!(0.05) + (dec!(0.10) * dec!(60.0) / dec!(90.0));
        assert_eq!(scorer_after, dec!(1000.0) * (dec!(1) + expected_pct), "scorer should be up by exactly {expected_pct}, got {scorer_after}");
        assert_eq!(conceder_after, dec!(1000.0) * (dec!(1) - expected_pct), "conceder should be down by exactly {expected_pct}, got {conceder_after}");
    }

    #[test]
    fn card_also_moves_the_benefiting_opponent_up_by_the_mirrored_percentage() {
        // Symmetric counterpart to the goal test above, for both card types
        // - going down a player is a real disadvantage for the carded club,
        // which should read as real (if small) good news for the opponent,
        // not nothing.
        let (engine, carded, carded_player, benefiting, _benefiting_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        engine.process_event(MatchEvent::RedCard { team_id: carded, opponent_id: benefiting, player_id: carded_player }, 0);

        let carded_after = engine.club_states.get(&carded).unwrap().intrinsic_value;
        let benefiting_after = engine.club_states.get(&benefiting).unwrap().intrinsic_value;

        assert_eq!(carded_after, dec!(1000.0) * (dec!(1) - dec!(0.075)), "carded club should be down 7.5%, got {carded_after}");
        assert_eq!(benefiting_after, dec!(1000.0) * (dec!(1) + dec!(0.075)), "benefiting opponent should be up 7.5%, got {benefiting_after}");
    }

    #[test]
    fn mirrored_impact_uses_each_sides_own_rivalry_factor_not_a_shared_one() {
        // The scorer's own configured rivalry factor toward the opponent
        // must not leak into the opponent's side of the move - each club's
        // reaction is sized by *its own* configured feeling about this
        // fixture, which real seed data isn't always symmetric about.
        let (engine, scorer, scorer_player, conceder, _conceder_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        {
            let mut scorer_state = engine.club_states.get_mut(&scorer).unwrap();
            scorer_state.set_rival_factor(conceder, dec!(2.0)); // scorer cares a lot about this fixture
        }
        // conceder has no configured factor toward scorer - stays at the 1.0x default.

        engine.process_event(MatchEvent::Goal { team_id: scorer, opponent_id: conceder, player_id: scorer_player, minute: 60 }, 0);

        let scorer_after = engine.club_states.get(&scorer).unwrap().intrinsic_value;
        let conceder_after = engine.club_states.get(&conceder).unwrap().intrinsic_value;

        let base_pct = dec!(0.05) + (dec!(0.10) * dec!(60.0) / dec!(90.0));
        assert_eq!(scorer_after, dec!(1000.0) * (dec!(1) + base_pct * dec!(2.0)), "scorer's own 2.0x rivalry factor should apply to its own move");
        assert_eq!(conceder_after, dec!(1000.0) * (dec!(1) - base_pct), "conceder has no configured factor, so its move should stay at the 1.0x default, not inherit the scorer's 2.0x");
    }
}

#[cfg(test)]
mod whistle_end_tests {
    use super::*;

    fn engine_with_one_player(intrinsic_value: Decimal, form_weight: Decimal) -> (ValuationEngine, ClubId, PlayerId) {
        let engine = ValuationEngine::new();
        let club_id = ClubId(1);
        engine.club_states.insert(club_id, ClubState::new(club_id));

        let player_id = PlayerId(1);
        engine.player_states.insert(
            player_id,
            PlayerValues {
                team_id: club_id,
                intrinsic_value,
                form_weight,
                sentiment_score: dec!(1.0),
                volatility_factor: dec!(1.0),
                performance_history: vec![intrinsic_value; 5],
                position: Position::CM,
                is_captain: false,
                active: true,
            },
        );
        (engine, club_id, player_id)
    }

    /// A player at neutral form/sentiment (both 1.0) should stay essentially
    /// flat across repeated match-ends with no goals/cards in between - this
    /// is the exact regression the old additive-blend formula failed: it
    /// divided intrinsic_value toward zero every single WhistleEnd
    /// regardless of form, because it blended an unbounded absolute value
    /// with two multipliers pinned near 1.0 as if they were the same scale.
    #[test]
    fn neutral_form_does_not_decay_value_over_repeated_matches() {
        let (engine, club_id, player_id) = engine_with_one_player(dec!(65.0), dec!(1.0));
        for ts in 0..10u64 {
            engine.process_event(MatchEvent::WhistleEnd { team_a_id: club_id, team_b_id: club_id }, ts);
        }
        let value = engine.player_states.get(&player_id).unwrap().intrinsic_value;
        assert!(value > dec!(60.0), "expected value to stay near 65.0, got {value}");
    }

    /// Degraded form (e.g. after a red card halves form_weight to 0.5)
    /// should pull the value down proportionally, not collapse it toward a
    /// tiny fraction of its original scale.
    #[test]
    fn degraded_form_pulls_value_down_proportionally_not_to_near_zero() {
        let (engine, club_id, player_id) = engine_with_one_player(dec!(65.0), dec!(0.5));
        engine.process_event(MatchEvent::WhistleEnd { team_a_id: club_id, team_b_id: club_id }, 0);
        let value = engine.player_states.get(&player_id).unwrap().intrinsic_value;
        assert!(value > dec!(30.0), "expected a proportional pull-down, not a collapse - got {value}");
        assert!(value < dec!(65.0), "degraded form should still pull the value down some - got {value}");
    }
}

#[cfg(test)]
mod heartbeat_drift_tests {
    use super::*;

    #[test]
    fn zero_shock_is_the_identity() {
        assert_eq!(heartbeat_multiplier(0.0), dec!(1.0));
    }

    /// The property the old `1.0 + shock` formula got wrong: an up-tick and
    /// its exact mirror-image down-tick should cancel out to the identity.
    /// `(1.0 + a) * (1.0 - a) = 1.0 - a²` - strictly less than 1, which is
    /// exactly the source of the drift. `exp(a) * exp(-a) = 1.0` exactly.
    #[test]
    fn a_shock_and_its_mirror_image_cancel_out_exactly() {
        let a = 0.0005;
        let round_trip = heartbeat_multiplier(a) * heartbeat_multiplier(-a);
        let diff = (round_trip - dec!(1.0)).abs();
        assert!(diff < dec!(0.0000000001), "expected an up/down pair to net to ~1.0 exactly, got {round_trip}");
    }

    /// Direct regression guard: the fixed multiplier must land measurably
    /// closer to 1.0 than the old, biased `1.0 + shock` formula would have,
    /// for the same shock - proof this isn't just algebraically different
    /// but actually less biased in the specific case that mattered (a
    /// negative shock, where the old formula's downward pull compounds).
    #[test]
    fn fixed_multiplier_is_less_biased_than_the_old_formula_for_a_negative_shock() {
        let a = -0.0005;
        let old_buggy_multiplier = dec!(1.0) + Decimal::from_f64_retain(a).unwrap();
        let fixed_multiplier = heartbeat_multiplier(a);
        assert!(
            fixed_multiplier > old_buggy_multiplier,
            "exp(shock) should exceed 1.0+shock for a negative shock (exp is convex) - old={old_buggy_multiplier}, fixed={fixed_multiplier}"
        );
    }

    /// The actual bug's real-world consequence, demonstrated directly:
    /// compounding the *old* formula over many ticks with perfectly
    /// symmetric shocks (an idealized fair coin flip, no real randomness)
    /// still nets a loss - proving the drift is structural, not a
    /// statistical fluke of any particular random seed. The *fixed*
    /// formula, given the same symmetric shock sequence, nets to exactly
    /// 1.0.
    #[test]
    fn symmetric_alternating_shocks_are_flat_under_the_fix_but_werent_before() {
        let a = 0.0005;
        let mut old_value = dec!(1.0);
        let mut fixed_value = dec!(1.0);
        for i in 0..1000 {
            let shock = if i % 2 == 0 { a } else { -a };
            let old_buggy_multiplier = dec!(1.0) + Decimal::from_f64_retain(shock).unwrap();
            old_value *= old_buggy_multiplier;
            fixed_value *= heartbeat_multiplier(shock);
        }
        assert!(old_value < dec!(0.9999), "expected the old formula to have drifted measurably down, got {old_value}");
        let fixed_diff = (fixed_value - dec!(1.0)).abs();
        assert!(fixed_diff < dec!(0.0000001), "expected the fixed formula to stay flat under symmetric shocks, got {fixed_value}");
    }
}

#[cfg(test)]
mod heartbeat_calibration_tests {
    use super::*;

    /// Round-trips the derivation: given the half-width `a` this function
    /// returns for a target, compounding N ticks' worth of that magnitude's
    /// variance should reproduce the target's variance - the actual
    /// mathematical property that makes "target monthly volatility" a
    /// meaningful, honest parameter rather than just a differently-shaped
    /// magic number.
    #[test]
    fn compounding_the_derived_half_width_reproduces_the_target_variance() {
        let target = 0.05; // 5% monthly
        let interval_secs = 2.0;
        let a = heartbeat_tick_half_width(target, interval_secs);

        let ticks_per_month = 30.0 * 24.0 * 3600.0 / interval_secs;
        let per_tick_variance = (a / 3f64.sqrt()).powi(2);
        let compounded_variance = ticks_per_month * per_tick_variance;
        let implied_monthly_std = compounded_variance.sqrt();

        let diff = (implied_monthly_std - target).abs();
        assert!(diff < 1e-9, "expected compounding the derived half-width to reproduce the {target} target, got {implied_monthly_std}");
    }

    /// The actual production default (5% monthly @ 2s) should land far
    /// below the old, uncalibrated 0.0005 (~33% monthly, see AGENTS.md's
    /// "Known gotchas") - the whole point of this function.
    #[test]
    fn default_calibration_is_far_smaller_than_the_old_uncalibrated_constant() {
        let a = heartbeat_tick_half_width(0.05, 2.0);
        assert!(a < 0.0005, "expected the calibrated half-width to be well under the old hardcoded 0.0005, got {a}");
    }

    /// Halving the interval doubles ticks/month, so - to hold the same
    /// target monthly volatility - the per-tick half-width must shrink by
    /// sqrt(2), not stay fixed. This is precisely the coupling that was
    /// missing before: the interval and the magnitude must move together.
    #[test]
    fn a_faster_interval_requires_a_smaller_half_width_for_the_same_target() {
        let target = 0.05;
        let a_slow = heartbeat_tick_half_width(target, 2.0);
        let a_fast = heartbeat_tick_half_width(target, 1.0); // half the interval
        let ratio = a_slow / a_fast;
        assert!((ratio - 2f64.sqrt()).abs() < 1e-9, "expected a sqrt(2) ratio, got {ratio}");
    }

    #[test]
    fn zero_target_volatility_gives_zero_half_width() {
        assert_eq!(heartbeat_tick_half_width(0.0, 2.0), 0.0);
    }
}

#[cfg(test)]
mod admin_correction_tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn engine_with_club(intrinsic_value: Decimal) -> (ValuationEngine, ClubId) {
        let engine = ValuationEngine::new();
        let club_id = ClubId(1);
        let mut club = ClubState::new(club_id);
        club.intrinsic_value = intrinsic_value;
        club.resting_value = Some(intrinsic_value);
        engine.club_states.insert(club_id, club);
        (engine, club_id)
    }

    #[test]
    fn positive_delta_increases_value_by_the_exact_percentage() {
        let (engine, club_id) = engine_with_club(dec!(1000.0));
        engine.process_event(MatchEvent::AdminCorrection { team_id: club_id, delta_pct: dec!(25.0) }, 0);
        let club = engine.club_states.get(&club_id).unwrap();
        assert_eq!(club.intrinsic_value, dec!(1250.0));
    }

    #[test]
    fn negative_delta_decreases_value_by_the_exact_percentage() {
        let (engine, club_id) = engine_with_club(dec!(1000.0));
        engine.process_event(MatchEvent::AdminCorrection { team_id: club_id, delta_pct: dec!(-25.0) }, 0);
        let club = engine.club_states.get(&club_id).unwrap();
        assert_eq!(club.intrinsic_value, dec!(750.0));
    }

    /// The property that actually matters operationally: without this, the
    /// very next `apply_stale_club_reversion` pass would see a gap between
    /// the (now-stale) resting_value and the corrected intrinsic_value and
    /// start pulling the correction straight back toward the pre-correction
    /// number - silently undoing the fix it was meant to apply.
    #[test]
    fn resting_value_is_anchored_to_the_corrected_value_not_left_stale() {
        let (engine, club_id) = engine_with_club(dec!(1000.0));
        engine.process_event(MatchEvent::AdminCorrection { team_id: club_id, delta_pct: dec!(25.0) }, 0);
        let club = engine.club_states.get(&club_id).unwrap();
        assert_eq!(club.resting_value, Some(dec!(1250.0)));
    }

    #[test]
    fn last_match_update_is_stamped_with_the_correction_timestamp() {
        let (engine, club_id) = engine_with_club(dec!(1000.0));
        engine.process_event(MatchEvent::AdminCorrection { team_id: club_id, delta_pct: dec!(10.0) }, 999_888);
        let club = engine.club_states.get(&club_id).unwrap();
        assert_eq!(club.last_match_update, 999_888);
    }

    #[test]
    fn an_unknown_club_id_is_a_harmless_no_op() {
        let engine = ValuationEngine::new();
        // No club_states entry for this id at all - must not panic.
        engine.process_event(MatchEvent::AdminCorrection { team_id: ClubId(999_999), delta_pct: dec!(50.0) }, 0);
    }
}

#[cfg(test)]
mod stale_club_reversion_tests {
    use super::*;

    const SEVEN_DAYS_SECS: u64 = 7 * 24 * 60 * 60;

    fn club_with(intrinsic_value: Decimal, resting_value: Decimal, last_match_update: u64) -> (ValuationEngine, ClubId) {
        let engine = ValuationEngine::new();
        let club_id = ClubId(1);
        let mut club = ClubState::new(club_id);
        club.intrinsic_value = intrinsic_value;
        club.resting_value = Some(resting_value);
        club.last_match_update = last_match_update;
        engine.club_states.insert(club_id, club);
        (engine, club_id)
    }

    /// The key fix: a club sitting *above* its resting anchor gets pulled
    /// down, proving this isn't the old one-directional decay.
    #[test]
    fn pulls_down_toward_resting_value_when_above_it() {
        let (engine, club_id) = club_with(dec!(120.0), dec!(100.0), 0);
        engine.apply_stale_club_reversion(SEVEN_DAYS_SECS + 3600);
        let value = engine.club_states.get(&club_id).unwrap().intrinsic_value;
        assert!(value < dec!(120.0) && value > dec!(100.0), "expected a partial pull down toward 100.0, got {value}");
    }

    /// And a club sitting *below* its resting anchor gets pulled up - this is
    /// the case the old always-multiply-down implementation could never do,
    /// and the whole reason it was a farmable, one-way bet during a long
    /// quiet spell.
    #[test]
    fn pulls_up_toward_resting_value_when_below_it() {
        let (engine, club_id) = club_with(dec!(80.0), dec!(100.0), 0);
        engine.apply_stale_club_reversion(SEVEN_DAYS_SECS + 3600);
        let value = engine.club_states.get(&club_id).unwrap().intrinsic_value;
        assert!(value > dec!(80.0) && value < dec!(100.0), "expected a partial pull up toward 100.0, got {value}");
    }

    #[test]
    fn does_nothing_within_the_seven_day_grace_period() {
        let (engine, club_id) = club_with(dec!(120.0), dec!(100.0), 0);
        engine.apply_stale_club_reversion(SEVEN_DAYS_SECS - 3600); // one hour short of the gate
        let value = engine.club_states.get(&club_id).unwrap().intrinsic_value;
        assert_eq!(value, dec!(120.0));
    }

    #[test]
    fn does_nothing_once_already_at_the_resting_value() {
        let (engine, club_id) = club_with(dec!(100.0), dec!(100.0), 0);
        engine.apply_stale_club_reversion(SEVEN_DAYS_SECS + 3600);
        let value = engine.club_states.get(&club_id).unwrap().intrinsic_value;
        assert_eq!(value, dec!(100.0));
    }

    /// Never touches `last_match_update` - repeated calls keep pulling
    /// (idempotent w.r.t. the gate), rather than the original bug where
    /// resetting it inside the function meant a fresh 7-day wait was needed
    /// before the *next* single dose of decay.
    #[test]
    fn repeated_calls_keep_converging_without_resetting_the_gate() {
        let (engine, club_id) = club_with(dec!(120.0), dec!(100.0), 0);
        let mut last_value = dec!(120.0);
        for _ in 0..5 {
            engine.apply_stale_club_reversion(SEVEN_DAYS_SECS + 3600);
            let value = engine.club_states.get(&club_id).unwrap().intrinsic_value;
            assert!(value < last_value, "expected continued convergence toward 100.0, got {value} after {last_value}");
            last_value = value;
        }
    }

    #[test]
    fn a_club_with_no_resting_value_yet_is_left_untouched() {
        let (engine, club_id) = club_with(dec!(120.0), dec!(100.0), 0);
        engine.club_states.get_mut(&club_id).unwrap().resting_value = None; // simulates a pre-migration Redis record
        engine.apply_stale_club_reversion(SEVEN_DAYS_SECS + 3600);
        let value = engine.club_states.get(&club_id).unwrap().intrinsic_value;
        assert_eq!(value, dec!(120.0));
    }
}

#[cfg(test)]
mod transfer_window_tests {
    use super::*;

    #[test]
    fn deep_summer_and_winter_are_open() {
        assert!(is_transfer_window_open(7, 15));
        assert!(is_transfer_window_open(1, 15));
    }

    #[test]
    fn deadline_days_are_the_last_open_day() {
        assert!(is_transfer_window_open(9, 1));
        assert!(!is_transfer_window_open(9, 2));
        assert!(is_transfer_window_open(2, 3));
        assert!(!is_transfer_window_open(2, 4));
    }

    #[test]
    fn mid_season_months_are_closed() {
        assert!(!is_transfer_window_open(3, 15));
        assert!(!is_transfer_window_open(10, 15));
        assert!(!is_transfer_window_open(5, 31));
    }

    #[test]
    fn window_opens_on_june_first() {
        assert!(is_transfer_window_open(6, 1));
    }
}