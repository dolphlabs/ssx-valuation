use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Manchester United".to_string());
    
    club.intrinsic_value = dec!(700.00); 
    
    // Rivals: Man City (1), Liverpool (5), Arsenal (4)
    club.set_rival_factor(ClubId(1), dec!(1.5));
    club.set_rival_factor(ClubId(5), dec!(1.6));
    club.set_rival_factor(ClubId(4), dec!(1.3));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1051, "Andre Onana", Position::GK, dec!(55.0), false),
        (1052, "Lisandro Martinez", Position::CB, dec!(65.0), false),
        (1053, "Bruno Fernandes", Position::CAM, dec!(90.0), true),
        (1054, "Marcus Rashford", Position::LW, dec!(80.0), false),
        (1055, "Rasmus Hojlund", Position::ST, dec!(70.0), false),
        (1056, "Kobbie Mainoo", Position::CM, dec!(60.0), false),
        (1057, "Alejandro Garnacho", Position::RW, dec!(65.0), false),
        (1058, "Casemiro", Position::CDM, dec!(50.0), false),
        (1059, "Diogo Dalot", Position::RB, dec!(55.0), false),
        (1060, "Luke Shaw", Position::LB, dec!(50.0), false),
        (1061, "Matthijs de Ligt", Position::CB, dec!(75.0), false),
        (1062, "Altay Bayindir", Position::GK, dec!(20.0), false),
        (1063, "Harry Maguire", Position::CB, dec!(45.0), false),
        (1064, "Leny Yoro", Position::CB, dec!(55.0), false),
        (1065, "Noussair Mazraoui", Position::RB, dec!(55.0), false),
        (1066, "Manuel Ugarte", Position::CDM, dec!(65.0), false),
        (1067, "Christian Eriksen", Position::CM, dec!(40.0), false),
        (1068, "Mason Mount", Position::CAM, dec!(50.0), false),
        (1069, "Amad Diallo", Position::RW, dec!(55.0), false),
        (1070, "Antony", Position::RW, dec!(40.0), false),
        (1071, "Joshua Zirkzee", Position::ST, dec!(65.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.names.insert(id, name.to_string());
        engine.player_states.insert(PlayerId(id), PlayerValues {
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
