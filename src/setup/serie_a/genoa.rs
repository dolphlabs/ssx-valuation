use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3011;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(380.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Genoa".to_string());

    let players = vec![
        (30501, "Pierluigi Gollini", Position::GK, dec!(30.0), false),
        (30502, "Alessandro Vogliacco", Position::CB, dec!(30.0), false),
        (30503, "Mattia Bani", Position::CB, dec!(25.0), true),
        (30504, "Johan Vásquez", Position::CB, dec!(35.0), false),
        (30505, "Stefano Sabelli", Position::RB, dec!(25.0), false),
        (30506, "Morten Frendrup", Position::CDM, dec!(55.0), false),
        (30507, "Milan Badelj", Position::CDM, dec!(25.0), false),
        (30508, "Junior Messias", Position::CAM, dec!(35.0), false),
        (30509, "Aarón Martín", Position::LB, dec!(30.0), false),
        (30510, "Vitinha", Position::ST, dec!(50.0), false),
        (30511, "Andrea Pinamonti", Position::ST, dec!(50.0), false),
        // Bench
        (30512, "Nicola Leali", Position::GK, dec!(10.0), false),
        (30513, "Koni De Winter", Position::CB, dec!(40.0), false),
        (30514, "Alan Matturro", Position::LB, dec!(20.0), false),
        (30515, "Alessandro Zanoli", Position::RB, dec!(30.0), false),
        (30516, "Morten Thorsby", Position::CM, dec!(35.0), false),
        (30517, "Ruslan Malinovskyi", Position::CAM, dec!(40.0), false),
        (30518, "Fabio Miretti", Position::CM, dec!(45.0), false),
        (30519, "Jeff Ekhator", Position::ST, dec!(20.0), false),
        (30520, "Caleb Ekuban", Position::ST, dec!(25.0), false),
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
