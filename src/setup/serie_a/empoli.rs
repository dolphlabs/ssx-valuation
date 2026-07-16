use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3017);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(310.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Empoli".to_string());

    let players = vec![
        (30801, "Devis Vasquez", Position::GK, dec!(25.0), dec!(1.0), false),
        (30802, "Saba Goglichidze", Position::CB, dec!(25.0), dec!(1.0), false),
        (30803, "Ardian Ismajli", Position::CB, dec!(35.0), dec!(1.0), true),
        (30804, "Mattia Viti", Position::CB, dec!(35.0), dec!(1.0), false),
        (30805, "Emmanuel Gyasi", Position::RM, dec!(30.0), dec!(1.0), false),
        (30806, "Liam Henderson", Position::CM, dec!(25.0), dec!(1.0), false),
        (30807, "Alberto Grassi", Position::CDM, dec!(25.0), dec!(1.0), false),
        (30808, "Giuseppe Pezzella", Position::LM, dec!(25.0), dec!(1.0), false),
        (30809, "Sebastiano Esposito", Position::CAM, dec!(45.0), dec!(1.0), false),
        (30810, "Lorenzo Colombo", Position::ST, dec!(50.0), dec!(1.0), false),
        (30811, "Pietro Pellegri", Position::ST, dec!(35.0), dec!(1.0), false),
        // Bench
        (30812, "Federico Brancolini", Position::GK, dec!(10.0), dec!(1.0), false),
        (30813, "Liberato Cacace", Position::LB, dec!(25.0), dec!(1.0), false),
        (30814, "Tyronne Ebuehi", Position::RB, dec!(20.0), dec!(1.0), false),
        (30815, "Youssef Maleh", Position::CM, dec!(35.0), dec!(1.0), false),
        (30816, "Nicolas Haas", Position::CM, dec!(20.0), dec!(1.0), false),
        (30817, "Faustino Anjorin", Position::CAM, dec!(30.0), dec!(1.0), false),
        (30818, "Jacopo Fazzini", Position::CAM, dec!(40.0), dec!(1.0), false),
        (30819, "Ola Solbakken", Position::RW, dec!(35.0), dec!(1.0), false),
        (30820, "Saba Sazonov", Position::CB, dec!(20.0), dec!(1.0), false),
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
