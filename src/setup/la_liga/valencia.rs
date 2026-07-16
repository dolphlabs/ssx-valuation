use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2009);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(722.99); 
    
    club.set_rival_factor(ClubId(2008), dec!(1.2)); // Villarreal (Derbi de la Comunitat)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Valencia CF".to_string());

    let players = vec![
        (20401, "Giorgi Mamardashvili", Position::GK, dec!(75.00), dec!(1.0), false),
        (20402, "Thierry Correia", Position::RB, dec!(35.26), dec!(1.043), false),
        (20403, "Cristhian Mosquera", Position::CB, dec!(50.00), dec!(1.0), false),
        (20404, "Cesar Tarrega", Position::CB, dec!(28.72), dec!(1.024), false),
        (20405, "Jose Gaya", Position::LB, dec!(35.36), dec!(1.096), true),
        (20406, "Pepelu", Position::CDM, dec!(64.03), dec!(1.030), false),
        (20407, "Javi Guerra", Position::CM, dec!(61.80), dec!(1.169), false),
        (20408, "Diego Lopez", Position::RW, dec!(72.35), dec!(1.055), false),
        (20409, "Luis Rioja", Position::LW, dec!(47.68), dec!(1.106), false),
        (20410, "Hugo Duro", Position::ST, dec!(79.47), dec!(1.028), false),
        (20411, "Rafa Mir", Position::ST, dec!(40.00), dec!(1.0), false),
        // Bench
        (20412, "Stole Dimitrievski", Position::GK, dec!(25.73), dec!(1.039), false),
        (20413, "Yarek Gasiorowski", Position::CB, dec!(35.00), dec!(1.0), false),
        (20414, "Dimitri Foulquier", Position::RB, dec!(26.39), dec!(0.971), false),
        (20415, "Enzo Barrenechea", Position::CDM, dec!(35.00), dec!(1.0), false),
        (20416, "Andre Almeida", Position::CAM, dec!(45.83), dec!(1.071), false),
        (20417, "Fran Perez", Position::RW, dec!(30.00), dec!(1.0), false),
        (20418, "Sergi Canos", Position::LW, dec!(30.00), dec!(1.0), false),
        (20419, "Dani Gomez", Position::ST, dec!(20.00), dec!(1.0), false),
        (20420, "Hugo Guillamon", Position::CDM, dec!(30.00), dec!(1.0), false),
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
