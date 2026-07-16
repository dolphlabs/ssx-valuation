use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Tottenham Hotspur".to_string());
    
    club.intrinsic_value = dec!(1041.10);
    
    // Rivals: Arsenal (4), Chelsea (6), West Ham (9)
    club.set_rival_factor(ClubId(4), dec!(1.6));
    club.set_rival_factor(ClubId(6), dec!(1.4));
    club.set_rival_factor(ClubId(9), dec!(1.2));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1101, "Guglielmo Vicario", Position::GK, dec!(59.97), dec!(0.994), false),
        (1102, "Cristian Romero", Position::CB, dec!(56.93), dec!(0.891), false),
        (1103, "Micky van de Ven", Position::CB, dec!(70.51), dec!(1.056), false),
        (1104, "James Maddison", Position::CAM, dec!(85.10), dec!(1.032), false),
        (1105, "Son Heung-min", Position::LW, dec!(110.00), dec!(1.0), true),
        (1106, "Dejan Kulusevski", Position::RW, dec!(75.00), dec!(1.0), false),
        (1107, "Yves Bissouma", Position::CDM, dec!(62.35), dec!(1.019), false),
        (1108, "Pedro Porro", Position::RB, dec!(73.15), dec!(1.109), false),
        (1109, "Destiny Udogie", Position::LB, dec!(64.11), dec!(1.085), false),
        (1110, "Dominic Solanke", Position::ST, dec!(92.81), dec!(1.074), false),
        (1111, "Brennan Johnson", Position::RW, dec!(63.62), dec!(1.020), false),
        (1112, "Fraser Forster", Position::GK, dec!(15.00), dec!(1.0), false),
        (1113, "Radu Dragusin", Position::CB, dec!(45.00), dec!(1.0), false),
        (1114, "Ben Davies", Position::LB, dec!(35.65), dec!(0.997), false),
        (1115, "Rodrigo Bentancur", Position::CM, dec!(69.07), dec!(1.065), false),
        (1116, "Pape Matar Sarr", Position::CM, dec!(67.45), dec!(1.029), false),
        (1117, "Archie Gray", Position::RB, dec!(54.75), dec!(1.027), false),
        (1118, "Lucas Bergvall", Position::CM, dec!(47.16), dec!(1.032), false),
        (1119, "Richarlison", Position::ST, dec!(93.80), dec!(1.050), false),
        (1120, "Timo Werner", Position::LW, dec!(50.00), dec!(1.0), false),
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
