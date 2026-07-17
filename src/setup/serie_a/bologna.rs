use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3010);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(770.61); // Champions League qualified
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Bologna".to_string());

    let players = vec![
        (30451, "Lukasz Skorupski", Position::GK, dec!(35.03), dec!(0.991), false),
        (30452, "Stefan Posch", Position::RB, dec!(40.00), dec!(1.0), false),
        (30453, "Sam Beukema", Position::CB, dec!(45.00), dec!(1.0), false),
        (30454, "Nicolo Casale", Position::CB, dec!(39.12), dec!(1.068), false),
        (30455, "Juan Miranda", Position::LB, dec!(38.06), dec!(1.075), false),
        (30456, "Remo Freuler", Position::CDM, dec!(43.38), dec!(1.063), true),
        (30457, "Michel Aebischer", Position::CM, dec!(35.00), dec!(1.0), false),
        (30458, "Giovanni Fabbian", Position::CM, dec!(55.29), dec!(1.085), false),
        (30459, "Riccardo Orsolini", Position::RW, dec!(94.64), dec!(1.084), false),
        (30460, "Dan Ndoye", Position::LW, dec!(45.00), dec!(1.0), false),
        (30461, "Santiago Castro", Position::ST, dec!(75.03), dec!(1.006), false),
        // Bench
        (30462, "Federico Ravaglia", Position::GK, dec!(14.99), dec!(0.964), false),
        (30463, "Jhon Lucumi", Position::CB, dec!(54.84), dec!(1.018), false),
        (30464, "Martin Erlic", Position::CB, dec!(30.00), dec!(1.0), false),
        (30465, "Emil Holm", Position::RB, dec!(35.60), dec!(0.997), false),
        (30466, "Nikola Moro", Position::CM, dec!(31.57), dec!(1.053), false),
        (30467, "Kacper Urbanski", Position::CAM, dec!(30.00), dec!(1.0), false),
        (30468, "Jens Odgaard", Position::RW, dec!(42.24), dec!(1.027), false),
        (30469, "Samuel Iling-Junior", Position::LW, dec!(40.00), dec!(1.0), false),
        (30470, "Thijs Dallinga", Position::ST, dec!(55.05), dec!(1.016), false),
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
