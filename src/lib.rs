use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

// --- Core Valuation Types ---

pub struct ValuationState {
    pub intrinsic_value: Decimal,
    pub volatility_factor: Decimal,
    pub last_update_ts: u64,
}

#[derive(Debug, Clone)]
pub enum MatchEvent {
    Goal { team_id: u32, player_id: u32, minute: u32 },
    YellowCard { team_id: u32, player_id: u32 },
    RedCard { team_id: u32, player_id: u32 },
    Injury { player_id: u32, severity: u32 },
    WhistleEnd,
}

// --- Position Modeling ---

/// Represents all standard football positions.
/// Using #[repr(u8)] ensures this occupies only 1 byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Position {
    GK = 0,
    CB, LB, RB, LWB, RWB,
    CDM, CM, CAM, LM, RM,
    ST, CF, LW, RW,
}

impl Position {
    /// Returns the "SSX Importance Map" weight.
    /// Compiled as a jump table for O(1) performance.
    pub fn base_weight(&self) -> Decimal {
        match self {
            Position::GK => dec!(1.00),
            Position::CB | Position::LB | Position::RB => dec!(0.90),
            Position::CAM | Position::ST | Position::CF => dec!(0.95),
            Position::LW | Position::RW => dec!(0.98),
            Position::LWB | Position::RWB | Position::LM | Position::RM => dec!(0.92),
            Position::CDM => dec!(0.99),
            Position::CM => dec!(0.94), // Added missing CM case
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Position::GK => "Goalkeeper",
            Position::CB => "Center Back",
            Position::ST => "Striker",
            Position::CAM => "Centre Attacking Midfielder",
            Position::CDM => "Centre Defending Midfielder",
            Position::LB => "Left Back",
            Position::RB => "Right Back",
            Position::LWB => "Left-Wing Back",
            Position::RWB => "Right-Wing Back",
            Position::LW => "Left Winger",
            Position::RW => "Right Winger",
            Position::CF => "Centre Forward",
            Position::LM => "Left Middle",
            Position::RM => "Right Middle",
            Position::CM => "Central Midfielder",
        }
    }
}

// --- State Definitions ---

/// "Hot" Player Data: Minimal size for high cache-hit ratio.
#[derive(Debug, Clone, Copy)]
pub struct PlayerValues {
    pub team_id: u32,
    pub intrinsic_value: Decimal,
    pub form_weight: Decimal,
    pub position: Position,
    pub is_captain: bool,
}

pub struct ClubState {
    pub id: u32,
    pub intrinsic_value: Decimal,
    /// Using BTreeMap for small, fixed-size relationship maps.
    pub top_oppositions: BTreeMap<u32, Decimal>,
    pub rivals: Vec<(u32, Decimal)>,
    /// List of player IDs belonging to this club.
    pub player_ids: Vec<u32>,
}

// --- Engine Implementation ---

pub struct ValuationEngine {
    /// Primary storage for arithmetic.
    pub player_states: HashMap<u32, PlayerValues>,
    pub club_states: HashMap<u32, ClubState>,
    /// Metadata: Kept separate to keep the states "thin".
    pub names: HashMap<u32, String>, 
}

impl ValuationEngine {
    pub fn new() -> Self {
        Self {
            player_states: HashMap::new(),
            club_states: HashMap::new(),
            names: HashMap::new(),
        }
    }

    /// Global decay function for off-season/inactivity.
    pub fn apply_global_decay(&mut self, decay_factor: Decimal) {
        for player in self.player_states.values_mut() {
            player.intrinsic_value *= decay_factor;
        }
        for club in self.club_states.values_mut() {
            club.intrinsic_value *= decay_factor;
        }
    }

    /// Orchestrates a match event and its cascading effects.
    pub fn process_event(&mut self, event: MatchEvent) {
        match event {
            MatchEvent::Goal { team_id, player_id, minute } => {
                // 1. Update Club Value
                if let Some(club) = self.club_states.get_mut(&team_id) {
                    let time_multiplier = Decimal::from(minute) / dec!(90.0);
                    let impact = dec!(5.0) + (dec!(10.0) * time_multiplier);
                    club.intrinsic_value += impact;
                }

                // 2. Update Player Value with Position Weighting
                if let Some(player) = self.player_states.get_mut(&player_id) {
                    let pos_multiplier = player.position.base_weight();
                    let player_impact = dec!(15.0) * pos_multiplier * player.form_weight;
                    player.intrinsic_value += player_impact;
                }
            }
            MatchEvent::Injury { player_id, severity } => {
                if let Some(player) = self.player_states.get_mut(&player_id) {
                    // Injuries reduce intrinsic value based on severity (0-100 scale)
                    let loss = player.intrinsic_value * (Decimal::from(severity) / dec!(100.0));
                    player.intrinsic_value -= loss;
                }
            }
            // Additional event logic (Red cards, etc.) would follow here...
            _ => {}
        }
    }
}