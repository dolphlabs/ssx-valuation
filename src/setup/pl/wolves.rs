use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 11;
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id, "Wolverhampton Wanderers".to_string());
    
    club.intrinsic_value = dec!(480.00);
    
    // Rivals: Aston Villa (8)
    club.set_rival_factor(8, dec!(1.3));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1501, "Jose Sa", Position::GK, dec!(35.0), false),
        (1502, "Matheus Cunha", Position::ST, dec!(75.0), false),
        (1503, "Hwang Hee-chan", Position::LW, dec!(60.0), false),
        (1504, "Mario Lemina", Position::CDM, dec!(50.0), true),
        (1505, "Joao Gomes", Position::CM, dec!(60.0), false),
        (1506, "Nelson Semedo", Position::RB, dec!(40.0), false),
        (1507, "Rayan Ait-Nouri", Position::LB, dec!(55.0), false),
        (1508, "Toti Gomes", Position::CB, dec!(45.0), false),
        (1509, "Yerson Mosquera", Position::CB, dec!(40.0), false),
        (1510, "Andre", Position::CDM, dec!(55.0), false),
        (1511, "Jorgen Strand Larsen", Position::ST, dec!(50.0), false),
        (1512, "Dan Bentley", Position::GK, dec!(15.0), false),
        (1513, "Craig Dawson", Position::CB, dec!(30.0), false),
        (1514, "Santiago Bueno", Position::CB, dec!(35.0), false),
        (1515, "Matt Doherty", Position::RB, dec!(25.0), false),
        (1516, "Tommy Doyle", Position::CM, dec!(40.0), false),
        (1517, "Jean-Ricner Bellegarde", Position::CM, dec!(40.0), false),
        (1518, "Rodrigo Gomes", Position::RW, dec!(40.0), false),
        (1519, "Goncalo Guedes", Position::ST, dec!(45.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.names.insert(id, name.to_string());
        engine.player_states.insert(id, PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight: dec!(1.0),
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![val; 5],
            position: pos,
            is_captain: captain,
        });
    }
}
