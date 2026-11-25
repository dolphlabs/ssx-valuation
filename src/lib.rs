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

/// Ceiling on `HEARTBEAT_NOISE_BAND` - a typo guard (`3` for `0.03`), not a
/// product limit.
pub const MAX_HEARTBEAT_NOISE_BAND: f64 = 0.25;

/// The heartbeat's noise: a deviation around what results say a club is
/// worth that keeps decaying back to zero (an Ornstein-Uhlenbeck process),
/// never a random walk. The walk this replaced compounded without limit:
/// at the 1.0-2.0/month settings production ran from Sep to Nov 2026, noise
/// accounted for ~96% of how clubs moved against each other, the spread
/// between the top and bottom club went from 8x to over 5,000x, and a club
/// could lose 73% over an international break without playing. Here the
/// price stays within a band of its fundamental however long a club goes
/// without a match, while still moving tick to tick.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HeartbeatNoise {
    /// Share of the current deviation each tick keeps: `0.5^(interval / half_life)`.
    pub keep: f64,
    /// Half-width of the uniform shock added on each tick.
    pub shock_half_width: f64,
}

impl HeartbeatNoise {
    /// `band` is the long-run std of `ln(price / fundamental)`; `half_life_secs`
    /// is how long a deviation takes to halve with no new shocks. A zero band
    /// still decays any deviation already there, so turning the noise off
    /// fades it out instead of freezing it into the price.
    ///
    /// Exact discretisation: `x' = keep * x + e` settles at variance
    /// `Var(e) / (1 - keep^2)`, and `Uniform(-w, w)` has variance `w^2 / 3`, so
    /// `w = band * sqrt(3 * (1 - keep^2))`.
    pub fn calibrate(band: f64, half_life_secs: f64, interval_secs: f64) -> Self {
        let band = if band.is_nan() { 0.0 } else { band.clamp(0.0, MAX_HEARTBEAT_NOISE_BAND) };
        let keep = if half_life_secs > 0.0 && interval_secs > 0.0 {
            0.5f64.powf(interval_secs / half_life_secs)
        } else {
            0.0
        };
        Self { keep, shock_half_width: band * (3.0 * (1.0 - keep * keep)).sqrt() }
    }

    /// The next deviation, from the current one and a uniform draw in `[-1, 1)`.
    pub fn step(&self, deviation: f64, unit_draw: f64) -> f64 {
        deviation * self.keep + unit_draw * self.shock_half_width
    }
}

/// Old results fade: a club's fundamental drifts back toward its anchor
/// (last season's value) so its distance halves every `half_life` with no
/// new events. Without it a club's value was a running total of every goal
/// it ever scored and conceded - the 2026/27 replay put Barcelona at 5.8x
/// its seed after 17 matches and the top/bottom spread at 99x, still
/// widening. With a 60-day half-life the same results level off at ~48x.
/// Every goal is worth the same to every club at the same age, so unlike a
/// cap that shrinks a leader's gains, the order clubs got there in doesn't
/// matter - only when.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResultFade {
    /// Share of the log distance from the anchor each tick keeps.
    pub keep: f64,
}

impl ResultFade {
    pub const NONE: Self = Self { keep: 1.0 };

    pub fn calibrate(half_life_secs: f64, interval_secs: f64) -> Self {
        if half_life_secs > 0.0 && interval_secs > 0.0 {
            Self { keep: 0.5f64.powf(interval_secs / half_life_secs) }
        } else {
            Self::NONE
        }
    }
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
    /// One tick of the club's mean-reverting noise - see `HeartbeatNoise`.
    /// Also takes the next step of a repricing in progress.
    Heartbeat { team_id: ClubId, noise: HeartbeatNoise, fade: ResultFade },
    /// An audited correction spread over `ticks` heartbeats instead of
    /// landing as one jump: moves the fundamental by `ln_delta` in total,
    /// in equal log steps (see `ClubState::repricing_ticks_remaining`).
    Repricing { team_id: ClubId, ln_delta: f64, ticks: u32 },
    /// A one-time, audited admin correction to `intrinsic_value` - see
    /// `admin_correction::AdminCorrectionFact` for why this exists and the
    /// isolation principle it preserves. `delta_pct` is applied
    /// multiplicatively against whatever the club's value is *right now*,
    /// not a value computed when the correction was requested.
    AdminCorrection { team_id: ClubId, delta_pct: Decimal },
    /// VAR took back a goal already applied: the same fields as the `Goal`
    /// it undoes, so the reversal can recompute exactly what it moved.
    GoalCancelled { team_id: ClubId, opponent_id: ClubId, player_id: PlayerId, minute: u32 },
    /// VAR overturned a red card already applied. Unlike a goal, a card's
    /// move depends on the disciplinary headroom left when it landed, so it
    /// can't be recomputed - it's undone from the carded club's own record
    /// of what that card actually applied (`ClubState::red_cards_this_match`).
    RedCardCancelled { team_id: ClubId, opponent_id: ClubId, player_id: PlayerId },
}

/// `value * (1 + pct)`, or its exact inverse - the one place a goal's moves
/// are applied, so applying and undoing can't drift apart.
fn scale(value: Decimal, pct: Decimal, undo: bool) -> Decimal {
    if undo { value / (Decimal::ONE + pct) } else { value + value * pct }
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
    /// The fundamental as of the last *real* event that touched this club -
    /// a record only since `ResultFade` replaced the 7-day reversion that
    /// used to pull quiet clubs back toward it. `Option` (not a bare
    /// `Decimal` defaulting to zero) so that a Redis-persisted `ClubState`
    /// from before this field existed deserializes safely (`#[serde(default)]`
    /// gives `None`) instead of silently anchoring toward zero.
    #[serde(default)]
    pub resting_value: Option<Decimal>,
    pub top_oppositions: BTreeMap<ClubId, Decimal>,
    pub rivals: Vec<(ClubId, Decimal)>,
    pub player_ids: Vec<PlayerId>,
    /// Net percentage this club has already gained/lost from card events
    /// (its own cards, and its opponent's) since the current match's
    /// kickoff - see `apply_capped_disciplinary_delta`. Reset to zero at
    /// `WhistleEnd`. `#[serde(default)]` for the same reason as
    /// `resting_value` above - a Redis-persisted `ClubState` from before
    /// this field existed deserializes to `0` (no accumulated impact yet),
    /// not a deploy-breaking error.
    #[serde(default)]
    pub disciplinary_impact_this_match: Decimal,
    /// Red cards this club received this match and exactly what each moved,
    /// so VAR can take one back (`RedCardCancelled`). Cleared at
    /// `WhistleEnd`; `default` for Redis-persisted states that predate it.
    #[serde(default)]
    pub red_cards_this_match: Vec<AppliedRedCard>,
    /// `ln(intrinsic_value / fundamental)`: how far the heartbeat has the
    /// price from what events made of the club. Every other move scales
    /// `intrinsic_value` multiplicatively, so dividing this back out always
    /// leaves the fundamental exactly.
    #[serde(default)]
    pub noise_log: f64,
    /// What's left of a `Repricing`: the log move still to apply and the
    /// heartbeats left to apply it over. While ticks remain, trading on the
    /// club is paused - the rest of the move is known in advance.
    #[serde(default)]
    pub repricing_ln_remaining: f64,
    /// What old results fade back toward (`ResultFade`): the club's value at
    /// the end of last season, i.e. its seed. `None` means no fading.
    #[serde(default)]
    pub anchor_value: Option<Decimal>,
    #[serde(default)]
    pub repricing_ticks_remaining: u32,
}

/// One applied red card, as `RedCardCancelled` needs it: the signed,
/// post-cap percentages actually applied to each side.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppliedRedCard {
    pub opponent_id: ClubId,
    pub player_id: PlayerId,
    pub carded_pct: Decimal,
    pub benefiting_pct: Decimal,
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
            disciplinary_impact_this_match: Decimal::ZERO,
            red_cards_this_match: Vec::new(),
            noise_log: 0.0,
            repricing_ln_remaining: 0.0,
            repricing_ticks_remaining: 0,
            anchor_value: None,
        }
    }

    pub fn is_repricing(&self) -> bool {
        self.repricing_ticks_remaining > 0
    }

    /// Moves the fundamental `1 - keep` of its log distance back toward the
    /// anchor; the noise on top is left as it is.
    pub fn apply_fade(&mut self, fade: ResultFade) {
        use rust_decimal::prelude::ToPrimitive;
        let Some(anchor) = self.anchor_value.filter(|a| *a > Decimal::ZERO) else { return };
        if fade.keep >= 1.0 {
            return;
        }
        let Some(ratio) = (self.fundamental_value() / anchor).to_f64().filter(|r| *r > 0.0) else { return };
        let offset = ratio.ln();
        let factor = heartbeat_multiplier(offset * (fade.keep - 1.0));
        self.intrinsic_value *= factor;
    }

    fn step_repricing(&mut self) {
        if self.repricing_ticks_remaining == 0 {
            return;
        }
        let step = self.repricing_ln_remaining / f64::from(self.repricing_ticks_remaining);
        let factor = heartbeat_multiplier(step);
        self.intrinsic_value *= factor;
        self.repricing_ticks_remaining -= 1;
        self.repricing_ln_remaining = if self.repricing_ticks_remaining == 0 { 0.0 } else { self.repricing_ln_remaining - step };
        self.anchor_resting_value();
    }

    /// What events alone make the club worth, with the heartbeat's noise taken out.
    pub fn fundamental_value(&self) -> Decimal {
        self.intrinsic_value / heartbeat_multiplier(self.noise_log)
    }

    /// Records the fundamental, not the noisy price, as of this event.
    pub(crate) fn anchor_resting_value(&mut self) {
        self.resting_value = Some(self.fundamental_value());
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

/// A goal's percentage move on the scorer (and, mirrored, the conceder):
/// `GOAL_BASE_IMPACT_PCT + GOAL_LATE_IMPACT_PCT * minute/90`, before the
/// rivalry multiplier. Named so read-only consumers (the terminal's risk
/// meter, via ssx-executor) quote the engine's own magnitude instead of a
/// hand-copied number that could drift from it.
pub const GOAL_BASE_IMPACT_PCT: Decimal = dec!(0.025);
pub const GOAL_LATE_IMPACT_PCT: Decimal = dec!(0.05);

/// One club's move for a goal at the average minute (45') with a neutral
/// rivalry multiplier - the "typical goal" the terminal sizes its risk
/// meter against. Descriptive only; `process_event` never reads it.
pub fn typical_goal_impact_pct() -> Decimal {
    GOAL_BASE_IMPACT_PCT + GOAL_LATE_IMPACT_PCT * dec!(0.5)
}

// --- Engine Update Notification ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineUpdate {
    Player { id: PlayerId, state: PlayerValues },
    /// `ts` is when this update was produced - deliberately *not* the same
    /// thing as `state.last_match_update` (which only advances on a real
    /// match event: Goal/Card/WhistleEnd/AdminCorrection/Transfer, never on
    /// ambient Heartbeat noise). Live price-history consumers
    /// (`ssx-node::storage`, `ssx-executor::pubsub`) need a timestamp that's
    /// always fresh, including for a club that hasn't played in weeks but is
    /// still getting heartbeat ticks - that's what this field is for.
    /// Carried on `EngineUpdate` itself (not `ClubState`) since this is only
    /// ever live-broadcast, never persisted, so it needs no serde-migration
    /// story the way `resting_value` did.
    Club { id: ClubId, state: ClubState, ts: u64 },
    Event { event: MatchEvent, ts: u64 },
    /// A live fact that has just been applied, with the display context
    /// (`minute`) that `MatchEvent` itself doesn't carry for cards - what
    /// the terminal's event chips/feed are built from. Purely descriptive:
    /// emitted *after* `process_event`, never read back by the engine.
    MatchEventApplied(live_match::AppliedMatchEvent),
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

    /// Lets the live-match subscriber broadcast an `AppliedMatchEvent` down
    /// the same update channel the storage worker already drains, so it's
    /// persisted and relayed in order with the club ticks it caused.
    pub fn announce_applied(&self, event: live_match::AppliedMatchEvent) {
        self.notify(EngineUpdate::MatchEventApplied(event));
    }

    fn notify(&self, update: EngineUpdate) {
        if let Some(tx) = &self.update_tx {
            let _ = tx.send(update);
        }
    }

    pub fn update_financial_metric(&self, is_window: bool) {
        self.is_transfer_window.store(is_window, std::sync::atomic::Ordering::Relaxed);
    }

    /// Fades every club by `fade` at once - the replay's stand-in for the
    /// heartbeats that would have run between two events.
    pub fn apply_result_fade(&self, fade: ResultFade) {
        for mut club in self.club_states.iter_mut() {
            club.apply_fade(fade);
        }
    }

    /// Hard ceiling on how much a single match's *disciplinary* events
    /// (a club's own cards, and its opponent's) can move a club's value,
    /// net, regardless of how many card events fire. Sized to roughly one
    /// average-minute goal's impact (`base_impact_pct` at minute 45 is
    /// 5%) - a card is a contributing signal, not the match outcome
    /// itself, so it should never be able to structurally outweigh the
    /// goals actually scored. Without this, per-event magnitude alone
    /// can't close the loophole: a sufficiently card-heavy match (a red
    /// plus several yellows isn't exotic) can still out-stack a single
    /// goal no matter how small each individual card's own weight is -
    /// this bounds the *cumulative* effect directly instead of trying to
    /// pick a per-event number small enough to survive an unbounded count
    /// of events, which no fixed per-event number can actually guarantee.
    /// Confirmed against a real incident (see the Sep 2026 Man City vs
    /// Man United match in `disciplinary_cap_tests` below): the club that
    /// lost 0-1 had gained +9.75% net from the winning side's 1 red + 3
    /// yellow cards before this cap existed - this constant is what turns
    /// that into a net loss instead, as losing a match should produce.
    const MAX_DISCIPLINARY_IMPACT_PCT_PER_MATCH: Decimal = dec!(0.06);

    /// Applies `proposed_pct` to `club`'s running per-match disciplinary
    /// accumulator, clamped to `[-MAX_DISCIPLINARY_IMPACT_PCT_PER_MATCH,
    /// +MAX_DISCIPLINARY_IMPACT_PCT_PER_MATCH]`, and returns the *actual*
    /// delta that fits within that remaining headroom - which may be less
    /// than `proposed_pct` (or even zero) once the cap is already close,
    /// and is always the full `proposed_pct` when it moves the
    /// accumulator back *toward* zero (shrinking an existing swing always
    /// has room, by construction - only growing an already-large swing
    /// gets throttled). Callers apply the returned value to
    /// `intrinsic_value` (never the raw `proposed_pct`) and feed the same
    /// returned value into `apply_rivalry_spillover`, so a third-party
    /// rival never reacts to more than what actually happened to the
    /// mover - the cap closes the spillover loophole too, not just the
    /// direct match effect.
    fn apply_capped_disciplinary_delta(club: &mut ClubState, proposed_pct: Decimal) -> Decimal {
        let current = club.disciplinary_impact_this_match;
        let new_total = (current + proposed_pct).clamp(-Self::MAX_DISCIPLINARY_IMPACT_PCT_PER_MATCH, Self::MAX_DISCIPLINARY_IMPACT_PCT_PER_MATCH);
        club.disciplinary_impact_this_match = new_total;
        new_total - current
    }

    /// Ripples a club's own real value movement out to every OTHER club
    /// that has `mover_id` explicitly listed in *its own* `rivals` - even
    /// when that observer isn't playing in this match at all (e.g. Real
    /// Madrid having a big night should nudge Barcelona too). Opposite sign
    /// of the mover's own move, damped, and weighted by the *observer's*
    /// own configured factor toward the mover - not the mover's factor
    /// toward them, since `rivals` isn't reliably symmetric in real seed
    /// data (confirmed: several one-directional-only pairs, several
    /// asymmetric-value pairs). A club with no configured entry for the
    /// mover gets exactly zero spillover - deliberately opt-in via the
    /// sparse `rivals` list itself, not a neutral default the way the
    /// direct-effect multiplier is (that one *has* to have some value,
    /// since the two match participants are definitely playing each other;
    /// spillover isn't owed to anyone who hasn't said they care).
    ///
    /// `signed_delta_pct` is the mover's own already-realized, signed move
    /// (e.g. `+impact_pct` for a scorer, after the scorer's own rivalry
    /// multiplier is already baked in) - the real signal that ripples out,
    /// not some hypothetical unmultiplied base. `exclude_id` is the other
    /// participant already in this same event, so it never double-dips as
    /// an outside "reactor" on top of its own direct mirrored move.
    ///
    /// Deliberately single-hop only: a reactor's own resulting move never
    /// triggers a second round of spillover to *its* rivals - only ever
    /// called from Goal/RedCard/YellowCard's direct movement sites, never
    /// from within this function itself. Prevents runaway cascades through
    /// rivalry chains or cycles. Does not touch `last_match_update`/
    /// `resting_value` for the reactor - spillover isn't the reactor's own
    /// real match activity.
    fn apply_rivalry_spillover(&self, mover_id: ClubId, exclude_id: ClubId, signed_delta_pct: Decimal, current_ts: u64) {
        self.rivalry_spillover(mover_id, exclude_id, signed_delta_pct, current_ts, false);
    }

    fn rivalry_spillover(&self, mover_id: ClubId, exclude_id: ClubId, signed_delta_pct: Decimal, current_ts: u64, undo: bool) {
        const SPILLOVER_DAMPING: Decimal = dec!(0.2);

        if signed_delta_pct == Decimal::ZERO {
            return;
        }

        // Snapshot first (immutable scan, no get_mut held), then mutate one
        // at a time - same pattern WhistleEnd uses to collect player ids
        // before looping get_mut. Must only be called with the mover's own
        // get_mut guard already dropped, or this deadlocks on its shard.
        let reactors: Vec<(ClubId, Decimal)> = self
            .club_states
            .iter()
            .filter(|entry| *entry.key() != mover_id && *entry.key() != exclude_id)
            .filter_map(|entry| entry.value().rivals.iter().find(|(rid, _)| *rid == mover_id).map(|(_, w)| (*entry.key(), *w)))
            .collect();

        for (reactor_id, weight) in reactors {
            if let Some(mut reactor) = self.club_states.get_mut(&reactor_id) {
                let reactor_delta_pct = -signed_delta_pct * SPILLOVER_DAMPING * weight;
                let base_value = reactor.intrinsic_value;
                reactor.intrinsic_value = scale(base_value, reactor_delta_pct, undo);
                self.notify(EngineUpdate::Club { id: reactor_id, state: reactor.clone(), ts: current_ts });
            }
        }
    }

    pub fn get_club_pair_exchange_rate(&self, base_id: ClubId, quote_id: ClubId) -> Decimal {
        let v_base = self.club_states.get(&base_id).map(|c| c.intrinsic_value).unwrap_or(dec!(0));
        let v_quote = self.club_states.get(&quote_id).map(|c| c.intrinsic_value).unwrap_or(dec!(1));
        if v_quote.is_zero() { dec!(0) } else { v_base / v_quote }
    }

    /// A goal's whole effect, or with `undo` its exact reversal (VAR). Every
    /// move here is `value *= 1 + pct` with a pct fixed by the minute,
    /// rivalries and the scorer's position - never by current values - so
    /// dividing by the same factors takes the goal back out precisely, even
    /// with heartbeat ticks and other events applied in between.
    fn apply_goal(&self, team_id: ClubId, opponent_id: ClubId, player_id: PlayerId, minute: u32, current_ts: u64, undo: bool) {
        let scorer_rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
        let time_multiplier = Decimal::from(minute) / dec!(90.0);
        // Percent of the club's *current* value, not a flat point
        // add - a flat number meant identical events were worth
        // wildly different relative moves depending purely on a
        // club's absolute valuation (confirmed live: ~0.7% for a
        // ~2,250-value club vs. ~5.6% for a ~270-value one, same
        // goal). `value += value * pct` also can't drive a value
        // negative the way repeated flat subtractions could.
        // Halved from the original 0.05/0.10 once goals became
        // mirrored (both sides move) - the effective spread-swing
        // had roughly doubled, and a multi-goal rout was compounding
        // to 60-70%+. This restores close to the pre-mirroring
        // spread magnitude even under real multi-goal compounding
        // (confirmed by modeling a 5-0 blowout both ways), not just
        // a linear approximation.
        let base_impact_pct = GOAL_BASE_IMPACT_PCT + (GOAL_LATE_IMPACT_PCT * time_multiplier);

        let mut scorer_impact_pct = Decimal::ZERO;
        if let Some(mut club) = self.club_states.get_mut(&team_id) {
            scorer_impact_pct = base_impact_pct * scorer_rivalry_multiplier;
            let base_value = club.intrinsic_value;
            club.intrinsic_value = scale(base_value, scorer_impact_pct, undo);
            club.last_match_update = current_ts;
            club.anchor_resting_value();
            self.notify(EngineUpdate::Club { id: team_id, state: club.clone(), ts: current_ts });
        }
        self.rivalry_spillover(team_id, opponent_id, scorer_impact_pct, current_ts, undo);

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
        let mut conceder_impact_pct = Decimal::ZERO;
        if let Some(mut opponent_club) = self.club_states.get_mut(&opponent_id) {
            conceder_impact_pct = base_impact_pct * conceder_rivalry_multiplier;
            let base_value = opponent_club.intrinsic_value;
            opponent_club.intrinsic_value = scale(base_value, -conceder_impact_pct, undo);
            opponent_club.last_match_update = current_ts;
            opponent_club.anchor_resting_value();
            self.notify(EngineUpdate::Club { id: opponent_id, state: opponent_club.clone(), ts: current_ts });
        }
        self.rivalry_spillover(opponent_id, team_id, -conceder_impact_pct, current_ts, undo);

        if let Some(mut player) = self.player_states.get_mut(&player_id) {
            let pos_multiplier = player.position.base_weight();
            let player_impact_pct = dec!(0.15) * pos_multiplier * player.form_weight * scorer_rivalry_multiplier;
            let base_value = player.intrinsic_value;
            player.intrinsic_value = scale(base_value, player_impact_pct, undo);
            self.notify(EngineUpdate::Player { id: player_id, state: player.clone() });
        }
    }

    /// Takes back the latest matching red card (same opponent, and the same
    /// player where it resolved). Both clubs' values and the spillover are
    /// divided back out by exactly what the card applied, and both clubs get
    /// that disciplinary headroom back. A card with no record (already
    /// past full time, or never applied) is a no-op.
    fn cancel_red_card(&self, team_id: ClubId, opponent_id: ClubId, player_id: PlayerId, current_ts: u64) {
        let Some(card) = self.club_states.get_mut(&team_id).and_then(|mut club| {
            let cards = &club.red_cards_this_match;
            let at = cards
                .iter()
                .rposition(|c| c.opponent_id == opponent_id && c.player_id == player_id)
                .or_else(|| cards.iter().rposition(|c| c.opponent_id == opponent_id))?;
            Some(club.red_cards_this_match.remove(at))
        }) else {
            return;
        };

        for (club_id, pct) in [(team_id, card.carded_pct), (opponent_id, card.benefiting_pct)] {
            if let Some(mut club) = self.club_states.get_mut(&club_id) {
                club.intrinsic_value = scale(club.intrinsic_value, pct, true);
                club.disciplinary_impact_this_match -= pct;
                club.last_match_update = current_ts;
                club.anchor_resting_value();
                self.notify(EngineUpdate::Club { id: club_id, state: club.clone(), ts: current_ts });
            }
        }
        self.rivalry_spillover(team_id, opponent_id, card.carded_pct, current_ts, true);
        self.rivalry_spillover(opponent_id, team_id, card.benefiting_pct, current_ts, true);

        if let Some(mut player) = self.player_states.get_mut(&player_id) {
            player.form_weight *= dec!(2);
            self.notify(EngineUpdate::Player { id: player_id, state: player.clone() });
        }
    }

    pub fn process_event(&self, event: MatchEvent, current_ts: u64) {
        self.notify(EngineUpdate::Event { event: event.clone(), ts: current_ts });
        
        match event {
            MatchEvent::Goal { team_id, opponent_id, player_id, minute } => {
                self.apply_goal(team_id, opponent_id, player_id, minute, current_ts, false);
            }
            MatchEvent::GoalCancelled { team_id, opponent_id, player_id, minute } => {
                self.apply_goal(team_id, opponent_id, player_id, minute, current_ts, true);
            }
            MatchEvent::RedCard { team_id, opponent_id, player_id } => {
                let carded_rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
                let mut carded_impact_pct = Decimal::ZERO;
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    // Base magnitude sits below an average-minute goal
                    // (5%) - a red card is genuinely significant but is a
                    // contributing signal, not the match outcome itself;
                    // see MAX_DISCIPLINARY_IMPACT_PCT_PER_MATCH for the
                    // other half of this calibration (the per-match cap).
                    let proposed_pct = dec!(0.04) * carded_rivalry_multiplier;
                    carded_impact_pct = Self::apply_capped_disciplinary_delta(&mut club, -proposed_pct);
                    let base_value = club.intrinsic_value;
                    club.intrinsic_value += base_value * carded_impact_pct;
                    club.last_match_update = current_ts;
                    club.anchor_resting_value();
                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone(), ts: current_ts });
                }
                self.apply_rivalry_spillover(team_id, opponent_id, carded_impact_pct, current_ts);

                // A red card against the opponent is real positive signal
                // too - down a player is a genuine disadvantage for them,
                // which reads as good news for the other side. Mirrors the
                // carded club's own move (same magnitude, opposite sign),
                // scaled by the *benefiting* club's own rivalry factor
                // toward the carded team, and capped by the same per-match
                // disciplinary ceiling - this is the side of the ceiling
                // that actually mattered in the real incident this was
                // calibrated against (see disciplinary_cap_tests).
                let benefiting_rivalry_multiplier = self.get_rivalry_multiplier(opponent_id, team_id);
                let mut benefiting_impact_pct = Decimal::ZERO;
                if let Some(mut opponent_club) = self.club_states.get_mut(&opponent_id) {
                    let proposed_pct = dec!(0.04) * benefiting_rivalry_multiplier;
                    benefiting_impact_pct = Self::apply_capped_disciplinary_delta(&mut opponent_club, proposed_pct);
                    let base_value = opponent_club.intrinsic_value;
                    opponent_club.intrinsic_value += base_value * benefiting_impact_pct;
                    opponent_club.last_match_update = current_ts;
                    opponent_club.anchor_resting_value();
                    self.notify(EngineUpdate::Club { id: opponent_id, state: opponent_club.clone(), ts: current_ts });
                }
                self.apply_rivalry_spillover(opponent_id, team_id, benefiting_impact_pct, current_ts);

                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    club.red_cards_this_match.push(AppliedRedCard {
                        opponent_id,
                        player_id,
                        carded_pct: carded_impact_pct,
                        benefiting_pct: benefiting_impact_pct,
                    });
                }

                if let Some(mut player) = self.player_states.get_mut(&player_id) {
                    player.form_weight *= dec!(0.5);
                    self.notify(EngineUpdate::Player { id: player_id, state: player.clone() });
                }
            }
            MatchEvent::RedCardCancelled { team_id, opponent_id, player_id } => {
                self.cancel_red_card(team_id, opponent_id, player_id, current_ts);
            }
            MatchEvent::YellowCard { team_id, opponent_id, player_id } => {
                let carded_rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
                let mut carded_impact_pct = Decimal::ZERO;
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    // Clearly below the smallest possible goal impact
                    // (2.5% at minute 0) - a yellow card is a minor,
                    // incidental signal on its own.
                    let proposed_pct = dec!(0.01) * carded_rivalry_multiplier;
                    carded_impact_pct = Self::apply_capped_disciplinary_delta(&mut club, -proposed_pct);
                    let base_value = club.intrinsic_value;
                    club.intrinsic_value += base_value * carded_impact_pct;
                    club.last_match_update = current_ts;
                    club.anchor_resting_value();
                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone(), ts: current_ts });
                }
                self.apply_rivalry_spillover(team_id, opponent_id, carded_impact_pct, current_ts);

                // Same mirrored-benefit reasoning as RedCard above, scaled
                // to the yellow-card magnitude and capped the same way.
                let benefiting_rivalry_multiplier = self.get_rivalry_multiplier(opponent_id, team_id);
                let mut benefiting_impact_pct = Decimal::ZERO;
                if let Some(mut opponent_club) = self.club_states.get_mut(&opponent_id) {
                    let proposed_pct = dec!(0.01) * benefiting_rivalry_multiplier;
                    benefiting_impact_pct = Self::apply_capped_disciplinary_delta(&mut opponent_club, proposed_pct);
                    let base_value = opponent_club.intrinsic_value;
                    opponent_club.intrinsic_value += base_value * benefiting_impact_pct;
                    opponent_club.last_match_update = current_ts;
                    opponent_club.anchor_resting_value();
                    self.notify(EngineUpdate::Club { id: opponent_id, state: opponent_club.clone(), ts: current_ts });
                }
                self.apply_rivalry_spillover(opponent_id, team_id, benefiting_impact_pct, current_ts);

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
                        club.anchor_resting_value();
                        // Fresh disciplinary headroom for the next match -
                        // see MAX_DISCIPLINARY_IMPACT_PCT_PER_MATCH.
                        club.disciplinary_impact_this_match = Decimal::ZERO;
                        club.red_cards_this_match.clear();
                        self.notify(EngineUpdate::Club { id: team_id, state: club.clone(), ts: current_ts });
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
            MatchEvent::Heartbeat { team_id, noise, fade } => {
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    club.apply_fade(fade);
                    // Deliberately NOT touching last_match_update here -
                    // it's reserved for real events (Goal/Card/WhistleEnd/
                    // AdminCorrection/Transfer) - see EngineUpdate::Club's
                    // own `ts` field for the "always fresh, every tick"
                    // timestamp live price-history storage actually needs.

                    // Only the change in the deviation touches the price, so
                    // the fundamental underneath is never moved by noise.
                    let draw = if noise.shock_half_width > 0.0 {
                        use rand::Rng;
                        rand::thread_rng().gen_range(-1.0..1.0)
                    } else {
                        0.0
                    };
                    let next = noise.step(club.noise_log, draw);
                    let step = heartbeat_multiplier(next - club.noise_log);
                    club.intrinsic_value *= step;
                    club.noise_log = next;
                    club.step_repricing();

                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone(), ts: current_ts });
                }
            }
            MatchEvent::Repricing { team_id, ln_delta, ticks } => {
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    if ticks == 0 {
                        let factor = heartbeat_multiplier(ln_delta);
                        club.intrinsic_value *= factor;
                    } else {
                        club.repricing_ln_remaining += ln_delta;
                        club.repricing_ticks_remaining = club.repricing_ticks_remaining.max(ticks);
                    }
                    club.last_match_update = current_ts;
                    club.anchor_resting_value();
                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone(), ts: current_ts });
                }
            }
            MatchEvent::AdminCorrection { team_id, delta_pct } => {
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    club.intrinsic_value *= dec!(1.0) + delta_pct / dec!(100.0);
                    club.last_match_update = current_ts;
                    club.anchor_resting_value();
                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone(), ts: current_ts });
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
mod var_reversal_tests {
    use super::*;

    /// Scorer, conceder (rivals both ways, so the rivalry multipliers aren't
    /// 1) and a third club rivalling the scorer, so the spillover runs too.
    fn engine() -> ValuationEngine {
        let engine = ValuationEngine::new();
        let mut scorer = ClubState::new(ClubId(1));
        scorer.intrinsic_value = dec!(2400);
        scorer.rivals = vec![(ClubId(2), dec!(1.5))];
        let mut conceder = ClubState::new(ClubId(2));
        conceder.intrinsic_value = dec!(1800);
        conceder.rivals = vec![(ClubId(1), dec!(1.3))];
        let mut neighbour = ClubState::new(ClubId(3));
        neighbour.intrinsic_value = dec!(900);
        neighbour.rivals = vec![(ClubId(1), dec!(0.8)), (ClubId(2), dec!(0.5))];
        for c in [scorer, conceder, neighbour] {
            engine.club_states.insert(c.id, c);
        }
        engine
    }

    fn values(engine: &ValuationEngine) -> Vec<Decimal> {
        (1..=3).map(|id| engine.club_states.get(&ClubId(id)).unwrap().intrinsic_value).collect()
    }

    fn close(a: &[Decimal], b: &[Decimal]) -> bool {
        a.iter().zip(b).all(|(x, y)| ((*x - *y) / *y).abs() < dec!(0.000000001))
    }

    #[test]
    fn a_cancelled_goal_leaves_every_club_where_it_started() {
        let engine = engine();
        let before = values(&engine);
        engine.process_event(MatchEvent::Goal { team_id: ClubId(1), opponent_id: ClubId(2), player_id: PlayerId(0), minute: 78 }, 0);
        assert!(!close(&values(&engine), &before), "the goal should have moved prices");
        engine.process_event(MatchEvent::GoalCancelled { team_id: ClubId(1), opponent_id: ClubId(2), player_id: PlayerId(0), minute: 78 }, 1);
        assert!(close(&values(&engine), &before));
    }

    #[test]
    fn the_reversal_is_exact_even_with_other_moves_in_between() {
        // Goal, then an admin nudge on the scorer, then VAR: the end state
        // must equal the nudge alone - not the nudge sized off a pre-goal value.
        let with_var = engine();
        with_var.process_event(MatchEvent::Goal { team_id: ClubId(1), opponent_id: ClubId(2), player_id: PlayerId(0), minute: 30 }, 0);
        with_var.process_event(MatchEvent::AdminCorrection { team_id: ClubId(1), delta_pct: dec!(10) }, 1);
        with_var.process_event(MatchEvent::GoalCancelled { team_id: ClubId(1), opponent_id: ClubId(2), player_id: PlayerId(0), minute: 30 }, 2);

        let nudge_only = engine();
        nudge_only.process_event(MatchEvent::AdminCorrection { team_id: ClubId(1), delta_pct: dec!(10) }, 1);
        assert!(close(&values(&with_var), &values(&nudge_only)));
    }

    fn red(team: u32, opp: u32) -> MatchEvent {
        MatchEvent::RedCard { team_id: ClubId(team), opponent_id: ClubId(opp), player_id: PlayerId(0) }
    }
    fn red_cancelled(team: u32, opp: u32) -> MatchEvent {
        MatchEvent::RedCardCancelled { team_id: ClubId(team), opponent_id: ClubId(opp), player_id: PlayerId(0) }
    }
    fn headroom(engine: &ValuationEngine, id: u32) -> Decimal {
        engine.club_states.get(&ClubId(id)).unwrap().disciplinary_impact_this_match
    }

    #[test]
    fn an_overturned_red_card_restores_values_and_disciplinary_headroom() {
        let engine = engine();
        let before = values(&engine);
        engine.process_event(red(2, 1), 0);
        assert!(!close(&values(&engine), &before), "the card should have moved prices");
        engine.process_event(red_cancelled(2, 1), 1);
        assert!(close(&values(&engine), &before));
        assert_eq!(headroom(&engine, 1), Decimal::ZERO);
        assert_eq!(headroom(&engine, 2), Decimal::ZERO);
        assert!(engine.club_states.get(&ClubId(2)).unwrap().red_cards_this_match.is_empty());
    }

    #[test]
    fn a_capped_second_card_is_undone_by_what_it_actually_applied() {
        // The second red runs into the per-match cap, so it applies less
        // than the full 4% x rivalry. Overturning it must take out only what
        // it moved - recomputing it fresh would overshoot.
        let engine = engine();
        engine.process_event(red(2, 1), 0);
        let after_one = values(&engine);
        let headroom_after_one = (headroom(&engine, 1), headroom(&engine, 2));
        engine.process_event(red(2, 1), 1);
        let second = engine.club_states.get(&ClubId(2)).unwrap().red_cards_this_match[1].clone();
        assert!(second.carded_pct.abs() > Decimal::ZERO && second.carded_pct.abs() < dec!(0.04) * dec!(1.3), "second card should be capped: {second:?}");
        engine.process_event(red_cancelled(2, 1), 2);
        assert!(close(&values(&engine), &after_one));
        assert_eq!((headroom(&engine, 1), headroom(&engine, 2)), headroom_after_one);
    }

    #[test]
    fn an_unknown_red_card_cancel_changes_nothing() {
        let engine = engine();
        engine.process_event(red(2, 1), 0);
        let after = values(&engine);
        engine.process_event(red_cancelled(1, 2), 1); // club 1 was never carded
        assert_eq!(values(&engine), after);
        engine.process_event(MatchEvent::WhistleEnd { team_a_id: ClubId(1), team_b_id: ClubId(2) }, 2);
        let after_ft = values(&engine);
        engine.process_event(red_cancelled(2, 1), 3); // full time cleared the record
        assert_eq!(values(&engine), after_ft);
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
        // ~5.83% (0.025 + 0.05 * 60/90) - halved from the original 0.05/0.10
        // once goals became mirrored (see goal_also_moves_the_conceding_
        // club_down_by_the_mirrored_percentage), so the effective spread
        // magnitude stays where it was pre-mirroring.
        assert!(small_pct > dec!(0.058) && small_pct < dec!(0.059), "expected ~5.83% impact, got {small_pct}");
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
        assert_eq!(after, dec!(1000.0) * (dec!(1) - dec!(0.04)), "expected exactly a 4% reduction, got {after}");
    }

    #[test]
    fn yellow_card_reduces_club_value_by_the_expected_percentage() {
        let (engine, small_club, small_player, _big_club, _big_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        engine.process_event(MatchEvent::YellowCard { team_id: small_club, opponent_id: ClubId(3), player_id: small_player }, 0);
        let after = engine.club_states.get(&small_club).unwrap().intrinsic_value;
        assert_eq!(after, dec!(1000.0) * (dec!(1) - dec!(0.01)), "expected exactly a 1% reduction, got {after}");
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
        let expected_pct = dec!(0.025) + (dec!(0.05) * dec!(60.0) / dec!(90.0));
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

        assert_eq!(carded_after, dec!(1000.0) * (dec!(1) - dec!(0.04)), "carded club should be down 4%, got {carded_after}");
        assert_eq!(benefiting_after, dec!(1000.0) * (dec!(1) + dec!(0.04)), "benefiting opponent should be up 4%, got {benefiting_after}");
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

        let base_pct = dec!(0.025) + (dec!(0.05) * dec!(60.0) / dec!(90.0));
        assert_eq!(scorer_after, dec!(1000.0) * (dec!(1) + base_pct * dec!(2.0)), "scorer's own 2.0x rivalry factor should apply to its own move");
        assert_eq!(conceder_after, dec!(1000.0) * (dec!(1) - base_pct), "conceder has no configured factor, so its move should stay at the 1.0x default, not inherit the scorer's 2.0x");
    }

    #[test]
    fn goal_ripples_to_a_non_participant_rival_with_opposite_sign() {
        let (engine, scorer, scorer_player, conceder, _conceder_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        let reactor = ClubId(99);
        let mut reactor_state = ClubState::new(reactor);
        reactor_state.intrinsic_value = dec!(1000.0);
        reactor_state.set_rival_factor(scorer, dec!(1.5));
        engine.club_states.insert(reactor, reactor_state);

        engine.process_event(MatchEvent::Goal { team_id: scorer, opponent_id: conceder, player_id: scorer_player, minute: 60 }, 0);

        let scorer_impact_pct = dec!(0.025) + (dec!(0.05) * dec!(60.0) / dec!(90.0));
        let reactor_after = engine.club_states.get(&reactor).unwrap().intrinsic_value;
        let expected_reactor_pct = -(scorer_impact_pct * dec!(0.2) * dec!(1.5));
        assert_eq!(
            reactor_after,
            dec!(1000.0) * (dec!(1) + expected_reactor_pct),
            "a non-participant rival should move opposite the scorer, damped and weighted by its own configured factor"
        );
    }

    #[test]
    fn no_spillover_for_a_club_with_no_configured_rivalry_toward_the_mover() {
        let (engine, scorer, scorer_player, conceder, _conceder_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        let bystander = ClubId(99);
        let mut bystander_state = ClubState::new(bystander);
        bystander_state.intrinsic_value = dec!(1000.0);
        // Deliberately no set_rival_factor call - bystander has no
        // configured opinion about the scorer at all.
        engine.club_states.insert(bystander, bystander_state);

        engine.process_event(MatchEvent::Goal { team_id: scorer, opponent_id: conceder, player_id: scorer_player, minute: 60 }, 0);

        let bystander_after = engine.club_states.get(&bystander).unwrap().intrinsic_value;
        assert_eq!(bystander_after, dec!(1000.0), "a club with no rivals entry for the mover must get exactly zero spillover, not a neutral-default nudge");
    }

    #[test]
    fn match_participant_never_double_dips_as_an_outside_spillover_reactor() {
        // The conceder's own configured factor toward the scorer already
        // powers its direct mirrored move (get_rivalry_multiplier(conceder,
        // scorer)) - the exclude_id filter must stop that *same* rivals
        // entry from also being picked up a second time by the reactor
        // scan and applying an extra spillover term on top. Expected value
        // uses the 1.5x multiplier since it genuinely does apply once, to
        // the direct move - proving there's no *second*, additional term.
        let (engine, scorer, scorer_player, conceder, _conceder_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        {
            let mut conceder_state = engine.club_states.get_mut(&conceder).unwrap();
            conceder_state.set_rival_factor(scorer, dec!(1.5));
        }

        engine.process_event(MatchEvent::Goal { team_id: scorer, opponent_id: conceder, player_id: scorer_player, minute: 60 }, 0);

        let base_impact_pct = dec!(0.025) + (dec!(0.05) * dec!(60.0) / dec!(90.0));
        let conceder_direct_impact_pct = base_impact_pct * dec!(1.5); // conceder's own 1.5x factor, applied once
        let conceder_after = engine.club_states.get(&conceder).unwrap().intrinsic_value;
        assert_eq!(
            conceder_after,
            dec!(1000.0) * (dec!(1) - conceder_direct_impact_pct),
            "the match's own opponent must never also receive spillover on top of its direct mirrored move"
        );
    }

    #[test]
    fn reactor_configured_against_both_match_participants_gets_two_independent_spillovers() {
        let (engine, scorer, scorer_player, conceder, _conceder_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        let reactor = ClubId(99);
        let mut reactor_state = ClubState::new(reactor);
        reactor_state.intrinsic_value = dec!(1000.0);
        reactor_state.set_rival_factor(scorer, dec!(1.5));
        reactor_state.set_rival_factor(conceder, dec!(1.2));
        engine.club_states.insert(reactor, reactor_state);

        engine.process_event(MatchEvent::Goal { team_id: scorer, opponent_id: conceder, player_id: scorer_player, minute: 60 }, 0);

        let impact_pct = dec!(0.025) + (dec!(0.05) * dec!(60.0) / dec!(90.0));
        let reactor_after = engine.club_states.get(&reactor).unwrap().intrinsic_value;
        // Two independent, sequential percent-of-current-value moves (same
        // "value * pct of whatever it currently is" convention as
        // everything else here), not deduped or combined into one delta:
        // the scorer's spillover (reactor down) lands first, then the
        // conceder's spillover (reactor up) compounds on top of that.
        let after_scorer_spillover = dec!(1000.0) * (dec!(1) - impact_pct * dec!(0.2) * dec!(1.5));
        let expected = after_scorer_spillover * (dec!(1) + impact_pct * dec!(0.2) * dec!(1.2));
        assert_eq!(
            reactor_after, expected,
            "a club rivaling both match participants should get two independent, opposite-signed spillovers, not be deduped"
        );
    }

    #[test]
    fn spillover_uses_the_movers_fully_realized_delta_and_the_reactors_own_weight_stacked() {
        // Both the mover's own rivalry multiplier (toward its real
        // opponent) and the reactor's own weight (toward the mover) are
        // != 1.0 together - confirms they stack multiplicatively rather
        // than one silently overriding the other.
        let (engine, scorer, scorer_player, conceder, _conceder_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        {
            let mut scorer_state = engine.club_states.get_mut(&scorer).unwrap();
            scorer_state.set_rival_factor(conceder, dec!(1.5)); // scorer's real rivalry with its actual opponent
        }
        let reactor = ClubId(99);
        let mut reactor_state = ClubState::new(reactor);
        reactor_state.intrinsic_value = dec!(1000.0);
        reactor_state.set_rival_factor(scorer, dec!(1.3)); // reactor's own weight toward the mover
        engine.club_states.insert(reactor, reactor_state);

        engine.process_event(MatchEvent::Goal { team_id: scorer, opponent_id: conceder, player_id: scorer_player, minute: 60 }, 0);

        let base_impact_pct = dec!(0.025) + (dec!(0.05) * dec!(60.0) / dec!(90.0));
        let scorer_realized_pct = base_impact_pct * dec!(1.5); // scorer's own multiplier already baked in
        let reactor_after = engine.club_states.get(&reactor).unwrap().intrinsic_value;
        let expected_reactor_pct = -(scorer_realized_pct * dec!(0.2) * dec!(1.3));
        assert_eq!(
            reactor_after,
            dec!(1000.0) * (dec!(1) + expected_reactor_pct),
            "spillover must use the mover's fully-realized (already-multiplied) delta, stacked with the reactor's own weight"
        );
    }

    #[test]
    fn spillover_does_not_cascade_to_a_second_hop() {
        // A rival of the spillover-affected reactor (not of the actual
        // mover) must feel nothing - spillover only ever fires from the
        // real event's direct participants, never recursively from a
        // reactor's own resulting move.
        let (engine, scorer, scorer_player, conceder, _conceder_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        let reactor = ClubId(98);
        let mut reactor_state = ClubState::new(reactor);
        reactor_state.intrinsic_value = dec!(1000.0);
        reactor_state.set_rival_factor(scorer, dec!(1.5));
        engine.club_states.insert(reactor, reactor_state);

        let second_hop = ClubId(97);
        let mut second_hop_state = ClubState::new(second_hop);
        second_hop_state.intrinsic_value = dec!(1000.0);
        second_hop_state.set_rival_factor(reactor, dec!(1.5)); // rivals the REACTOR, not the mover
        engine.club_states.insert(second_hop, second_hop_state);

        engine.process_event(MatchEvent::Goal { team_id: scorer, opponent_id: conceder, player_id: scorer_player, minute: 60 }, 0);

        let second_hop_after = engine.club_states.get(&second_hop).unwrap().intrinsic_value;
        assert_eq!(second_hop_after, dec!(1000.0), "a rival of the spillover-affected reactor must feel nothing - single-hop only");
    }

    #[test]
    fn repeated_cards_in_one_match_stop_accumulating_once_the_cap_is_hit() {
        let (engine, club, player, opponent, _opponent_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        // No rivalry factor configured (defaults to 1.0x), so each yellow
        // card proposes exactly -1% - six of them exactly exhausts the 6%
        // cap with no remainder, making the boundary easy to check exactly.
        for _ in 0..10 {
            engine.process_event(MatchEvent::YellowCard { team_id: club, opponent_id: opponent, player_id: player }, 0);
        }
        let accumulated = engine.club_states.get(&club).unwrap().disciplinary_impact_this_match;
        assert_eq!(accumulated, dec!(-0.06), "10 yellow cards should still cap at exactly -6%, not -10%");

        let after = engine.club_states.get(&club).unwrap().intrinsic_value;
        assert!(after < dec!(1000.0), "the club should still be down some");
        assert!(after > dec!(900.0), "but nowhere near 10 uncapped 1% reductions worth (~904.4 vs the true 6-card-equivalent ~941)");
    }

    #[test]
    fn cap_resets_at_whistle_end_giving_the_next_match_fresh_headroom() {
        let (engine, club, player, opponent, _opponent_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        for _ in 0..10 {
            engine.process_event(MatchEvent::YellowCard { team_id: club, opponent_id: opponent, player_id: player }, 0);
        }
        assert_eq!(engine.club_states.get(&club).unwrap().disciplinary_impact_this_match, dec!(-0.06));

        engine.process_event(MatchEvent::WhistleEnd { team_a_id: club, team_b_id: opponent }, 0);
        assert_eq!(
            engine.club_states.get(&club).unwrap().disciplinary_impact_this_match,
            dec!(0),
            "the accumulator must reset at full time, or a club that had a rough match would start its NEXT match already capped"
        );

        let value_before_next_card = engine.club_states.get(&club).unwrap().intrinsic_value;
        engine.process_event(MatchEvent::YellowCard { team_id: club, opponent_id: opponent, player_id: player }, 0);
        let value_after_next_card = engine.club_states.get(&club).unwrap().intrinsic_value;
        assert_eq!(
            value_after_next_card,
            value_before_next_card * (dec!(1) - dec!(0.01)),
            "the first card of a new match should apply at full magnitude, proving the reset actually freed up headroom"
        );
    }

    #[test]
    fn moving_back_toward_zero_is_never_throttled_by_the_cap() {
        let (engine, club, player, opponent, opponent_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        // Saturate `club`'s accumulator at the positive cap by having the
        // opponent get carded twice (club is the benefiting side).
        for _ in 0..2 {
            engine.process_event(MatchEvent::RedCard { team_id: opponent, opponent_id: club, player_id: opponent_player }, 0);
        }
        assert_eq!(engine.club_states.get(&club).unwrap().disciplinary_impact_this_match, dec!(0.06));

        // Now `club` itself gets carded - a move back toward zero - and
        // must apply at its full proposed magnitude despite being at the
        // opposite boundary, since shrinking an existing swing always has
        // room by construction.
        engine.process_event(MatchEvent::YellowCard { team_id: club, opponent_id: opponent, player_id: player }, 0);
        let accumulated = engine.club_states.get(&club).unwrap().disciplinary_impact_this_match;
        assert_eq!(accumulated, dec!(0.06) - dec!(0.01), "a card moving the total back toward zero must apply in full, never throttled");
    }

    #[test]
    fn spillover_reflects_the_capped_delta_not_the_proposed_one() {
        let (engine, club, player, opponent, _opponent_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        let reactor = ClubId(99);
        let mut reactor_state = ClubState::new(reactor);
        reactor_state.intrinsic_value = dec!(1000.0);
        reactor_state.set_rival_factor(club, dec!(1.0));
        engine.club_states.insert(reactor, reactor_state);

        // Saturate `club`'s negative cap first (6 yellow cards, -1% each).
        for _ in 0..6 {
            engine.process_event(MatchEvent::YellowCard { team_id: club, opponent_id: opponent, player_id: player }, 0);
        }
        let reactor_after_saturating = engine.club_states.get(&reactor).unwrap().intrinsic_value;

        // A 7th card proposes another -1% for `club`, but the cap means the
        // ACTUAL delta is zero - spillover must reflect that zero, not the
        // proposed -1%, or a third party would react to a move that never
        // actually happened to the mover.
        engine.process_event(MatchEvent::YellowCard { team_id: club, opponent_id: opponent, player_id: player }, 0);
        let reactor_after_seventh_card = engine.club_states.get(&reactor).unwrap().intrinsic_value;

        assert_eq!(
            reactor_after_seventh_card, reactor_after_saturating,
            "once the mover's own delta is capped to zero, a rival reactor must see zero spillover too, not spillover on the pre-cap proposal"
        );
    }

    /// Reproduces the real Sep 2026 Man City (rivalry 1.5x each way) vs Man
    /// United match that motivated this whole cap: City picked up 1 red
    /// card + 3 yellow cards, scored 1 goal, United picked up 1 yellow of
    /// their own - a match United lost 0-1. Before this cap existed,
    /// United's net move from this match was +9.75% (a losing side gaining
    /// value). This pins the corrected behavior: the loss must now cost
    /// United value overall, not hand them a net gain.
    #[test]
    fn the_losing_side_of_a_card_heavy_match_now_nets_a_loss_not_a_gain() {
        let (engine, city, city_player, united, united_player) = engine_with_two_clubs(dec!(1000.0), dec!(1000.0));
        {
            let mut city_state = engine.club_states.get_mut(&city).unwrap();
            city_state.set_rival_factor(united, dec!(1.5));
        }
        {
            let mut united_state = engine.club_states.get_mut(&united).unwrap();
            united_state.set_rival_factor(city, dec!(1.5));
        }

        engine.process_event(MatchEvent::RedCard { team_id: city, opponent_id: united, player_id: city_player }, 0);
        engine.process_event(MatchEvent::YellowCard { team_id: city, opponent_id: united, player_id: city_player }, 0);
        engine.process_event(MatchEvent::Goal { team_id: city, opponent_id: united, player_id: city_player, minute: 63 }, 0);
        engine.process_event(MatchEvent::YellowCard { team_id: city, opponent_id: united, player_id: city_player }, 0);
        engine.process_event(MatchEvent::YellowCard { team_id: city, opponent_id: united, player_id: city_player }, 0);
        engine.process_event(MatchEvent::YellowCard { team_id: united, opponent_id: city, player_id: united_player }, 0);

        let united_after = engine.club_states.get(&united).unwrap().intrinsic_value;
        assert!(
            united_after < dec!(1000.0),
            "United lost this match 0-1 - they must end up down overall, not up, no matter how many cards City picked up. Got {united_after}"
        );
        assert!(
            united_after < dec!(970.0),
            "expected a real, substantial net loss (~950, not a rounding-distance graze under 1000) - got {united_after}"
        );
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
mod heartbeat_noise_tests {
    use super::*;

    const TICK_SECS: f64 = 2.0;
    const HOUR: f64 = 3600.0;

    fn engine_with_club(value: Decimal) -> (ValuationEngine, ClubId) {
        let engine = ValuationEngine::new();
        let id = ClubId(1);
        let mut club = ClubState::new(id);
        club.intrinsic_value = value;
        club.resting_value = Some(value);
        engine.club_states.insert(id, club);
        (engine, id)
    }

    fn tick(engine: &ValuationEngine, id: ClubId, noise: HeartbeatNoise, n: usize) {
        for _ in 0..n {
            engine.process_event(MatchEvent::Heartbeat { team_id: id, noise, fade: ResultFade::NONE }, 0);
        }
    }

    #[test]
    fn calibration_settles_at_the_requested_band() {
        let noise = HeartbeatNoise::calibrate(0.03, HOUR, TICK_SECS);
        let per_tick_variance = noise.shock_half_width.powi(2) / 3.0;
        let stationary_std = (per_tick_variance / (1.0 - noise.keep.powi(2))).sqrt();
        assert!((stationary_std - 0.03).abs() < 1e-12, "got {stationary_std}");
    }

    #[test]
    fn a_deviation_halves_over_one_half_life_without_new_shocks() {
        let noise = HeartbeatNoise::calibrate(0.03, HOUR, TICK_SECS);
        let mut x = 0.1;
        for _ in 0..(HOUR / TICK_SECS) as usize {
            x = noise.step(x, 0.0);
        }
        assert!((x - 0.05).abs() < 1e-9, "got {x}");
    }

    #[test]
    fn the_band_is_held_to_its_ceiling_and_nonsense_means_no_new_noise() {
        let capped = HeartbeatNoise::calibrate(3.0, HOUR, TICK_SECS);
        assert_eq!(capped, HeartbeatNoise::calibrate(MAX_HEARTBEAT_NOISE_BAND, HOUR, TICK_SECS));
        assert_eq!(HeartbeatNoise::calibrate(f64::NAN, HOUR, TICK_SECS).shock_half_width, 0.0);
        assert_eq!(HeartbeatNoise::calibrate(-1.0, HOUR, TICK_SECS).shock_half_width, 0.0);
    }

    /// The bug this replaced: a month of ticks used to leave a club anywhere
    /// from a fraction to many times its value. Now it stays within a few
    /// bands of where it started, and the fundamental doesn't move at all.
    #[test]
    fn a_month_without_matches_stays_near_the_fundamental() {
        let noise = HeartbeatNoise::calibrate(0.03, HOUR, TICK_SECS);
        let ticks = (30.0 * 24.0 * HOUR / TICK_SECS) as usize;
        let (engine, id) = engine_with_club(dec!(1000));
        let mut worst: f64 = 0.0;
        for _ in 0..ticks / 1000 {
            tick(&engine, id, noise, 1000);
            worst = worst.max(engine.club_states.get(&id).unwrap().noise_log.abs());
        }
        let club = engine.club_states.get(&id).unwrap();
        assert!(worst < 0.03 * 6.0, "deviation reached {worst}");
        assert!((club.fundamental_value() - dec!(1000)).abs() < dec!(0.000001), "fundamental moved to {}", club.fundamental_value());
    }

    #[test]
    fn a_goal_during_noise_moves_the_fundamental_by_exactly_its_own_amount() {
        let noise = HeartbeatNoise::calibrate(0.03, HOUR, TICK_SECS);
        let (engine, scorer) = engine_with_club(dec!(1000));
        let mut other = ClubState::new(ClubId(2));
        other.intrinsic_value = dec!(1000);
        engine.club_states.insert(ClubId(2), other);
        tick(&engine, scorer, noise, 500);
        engine.process_event(MatchEvent::Goal { team_id: scorer, opponent_id: ClubId(2), player_id: PlayerId(0), minute: 45 }, 0);
        tick(&engine, scorer, noise, 500);
        let club = engine.club_states.get(&scorer).unwrap();
        let expected = dec!(1000) * (Decimal::ONE + GOAL_BASE_IMPACT_PCT + GOAL_LATE_IMPACT_PCT * dec!(0.5));
        assert!((club.fundamental_value() - expected).abs() < dec!(0.000001), "got {}", club.fundamental_value());
        assert_eq!(club.resting_value.map(|r| (r - expected).abs() < dec!(0.000001)), Some(true), "anchor should be the fundamental, not the noisy price");
    }

    #[test]
    fn a_zero_band_fades_existing_noise_out_instead_of_freezing_it() {
        let (engine, id) = engine_with_club(dec!(1000));
        {
            let mut club = engine.club_states.get_mut(&id).unwrap();
            club.noise_log = 0.1;
            club.intrinsic_value = dec!(1000) * heartbeat_multiplier(0.1);
        }
        tick(&engine, id, HeartbeatNoise::calibrate(0.0, HOUR, TICK_SECS), (4.0 * HOUR / TICK_SECS) as usize);
        let club = engine.club_states.get(&id).unwrap();
        assert!((club.noise_log - 0.1 / 16.0).abs() < 1e-9, "got {}", club.noise_log);
        assert!((club.fundamental_value() - dec!(1000)).abs() < dec!(0.000001));
    }

    #[test]
    fn a_repricing_lands_exactly_on_target_over_its_ticks_with_noise_running() {
        let noise = HeartbeatNoise::calibrate(0.05, 1800.0, TICK_SECS);
        let (engine, id) = engine_with_club(dec!(95000));
        tick(&engine, id, noise, 200);
        let start = engine.club_states.get(&id).unwrap().fundamental_value();
        let ln_delta = (1000.0f64 / 95000.0).ln();
        engine.process_event(MatchEvent::Repricing { team_id: id, ln_delta, ticks: 100 }, 1);
        tick(&engine, id, noise, 50);
        {
            let club = engine.club_states.get(&id).unwrap();
            assert!(club.is_repricing());
            let halfway = start * heartbeat_multiplier(ln_delta / 2.0);
            assert!((club.fundamental_value() / halfway - Decimal::ONE).abs() < dec!(0.000001), "halfway at {}", club.fundamental_value());
        }
        tick(&engine, id, noise, 60);
        let club = engine.club_states.get(&id).unwrap();
        assert!(!club.is_repricing());
        let target = start * heartbeat_multiplier(ln_delta);
        assert!((club.fundamental_value() / target - Decimal::ONE).abs() < dec!(0.000001), "ended at {} not {}", club.fundamental_value(), target);
        assert!((club.resting_value.unwrap() / target - Decimal::ONE).abs() < dec!(0.000001), "anchor should follow the repricing");
    }

    #[test]
    fn a_goal_mid_repricing_still_counts_in_full() {
        let (engine, id) = engine_with_club(dec!(1000));
        let mut other = ClubState::new(ClubId(2));
        other.intrinsic_value = dec!(1000);
        engine.club_states.insert(ClubId(2), other);
        engine.process_event(MatchEvent::Repricing { team_id: id, ln_delta: 2f64.ln(), ticks: 10 }, 0);
        tick(&engine, id, HeartbeatNoise::calibrate(0.0, 1800.0, TICK_SECS), 5);
        engine.process_event(MatchEvent::Goal { team_id: id, opponent_id: ClubId(2), player_id: PlayerId(0), minute: 0 }, 0);
        tick(&engine, id, HeartbeatNoise::calibrate(0.0, 1800.0, TICK_SECS), 5);
        let expected = dec!(2000) * (Decimal::ONE + GOAL_BASE_IMPACT_PCT);
        let got = engine.club_states.get(&id).unwrap().fundamental_value();
        assert!((got / expected - Decimal::ONE).abs() < dec!(0.000001), "got {got}, expected {expected}");
    }

    #[test]
    fn a_repricing_with_no_ticks_lands_at_once() {
        let (engine, id) = engine_with_club(dec!(1000));
        engine.process_event(MatchEvent::Repricing { team_id: id, ln_delta: 2f64.ln(), ticks: 0 }, 0);
        let club = engine.club_states.get(&id).unwrap();
        assert!(!club.is_repricing());
        assert!((club.intrinsic_value - dec!(2000)).abs() < dec!(0.000001));
    }

    fn anchored(value: Decimal, anchor: Decimal) -> (ValuationEngine, ClubId) {
        let (engine, id) = engine_with_club(value);
        engine.club_states.get_mut(&id).unwrap().anchor_value = Some(anchor);
        (engine, id)
    }

    #[test]
    fn old_results_halve_their_distance_from_the_anchor_every_half_life() {
        let (engine, id) = anchored(dec!(4000), dec!(1000));
        let fade = ResultFade::calibrate(60.0 * 86_400.0, 86_400.0);
        for _ in 0..60 {
            engine.process_event(MatchEvent::Heartbeat { team_id: id, noise: HeartbeatNoise::calibrate(0.0, 1800.0, 2.0), fade }, 0);
        }
        let got = engine.club_states.get(&id).unwrap().fundamental_value();
        assert!((got - dec!(2000)).abs() < dec!(0.001), "4x the anchor should be 2x after one half-life, got {got}");
    }

    #[test]
    fn a_club_below_its_anchor_fades_back_up_the_same_way() {
        let (engine, id) = anchored(dec!(250), dec!(1000));
        engine.apply_result_fade(ResultFade { keep: 0.5 });
        let got = engine.club_states.get(&id).unwrap().fundamental_value();
        assert!((got - dec!(500)).abs() < dec!(0.001), "got {got}");
    }

    #[test]
    fn fading_leaves_the_noise_alone() {
        let (engine, id) = anchored(dec!(4000), dec!(1000));
        {
            let mut club = engine.club_states.get_mut(&id).unwrap();
            club.noise_log = 0.04;
            club.intrinsic_value = dec!(4000) * heartbeat_multiplier(0.04);
        }
        engine.apply_result_fade(ResultFade { keep: 0.5 });
        let club = engine.club_states.get(&id).unwrap();
        assert_eq!(club.noise_log, 0.04);
        assert!((club.fundamental_value() - dec!(2000)).abs() < dec!(0.001), "got {}", club.fundamental_value());
    }

    /// Two clubs with the same results in the same weeks end up the same
    /// distance from their anchors, whichever was ahead before - no handicap.
    #[test]
    fn the_same_goals_at_the_same_time_are_worth_the_same_to_a_leader_and_a_chaser() {
        let engine = ValuationEngine::new();
        for (id, value) in [(1, dec!(3000)), (2, dec!(1000)), (9, dec!(1000))] {
            let mut club = ClubState::new(ClubId(id));
            club.intrinsic_value = value;
            club.anchor_value = Some(dec!(1000));
            engine.club_states.insert(ClubId(id), club);
        }
        let before: Vec<Decimal> = [1, 2].iter().map(|id| engine.club_states.get(&ClubId(*id)).unwrap().fundamental_value()).collect();
        for id in [1, 2] {
            engine.process_event(MatchEvent::Goal { team_id: ClubId(id), opponent_id: ClubId(9), player_id: PlayerId(0), minute: 45 }, 0);
        }
        engine.apply_result_fade(ResultFade { keep: 0.9 });
        let gain = |id: u32, b: Decimal| {
            use rust_decimal::prelude::ToPrimitive;
            let anchor = 1000.0;
            let after = engine.club_states.get(&ClubId(id)).unwrap().fundamental_value().to_f64().unwrap();
            (after / anchor).ln() - 0.9 * (b.to_f64().unwrap() / anchor).ln()
        };
        assert!((gain(1, before[0]) - gain(2, before[1])).abs() < 1e-9);
    }

    #[test]
    fn a_club_without_an_anchor_never_fades() {
        let (engine, id) = engine_with_club(dec!(4000));
        engine.apply_result_fade(ResultFade { keep: 0.5 });
        assert_eq!(engine.club_states.get(&id).unwrap().intrinsic_value, dec!(4000));
    }

    #[test]
    fn a_redis_record_from_before_this_field_reads_as_no_noise() {
        let mut json = serde_json::to_value(ClubState::new(ClubId(1))).unwrap();
        json.as_object_mut().unwrap().remove("noise_log");
        let club: ClubState = serde_json::from_value(json).unwrap();
        assert_eq!(club.noise_log, 0.0);
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
mod heartbeat_publish_timestamp_tests {
    use super::*;

    /// A Heartbeat-only club (no real match event) must still get a fresh
    /// `EngineUpdate::Club.ts` on every tick - this is the field live price-
    /// history storage (`ssx-node::storage`, `ssx-executor::pubsub`) reads,
    /// specifically so removing Heartbeat's old `last_match_update` write
    /// (see the Heartbeat handler's own comment) doesn't silently freeze
    /// live price timestamps for a club that hasn't played in weeks.
    #[test]
    fn heartbeat_tick_publishes_a_fresh_ts_without_touching_last_match_update() {
        let mut engine = ValuationEngine::new();
        let (tx, mut rx) = mpsc::unbounded_channel();
        engine.set_update_channel(tx);

        let club_id = ClubId(1);
        engine.club_states.insert(club_id, ClubState::new(club_id));

        engine.process_event(MatchEvent::Heartbeat { team_id: club_id, noise: HeartbeatNoise::calibrate(0.0, 3600.0, 2.0), fade: ResultFade::NONE }, 555_000);

        let club = engine.club_states.get(&club_id).unwrap();
        assert_eq!(club.last_match_update, 0, "Heartbeat must not touch last_match_update - only a real event does");
        drop(club);

        let mut saw_fresh_club_ts = false;
        while let Ok(update) = rx.try_recv() {
            if let EngineUpdate::Club { id, ts, .. } = update {
                if id == club_id {
                    assert_eq!(ts, 555_000, "EngineUpdate::Club.ts should carry the tick's own current_ts even though last_match_update didn't move");
                    saw_fresh_club_ts = true;
                }
            }
        }
        assert!(saw_fresh_club_ts, "expected a Club update to have been published for this Heartbeat tick");
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