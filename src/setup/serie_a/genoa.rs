use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3011);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(615.28); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Genoa".to_string());

    let players = vec![
        (30501, "Pierluigi Gollini", Position::GK, dec!(30.00), dec!(1.0), false),
        (30502, "Alessandro Vogliacco", Position::CB, dec!(30.00), dec!(1.0), false),
        (30503, "Mattia Bani", Position::CB, dec!(25.00), dec!(1.0), true),
        (30504, "Johan Vásquez", Position::CB, dec!(42.27), dec!(1.134), false),
        (30505, "Stefano Sabelli", Position::RB, dec!(25.33), dec!(1.019), false),
        (30506, "Morten Frendrup", Position::CDM, dec!(64.43), dec!(1.052), false),
        (30507, "Milan Badelj", Position::CDM, dec!(25.00), dec!(1.0), false),
        (30508, "Junior Messias", Position::CAM, dec!(50.66), dec!(1.007), false),
        (30509, "Aarón Martín", Position::LB, dec!(37.04), dec!(1.067), false),
        (30510, "Vitinha", Position::ST, dec!(70.39), dec!(1.049), false),
        (30511, "Andrea Pinamonti", Position::ST, dec!(50.00), dec!(1.0), false),
        // Bench
        (30512, "Nicola Leali", Position::GK, dec!(8.18), dec!(1.043), false),
        (30513, "Koni De Winter", Position::CB, dec!(40.00), dec!(1.0), false),
        (30514, "Alan Matturro", Position::LB, dec!(20.00), dec!(1.0), false),
        (30515, "Alessandro Zanoli", Position::RB, dec!(30.00), dec!(1.0), false),
        (30516, "Morten Thorsby", Position::CM, dec!(40.13), dec!(0.998), false),
        (30517, "Ruslan Malinovskyi", Position::CAM, dec!(50.89), dec!(1.039), false),
        (30518, "Fabio Miretti", Position::CM, dec!(45.00), dec!(1.0), false),
        (30519, "Jeff Ekhator", Position::ST, dec!(24.73), dec!(1.038), false),
        (30520, "Caleb Ekuban", Position::ST, dec!(39.21), dec!(1.032), false),
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
