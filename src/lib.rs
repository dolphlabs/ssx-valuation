use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

pub mod replay;

// --- Core Valuation Types ---

#[derive(Debug, Clone)]
pub enum MatchEvent {
    Goal { team_id: u32, opponent_id: u32, player_id: u32, minute: u32 },
    YellowCard { team_id: u32, opponent_id: u32, player_id: u32 },
    RedCard { team_id: u32, opponent_id: u32, player_id: u32 },
    Injury { player_id: u32, severity: u32 },
    WhistleEnd { team_a_id: u32, team_b_id: u32 },
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

#[derive(Debug, Clone)]
pub struct PlayerValues {
    pub team_id: u32,
    pub intrinsic_value: Decimal, // Performance Index (P)
    pub form_weight: Decimal,     // Form (F)
    pub sentiment_score: Decimal, // Sentiment (S)
    pub volatility_factor: Decimal,
    pub performance_history: Vec<Decimal>, // Rolling 10-match average
    pub position: Position,
    pub is_captain: bool,
}

pub struct ClubState {
    pub id: u32,
    pub intrinsic_value: Decimal,
    pub last_match_update: u64,
    pub top_oppositions: BTreeMap<u32, Decimal>,
    pub rivals: Vec<(u32, Decimal)>,
    pub player_ids: Vec<u32>,
}

// --- Engine Implementation ---

pub struct ValuationEngine {
    pub player_states: HashMap<u32, PlayerValues>,
    pub club_states: HashMap<u32, ClubState>,
    pub names: HashMap<u32, String>,
    pub is_transfer_window: bool,
}

impl ValuationEngine {
    pub fn new() -> Self {
        Self::with_capacity(2000)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            player_states: HashMap::with_capacity(cap),
            club_states: HashMap::with_capacity(cap / 11),
            names: HashMap::with_capacity(cap),
            is_transfer_window: false,
        }
    }

    pub fn update_financial_metric(&mut self, is_window: bool) {
        self.is_transfer_window = is_window;
    }

    pub fn apply_leaking_value(&mut self, current_ts: u64) {
        let seven_days_secs = 7 * 24 * 60 * 60;
        let one_day_secs = 24 * 60 * 60;
        let leak_factor = dec!(1.0) - dec!(0.01);

        for club in self.club_states.values_mut() {
            if current_ts > club.last_match_update + seven_days_secs {
                let days_inactive = (current_ts - club.last_match_update) / one_day_secs;
                for _ in 0..days_inactive {
                    club.intrinsic_value *= leak_factor;
                }
                club.last_match_update = current_ts; 
            }
        }
    }

    pub fn get_club_pair_exchange_rate(&self, base_id: u32, quote_id: u32) -> Decimal {
        let v_base = self.club_states.get(&base_id).map(|c| c.intrinsic_value).unwrap_or(dec!(0));
        let v_quote = self.club_states.get(&quote_id).map(|c| c.intrinsic_value).unwrap_or(dec!(1));
        if v_quote.is_zero() { dec!(0) } else { v_base / v_quote }
    }

    pub fn process_event(&mut self, event: MatchEvent, current_ts: u64) {
        match event {
            MatchEvent::Goal { team_id, opponent_id, player_id, minute } => {
                let rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
                if let Some(club) = self.club_states.get_mut(&team_id) {
                    let time_multiplier = Decimal::from(minute) / dec!(90.0);
                    let impact = (dec!(5.0) + (dec!(10.0) * time_multiplier)) * rivalry_multiplier;
                    club.intrinsic_value += impact;
                    club.last_match_update = current_ts;
                }
                if let Some(player) = self.player_states.get_mut(&player_id) {
                    let pos_multiplier = player.position.base_weight();
                    let player_impact = dec!(15.0) * pos_multiplier * player.form_weight * rivalry_multiplier;
                    player.intrinsic_value += player_impact;
                }
            }
            MatchEvent::RedCard { team_id, opponent_id, player_id } => {
                let rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
                if let Some(club) = self.club_states.get_mut(&team_id) {
                    club.intrinsic_value -= dec!(7.5) * rivalry_multiplier;
                    club.last_match_update = current_ts;
                }
                if let Some(player) = self.player_states.get_mut(&player_id) {
                    player.form_weight *= dec!(0.5);
                }
            }
            MatchEvent::YellowCard { team_id, opponent_id, player_id } => {
                let rivalry_multiplier = self.get_rivalry_multiplier(team_id, opponent_id);
                if let Some(club) = self.club_states.get_mut(&team_id) {
                    club.intrinsic_value -= dec!(2.5) * rivalry_multiplier;
                    club.last_match_update = current_ts;
                }
                if let Some(player) = self.player_states.get_mut(&player_id) {
                    player.form_weight *= dec!(0.9);
                }
            }
            MatchEvent::Injury { player_id, severity } => {
                if let Some(player) = self.player_states.get_mut(&player_id) {
                    let loss = player.intrinsic_value * (Decimal::from(severity) / dec!(100.0));
                    player.intrinsic_value -= loss;
                }
            }
            MatchEvent::WhistleEnd { team_a_id, team_b_id } => {
                let is_window = self.is_transfer_window;
                for &team_id in &[team_a_id, team_b_id] {
                    if let Some(club) = self.club_states.get_mut(&team_id) {
                        club.last_match_update = current_ts;
                    }
                    
                    let pids: Vec<u32> = self.player_states.iter()
                        .filter(|(_, p)| p.team_id == team_id)
                        .map(|(&id, _)| id)
                        .collect();

                    for pid in pids {
                        if let Some(player) = self.player_states.get_mut(&pid) {
                            player.volatility_factor = dec!(1.0);

                            // Calculate V manually to avoid double borrow on self
                            let (wp, wf, ws) = if is_window {
                                (dec!(0.10), dec!(0.80), dec!(0.10))
                            } else {
                                (dec!(0.60), dec!(0.30), dec!(0.10))
                            };
                            let v = (wp * player.intrinsic_value) + (wf * player.form_weight) + (ws * player.sentiment_score);

                            player.performance_history.push(v);
                            if player.performance_history.len() > 10 {
                                player.performance_history.remove(0);
                            }

                            let sum: Decimal = player.performance_history.iter().sum();
                            player.intrinsic_value = sum / Decimal::from(player.performance_history.len());
                        }
                    }
                }
            }
        }
    }

    fn get_rivalry_multiplier(&self, team_id: u32, opponent_id: u32) -> Decimal {
        self.club_states.get(&team_id)
            .and_then(|club| {
                club.rivals.iter()
                    .find(|(rid, _)| *rid == opponent_id)
                    .map(|(_, multiplier)| *multiplier)
            })
            .unwrap_or(dec!(1.0))
    }
}

// --- Valuation Trait Definitions ---

pub trait ValuationCalculator {
    fn calculate_base_valuation(&self, p: Decimal, f: Decimal, s: Decimal) -> Decimal;
    fn calculate_goal_impact(&self, v_base: Decimal, t_remaining: Decimal) -> Decimal;
}

impl ValuationCalculator for ValuationEngine {
    fn calculate_base_valuation(&self, p: Decimal, f: Decimal, s: Decimal) -> Decimal {
        let (wp, wf, ws) = if self.is_transfer_window {
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
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_base_valuation() {
        let engine = ValuationEngine::new();
        let p = dec!(100.0);
        let f = dec!(1.0);
        let s = dec!(1.2);
        // V = (0.6 * 100) + (0.3 * 1.0) + (0.1 * 1.2) = 60.42
        let result = engine.calculate_base_valuation(p, f, s);
        assert_eq!(result, dec!(60.42));
    }

    #[test]
    fn test_goal_impact() {
        let engine = ValuationEngine::new();
        let v_base = dec!(100.0);
        let result = engine.calculate_goal_impact(v_base, dec!(10.0));
        assert_eq!(result, dec!(100.5));
        let result_cap = engine.calculate_goal_impact(v_base, dec!(0.0));
        assert_eq!(result_cap.round_dp(12), dec!(400.0));
    }

    #[test]
    fn test_rivalry_and_cards() {
        let mut engine = ValuationEngine::with_capacity(10);
        let team_a_id = 1;
        let team_b_id = 2; // Rival
        
        engine.club_states.insert(team_a_id, ClubState {
            id: team_a_id,
            intrinsic_value: dec!(1000.0),
            last_match_update: 0,
            top_oppositions: BTreeMap::new(),
            rivals: vec![(team_b_id, dec!(2.0))],
            player_ids: vec![101],
        });

        engine.player_states.insert(101, PlayerValues {
            team_id: team_a_id,
            intrinsic_value: dec!(500.0),
            form_weight: dec!(1.0),
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![],
            position: Position::ST,
            is_captain: false,
        });

        // Test Goal with Rivalry (2x)
        engine.process_event(MatchEvent::Goal { team_id: team_a_id, opponent_id: team_b_id, player_id: 101, minute: 45 }, 100);
        assert_eq!(engine.club_states[&team_a_id].intrinsic_value, dec!(1020.0));
        assert_eq!(engine.player_states[&101].intrinsic_value, dec!(528.5));

        // Test Yellow Card (2x rivalry)
        engine.process_event(MatchEvent::YellowCard { team_id: team_a_id, opponent_id: team_b_id, player_id: 101 }, 200);
        assert_eq!(engine.club_states[&team_a_id].intrinsic_value, dec!(1015.0));
        assert_eq!(engine.player_states[&101].form_weight, dec!(0.9));
    }

    #[test]
    fn test_off_season_leak() {
        let mut engine = ValuationEngine::new();
        let club_id = 1;
        engine.club_states.insert(club_id, ClubState {
            id: club_id,
            intrinsic_value: dec!(100.0),
            last_match_update: 1000,
            top_oppositions: BTreeMap::new(),
            rivals: vec![],
            player_ids: vec![],
        });

        let current_ts = 1000 + 691200;
        engine.apply_leaking_value(current_ts);

        let val = engine.club_states[&club_id].intrinsic_value;
        assert!(val < dec!(93.0) && val > dec!(92.0));
    }

    #[test]
    fn test_transfer_window_weights() {
        let mut engine = ValuationEngine::new();
        let p = dec!(100.0);
        let f = dec!(10.0);
        let s = dec!(1.0);

        assert_eq!(engine.calculate_base_valuation(p, f, s), dec!(63.1));

        engine.update_financial_metric(true);
        assert_eq!(engine.calculate_base_valuation(p, f, s), dec!(18.1));
    }

    #[test]
    fn test_whistle_end_stabilization() {
        let mut engine = ValuationEngine::new();
        let player_id = 101;
        engine.player_states.insert(player_id, PlayerValues {
            team_id: 1,
            intrinsic_value: dec!(100.0),
            form_weight: dec!(1.0),
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(5.5),
            performance_history: vec![dec!(100.0); 9],
            position: Position::ST,
            is_captain: false,
        });
        engine.club_states.insert(1, ClubState {
            id: 1,
            intrinsic_value: dec!(1000.0),
            last_match_update: 0,
            top_oppositions: BTreeMap::new(),
            rivals: vec![],
            player_ids: vec![101],
        });

        engine.process_event(MatchEvent::WhistleEnd { team_a_id: 1, team_b_id: 2 }, 1000);

        let p = &engine.player_states[&player_id];
        assert_eq!(p.volatility_factor, dec!(1.0));
        assert_eq!(p.performance_history.len(), 10);
    }
}