use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 9;
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id, "West Ham United".to_string());
    
    club.intrinsic_value = dec!(650.00);
    
    // Rivals: Tottenham (3), Chelsea (6), Arsenal (4)
    club.set_rival_factor(3, dec!(1.4));
    club.set_rival_factor(6, dec!(1.3));
    club.set_rival_factor(4, dec!(1.2));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1401, "Alphonse Areola", Position::GK, dec!(45.0), false),
        (1402, "Jarrod Bowen", Position::RW, dec!(85.0), true),
        (1403, "Mohammed Kudus", Position::RW, dec!(85.0), false),
        (1404, "Lucas Paqueta", Position::CAM, dec!(80.0), false),
        (1405, "Edson Alvarez", Position::CDM, dec!(65.0), false),
        (1406, "Max Kilman", Position::CB, dec!(65.0), false),
        (1407, "Jean-Clair Todibo", Position::CB, dec!(60.0), false),
        (1408, "Emerson Palmieri", Position::LB, dec!(50.0), false),
        (1409, "Aaron Wan-Bissaka", Position::RB, dec!(55.0), false),
        (1410, "Niclas Fullkrug", Position::ST, dec!(60.0), false),
        (1411, "Guido Rodriguez", Position::CDM, dec!(45.0), false),
        (1412, "Lukasz Fabianski", Position::GK, dec!(15.0), false),
        (1413, "Konstantinos Mavropanos", Position::CB, dec!(45.0), false),
        (1414, "Vladimir Coufal", Position::RB, dec!(35.0), false),
        (1415, "Tomas Soucek", Position::CM, dec!(50.0), false),
        (1416, "Carlos Soler", Position::CM, dec!(55.0), false),
        (1417, "Crysencio Summerville", Position::LW, dec!(60.0), false),
        (1418, "Luis Guilherme", Position::RW, dec!(45.0), false),
        (1419, "Michail Antonio", Position::ST, dec!(40.0), false),
        (1420, "Danny Ings", Position::ST, dec!(30.0), false),
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
