use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3015;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(320.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Cagliari".to_string());

    let players = vec![
        (30701, "Simone Scuffet", Position::GK, dec!(25.0), false),
        (30702, "Gabriele Zappa", Position::RB, dec!(25.0), false),
        (30703, "Yerry Mina", Position::CB, dec!(35.0), false),
        (30704, "Sebastiano Luperto", Position::CB, dec!(35.0), false),
        (30705, "Tommaso Augello", Position::LB, dec!(25.0), false),
        (30706, "Antoine Makoumbou", Position::CDM, dec!(35.0), false),
        (30707, "Michel Adopo", Position::CM, dec!(25.0), false),
        (30708, "Razvan Marin", Position::CM, dec!(40.0), false),
        (30709, "Gianluca Gaetano", Position::CAM, dec!(45.0), false),
        (30710, "Zito Luvumbo", Position::LW, dec!(45.0), false),
        (30711, "Roberto Piccoli", Position::ST, dec!(40.0), false),
        // Bench
        (30712, "Alen Sherri", Position::GK, dec!(10.0), false),
        (30713, "Jose Luis Palomino", Position::CB, dec!(20.0), false),
        (30714, "Adam Obert", Position::CB, dec!(25.0), false),
        (30715, "Paulo Azzi", Position::LB, dec!(25.0), false),
        (30716, "Alessandro Deiola", Position::CM, dec!(20.0), true),
        (30717, "Matteo Prati", Position::CDM, dec!(35.0), false),
        (30718, "Nicolas Viola", Position::CAM, dec!(25.0), false),
        (30719, "Kingstone Mutandwa", Position::ST, dec!(15.0), false),
        (30720, "Gianluca Lapadula", Position::ST, dec!(30.0), false),
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
