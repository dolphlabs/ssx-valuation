use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5012);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(300.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Montpellier HSC".to_string());

    let players = vec![
        (50551, "Benjamin Lecomte", Position::GK, dec!(30.0), dec!(1.0), false),
        (50552, "Falaye Sacko", Position::RB, dec!(25.0), dec!(1.0), false),
        (50553, "Becir Omeragic", Position::CB, dec!(35.0), dec!(1.0), false),
        (50554, "Modibo Sagnan", Position::CB, dec!(30.0), dec!(1.0), false),
        (50555, "Lucas Mincarelli", Position::LB, dec!(20.0), dec!(1.0), false),
        (50556, "Othmane Maamma", Position::CDM, dec!(15.0), dec!(1.0), false),
        (50557, "Jordan Ferri", Position::CM, dec!(25.0), dec!(1.0), false),
        (50558, "Teji Savanier", Position::CAM, dec!(50.0), dec!(1.0), true),
        (50559, "Arnaud Nordin", Position::RW, dec!(35.0), dec!(1.0), false),
        (50560, "Mousa Tamari", Position::LW, dec!(45.0), dec!(1.0), false),
        (50561, "Akor Adams", Position::ST, dec!(40.0), dec!(1.0), false),
        // Bench
        (50562, "Dimitry Bertaud", Position::GK, dec!(15.0), dec!(1.0), false),
        (50563, "Christopher Jullien", Position::CB, dec!(20.0), dec!(1.0), false),
        (50564, "Issiaga Sylla", Position::LB, dec!(20.0), dec!(1.0), false),
        (50565, "Joris Chotard", Position::CDM, dec!(40.0), dec!(1.0), false),
        (50566, "Khalil Fayad", Position::CM, dec!(25.0), dec!(1.0), false),
        (50567, "Wahbi Khazri", Position::CAM, dec!(30.0), dec!(1.0), false),
        (50568, "Junior Ndiaye", Position::ST, dec!(15.0), dec!(1.0), false),
        (50569, "Theo Chennahi", Position::CM, dec!(10.0), dec!(1.0), false),
        (50570, "Stefan Dzodic", Position::CB, dec!(10.0), dec!(1.0), false),
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
