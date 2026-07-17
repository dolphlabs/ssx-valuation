use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4004);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1335.11); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "RB Leipzig".to_string());

    let players = vec![
        (40151, "Peter Gulacsi", Position::GK, dec!(39.11), dec!(0.863), true),
        (40152, "Lutsharel Geertruida", Position::RB, dec!(65.00), dec!(1.0), false),
        (40153, "Willi Orban", Position::CB, dec!(59.67), dec!(1.080), false),
        (40154, "Castello Lukeba", Position::CB, dec!(98.25), dec!(1.120), false),
        (40155, "David Raum", Position::LB, dec!(74.14), dec!(1.089), false),
        (40156, "Amadou Haidara", Position::CDM, dec!(55.00), dec!(1.0), false),
        (40157, "Arthur Vermeeren", Position::CM, dec!(45.00), dec!(1.0), false),
        (40158, "Xavi Simons", Position::CAM, dec!(115.26), dec!(1.016), false),
        (40159, "Christoph Baumgartner", Position::CAM, dec!(90.90), dec!(1.027), false),
        (40160, "Lois Openda", Position::ST, dec!(99.04), dec!(1.034), false),
        (40161, "Benjamin Sesko", Position::ST, dec!(85.00), dec!(1.0), false),
        // Bench
        (40162, "Maarten Vandevoordt", Position::GK, dec!(37.17), dec!(1.005), false),
        (40163, "Lukas Klostermann", Position::CB, dec!(36.52), dec!(1.069), false),
        (40164, "Benjamin Henrichs", Position::RB, dec!(49.26), dec!(1.014), false),
        (40165, "El Chadaille Bitshiabu", Position::CB, dec!(33.71), dec!(1.086), false),
        (40166, "Nicolas Seiwald", Position::CDM, dec!(47.68), dec!(1.069), false),
        (40167, "Kevin Kampl", Position::CM, dec!(25.00), dec!(1.0), false),
        (40168, "Assan Ouedraogo", Position::CAM, dec!(44.82), dec!(1.121), false),
        (40169, "Eljif Elmas", Position::CAM, dec!(45.00), dec!(1.0), false),
        (40170, "Yussuf Poulsen", Position::ST, dec!(25.00), dec!(1.0), false),
    ];

    for (id, name, pos, val, form_weight, captain) in players {
        engine.player_states.insert(PlayerId(id), PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight,
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![],
            position: pos,
            is_captain: captain,
            active: true,
        });
        engine.names.insert(id, name.to_string());
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(PlayerId(id));
        }
    }
}
