use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 5;
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id, "Liverpool".to_string());
    
    club.intrinsic_value = dec!(1100.00);
    
    // Rivals: Man City (1), Man Utd (2), Everton (16)
    club.set_rival_factor(1, dec!(1.4));
    club.set_rival_factor(2, dec!(1.6));
    club.set_rival_factor(16, dec!(1.5));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1201, "Alisson Becker", Position::GK, dec!(75.0), false),
        (1202, "Virgil van Dijk", Position::CB, dec!(95.0), true),
        (1203, "Mohamed Salah", Position::RW, dec!(160.0), false),
        (1204, "Trent Alexander-Arnold", Position::RB, dec!(90.0), false),
        (1205, "Alexis Mac Allister", Position::CM, dec!(85.0), false),
        (1206, "Dominik Szoboszlai", Position::CM, dec!(80.0), false),
        (1207, "Luis Diaz", Position::LW, dec!(85.0), false),
        (1208, "Darwin Nunez", Position::ST, dec!(80.0), false),
        (1209, "Ibrahima Konate", Position::CB, dec!(75.0), false),
        (1210, "Andrew Robertson", Position::LB, dec!(70.0), false),
        (1211, "Ryan Gravenberch", Position::CDM, dec!(70.0), false),
        (1212, "Caoimhin Kelleher", Position::GK, dec!(35.0), false),
        (1213, "Joe Gomez", Position::CB, dec!(55.0), false),
        (1214, "Jarell Quansah", Position::CB, dec!(50.0), false),
        (1215, "Conor Bradley", Position::RB, dec!(55.0), false),
        (1216, "Wataru Endo", Position::CDM, dec!(50.0), false),
        (1217, "Curtis Jones", Position::CM, dec!(60.0), false),
        (1218, "Harvey Elliott", Position::CAM, dec!(65.0), false),
        (1219, "Cody Gakpo", Position::LW, dec!(80.0), false),
        (1220, "Diogo Jota", Position::ST, dec!(85.0), false),
        (1221, "Federico Chiesa", Position::RW, dec!(70.0), false),
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
