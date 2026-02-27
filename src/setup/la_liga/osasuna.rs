use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2011;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(350.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "CA Osasuna".to_string());

    let players = vec![
        (20501, "Sergio Herrera", Position::GK, dec!(30.0), true),
        (20502, "Jesús Areso", Position::RB, dec!(35.0), false),
        (20503, "Alejandro Catena", Position::CB, dec!(35.0), false),
        (20504, "Enzo Boyomo", Position::CB, dec!(40.0), false),
        (20505, "Abel Bretones", Position::LB, dec!(30.0), false),
        (20506, "Lucas Torró", Position::CDM, dec!(35.0), false),
        (20507, "Jon Moncayola", Position::CM, dec!(40.0), false),
        (20508, "Aimar Oroz", Position::CAM, dec!(50.0), false),
        (20509, "Rubén García", Position::RW, dec!(30.0), false),
        (20510, "Bryan Zaragoza", Position::LW, dec!(60.0), false),
        (20511, "Ante Budimir", Position::ST, dec!(55.0), false),
        // Bench
        (20512, "Aitor Fernández", Position::GK, dec!(15.0), false),
        (20513, "Juan Cruz", Position::LB, dec!(25.0), false),
        (20514, "Jorge Herrando", Position::CB, dec!(30.0), false),
        (20515, "Rubén Peña", Position::RB, dec!(20.0), false),
        (20516, "Iker Muñoz", Position::CDM, dec!(35.0), false),
        (20517, "Moi Gómez", Position::CAM, dec!(30.0), false),
        (20518, "Pablo Ibáñez", Position::CM, dec!(25.0), false),
        (20519, "Raúl García de Haro", Position::ST, dec!(35.0), false),
        (20520, "Jose Arnaiz", Position::LW, dec!(20.0), false),
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
