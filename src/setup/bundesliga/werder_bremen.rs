use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4009);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(420.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Werder Bremen".to_string());

    let players = vec![
        (40401, "Michael Zetterer", Position::GK, dec!(35.0), false),
        (40402, "Mitchell Weiser", Position::RB, dec!(40.0), false),
        (40403, "Milos Veljkovic", Position::CB, dec!(30.0), false),
        (40404, "Marco Friedl", Position::CB, dec!(40.0), true),
        (40405, "Anthony Jung", Position::LB, dec!(20.0), false),
        (40406, "Senne Lynen", Position::CDM, dec!(35.0), false),
        (40407, "Jens Stage", Position::CM, dec!(45.0), false),
        (40408, "Romano Schmid", Position::CAM, dec!(45.0), false),
        (40409, "Felix Agu", Position::RM, dec!(30.0), false),
        (40410, "Marvin Ducksch", Position::ST, dec!(50.0), false),
        (40411, "Keke Topp", Position::ST, dec!(35.0), false),
        // Bench
        (40412, "Mio Backhaus", Position::GK, dec!(15.0), false),
        (40413, "Amos Pieper", Position::CB, dec!(30.0), false),
        (40414, "Niklas Stark", Position::CB, dec!(35.0), false),
        (40415, "Oliver Deman", Position::LB, dec!(30.0), false),
        (40416, "Skelly Alvero", Position::CDM, dec!(35.0), false),
        (40417, "Isak Hansen-Aaroen", Position::CAM, dec!(25.0), false),
        (40418, "Marco Grüll", Position::LW, dec!(40.0), false),
        (40419, "Justin Njinmah", Position::ST, dec!(40.0), false),
        (40420, "Derrick Köhn", Position::LB, dec!(35.0), false),
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
