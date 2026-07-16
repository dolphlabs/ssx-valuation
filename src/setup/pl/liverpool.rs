use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Liverpool".to_string());
    
    club.intrinsic_value = dec!(1626.91);
    
    // Rivals: Man City (1), Man Utd (2), Everton (16)
    club.set_rival_factor(ClubId(1), dec!(1.4));
    club.set_rival_factor(ClubId(2), dec!(1.6));
    club.set_rival_factor(ClubId(16), dec!(1.5));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1201, "Alisson Becker", Position::GK, dec!(75.00), dec!(1.0), false),
        (1202, "Virgil van Dijk", Position::CB, dec!(127.79), dec!(1.125), true),
        (1203, "Mohamed Salah", Position::RW, dec!(209.95), dec!(1.156), false),
        (1204, "Trent Alexander-Arnold", Position::RB, dec!(90.00), dec!(1.0), false),
        (1205, "Alexis Mac Allister", Position::CM, dec!(95.66), dec!(1.058), false),
        (1206, "Dominik Szoboszlai", Position::CM, dec!(97.89), dec!(1.100), false),
        (1207, "Luis Diaz", Position::LW, dec!(85.00), dec!(1.0), false),
        (1208, "Darwin Nunez", Position::ST, dec!(80.00), dec!(1.0), false),
        (1209, "Ibrahima Konate", Position::CB, dec!(84.58), dec!(1.073), false),
        (1210, "Andrew Robertson", Position::LB, dec!(79.42), dec!(1.042), false),
        (1211, "Ryan Gravenberch", Position::CDM, dec!(94.68), dec!(1.066), false),
        (1212, "Caoimhin Kelleher", Position::GK, dec!(35.00), dec!(1.0), false),
        (1213, "Joe Gomez", Position::CB, dec!(60.55), dec!(1.017), false),
        (1214, "Jarell Quansah", Position::CB, dec!(50.00), dec!(1.0), false),
        (1215, "Conor Bradley", Position::RB, dec!(59.76), dec!(1.079), false),
        (1216, "Wataru Endo", Position::CDM, dec!(52.32), dec!(1.038), false),
        (1217, "Curtis Jones", Position::CM, dec!(69.54), dec!(1.145), false),
        (1218, "Harvey Elliott", Position::CAM, dec!(65.96), dec!(1.006), false),
        (1219, "Cody Gakpo", Position::LW, dec!(117.23), dec!(1.015), false),
        (1220, "Diogo Jota", Position::ST, dec!(85.00), dec!(1.0), false),
        (1221, "Federico Chiesa", Position::RW, dec!(76.57), dec!(0.995), false),
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
