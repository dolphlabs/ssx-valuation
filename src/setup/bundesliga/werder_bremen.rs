use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4009);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(596.11); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Werder Bremen".to_string());

    let players = vec![
        (40401, "Michael Zetterer", Position::GK, dec!(35.00), dec!(1.0), false),
        (40402, "Mitchell Weiser", Position::RB, dec!(40.00), dec!(1.0), false),
        (40403, "Milos Veljkovic", Position::CB, dec!(30.00), dec!(1.0), false),
        (40404, "Marco Friedl", Position::CB, dec!(45.91), dec!(1.034), true),
        (40405, "Anthony Jung", Position::LB, dec!(20.00), dec!(1.0), false),
        (40406, "Senne Lynen", Position::CDM, dec!(39.51), dec!(1.040), false),
        (40407, "Jens Stage", Position::CM, dec!(73.01), dec!(1.069), false),
        (40408, "Romano Schmid", Position::CAM, dec!(71.80), dec!(1.055), false),
        (40409, "Felix Agu", Position::RM, dec!(34.07), dec!(1.036), false),
        (40410, "Marvin Ducksch", Position::ST, dec!(50.00), dec!(1.0), false),
        (40411, "Keke Topp", Position::ST, dec!(37.85), dec!(0.981), false),
        // Bench
        (40412, "Mio Backhaus", Position::GK, dec!(15.25), dec!(1.169), false),
        (40413, "Amos Pieper", Position::CB, dec!(32.81), dec!(1.069), false),
        (40414, "Niklas Stark", Position::CB, dec!(26.49), dec!(1.041), false),
        (40415, "Oliver Deman", Position::LB, dec!(35.23), dec!(1.104), false),
        (40416, "Skelly Alvero", Position::CDM, dec!(37.79), dec!(1.030), false),
        (40417, "Isak Hansen-Aaroen", Position::CAM, dec!(25.00), dec!(1.0), false),
        (40418, "Marco Grüll", Position::LW, dec!(52.78), dec!(1.049), false),
        (40419, "Justin Njinmah", Position::ST, dec!(73.54), dec!(1.032), false),
        (40420, "Derrick Köhn", Position::LB, dec!(35.00), dec!(1.0), false),
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
