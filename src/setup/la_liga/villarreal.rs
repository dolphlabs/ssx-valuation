use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2008);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(855.72); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Villarreal CF".to_string());

    let players = vec![
        (20351, "Diego Conde", Position::GK, dec!(30.00), dec!(1.0), false),
        (20352, "Kiko Femenia", Position::RB, dec!(30.00), dec!(1.0), false),
        (20353, "Raul Albiol", Position::CB, dec!(30.00), dec!(1.0), true),
        (20354, "Eric Bailly", Position::CB, dec!(35.00), dec!(1.0), false),
        (20355, "Sergi Cardona", Position::LB, dec!(39.10), dec!(1.044), false),
        (20356, "Dani Parejo", Position::CM, dec!(51.32), dec!(1.069), false),
        (20357, "Santi Comesana", Position::CM, dec!(44.65), dec!(1.052), false),
        (20358, "Alex Baena", Position::CAM, dec!(85.00), dec!(1.0), false),
        (20359, "Yeremy Pino", Position::RW, dec!(63.17), dec!(1.046), false),
        (20360, "Ayoze Perez", Position::LW, dec!(75.62), dec!(1.101), false),
        (20361, "Thierno Barry", Position::ST, dec!(40.00), dec!(1.0), false),
        // Bench
        (20362, "Luiz Junior", Position::GK, dec!(37.75), dec!(1.057), false),
        (20363, "Logan Costa", Position::CB, dec!(45.14), dec!(1.035), false),
        (20364, "Juan Foyth", Position::RB, dec!(48.15), dec!(0.976), false),
        (20365, "Pape Gueye", Position::CDM, dec!(50.80), dec!(1.078), false),
        (20366, "Nicolas Pepe", Position::RW, dec!(87.19), dec!(1.086), false),
        (20367, "Dennis Suarez", Position::CAM, dec!(25.00), dec!(1.0), false),
        (20368, "Ilias Akhomach", Position::RW, dec!(44.16), dec!(0.977), false),
        (20369, "Gerard Moreno", Position::ST, dec!(98.50), dec!(1.047), false),
        (20370, "Ramon Terrats", Position::CM, dec!(25.00), dec!(1.0), false),
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
