use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2017);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(320.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Rayo Vallecano".to_string());

    let players = vec![
        (20801, "Augusto Batalla", Position::GK, dec!(25.0), false),
        (20802, "Andrei Ratiu", Position::RB, dec!(35.0), false),
        (20803, "Abdul Mumin", Position::CB, dec!(30.0), false),
        (20804, "Florian Lejeune", Position::CB, dec!(30.0), false),
        (20805, "Pep Chavarria", Position::LB, dec!(25.0), false),
        (20806, "Oscar Valentin", Position::CDM, dec!(35.0), true),
        (20807, "Unai Lopez", Position::CM, dec!(30.0), false),
        (20808, "Isi Palazon", Position::RW, dec!(50.0), false),
        (20809, "James Rodriguez", Position::CAM, dec!(55.0), false),
        (20810, "Alvaro Garcia", Position::LW, dec!(40.0), false),
        (20811, "Sergio Camello", Position::ST, dec!(45.0), false),
        // Bench
        (20812, "Dani Cardenas", Position::GK, dec!(15.0), false),
        (20813, "Aridane Hernandez", Position::CB, dec!(20.0), false),
        (20814, "Ivan Balliu", Position::RB, dec!(25.0), false),
        (20815, "Pathé Ciss", Position::CDM, dec!(30.0), false),
        (20816, "Gerard Gumbau", Position::CM, dec!(25.0), false),
        (20817, "Oscar Trejo", Position::CAM, dec!(20.0), false),
        (20818, "Randy Nteka", Position::ST, dec!(25.0), false),
        (20819, "Raul de Tomas", Position::ST, dec!(40.0), false),
        (20820, "Adri Embarba", Position::RW, dec!(30.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.player_states.insert(PlayerId(id), PlayerValues {
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
            c.player_ids.push(PlayerId(id));
        }
    }
}
