use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2019;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(290.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Real Valladolid".to_string());

    let players = vec![
        (20901, "Karl Hein", Position::GK, dec!(25.0), false),
        (20902, "Luis Perez", Position::RB, dec!(25.0), false),
        (20903, "Javi Sanchez", Position::CB, dec!(25.0), true),
        (20904, "Eray Comert", Position::CB, dec!(30.0), false),
        (20905, "Lucas Rosa", Position::LB, dec!(25.0), false),
        (20906, "Stanko Juric", Position::CDM, dec!(25.0), false),
        (20907, "Kike Perez", Position::CM, dec!(25.0), false),
        (20908, "Ivan Sanchez", Position::RW, dec!(20.0), false),
        (20909, "Selim Amallah", Position::CAM, dec!(35.0), false),
        (20910, "Raul Moro", Position::LW, dec!(40.0), false),
        (20911, "Mamadou Sylla", Position::ST, dec!(30.0), false),
        // Bench
        (20912, "Andre Ferreira", Position::GK, dec!(15.0), false),
        (20913, "Cenk Ozkacar", Position::CB, dec!(30.0), false),
        (20914, "David Torres", Position::CB, dec!(15.0), false),
        (20915, "Victor Meseguer", Position::CM, dec!(25.0), false),
        (20916, "Monchu", Position::CM, dec!(30.0), false), // Monchu left for Aris, let's use Mario Martin
        (20917, "Mario Martin", Position::CDM, dec!(30.0), false),
        (20918, "Anuar", Position::RM, dec!(15.0), false),
        (20919, "Amath Ndiaye", Position::LW, dec!(25.0), false),
        (20920, "Latasa", Position::ST, dec!(30.0), false),
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
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(id);
        }
    }
}
