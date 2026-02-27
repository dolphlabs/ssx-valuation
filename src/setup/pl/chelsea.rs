use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 6;
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id, "Chelsea".to_string());
    
    club.intrinsic_value = dec!(850.00);
    
    // Rivals: Arsenal (4), Tottenham (3), West Ham (9)
    club.set_rival_factor(4, dec!(1.4));
    club.set_rival_factor(3, dec!(1.5));
    club.set_rival_factor(9, dec!(1.2));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1251, "Robert Sanchez", Position::GK, dec!(45.0), false),
        (1252, "Levi Colwill", Position::CB, dec!(60.0), false),
        (1253, "Cole Palmer", Position::CAM, dec!(110.0), false),
        (1254, "Enzo Fernandez", Position::CM, dec!(80.0), true),
        (1255, "Moises Caicedo", Position::CDM, dec!(85.0), false),
        (1256, "Nicolas Jackson", Position::ST, dec!(70.0), false),
        (1257, "Noni Madueke", Position::RW, dec!(60.0), false),
        (1258, "Jadon Sancho", Position::LW, dec!(65.0), false),
        (1259, "Reece James", Position::RB, dec!(70.0), false),
        (1260, "Marc Cucurella", Position::LB, dec!(55.0), false),
        (1261, "Christopher Nkunku", Position::CF, dec!(80.0), false),
        (1262, "Filip Jorgensen", Position::GK, dec!(30.0), false),
        (1263, "Axel Disasi", Position::CB, dec!(50.0), false),
        (1264, "Tosin Adarabioyo", Position::CB, dec!(45.0), false),
        (1265, "Malo Gusto", Position::RB, dec!(60.0), false),
        (1266, "Romeo Lavia", Position::CDM, dec!(55.0), false),
        (1267, "Kiernan Dewsbury-Hall", Position::CM, dec!(50.0), false),
        (1268, "Joao Felix", Position::CAM, dec!(75.0), false),
        (1269, "Pedro Neto", Position::LW, dec!(80.0), false),
        (1270, "Mykhailo Mudryk", Position::LW, dec!(60.0), false),
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
