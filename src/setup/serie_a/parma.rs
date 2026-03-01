use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3018;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(300.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Parma".to_string());

    let players = vec![
        (30851, "Zion Suzuki", Position::GK, dec!(30.0), false),
        (30852, "Enrico Del Prato", Position::RB, dec!(30.0), true),
        (30853, "Botond Balogh", Position::CB, dec!(30.0), false),
        (30854, "Alessandro Circati", Position::CB, dec!(35.0), false),
        (30855, "Emanuele Valeri", Position::LB, dec!(25.0), false),
        (30856, "Adrian Bernabe", Position::CM, dec!(55.0), false),
        (30857, "Simon Sohm", Position::CDM, dec!(35.0), false),
        (30858, "Dennis Man", Position::RW, dec!(60.0), false),
        (30859, "Valentin Mihaila", Position::LW, dec!(45.0), false),
        (30860, "Mateusz Kowalski", Position::CAM, dec!(30.0), false),
        (30861, "Ange-Yoan Bonny", Position::ST, dec!(45.0), false),
        // Bench
        (30862, "Leandro Chichizola", Position::GK, dec!(10.0), false),
        (30863, "Yordan Osorio", Position::CB, dec!(25.0), false),
        (30864, "Woyo Coulibaly", Position::RB, dec!(25.0), false),
        (30865, "Nahuel Estevez", Position::CM, dec!(25.0), false),
        (30866, "Mandela Keita", Position::CDM, dec!(40.0), false),
        (30867, "Pontus Almqvist", Position::RW, dec!(35.0), false),
        (30868, "Cancellieri", Position::LW, dec!(40.0), false),
        (30869, "Gabriel Charpentier", Position::ST, dec!(25.0), false),
        (30870, "Haj Mohamed", Position::CAM, dec!(15.0), false),
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
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(id);
        }
    }
}
