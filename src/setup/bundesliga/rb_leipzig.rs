use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 4004;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(850.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "RB Leipzig".to_string());

    let players = vec![
        (40151, "Peter Gulacsi", Position::GK, dec!(35.0), true),
        (40152, "Lutsharel Geertruida", Position::RB, dec!(65.0), false),
        (40153, "Willi Orban", Position::CB, dec!(45.0), false),
        (40154, "Castello Lukeba", Position::CB, dec!(75.0), false),
        (40155, "David Raum", Position::LB, dec!(55.0), false),
        (40156, "Amadou Haidara", Position::CDM, dec!(55.0), false),
        (40157, "Arthur Vermeeren", Position::CM, dec!(45.0), false),
        (40158, "Xavi Simons", Position::CAM, dec!(110.0), false),
        (40159, "Christoph Baumgartner", Position::CAM, dec!(50.0), false),
        (40160, "Lois Openda", Position::ST, dec!(90.0), false),
        (40161, "Benjamin Sesko", Position::ST, dec!(85.0), false),
        // Bench
        (40162, "Maarten Vandevoordt", Position::GK, dec!(35.0), false),
        (40163, "Lukas Klostermann", Position::CB, dec!(35.0), false),
        (40164, "Benjamin Henrichs", Position::RB, dec!(45.0), false),
        (40165, "El Chadaille Bitshiabu", Position::CB, dec!(30.0), false),
        (40166, "Nicolas Seiwald", Position::CDM, dec!(40.0), false),
        (40167, "Kevin Kampl", Position::CM, dec!(25.0), false),
        (40168, "Assan Ouedraogo", Position::CAM, dec!(30.0), false),
        (40169, "Eljif Elmas", Position::CAM, dec!(45.0), false),
        (40170, "Yussuf Poulsen", Position::ST, dec!(25.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.player_states.insert(id, PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight: dec!(1.0),
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![],
            position: pos,
            is_captain: captain,
        });
        engine.names.insert(id, name.to_string());
        if let Some(c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(id);
        }
    }
}
