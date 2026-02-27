use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 8;
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id, "Aston Villa".to_string());
    
    club.intrinsic_value = dec!(820.00);
    
    // Rivals: Wolves (11), Leicester (19)
    club.set_rival_factor(11, dec!(1.2));
    club.set_rival_factor(19, dec!(1.1));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1351, "Emiliano Martinez", Position::GK, dec!(65.0), false),
        (1352, "Ollie Watkins", Position::ST, dec!(100.0), false),
        (1353, "Youri Tielemans", Position::CM, dec!(70.0), false),
        (1354, "Leon Bailey", Position::RW, dec!(75.0), false),
        (1355, "John McGinn", Position::CM, dec!(65.0), true),
        (1356, "Ezri Konsa", Position::CB, dec!(60.0), false),
        (1357, "Pau Torres", Position::CB, dec!(65.0), false),
        (1358, "Lucas Digne", Position::LB, dec!(50.0), false),
        (1359, "Matty Cash", Position::RB, dec!(45.0), false),
        (1360, "Amadou Onana", Position::CDM, dec!(75.0), false),
        (1361, "Morgan Rogers", Position::CAM, dec!(65.0), false),
        (1362, "Robin Olsen", Position::GK, dec!(20.0), false),
        (1363, "Diego Carlos", Position::CB, dec!(45.0), false),
        (1364, "Ian Maatsen", Position::LB, dec!(60.0), false),
        (1365, "Boubacar Kamara", Position::CDM, dec!(65.0), false),
        (1366, "Ross Barkley", Position::CM, dec!(40.0), false),
        (1367, "Jacob Ramsey", Position::CAM, dec!(60.0), false),
        (1368, "Jhon Duran", Position::ST, dec!(75.0), false),
        (1369, "Emi Buendia", Position::CAM, dec!(50.0), false),
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
