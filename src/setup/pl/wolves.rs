use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(11);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Wolverhampton Wanderers".to_string());
    
    club.intrinsic_value = dec!(518.45);
    
    // Rivals: Aston Villa (8)
    club.set_rival_factor(ClubId(8), dec!(1.3));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1501, "Jose Sa", Position::GK, dec!(33.97), dec!(1.033), false),
        (1502, "Matheus Cunha", Position::ST, dec!(75.00), dec!(1.0), false),
        (1503, "Hwang Hee-chan", Position::LW, dec!(65.05), dec!(1.017), false),
        (1504, "Mario Lemina", Position::CDM, dec!(50.00), dec!(1.0), true),
        (1505, "Joao Gomes", Position::CM, dec!(66.22), dec!(1.051), false),
        (1506, "Nelson Semedo", Position::RB, dec!(40.00), dec!(1.0), false),
        (1507, "Rayan Ait-Nouri", Position::LB, dec!(55.00), dec!(1.0), false),
        (1508, "Toti Gomes", Position::CB, dec!(45.87), dec!(1.063), false),
        (1509, "Yerson Mosquera", Position::CB, dec!(39.04), dec!(1.037), false),
        (1510, "Andre", Position::CDM, dec!(58.75), dec!(0.982), false),
        (1511, "Jorgen Strand Larsen", Position::ST, dec!(51.23), dec!(1.021), false),
        (1512, "Dan Bentley", Position::GK, dec!(14.99), dec!(0.991), false),
        (1513, "Craig Dawson", Position::CB, dec!(30.00), dec!(1.0), false),
        (1514, "Santiago Bueno", Position::CB, dec!(47.34), dec!(1.028), false),
        (1515, "Matt Doherty", Position::RB, dec!(24.14), dec!(1.063), false),
        (1516, "Tommy Doyle", Position::CM, dec!(40.00), dec!(1.0), false),
        (1517, "Jean-Ricner Bellegarde", Position::CM, dec!(46.97), dec!(1.018), false),
        (1518, "Rodrigo Gomes", Position::RW, dec!(46.42), dec!(1.030), false),
        (1519, "Goncalo Guedes", Position::ST, dec!(45.00), dec!(1.0), false),
    ];

    for (id, name, pos, val, form_weight, captain) in players {
        engine.names.insert(id, name.to_string());
        engine.player_states.insert(PlayerId(id), PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight,
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![val; 5],
            position: pos,
            is_captain: captain,
        });
    }
}
