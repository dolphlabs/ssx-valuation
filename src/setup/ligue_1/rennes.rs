use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5010);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(450.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Stade Rennais FC".to_string());

    let players = vec![
        (50451, "Steve Mandanda", Position::GK, dec!(25.0), true),
        (50452, "Lorenz Assignon", Position::RB, dec!(35.0), false),
        (50453, "Leo Ostigard", Position::CB, dec!(35.0), false),
        (50454, "Christopher Wooh", Position::CB, dec!(30.0), false),
        (50455, "Adrien Truffert", Position::LB, dec!(45.0), false),
        (50456, "Azor Matusiwa", Position::CDM, dec!(35.0), false),
        (50457, "Glen Kamara", Position::CM, dec!(40.0), false),
        (50458, "Ludovic Blas", Position::CAM, dec!(45.0), false),
        (50459, "Albert Gronbaek", Position::CAM, dec!(40.0), false),
        (50460, "Jota", Position::LW, dec!(45.0), false),
        (50461, "Amine Gouiri", Position::ST, dec!(55.0), false),
        // Bench
        (50462, "Gauthier Gallon", Position::GK, dec!(10.0), false),
        (50463, "Mikayil Faye", Position::CB, dec!(45.0), false),
        (50464, "Alidu Seidu", Position::RB, dec!(35.0), false),
        (50465, "Baptiste Santamaria", Position::CDM, dec!(35.0), false),
        (50466, "Jordan James", Position::CM, dec!(35.0), false),
        (50467, "Henrik Meister", Position::ST, dec!(25.0), false),
        (50468, "Arnaud Kalimuendo", Position::ST, dec!(55.0), false),
        (50469, "Carlos Gomez", Position::RW, dec!(30.0), false),
        (50470, "Hans Hateboer", Position::RB, dec!(30.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.player_states.insert(PlayerId(id), PlayerValues {
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
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(PlayerId(id));
        }
    }
}
