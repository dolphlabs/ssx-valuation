use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3004);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(600.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Atalanta".to_string());

    let players = vec![
        (30151, "Marco Carnesecchi", Position::GK, dec!(50.0), false),
        (30152, "Berat Djimsiti", Position::CB, dec!(35.0), false),
        (30153, "Isak Hien", Position::CB, dec!(45.0), false),
        (30154, "Sead Kolasinac", Position::CB, dec!(30.0), false),
        (30155, "Raoul Bellanova", Position::RM, dec!(45.0), false),
        (30156, "Ederson", Position::CDM, dec!(75.0), false),
        (30157, "Marten de Roon", Position::CDM, dec!(40.0), true),
        (30158, "Matteo Ruggeri", Position::LM, dec!(40.0), false),
        (30159, "Charles De Ketelaere", Position::RW, dec!(65.0), false),
        (30160, "Ademola Lookman", Position::LW, dec!(85.0), false),
        (30161, "Mateo Retegui", Position::ST, dec!(55.0), false),
        // Bench
        (30162, "Rui Patricio", Position::GK, dec!(10.0), false),
        (30163, "Odilon Kossounou", Position::CB, dec!(50.0), false),
        (30164, "Ben Godfrey", Position::CB, dec!(25.0), false),
        (30165, "Davide Zappacosta", Position::RM, dec!(30.0), false),
        (30166, "Mario Pasalic", Position::CM, dec!(40.0), false),
        (30167, "Lazar Samardzic", Position::CAM, dec!(45.0), false),
        (30168, "Brescianini", Position::CM, dec!(30.0), false),
        (30169, "Gianluca Scamacca", Position::ST, dec!(65.0), false),
        (30170, "Nicolo Zaniolo", Position::LW, dec!(40.0), false),
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
