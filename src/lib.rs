use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub struct ValuationState {
    pub intrinsic_value: Decimal,
    pub volatility_factor: Decimal,
    pub last_update_ts: u64,
}

#[derive(Debug, Clone)]
pub enum MatchEvent {
    Goal { team_id: u32, minute: u32 },
    YellowCard { team_id: u32 },
    RedCard { team_id: u32 },
    WhistleEnd,
}

pub struct  ClubState {
    pub id: u32,
    pub name: String,
    pub intrinsic_value: Decimal,
}

pub trait Valuator {
    /// Calculates the new fair price based on a match event.
    fn process_event(&mut self, event: MatchEvent) -> ValuationState;
    
    /// Applies the 24/7 "Value Leakage" for off-season/quiet periods.
    fn apply_decay(&mut self, current_ts: u64);
}

impl ClubState {
    pub fn new(id: u32, name: &str) -> Self {
        // initialise with '"100.00" as starting index
        Self { 
            id, 
            name: (name.to_string()), 
            intrinsic_value: (dec!(100.00)) 
        }
    }

    /// This is just the basic implementation as they'll be need to look at the match priority scale based on what tournament they are playing, their position n the table, the importance of the game and the condition of their players.

    pub fn apply_event(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::Goal { team_id, minute } => {
                if *team_id == self.id {
                    // Impact increases as the game progresses (Time-Weighting)
                    let time_multiplier = Decimal::from(*minute) / dec!(90.0);
                    let impact = dec!(5.0) + (dec!(10.0) * time_multiplier);
                    self.intrinsic_value += impact;
                }
            }
            MatchEvent::YellowCard { team_id } => {
                if *team_id == self.id {
                    // This value will not be a constant but rather be affected by several factors
                    self.intrinsic_value -= dec!(0.5);
                }
            }
            MatchEvent::RedCard { team_id } => {
                if *team_id == self.id {
                    // This value will not be a constant but rather be affected by several factors
                    self.intrinsic_value -= dec!(0.75);
                }
            }
            _ => {}
        }
    }
}