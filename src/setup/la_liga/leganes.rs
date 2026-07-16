use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2018);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(280.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "CD Leganes".to_string());

    let players = vec![
        (20851, "Marko Dmitrovic", Position::GK, dec!(20.0), dec!(1.0), false),
        (20852, "Valentin Rosier", Position::RB, dec!(30.0), dec!(1.0), false),
        (20853, "Jorge Saenz", Position::CB, dec!(25.0), dec!(1.0), false),
        (20854, "Matija Nastasic", Position::CB, dec!(30.0), dec!(1.0), false),
        (20855, "Javi Hernandez", Position::LB, dec!(25.0), dec!(1.0), false),
        (20856, "Yvan Neyou", Position::CDM, dec!(35.0), dec!(1.0), false),
        (20857, "Seydouba Cisse", Position::CM, dec!(30.0), dec!(1.0), false),
        (20858, "Oscar Rodriguez", Position::CAM, dec!(35.0), dec!(1.0), false),
        (20859, "Munir El Haddadi", Position::RW, dec!(40.0), dec!(1.0), false),
        (20860, "Roberto Lopez", Position::LW, dec!(30.0), dec!(1.0), false),
        (20861, "Miguel de la Fuente", Position::ST, dec!(35.0), dec!(1.0), true),
        // Bench
        (20862, "Juan Soriano", Position::GK, dec!(15.0), dec!(1.0), false),
        (20863, "Renato Tapia", Position::CDM, dec!(35.0), dec!(1.0), false),
        (20864, "Darko Brasanac", Position::CM, dec!(20.0), dec!(1.0), false),
        (20865, "Sébastien Haller", Position::ST, dec!(50.0), dec!(1.0), false),
        (20866, "Dani Raba", Position::RW, dec!(25.0), dec!(1.0), false),
        (20867, "Enric Franquesa", Position::LB, dec!(20.0), dec!(1.0), false),
        (20868, "Adria Alti", Position::RB, dec!(20.0), dec!(1.0), false),
        (20869, "Jackson Porozo", Position::CB, dec!(25.0), dec!(1.0), false),
        (20870, "Diego Garcia", Position::ST, dec!(20.0), dec!(1.0), false),
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
