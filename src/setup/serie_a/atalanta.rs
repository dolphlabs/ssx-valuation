use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3004);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(933.94); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Atalanta".to_string());

    let players = vec![
        (30151, "Marco Carnesecchi", Position::GK, dec!(64.68), dec!(1.094), false),
        (30152, "Berat Djimsiti", Position::CB, dec!(39.52), dec!(1.040), false),
        (30153, "Isak Hien", Position::CB, dec!(50.15), dec!(1.011), false),
        (30154, "Sead Kolasinac", Position::CB, dec!(30.17), dec!(1.023), false),
        (30155, "Raoul Bellanova", Position::RM, dec!(50.96), dec!(0.979), false),
        (30156, "Ederson", Position::CDM, dec!(79.33), dec!(1.0), false),
        (30157, "Marten de Roon", Position::CDM, dec!(35.13), dec!(1.070), true),
        (30158, "Matteo Ruggeri", Position::LM, dec!(40.00), dec!(1.0), false),
        (30159, "Charles De Ketelaere", Position::RW, dec!(96.47), dec!(1.043), false),
        (30160, "Ademola Lookman", Position::LW, dec!(99.48), dec!(1.018), false),
        (30161, "Mateo Retegui", Position::ST, dec!(55.00), dec!(1.0), false),
        // Bench
        (30162, "Rui Patricio", Position::GK, dec!(10.00), dec!(1.0), false),
        (30163, "Odilon Kossounou", Position::CB, dec!(60.57), dec!(1.035), false),
        (30164, "Ben Godfrey", Position::CB, dec!(25.00), dec!(1.0), false),
        (30165, "Davide Zappacosta", Position::RM, dec!(37.15), dec!(1.054), false),
        (30166, "Mario Pasalic", Position::CM, dec!(59.02), dec!(1.0), false),
        (30167, "Lazar Samardzic", Position::CAM, dec!(50.73), dec!(1.0), false),
        (30168, "Brescianini", Position::CM, dec!(30.21), dec!(1.013), false),
        (30169, "Gianluca Scamacca", Position::ST, dec!(98.32), dec!(1.059), false),
        (30170, "Nicolo Zaniolo", Position::LW, dec!(40.00), dec!(1.0), false),
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
        });
        engine.names.insert(id, name.to_string());
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(PlayerId(id));
        }
    }
}
