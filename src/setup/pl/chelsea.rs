use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(6);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Chelsea".to_string());
    
    club.intrinsic_value = dec!(1152.66);
    
    // Rivals: Arsenal (4), Tottenham (3), West Ham (9)
    club.set_rival_factor(ClubId(4), dec!(1.4));
    club.set_rival_factor(ClubId(3), dec!(1.5));
    club.set_rival_factor(ClubId(9), dec!(1.2));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1251, "Robert Sanchez", Position::GK, dec!(45.71), dec!(0.987), false),
        (1252, "Levi Colwill", Position::CB, dec!(60.17), dec!(1.039), false),
        (1253, "Cole Palmer", Position::CAM, dec!(154.79), dec!(1.060), false),
        (1254, "Enzo Fernandez", Position::CM, dec!(113.50), dec!(1.070), true),
        (1255, "Moises Caicedo", Position::CDM, dec!(88.32), dec!(0.999), false),
        (1256, "Nicolas Jackson", Position::ST, dec!(70.00), dec!(1.0), false),
        (1257, "Noni Madueke", Position::RW, dec!(60.00), dec!(1.0), false),
        (1258, "Jadon Sancho", Position::LW, dec!(65.00), dec!(1.0), false),
        (1259, "Reece James", Position::RB, dec!(88.97), dec!(1.091), false),
        (1260, "Marc Cucurella", Position::LB, dec!(59.96), dec!(0.921), false),
        (1261, "Christopher Nkunku", Position::CF, dec!(80.00), dec!(1.0), false),
        (1262, "Filip Jorgensen", Position::GK, dec!(30.00), dec!(1.0), false),
        (1263, "Axel Disasi", Position::CB, dec!(50.00), dec!(1.0), false),
        (1264, "Tosin Adarabioyo", Position::CB, dec!(48.51), dec!(1.048), false),
        (1265, "Malo Gusto", Position::RB, dec!(66.01), dec!(0.997), false),
        (1266, "Romeo Lavia", Position::CDM, dec!(59.82), dec!(1.056), false),
        (1267, "Kiernan Dewsbury-Hall", Position::CM, dec!(50.00), dec!(1.0), false),
        (1268, "Joao Felix", Position::CAM, dec!(75.00), dec!(1.0), false),
        (1269, "Pedro Neto", Position::LW, dec!(83.65), dec!(1.021), false),
        (1270, "Mykhailo Mudryk", Position::LW, dec!(60.00), dec!(1.0), false),
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
