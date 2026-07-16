use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4008);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(627.67); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "FCH Heidenheim".to_string());

    let players = vec![
        (40351, "Kevin Muller", Position::GK, dec!(30.00), dec!(1.0), false),
        (40352, "Marnon Busch", Position::RB, dec!(25.27), dec!(1.049), false),
        (40353, "Patrick Mainka", Position::CB, dec!(43.26), dec!(1.092), true),
        (40354, "Benedikt Gimber", Position::CB, dec!(27.27), dec!(1.084), false),
        (40355, "Jonas Fohrenbach", Position::LB, dec!(32.60), dec!(1.085), false),
        (40356, "Lennard Maloney", Position::CDM, dec!(35.00), dec!(1.0), false),
        (40357, "Jan Schoppner", Position::CM, dec!(36.06), dec!(1.062), false),
        (40358, "Paul Wanner", Position::CAM, dec!(55.00), dec!(1.0), false),
        (40359, "Leo Scienza", Position::LW, dec!(66.34), dec!(1.084), false),
        (40360, "Sirlord Conteh", Position::RW, dec!(38.74), dec!(1.014), false),
        (40361, "Marvin Pieringer", Position::ST, dec!(52.03), dec!(1.014), false),
        // Bench
        (40362, "Frank Feller", Position::GK, dec!(5.12), dec!(1.117), false),
        (40363, "Tim Siersleben", Position::CB, dec!(22.19), dec!(1.006), false),
        (40364, "Haktab Traore", Position::RB, dec!(16.21), dec!(1.071), false),
        (40365, "Norman Theuerkauf", Position::LB, dec!(10.00), dec!(1.0), false),
        (40366, "Niklas Dorsch", Position::CM, dec!(40.23), dec!(1.055), false),
        (40367, "Adrian Beck", Position::CAM, dec!(25.90), dec!(1.045), false),
        (40368, "Mathias Honsak", Position::LW, dec!(33.35), dec!(1.045), false),
        (40369, "Mikkel Kaufmann", Position::ST, dec!(36.93), dec!(1.025), false),
        (40370, "Stefan Schimmer", Position::ST, dec!(31.67), dec!(1.029), false),
    ];

    for (id, name, pos, val, form_weight, captain) in players {
        engine.player_states.insert(PlayerId(id), PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight,
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
