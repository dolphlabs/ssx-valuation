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
    Heartbeat { team_id: ClubId },
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

    pub fn apply_leaking_value(&self, current_ts: u64) {
        let seven_days_secs = 7 * 24 * 60 * 60;
        let one_day_secs = 24 * 60 * 60;
        let leak_factor = dec!(1.0) - dec!(0.01);

        for mut club in self.club_states.iter_mut() {
            if current_ts > club.last_match_update + seven_days_secs {
                let days_inactive = (current_ts - club.last_match_update) / one_day_secs;
                for _ in 0..days_inactive {
                    club.intrinsic_value *= leak_factor;
                }
                club.last_match_update = current_ts;
                self.notify(EngineUpdate::Club { id: club.id, state: club.clone() });
            }
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
                let rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    let time_multiplier = Decimal::from(minute) / dec!(90.0);
                    let impact = (dec!(5.0) + (dec!(10.0) * time_multiplier)) * rivalry_multiplier;
                    club.intrinsic_value += impact;
                    club.last_match_update = current_ts;
                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone() });
                }
                if let Some(mut player) = self.player_states.get_mut(&player_id) {
                    let pos_multiplier = player.position.base_weight();
                    let player_impact = dec!(15.0) * pos_multiplier * player.form_weight * rivalry_multiplier;
                    player.intrinsic_value += player_impact;
                    self.notify(EngineUpdate::Player { id: player_id, state: player.clone() });
                }
            }
            MatchEvent::RedCard { team_id, opponent_id, player_id } => {
                let rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    club.intrinsic_value -= dec!(7.5) * rivalry_multiplier;
                    club.last_match_update = current_ts;
                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone() });
                }
                if let Some(mut player) = self.player_states.get_mut(&player_id) {
                    player.form_weight *= dec!(0.5);
                    self.notify(EngineUpdate::Player { id: player_id, state: player.clone() });
                }
            }
            MatchEvent::YellowCard { team_id, opponent_id, player_id } => {
                let rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    club.intrinsic_value -= dec!(2.5) * rivalry_multiplier;
                    club.last_match_update = current_ts;
                    self.notify(EngineUpdate::Club { id: team_id, state: club.clone() });
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
            MatchEvent::Heartbeat { team_id } => {
                if let Some(mut club) = self.club_states.get_mut(&team_id) {
                    club.last_match_update = current_ts;
                    
                    // Live Market Simulation: Geometric Brownian Motion-style drift
                    // We use the volatility factor of the captain or a default
                    // Todo: it should not be random
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    
                    // Random walk: -0.05% to +0.05% change per heartbeat
                    let change_pct = dec!(1.0) + (Decimal::from_f64_retain(rng.gen_range(-0.0005..0.0005)).unwrap_or(dec!(0)));
                    club.intrinsic_value *= change_pct;
                    
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

pub trait ValuationCalculator {
    fn calculate_base_valuation(&self, p: Decimal, f: Decimal, s: Decimal) -> Decimal;
    fn calculate_goal_impact(&self, v_base: Decimal, t_remaining: Decimal) -> Decimal;
}

impl ValuationCalculator for ValuationEngine {
    fn calculate_base_valuation(&self, p: Decimal, f: Decimal, s: Decimal) -> Decimal {
        let (wp, wf, ws) = if self.is_transfer_window.load(std::sync::atomic::Ordering::Relaxed) {
            (dec!(0.10), dec!(0.80), dec!(0.10))
        } else {
            (dec!(0.60), dec!(0.30), dec!(0.10))
        };
        (wp * p) + (wf * f) + (ws * s)
    }

    fn calculate_goal_impact(&self, v_base: Decimal, t_remaining: Decimal) -> Decimal {
        let g = dec!(0.05);
        let one_second = dec!(1) / dec!(60);
        let t_rem_capped = if t_remaining < one_second { one_second } else { t_remaining };
        v_base * (dec!(1) + (g / t_rem_capped))
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