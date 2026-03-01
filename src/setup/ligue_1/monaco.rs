use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 5002;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(650.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "AS Monaco".to_string());

    let players = vec![
        (50051, "Philipp Köhn", Position::GK, dec!(35.0), false),
        (50052, "Vanderson", Position::RB, dec!(45.0), false),
        (50053, "Thilo Kehrer", Position::CB, dec!(35.0), false),
        (50054, "Mohammed Salisu", Position::CB, dec!(35.0), false),
        (50055, "Caio Henrique", Position::LB, dec!(40.0), false),
        (50056, "Denis Zakaria", Position::CDM, dec!(50.0), true),
        (50057, "Lamine Camara", Position::CM, dec!(45.0), false),
        (50058, "Aleksandr Golovin", Position::CAM, dec!(55.0), false),
        (50059, "Maghnes Akliouche", Position::RW, dec!(45.0), false),
        (50060, "Eliesse Ben Seghir", Position::LW, dec!(45.0), false),
        (50061, "Breel Embolo", Position::ST, dec!(45.0), false),
        // Bench
        (50062, "Radoslaw Majecki", Position::GK, dec!(25.0), false),
        (50063, "Wilfried Singo", Position::CB, dec!(45.0), false),
        (50064, "Christian Mawissa", Position::CB, dec!(30.0), false),
        (50065, "Jordan Teze", Position::RB, dec!(35.0), false),
        (50066, "Soungoutou Magassa", Position::CDM, dec!(30.0), false),
        (50067, "Takumi Minamino", Position::CAM, dec!(40.0), false),
        (50068, "Folarin Balogun", Position::ST, dec!(50.0), false),
        (50069, "George Ilenikhena", Position::ST, dec!(35.0), false),
        (50070, "Krépin Diatta", Position::RW, dec!(25.0), false),
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
