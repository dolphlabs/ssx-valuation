use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 5017;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(260.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Angers SCO".to_string());

    let players = vec![
        (50801, "Yahia Fofana", Position::GK, dec!(35.0), false),
        (50802, "Carlens Arcus", Position::RB, dec!(20.0), false),
        (50803, "Cedric Hountondji", Position::CB, dec!(20.0), false),
        (50804, "Jordan Lefort", Position::CB, dec!(15.0), false),
        (50805, "Jacques Ekomie", Position::LB, dec!(20.0), false),
        (50806, "Jean-Eudes Aholou", Position::CDM, dec!(25.0), false),
        (50807, "Haris Belkebla", Position::CM, dec!(30.0), false),
        (50808, "Pierrick Capelle", Position::CM, dec!(10.0), true),
        (50809, "Himad Abdelli", Position::CAM, dec!(40.0), false),
        (50810, "Farid El Melali", Position::RW, dec!(25.0), false),
        (50811, "Bamba Dieng", Position::ST, dec!(40.0), false),
        // Bench
        (50812, "Melvin Zinga", Position::GK, dec!(5.0), false),
        (50813, "Abdoulaye Bamba", Position::RB, dec!(10.0), false),
        (50814, "Emmanuel Biumla", Position::CB, dec!(25.0), false),
        (50815, "Lilian Rao-Lisoa", Position::RB, dec!(20.0), false),
        (50816, "Joseph Lopy", Position::CM, dec!(15.0), false),
        (50817, "Jim Allevinah", Position::LW, dec!(30.0), false),
        (50818, "Esteban Lepaul", Position::ST, dec!(20.0), false),
        (50819, "Sidiki Cherif", Position::ST, dec!(15.0), false),
        (50820, "Zinedine Ferhat", Position::CAM, dec!(25.0), false),
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
