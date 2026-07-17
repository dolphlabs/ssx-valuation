use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3015);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(489.33); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Cagliari".to_string());

    let players = vec![
        (30701, "Simone Scuffet", Position::GK, dec!(25.00), dec!(1.0), false),
        (30702, "Gabriele Zappa", Position::RB, dec!(26.10), dec!(1.024), false),
        (30703, "Yerry Mina", Position::CB, dec!(36.50), dec!(1.061), false),
        (30704, "Sebastiano Luperto", Position::CB, dec!(54.58), dec!(1.045), false),
        (30705, "Tommaso Augello", Position::LB, dec!(25.00), dec!(1.0), false),
        (30706, "Antoine Makoumbou", Position::CDM, dec!(35.00), dec!(1.0), false),
        (30707, "Michel Adopo", Position::CM, dec!(27.23), dec!(1.036), false),
        (30708, "Razvan Marin", Position::CM, dec!(40.00), dec!(1.0), false),
        (30709, "Gianluca Gaetano", Position::CAM, dec!(52.69), dec!(1.099), false),
        (30710, "Zito Luvumbo", Position::LW, dec!(47.63), dec!(1.019), false),
        (30711, "Roberto Piccoli", Position::ST, dec!(40.00), dec!(1.0), false),
        // Bench
        (30712, "Alen Sherri", Position::GK, dec!(10.00), dec!(1.0), false),
        (30713, "Jose Luis Palomino", Position::CB, dec!(20.00), dec!(1.0), false),
        (30714, "Adam Obert", Position::CB, dec!(25.57), dec!(1.080), false),
        (30715, "Paulo Azzi", Position::LB, dec!(25.00), dec!(1.0), false),
        (30716, "Alessandro Deiola", Position::CM, dec!(21.95), dec!(1.026), true),
        (30717, "Matteo Prati", Position::CDM, dec!(39.66), dec!(1.044), false),
        (30718, "Nicolas Viola", Position::CAM, dec!(25.00), dec!(1.0), false),
        (30719, "Kingstone Mutandwa", Position::ST, dec!(15.00), dec!(1.0), false),
        (30720, "Gianluca Lapadula", Position::ST, dec!(30.00), dec!(1.0), false),
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
