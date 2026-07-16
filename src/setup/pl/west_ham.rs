use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(9);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "West Ham United".to_string());
    
    club.intrinsic_value = dec!(843.16);
    
    // Rivals: Tottenham (3), Chelsea (6), Arsenal (4)
    club.set_rival_factor(ClubId(3), dec!(1.4));
    club.set_rival_factor(ClubId(6), dec!(1.3));
    club.set_rival_factor(ClubId(4), dec!(1.2));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1401, "Alphonse Areola", Position::GK, dec!(49.28), dec!(1.038), false),
        (1402, "Jarrod Bowen", Position::RW, dec!(124.63), dec!(1.061), true),
        (1403, "Mohammed Kudus", Position::RW, dec!(85.00), dec!(1.0), false),
        (1404, "Lucas Paqueta", Position::CAM, dec!(98.73), dec!(1.089), false),
        (1405, "Edson Alvarez", Position::CDM, dec!(65.00), dec!(1.0), false),
        (1406, "Max Kilman", Position::CB, dec!(67.89), dec!(1.015), false),
        (1407, "Jean-Clair Todibo", Position::CB, dec!(51.20), dec!(0.964), false),
        (1408, "Emerson Palmieri", Position::LB, dec!(50.00), dec!(1.0), false),
        (1409, "Aaron Wan-Bissaka", Position::RB, dec!(57.05), dec!(1.044), false),
        (1410, "Niclas Fullkrug", Position::ST, dec!(62.52), dec!(1.017), false),
        (1411, "Guido Rodriguez", Position::CDM, dec!(46.36), dec!(1.026), false),
        (1412, "Lukasz Fabianski", Position::GK, dec!(15.00), dec!(1.0), false),
        (1413, "Konstantinos Mavropanos", Position::CB, dec!(58.98), dec!(1.090), false),
        (1414, "Vladimir Coufal", Position::RB, dec!(35.00), dec!(1.0), false),
        (1415, "Tomas Soucek", Position::CM, dec!(60.10), dec!(1.0), false),
        (1416, "Carlos Soler", Position::CM, dec!(55.00), dec!(1.0), false),
        (1417, "Crysencio Summerville", Position::LW, dec!(78.97), dec!(1.020), false),
        (1418, "Luis Guilherme", Position::RW, dec!(48.79), dec!(1.040), false),
        (1419, "Michail Antonio", Position::ST, dec!(40.00), dec!(1.0), false),
        (1420, "Danny Ings", Position::ST, dec!(30.00), dec!(1.0), false),
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
