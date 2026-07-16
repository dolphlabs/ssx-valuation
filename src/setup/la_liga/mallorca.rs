use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2015);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(549.72); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "RCD Mallorca".to_string());

    let players = vec![
        (20701, "Dominik Greif", Position::GK, dec!(30.00), dec!(1.0), false),
        (20702, "Pablo Maffeo", Position::RB, dec!(34.27), dec!(0.982), false),
        (20703, "Antonio Raillo", Position::CB, dec!(39.67), dec!(1.079), true),
        (20704, "Martin Valjent", Position::CB, dec!(30.41), dec!(1.037), false),
        (20705, "Johan Mojica", Position::LB, dec!(30.66), dec!(1.007), false),
        (20706, "Omar Mascarell", Position::CDM, dec!(31.43), dec!(1.065), false),
        (20707, "Samu Costa", Position::CDM, dec!(64.77), dec!(1.025), false),
        (20708, "Sergi Darder", Position::CAM, dec!(54.33), dec!(1.048), false),
        (20709, "Dani Rodriguez", Position::RM, dec!(31.95), dec!(1.021), false),
        (20710, "Robert Navarro", Position::LM, dec!(35.00), dec!(1.0), false),
        (20711, "Vedat Muriqi", Position::ST, dec!(102.94), dec!(1.021), false),
        // Bench
        (20712, "Leo Roman", Position::GK, dec!(25.10), dec!(0.964), false),
        (20713, "Copete", Position::CB, dec!(25.00), dec!(1.0), false),
        (20714, "Mateu Morey", Position::RB, dec!(19.80), dec!(1.037), false),
        (20715, "Manu Morlanes", Position::CM, dec!(22.74), dec!(1.040), false),
        (20716, "Antonio Sanchez", Position::CM, dec!(22.56), dec!(0.984), false),
        (20717, "Takuma Asano", Position::RW, dec!(43.27), dec!(1.021), false),
        (20718, "Cyle Larin", Position::ST, dec!(45.00), dec!(1.0), false),
        (20719, "Abdon Prats", Position::ST, dec!(20.96), dec!(1.015), false),
        (20720, "Valery Fernandez", Position::RM, dec!(25.00), dec!(1.0), false),
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
