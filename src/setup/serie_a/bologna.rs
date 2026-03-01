use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3010);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(520.00); // Champions League qualified
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Bologna".to_string());

    let players = vec![
        (30451, "Lukasz Skorupski", Position::GK, dec!(35.0), false),
        (30452, "Stefan Posch", Position::RB, dec!(40.0), false),
        (30453, "Sam Beukema", Position::CB, dec!(45.0), false),
        (30454, "Nicolo Casale", Position::CB, dec!(35.0), false),
        (30455, "Juan Miranda", Position::LB, dec!(35.0), false),
        (30456, "Remo Freuler", Position::CDM, dec!(40.0), true),
        (30457, "Michel Aebischer", Position::CM, dec!(35.0), false),
        (30458, "Giovanni Fabbian", Position::CM, dec!(45.0), false),
        (30459, "Riccardo Orsolini", Position::RW, dec!(55.0), false),
        (30460, "Dan Ndoye", Position::LW, dec!(45.0), false),
        (30461, "Santiago Castro", Position::ST, dec!(50.0), false),
        // Bench
        (30462, "Federico Ravaglia", Position::GK, dec!(15.0), false),
        (30463, "Jhon Lucumi", Position::CB, dec!(45.0), false),
        (30464, "Martin Erlic", Position::CB, dec!(30.0), false),
        (30465, "Emil Holm", Position::RB, dec!(35.0), false),
        (30466, "Nikola Moro", Position::CM, dec!(25.0), false),
        (30467, "Kacper Urbanski", Position::CAM, dec!(30.0), false),
        (30468, "Jens Odgaard", Position::RW, dec!(30.0), false),
        (30469, "Samuel Iling-Junior", Position::LW, dec!(40.0), false),
        (30470, "Thijs Dallinga", Position::ST, dec!(50.0), false),
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
