use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3006);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1088.11); 
    
    club.set_rival_factor(ClubId(3007), dec!(1.5)); // Lazio (Derby della Capitale)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "AS Roma".to_string());

    let players = vec![
        (30251, "Mile Svilar", Position::GK, dec!(52.97), dec!(1.120), false),
        (30252, "Zeki Celik", Position::RB, dec!(32.48), dec!(1.0), false),
        (30253, "Gianluca Mancini", Position::CB, dec!(57.24), dec!(1.118), false),
        (30254, "Evan Ndicka", Position::CB, dec!(54.49), dec!(1.052), false),
        (30255, "Angelino", Position::LB, dec!(34.99), dec!(0.958), false),
        (30256, "Bryan Cristante", Position::CDM, dec!(52.50), dec!(1.097), false),
        (30257, "Manu Kone", Position::CM, dec!(69.35), dec!(1.030), false),
        (30258, "Lorenzo Pellegrini", Position::CAM, dec!(82.92), dec!(1.095), true),
        (30259, "Paulo Dybala", Position::RW, dec!(88.25), dec!(1.205), false),
        (30260, "Matias Soule", Position::RW, dec!(76.48), dec!(1.039), false),
        (30261, "Artem Dovbyk", Position::ST, dec!(90.63), dec!(1.019), false),
        // Bench
        (30262, "Mathew Ryan", Position::GK, dec!(15.00), dec!(1.0), false),
        (30263, "Mario Hermoso", Position::CB, dec!(58.68), dec!(1.059), false),
        (30264, "Mats Hummels", Position::CB, dec!(30.00), dec!(1.0), false),
        (30265, "Saud Abdulhamid", Position::RB, dec!(20.00), dec!(1.0), false),
        (30266, "Leandro Paredes", Position::CDM, dec!(35.00), dec!(1.0), false),
        (30267, "Enzo Le Fee", Position::CM, dec!(40.00), dec!(1.0), false),
        (30268, "Nicola Zalewski", Position::LW, dec!(35.00), dec!(1.0), false),
        (30269, "Stephan El Shaarawy", Position::LW, dec!(31.01), dec!(1.080), false),
        (30270, "Tommaso Baldanzi", Position::CAM, dec!(44.97), dec!(1.038), false),
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
