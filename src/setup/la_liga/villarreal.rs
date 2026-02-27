use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2008;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(460.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Villarreal CF".to_string());

    let players = vec![
        (20351, "Diego Conde", Position::GK, dec!(30.0), false),
        (20352, "Kiko Femenia", Position::RB, dec!(30.0), false),
        (20353, "Raul Albiol", Position::CB, dec!(30.0), true),
        (20354, "Eric Bailly", Position::CB, dec!(35.0), false),
        (20355, "Sergi Cardona", Position::LB, dec!(35.0), false),
        (20356, "Dani Parejo", Position::CM, dec!(45.0), false),
        (20357, "Santi Comesana", Position::CM, dec!(40.0), false),
        (20358, "Alex Baena", Position::CAM, dec!(85.0), false),
        (20359, "Yeremy Pino", Position::RW, dec!(55.0), false),
        (20360, "Ayoze Perez", Position::LW, dec!(60.0), false),
        (20361, "Thierno Barry", Position::ST, dec!(40.0), false),
        // Bench
        (20362, "Luiz Junior", Position::GK, dec!(35.0), false),
        (20363, "Logan Costa", Position::CB, dec!(45.0), false),
        (20364, "Juan Foyth", Position::RB, dec!(45.0), false),
        (20365, "Pape Gueye", Position::CDM, dec!(40.0), false),
        (20366, "Nicolas Pepe", Position::RW, dec!(40.0), false),
        (20367, "Dennis Suarez", Position::CAM, dec!(25.0), false),
        (20368, "Ilias Akhomach", Position::RW, dec!(45.0), false),
        (20369, "Gerard Moreno", Position::ST, dec!(55.0), false),
        (20370, "Ramon Terrats", Position::CM, dec!(25.0), false),
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
        if let Some(c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(id);
        }
    }
}
