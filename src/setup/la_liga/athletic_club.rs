use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2005;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(550.00); 
    
    club.set_rival_factor(2006, dec!(1.4)); // Real Sociedad (Basque Derby)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Athletic Club".to_string());

    let players = vec![
        (20201, "Unai Simon", Position::GK, dec!(65.0), false),
        (20202, "Oscar de Marcos", Position::RB, dec!(30.0), true),
        (20203, "Dani Vivian", Position::CB, dec!(55.0), false),
        (20204, "Yeray Alvarez", Position::CB, dec!(35.0), false),
        (20205, "Yuri Berchiche", Position::LB, dec!(35.0), false),
        (20206, "Inigo Ruiz de Galarreta", Position::CDM, dec!(40.0), false),
        (20207, "Oihan Sancet", Position::CAM, dec!(65.0), false),
        (20208, "Benat Prados", Position::CM, dec!(35.0), false),
        (20209, "Inaki Williams", Position::RW, dec!(75.0), false),
        (20210, "Nico Williams", Position::LW, dec!(95.0), false),
        (20211, "Gorka Guruzeta", Position::ST, dec!(50.0), false),
        // Bench
        (20212, "Julen Agirrezabala", Position::GK, dec!(25.0), false),
        (20213, "Aitor Paredes", Position::CB, dec!(40.0), false),
        (20214, "Andoni Gorosabel", Position::RB, dec!(30.0), false),
        (20215, "Mikel Vesga", Position::CDM, dec!(30.0), false),
        (20216, "Ander Herrera", Position::CM, dec!(25.0), false),
        (20217, "Unai Gomez", Position::CAM, dec!(30.0), false),
        (20218, "Berenguer", Position::LW, dec!(45.0), false),
        (20219, "Alvaro Djalo", Position::RW, dec!(45.0), false),
        (20220, "Javier Marton", Position::ST, dec!(15.0), false),
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
