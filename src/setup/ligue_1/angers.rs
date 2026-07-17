use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5017);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(426.83); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Angers SCO".to_string());

    let players = vec![
        (50801, "Yahia Fofana", Position::GK, dec!(35.00), dec!(1.0), false),
        (50802, "Carlens Arcus", Position::RB, dec!(20.10), dec!(1.032), false),
        (50803, "Cedric Hountondji", Position::CB, dec!(20.00), dec!(1.0), false),
        (50804, "Jordan Lefort", Position::CB, dec!(17.54), dec!(1.080), false),
        (50805, "Jacques Ekomie", Position::LB, dec!(21.72), dec!(1.039), false),
        (50806, "Jean-Eudes Aholou", Position::CDM, dec!(25.00), dec!(1.0), false),
        (50807, "Haris Belkebla", Position::CM, dec!(31.90), dec!(1.040), false),
        (50808, "Pierrick Capelle", Position::CM, dec!(10.42), dec!(1.010), true),
        (50809, "Himad Abdelli", Position::CAM, dec!(48.08), dec!(1.053), false),
        (50810, "Farid El Melali", Position::RW, dec!(25.00), dec!(1.0), false),
        (50811, "Bamba Dieng", Position::ST, dec!(40.00), dec!(1.0), false),
        // Bench
        (50812, "Melvin Zinga", Position::GK, dec!(5.13), dec!(1.000), false),
        (50813, "Abdoulaye Bamba", Position::RB, dec!(10.82), dec!(1.056), false),
        (50814, "Emmanuel Biumla", Position::CB, dec!(26.05), dec!(1.029), false),
        (50815, "Lilian Rao-Lisoa", Position::RB, dec!(20.00), dec!(1.0), false),
        (50816, "Joseph Lopy", Position::CM, dec!(15.00), dec!(1.0), false),
        (50817, "Jim Allevinah", Position::LW, dec!(35.86), dec!(1.052), false),
        (50818, "Esteban Lepaul", Position::ST, dec!(20.15), dec!(1.003), false),
        (50819, "Sidiki Cherif", Position::ST, dec!(24.28), dec!(1.032), false),
        (50820, "Zinedine Ferhat", Position::CAM, dec!(25.00), dec!(1.0), false),
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
