use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2009;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(420.00); 
    
    club.set_rival_factor(2008, dec!(1.2)); // Villarreal (Derbi de la Comunitat)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Valencia CF".to_string());

    let players = vec![
        (20401, "Giorgi Mamardashvili", Position::GK, dec!(75.0), false),
        (20402, "Thierry Correia", Position::RB, dec!(35.0), false),
        (20403, "Cristhian Mosquera", Position::CB, dec!(50.0), false),
        (20404, "Cesar Tarrega", Position::CB, dec!(30.0), false),
        (20405, "Jose Gaya", Position::LB, dec!(45.0), true),
        (20406, "Pepelu", Position::CDM, dec!(55.0), false),
        (20407, "Javi Guerra", Position::CM, dec!(50.0), false),
        (20408, "Diego Lopez", Position::RW, dec!(45.0), false),
        (20409, "Luis Rioja", Position::LW, dec!(35.0), false),
        (20410, "Hugo Duro", Position::ST, dec!(50.0), false),
        (20411, "Rafa Mir", Position::ST, dec!(40.0), false),
        // Bench
        (20412, "Stole Dimitrievski", Position::GK, dec!(25.0), false),
        (20413, "Yarek Gasiorowski", Position::CB, dec!(35.0), false),
        (20414, "Dimitri Foulquier", Position::RB, dec!(25.0), false),
        (20415, "Enzo Barrenechea", Position::CDM, dec!(35.0), false),
        (20416, "Andre Almeida", Position::CAM, dec!(40.0), false),
        (20417, "Fran Perez", Position::RW, dec!(30.0), false),
        (20418, "Sergi Canos", Position::LW, dec!(30.0), false),
        (20419, "Dani Gomez", Position::ST, dec!(20.0), false),
        (20420, "Hugo Guillamon", Position::CDM, dec!(30.0), false),
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
