use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2020;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(310.00); 
    
    club.set_rival_factor(2002, dec!(1.2)); // Barcelona (Barcelona Derby)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "RCD Espanyol".to_string());

    let players = vec![
        (20951, "Joan Garcia", Position::GK, dec!(45.0), false),
        (20952, "Omar El Hilali", Position::RB, dec!(30.0), false),
        (20953, "Marash Kumbulla", Position::CB, dec!(35.0), false),
        (20954, "Leandro Cabrera", Position::CB, dec!(30.0), true),
        (20955, "Brian Olivan", Position::LB, dec!(25.0), false),
        (20956, "Jose Gragera", Position::CDM, dec!(30.0), false),
        (20957, "Alex Kral", Position::CDM, dec!(35.0), false),
        (20958, "Alvaro Aguado", Position::CM, dec!(30.0), false),
        (20959, "Jofre Carreras", Position::RW, dec!(30.0), false),
        (20960, "Irvin Cardona", Position::LW, dec!(35.0), false),
        (20961, "Javi Puado", Position::ST, dec!(50.0), false),
        // Bench
        (20962, "Fernando Pacheco", Position::GK, dec!(15.0), false),
        (20963, "Sergi Gomez", Position::CB, dec!(20.0), false),
        (20964, "Carlos Romero", Position::LB, dec!(25.0), false),
        (20965, "Pol Lozano", Position::CDM, dec!(20.0), false),
        (20966, "Edu Exposito", Position::CM, dec!(30.0), false),
        (20967, "Cheddira", Position::ST, dec!(35.0), false),
        (20968, "Alejo Veliz", Position::ST, dec!(40.0), false),
        (20969, "Pere Milla", Position::LW, dec!(25.0), false),
        (20970, "Salvi Sanchez", Position::RW, dec!(15.0), false),
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
