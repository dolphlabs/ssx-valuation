use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2005);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(707.23); 
    
    club.set_rival_factor(ClubId(2006), dec!(1.4)); // Real Sociedad (Basque Derby)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Athletic Club".to_string());

    let players = vec![
        (20201, "Unai Simon", Position::GK, dec!(67.27), dec!(0.999), false),
        (20202, "Oscar de Marcos", Position::RB, dec!(30.00), dec!(1.0), true),
        (20203, "Dani Vivian", Position::CB, dec!(53.01), dec!(1.037), false),
        (20204, "Yeray Alvarez", Position::CB, dec!(35.00), dec!(1.0), false),
        (20205, "Yuri Berchiche", Position::LB, dec!(39.38), dec!(1.155), false),
        (20206, "Inigo Ruiz de Galarreta", Position::CDM, dec!(46.54), dec!(1.058), false),
        (20207, "Oihan Sancet", Position::CAM, dec!(68.63), dec!(1.100), false),
        (20208, "Benat Prados", Position::CM, dec!(30.43), dec!(0.953), false),
        (20209, "Inaki Williams", Position::RW, dec!(82.93), dec!(1.139), false),
        (20210, "Nico Williams", Position::LW, dec!(113.07), dec!(1.110), false),
        (20211, "Gorka Guruzeta", Position::ST, dec!(56.67), dec!(1.099), false),
        // Bench
        (20212, "Julen Agirrezabala", Position::GK, dec!(25.00), dec!(1.0), false),
        (20213, "Aitor Paredes", Position::CB, dec!(50.13), dec!(1.007), false),
        (20214, "Andoni Gorosabel", Position::RB, dec!(31.24), dec!(1.014), false),
        (20215, "Mikel Vesga", Position::CDM, dec!(33.39), dec!(1.045), false),
        (20216, "Ander Herrera", Position::CM, dec!(25.00), dec!(1.0), false),
        (20217, "Unai Gomez", Position::CAM, dec!(35.54), dec!(1.003), false),
        (20218, "Berenguer", Position::LW, dec!(51.51), dec!(1.035), false),
        (20219, "Alvaro Djalo", Position::RW, dec!(45.00), dec!(1.0), false),
        (20220, "Javier Marton", Position::ST, dec!(15.00), dec!(1.0), false),
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
