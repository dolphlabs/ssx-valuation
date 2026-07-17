use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3018);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(383.11); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Parma".to_string());

    let players = vec![
        (30851, "Zion Suzuki", Position::GK, dec!(32.61), dec!(1.146), false),
        (30852, "Enrico Del Prato", Position::RB, dec!(35.17), dec!(1.035), true),
        (30853, "Botond Balogh", Position::CB, dec!(30.00), dec!(1.0), false),
        (30854, "Alessandro Circati", Position::CB, dec!(43.66), dec!(1.061), false),
        (30855, "Emanuele Valeri", Position::LB, dec!(28.67), dec!(1.056), false),
        (30856, "Adrian Bernabe", Position::CM, dec!(77.92), dec!(1.074), false),
        (30857, "Simon Sohm", Position::CDM, dec!(35.00), dec!(1.0), false),
        (30858, "Dennis Man", Position::RW, dec!(60.00), dec!(1.0), false),
        (30859, "Valentin Mihaila", Position::LW, dec!(45.00), dec!(1.0), false),
        (30860, "Mateusz Kowalski", Position::CAM, dec!(30.00), dec!(1.0), false),
        (30861, "Ange-Yoan Bonny", Position::ST, dec!(45.00), dec!(1.0), false),
        // Bench
        (30862, "Leandro Chichizola", Position::GK, dec!(10.00), dec!(1.0), false),
        (30863, "Yordan Osorio", Position::CB, dec!(25.00), dec!(1.0), false),
        (30864, "Woyo Coulibaly", Position::RB, dec!(25.00), dec!(1.0), false),
        (30865, "Nahuel Estevez", Position::CM, dec!(26.66), dec!(1.041), false),
        (30866, "Mandela Keita", Position::CDM, dec!(49.66), dec!(1.067), false),
        (30867, "Pontus Almqvist", Position::RW, dec!(38.44), dec!(1.021), false),
        (30868, "Cancellieri", Position::LW, dec!(40.00), dec!(1.0), false),
        (30869, "Gabriel Charpentier", Position::ST, dec!(25.00), dec!(1.0), false),
        (30870, "Haj Mohamed", Position::CAM, dec!(15.00), dec!(1.0), false),
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
