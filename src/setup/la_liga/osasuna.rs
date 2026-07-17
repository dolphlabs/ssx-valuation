use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2011);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(537.33); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "CA Osasuna".to_string());

    let players = vec![
        (20501, "Sergio Herrera", Position::GK, dec!(32.16), dec!(1.020), true),
        (20502, "Jesús Areso", Position::RB, dec!(35.00), dec!(1.0), false),
        (20503, "Alejandro Catena", Position::CB, dec!(41.89), dec!(1.044), false),
        (20504, "Enzo Boyomo", Position::CB, dec!(43.94), dec!(1.043), false),
        (20505, "Abel Bretones", Position::LB, dec!(20.90), dec!(1.037), false),
        (20506, "Lucas Torró", Position::CDM, dec!(36.43), dec!(1.067), false),
        (20507, "Jon Moncayola", Position::CM, dec!(39.32), dec!(1.038), false),
        (20508, "Aimar Oroz", Position::CAM, dec!(50.76), dec!(1.068), false),
        (20509, "Rubén García", Position::RW, dec!(54.67), dec!(1.057), false),
        (20510, "Bryan Zaragoza", Position::LW, dec!(60.00), dec!(1.0), false),
        (20511, "Ante Budimir", Position::ST, dec!(115.89), dec!(1.022), false),
        // Bench
        (20512, "Aitor Fernández", Position::GK, dec!(10.88), dec!(0.777), false),
        (20513, "Juan Cruz", Position::LB, dec!(25.74), dec!(0.870), false),
        (20514, "Jorge Herrando", Position::CB, dec!(31.62), dec!(1.031), false),
        (20515, "Rubén Peña", Position::RB, dec!(20.00), dec!(1.0), false),
        (20516, "Iker Muñoz", Position::CDM, dec!(44.11), dec!(1.075), false),
        (20517, "Moi Gómez", Position::CAM, dec!(31.70), dec!(1.032), false),
        (20518, "Pablo Ibáñez", Position::CM, dec!(25.00), dec!(1.0), false),
        (20519, "Raúl García de Haro", Position::ST, dec!(32.83), dec!(1.032), false),
        (20520, "Jose Arnaiz", Position::LW, dec!(20.00), dec!(1.0), false),
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
