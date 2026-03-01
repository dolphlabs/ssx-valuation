use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4008);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(350.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "FCH Heidenheim".to_string());

    let players = vec![
        (40351, "Kevin Muller", Position::GK, dec!(30.0), false),
        (40352, "Marnon Busch", Position::RB, dec!(20.0), false),
        (40353, "Patrick Mainka", Position::CB, dec!(30.0), true),
        (40354, "Benedikt Gimber", Position::CB, dec!(25.0), false),
        (40355, "Jonas Fohrenbach", Position::LB, dec!(25.0), false),
        (40356, "Lennard Maloney", Position::CDM, dec!(35.0), false),
        (40357, "Jan Schoppner", Position::CM, dec!(30.0), false),
        (40358, "Paul Wanner", Position::CAM, dec!(55.0), false),
        (40359, "Leo Scienza", Position::LW, dec!(35.0), false),
        (40360, "Sirlord Conteh", Position::RW, dec!(25.0), false),
        (40361, "Marvin Pieringer", Position::ST, dec!(40.0), false),
        // Bench
        (40362, "Frank Feller", Position::GK, dec!(5.0), false),
        (40363, "Tim Siersleben", Position::CB, dec!(20.0), false),
        (40364, "Haktab Traore", Position::RB, dec!(15.0), false),
        (40365, "Norman Theuerkauf", Position::LB, dec!(10.0), false),
        (40366, "Niklas Dorsch", Position::CM, dec!(35.0), false),
        (40367, "Adrian Beck", Position::CAM, dec!(20.0), false),
        (40368, "Mathias Honsak", Position::LW, dec!(25.0), false),
        (40369, "Mikkel Kaufmann", Position::ST, dec!(30.0), false),
        (40370, "Stefan Schimmer", Position::ST, dec!(15.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.player_states.insert(PlayerId(id), PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight: dec!(1.0),
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![],
            position: pos,
            is_captain: captain,
        });
        engine.names.insert(id, name.to_string());
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(PlayerId(id));
        }
    }
}
