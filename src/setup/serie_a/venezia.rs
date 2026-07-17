use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3020);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(290.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Venezia".to_string());

    let players = vec![
        (30951, "Jesse Joronen", Position::GK, dec!(25.0), dec!(1.0), false),
        (30952, "Antonio Candela", Position::RB, dec!(25.0), dec!(1.0), false),
        (30953, "Jay Idzes", Position::CB, dec!(35.0), dec!(1.0), false),
        (30954, "Michael Svoboda", Position::CB, dec!(20.0), dec!(1.0), false),
        (30955, "Francesco Zampano", Position::LB, dec!(20.0), dec!(1.0), false),
        (30956, "Hans Nicolussi Caviglia", Position::CDM, dec!(35.0), dec!(1.0), false),
        (30957, "Alfred Duncan", Position::CM, dec!(35.0), dec!(1.0), false),
        (30958, "Gianluca Busio", Position::CM, dec!(45.0), dec!(1.0), false),
        (30959, "Mikael Egill Ellertsson", Position::LW, dec!(30.0), dec!(1.0), false),
        (30960, "Gaetano Oristanio", Position::CAM, dec!(40.0), dec!(1.0), false),
        (30961, "Joel Pohjanpalo", Position::ST, dec!(55.0), dec!(1.0), true),
        // Bench
        (30962, "Filip Stankovic", Position::GK, dec!(20.0), dec!(1.0), false),
        (30963, "Joel Schingtienne", Position::CB, dec!(25.0), dec!(1.0), false),
        (30964, "Marin Sverko", Position::CB, dec!(20.0), dec!(1.0), false),
        (30965, "Magnus Kofod Andersen", Position::CM, dec!(25.0), dec!(1.0), false),
        (30966, "Issa Doumbia", Position::CM, dec!(20.0), dec!(1.0), false),
        (30967, "John Yeboah", Position::RW, dec!(35.0), dec!(1.0), false),
        (30968, "Christian Gytkjaer", Position::ST, dec!(25.0), dec!(1.0), false),
        (30969, "Antonio Raimondo", Position::ST, dec!(30.0), dec!(1.0), false),
        (30970, "Ridgeciano Haps", Position::LB, dec!(20.0), dec!(1.0), false),
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
