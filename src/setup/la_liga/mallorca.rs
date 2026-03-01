use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2015;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(330.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "RCD Mallorca".to_string());

    let players = vec![
        (20701, "Dominik Greif", Position::GK, dec!(30.0), false),
        (20702, "Pablo Maffeo", Position::RB, dec!(35.0), false),
        (20703, "Antonio Raillo", Position::CB, dec!(35.0), true),
        (20704, "Martin Valjent", Position::CB, dec!(30.0), false),
        (20705, "Johan Mojica", Position::LB, dec!(30.0), false),
        (20706, "Omar Mascarell", Position::CDM, dec!(30.0), false),
        (20707, "Samu Costa", Position::CDM, dec!(45.0), false),
        (20708, "Sergi Darder", Position::CAM, dec!(45.0), false),
        (20709, "Dani Rodriguez", Position::RM, dec!(30.0), false),
        (20710, "Robert Navarro", Position::LM, dec!(35.0), false),
        (20711, "Vedat Muriqi", Position::ST, dec!(60.0), false),
        // Bench
        (20712, "Leo Roman", Position::GK, dec!(25.0), false),
        (20713, "Copete", Position::CB, dec!(25.0), false),
        (20714, "Mateu Morey", Position::RB, dec!(20.0), false),
        (20715, "Manu Morlanes", Position::CM, dec!(35.0), false),
        (20716, "Antonio Sanchez", Position::CM, dec!(25.0), false),
        (20717, "Takuma Asano", Position::RW, dec!(40.0), false),
        (20718, "Cyle Larin", Position::ST, dec!(45.0), false),
        (20719, "Abdon Prats", Position::ST, dec!(20.0), false),
        (20720, "Valery Fernandez", Position::RM, dec!(25.0), false),
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
