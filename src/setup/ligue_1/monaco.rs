use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5002);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(986.11); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "AS Monaco".to_string());

    let players = vec![
        (50051, "Philipp Köhn", Position::GK, dec!(36.67), dec!(1.108), false),
        (50052, "Vanderson", Position::RB, dec!(48.66), dec!(1.038), false),
        (50053, "Thilo Kehrer", Position::CB, dec!(31.68), dec!(1.042), false),
        (50054, "Mohammed Salisu", Position::CB, dec!(40.93), dec!(1.064), false),
        (50055, "Caio Henrique", Position::LB, dec!(42.44), dec!(1.056), false),
        (50056, "Denis Zakaria", Position::CDM, dec!(49.88), dec!(1.060), true),
        (50057, "Lamine Camara", Position::CM, dec!(54.29), dec!(1.185), false),
        (50058, "Aleksandr Golovin", Position::CAM, dec!(64.80), dec!(1.056), false),
        (50059, "Maghnes Akliouche", Position::RW, dec!(88.99), dec!(1.069), false),
        (50060, "Eliesse Ben Seghir", Position::LW, dec!(46.82), dec!(1.015), false),
        (50061, "Breel Embolo", Position::ST, dec!(45.00), dec!(1.0), false),
        // Bench
        (50062, "Radoslaw Majecki", Position::GK, dec!(25.00), dec!(1.0), false),
        (50063, "Wilfried Singo", Position::CB, dec!(45.00), dec!(1.0), false),
        (50064, "Christian Mawissa", Position::CB, dec!(33.91), dec!(1.026), false),
        (50065, "Jordan Teze", Position::RB, dec!(37.93), dec!(1.058), false),
        (50066, "Soungoutou Magassa", Position::CDM, dec!(30.00), dec!(1.0), false),
        (50067, "Takumi Minamino", Position::CAM, dec!(51.74), dec!(0.999), false),
        (50068, "Folarin Balogun", Position::ST, dec!(76.79), dec!(1.011), false),
        (50069, "George Ilenikhena", Position::ST, dec!(40.73), dec!(1.013), false),
        (50070, "Krépin Diatta", Position::RW, dec!(26.36), dec!(1.042), false),
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
