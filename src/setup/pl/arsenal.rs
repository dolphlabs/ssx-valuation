use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Arsenal".to_string());
    
    club.intrinsic_value = dec!(1670.85);
    
    // Rivals: Tottenham (3), Man City (1), Chelsea (6)
    club.set_rival_factor(ClubId(3), dec!(1.6));
    club.set_rival_factor(ClubId(1), dec!(1.4));
    club.set_rival_factor(ClubId(6), dec!(1.3));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1151, "David Raya", Position::GK, dec!(71.43), dec!(1.078), false),
        (1152, "William Saliba", Position::CB, dec!(117.78), dec!(1.016), false),
        (1153, "Gabriel Magalhaes", Position::CB, dec!(102.86), dec!(1.041), false),
        (1154, "Martin Odegaard", Position::CAM, dec!(123.91), dec!(1.055), true),
        (1155, "Bukayo Saka", Position::RW, dec!(210.29), dec!(1.037), false),
        (1156, "Declan Rice", Position::CDM, dec!(165.30), dec!(1.069), false),
        (1157, "Kai Havertz", Position::ST, dec!(103.05), dec!(1.074), false),
        (1158, "Gabriel Martinelli", Position::LW, dec!(91.82), dec!(1.128), false),
        (1159, "Ben White", Position::RB, dec!(79.63), dec!(1.054), false),
        (1160, "Jurrien Timber", Position::LB, dec!(70.15), dec!(1.077), false),
        (1161, "Leandro Trossard", Position::LW, dec!(103.34), dec!(1.065), false),
        (1162, "Neto", Position::GK, dec!(25.00), dec!(1.0), false),
        (1163, "Riccardo Calafiori", Position::CB, dec!(70.99), dec!(1.058), false),
        (1164, "Oleksandr Zinchenko", Position::LB, dec!(50.00), dec!(1.0), false),
        (1165, "Thomas Partey", Position::CDM, dec!(60.00), dec!(1.0), false),
        (1166, "Mikel Merino", Position::CM, dec!(89.40), dec!(1.013), false),
        (1167, "Jorginho", Position::CDM, dec!(45.00), dec!(1.0), false),
        (1168, "Raheem Sterling", Position::LW, dec!(65.00), dec!(1.0), false),
        (1169, "Gabriel Jesus", Position::ST, dec!(72.68), dec!(1.102), false),
        (1170, "Ethan Nwaneri", Position::CAM, dec!(31.83), dec!(1.025), false),
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
            active: true,
        });
    }
}
