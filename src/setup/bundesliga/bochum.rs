use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 4016;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(280.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "VfL Bochum".to_string());

    let players = vec![
        (40751, "Patrick Drewes", Position::GK, dec!(20.0), false),
        (40752, "Felix Passlack", Position::RB, dec!(25.0), false),
        (40753, "Ivan Ordets", Position::CB, dec!(30.0), false),
        (40754, "Jakov Medic", Position::CB, dec!(30.0), false),
        (40755, "Maximilian Wittek", Position::LB, dec!(35.0), false),
        (40756, "Anthony Losilla", Position::CDM, dec!(20.0), true),
        (40757, "Ibrahima Sissoko", Position::CDM, dec!(35.0), false),
        (40758, "Dani de Wit", Position::CAM, dec!(45.0), false),
        (40759, "Matus Bero", Position::CM, dec!(30.0), false),
        (40760, "Philipp Hofmann", Position::ST, dec!(25.0), false),
        (40761, "Myron Boadu", Position::ST, dec!(50.0), false),
        // Bench
        (40762, "Timo Horn", Position::GK, dec!(20.0), false),
        (40763, "Erhan Masovic", Position::CB, dec!(35.0), false),
        (40764, "Noah Loosli", Position::CB, dec!(15.0), false),
        (40765, "Cristian Gamboa", Position::RB, dec!(10.0), false),
        (40766, "Lukas Daschner", Position::CAM, dec!(25.0), false),
        (40767, "Koji Miyoshi", Position::CAM, dec!(30.0), false),
        (40768, "Moritz-Broni Kwarteng", Position::LW, dec!(30.0), false),
        (40769, "Aliou Balde", Position::LW, dec!(25.0), false),
        (40770, "Moritz Broschinski", Position::ST, dec!(25.0), false),
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
