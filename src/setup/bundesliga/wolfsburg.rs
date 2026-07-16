use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4012);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(700.22); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "VfL Wolfsburg".to_string());

    let players = vec![
        (40551, "Kamil Grabara", Position::GK, dec!(46.76), dec!(1.025), false),
        (40552, "Ridle Baku", Position::RB, dec!(40.00), dec!(1.0), false),
        (40553, "Sebastiaan Bornauw", Position::CB, dec!(35.00), dec!(1.0), false),
        (40554, "Konstantinos Koulierakis", Position::CB, dec!(56.46), dec!(1.157), false),
        (40555, "Jakub Kaminski", Position::LB, dec!(40.00), dec!(1.0), false),
        (40556, "Salih Ozcan", Position::CDM, dec!(45.00), dec!(1.0), false),
        (40557, "Maximilian Arnold", Position::CM, dec!(53.54), dec!(1.029), true),
        (40558, "Mattias Svanberg", Position::CM, dec!(53.74), dec!(1.044), false),
        (40559, "Tiago Tomas", Position::LW, dec!(40.00), dec!(1.0), false),
        (40560, "Patrick Wimmer", Position::RW, dec!(66.56), dec!(1.082), false),
        (40561, "Jonas Wind", Position::ST, dec!(64.21), dec!(1.153), false),
        // Bench
        (40562, "Marius Muller", Position::GK, dec!(18.14), dec!(0.90), false),
        (40563, "Cedric Zesiger", Position::CB, dec!(35.00), dec!(1.0), false),
        (40564, "Kilian Fischer", Position::RB, dec!(17.01), dec!(1.023), false),
        (40565, "Yannick Gerhardt", Position::CM, dec!(35.67), dec!(1.059), false),
        (40566, "Bence Dardai", Position::CAM, dec!(20.00), dec!(1.0), false),
        (40567, "Lovro Majer", Position::CAM, dec!(67.09), dec!(1.033), false),
        (40568, "Kevin Paredes", Position::LW, dec!(41.32), dec!(1.023), false),
        (40569, "Mohamed Amoura", Position::ST, dec!(99.93), dec!(1.028), false),
        (40570, "Lukas Nmecha", Position::ST, dec!(40.00), dec!(1.0), false),
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
