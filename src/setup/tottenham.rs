use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3;
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id, "Tottenham Hotspur".to_string());
    
    club.intrinsic_value = dec!(850.00);
    
    // Rivals: Arsenal (4), Chelsea (6), West Ham (9)
    club.set_rival_factor(4, dec!(1.6));
    club.set_rival_factor(6, dec!(1.4));
    club.set_rival_factor(9, dec!(1.2));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1101, "Guglielmo Vicario", Position::GK, dec!(60.0), false),
        (1102, "Cristian Romero", Position::CB, dec!(75.0), false),
        (1103, "Micky van de Ven", Position::CB, dec!(70.0), false),
        (1104, "James Maddison", Position::CAM, dec!(85.0), false),
        (1105, "Son Heung-min", Position::LW, dec!(110.0), true),
        (1106, "Dejan Kulusevski", Position::RW, dec!(75.0), false),
        (1107, "Yves Bissouma", Position::CDM, dec!(65.0), false),
        (1108, "Pedro Porro", Position::RB, dec!(65.0), false),
        (1109, "Destiny Udogie", Position::LB, dec!(65.0), false),
        (1110, "Dominic Solanke", Position::ST, dec!(80.0), false),
        (1111, "Brennan Johnson", Position::RW, dec!(60.0), false),
        (1112, "Fraser Forster", Position::GK, dec!(15.0), false),
        (1113, "Radu Dragusin", Position::CB, dec!(45.0), false),
        (1114, "Ben Davies", Position::LB, dec!(35.0), false),
        (1115, "Rodrigo Bentancur", Position::CM, dec!(60.0), false),
        (1116, "Pape Matar Sarr", Position::CM, dec!(55.0), false),
        (1117, "Archie Gray", Position::RB, dec!(50.0), false),
        (1118, "Lucas Bergvall", Position::CM, dec!(40.0), false),
        (1119, "Richarlison", Position::ST, dec!(65.0), false),
        (1120, "Timo Werner", Position::LW, dec!(50.0), false),
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
