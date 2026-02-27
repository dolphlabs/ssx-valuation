use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3006;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(780.00); 
    
    club.set_rival_factor(3007, dec!(1.5)); // Lazio (Derby della Capitale)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "AS Roma".to_string());

    let players = vec![
        (30251, "Mile Svilar", Position::GK, dec!(45.0), false),
        (30252, "Zeki Celik", Position::RB, dec!(30.0), false),
        (30253, "Gianluca Mancini", Position::CB, dec!(50.0), false),
        (30254, "Evan Ndicka", Position::CB, dec!(45.0), false),
        (30255, "Angelino", Position::LB, dec!(35.0), false),
        (30256, "Bryan Cristante", Position::CDM, dec!(45.0), false),
        (30257, "Manu Kone", Position::CM, dec!(55.0), false),
        (30258, "Lorenzo Pellegrini", Position::CAM, dec!(60.0), true),
        (30259, "Paulo Dybala", Position::RW, dec!(75.0), false),
        (30260, "Matias Soule", Position::RW, dec!(55.0), false),
        (30261, "Artem Dovbyk", Position::ST, dec!(70.0), false),
        // Bench
        (30262, "Mathew Ryan", Position::GK, dec!(15.0), false),
        (30263, "Mario Hermoso", Position::CB, dec!(50.0), false),
        (30264, "Mats Hummels", Position::CB, dec!(30.0), false),
        (30265, "Saud Abdulhamid", Position::RB, dec!(20.0), false),
        (30266, "Leandro Paredes", Position::CDM, dec!(35.0), false),
        (30267, "Enzo Le Fee", Position::CM, dec!(40.0), false),
        (30268, "Nicola Zalewski", Position::LW, dec!(35.0), false),
        (30269, "Stephan El Shaarawy", Position::LW, dec!(30.0), false),
        (30270, "Tommaso Baldanzi", Position::CAM, dec!(40.0), false),
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
