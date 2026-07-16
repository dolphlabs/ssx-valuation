use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(8);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Aston Villa".to_string());
    
    club.intrinsic_value = dec!(1175.16);
    
    // Rivals: Wolves (11), Leicester (19)
    club.set_rival_factor(ClubId(11), dec!(1.2));
    club.set_rival_factor(ClubId(19), dec!(1.1));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1351, "Emiliano Martinez", Position::GK, dec!(76.02), dec!(1.031), false),
        (1352, "Ollie Watkins", Position::ST, dec!(144.18), dec!(1.207), false),
        (1353, "Youri Tielemans", Position::CM, dec!(85.46), dec!(1.077), false),
        (1354, "Leon Bailey", Position::RW, dec!(77.12), dec!(1.027), false),
        (1355, "John McGinn", Position::CM, dec!(84.06), dec!(1.039), true),
        (1356, "Ezri Konsa", Position::CB, dec!(61.51), dec!(1.054), false),
        (1357, "Pau Torres", Position::CB, dec!(75.50), dec!(1.023), false),
        (1358, "Lucas Digne", Position::LB, dec!(57.35), dec!(1.072), false),
        (1359, "Matty Cash", Position::RB, dec!(54.46), dec!(1.019), false),
        (1360, "Amadou Onana", Position::CDM, dec!(95.00), dec!(1.035), false),
        (1361, "Morgan Rogers", Position::CAM, dec!(92.55), dec!(1.040), false),
        (1362, "Robin Olsen", Position::GK, dec!(20.00), dec!(1.0), false),
        (1363, "Diego Carlos", Position::CB, dec!(45.00), dec!(1.0), false),
        (1364, "Ian Maatsen", Position::LB, dec!(68.56), dec!(1.074), false),
        (1365, "Boubacar Kamara", Position::CDM, dec!(74.65), dec!(1.055), false),
        (1366, "Ross Barkley", Position::CM, dec!(50.67), dec!(1.065), false),
        (1367, "Jacob Ramsey", Position::CAM, dec!(60.00), dec!(1.0), false),
        (1368, "Jhon Duran", Position::ST, dec!(75.00), dec!(1.0), false),
        (1369, "Emi Buendia", Position::CAM, dec!(72.01), dec!(1.101), false),
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
