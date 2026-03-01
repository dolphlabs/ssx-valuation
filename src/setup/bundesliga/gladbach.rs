use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 4014;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(440.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Borussia Monchengladbach".to_string());

    let players = vec![
        (40651, "Jonas Omlin", Position::GK, dec!(35.0), true),
        (40652, "Joe Scally", Position::RB, dec!(35.0), false),
        (40653, "Ko Itakura", Position::CB, dec!(45.0), false),
        (40654, "Nico Elvedi", Position::CB, dec!(40.0), false),
        (40655, "Luca Netz", Position::LB, dec!(40.0), false),
        (40656, "Julian Weigl", Position::CDM, dec!(40.0), false),
        (40657, "Rocco Reitz", Position::CM, dec!(45.0), false),
        (40658, "Kevin Stoger", Position::CAM, dec!(45.0), false),
        (40659, "Franck Honorat", Position::RW, dec!(50.0), false),
        (40660, "Alassane Plea", Position::LW, dec!(45.0), false),
        (40661, "Tim Kleindienst", Position::ST, dec!(50.0), false),
        // Bench
        (40662, "Moritz Nicolas", Position::GK, dec!(25.0), false),
        (40663, "Marvin Friedrich", Position::CB, dec!(25.0), false),
        (40664, "Fabio Chiarodia", Position::CB, dec!(20.0), false),
        (40665, "Stefan Lainer", Position::RB, dec!(15.0), false),
        (40666, "Philipp Sander", Position::CM, dec!(30.0), false),
        (40667, "Florian Neuhaus", Position::CAM, dec!(40.0), false),
        (40668, "Robin Hack", Position::LW, dec!(45.0), false),
        (40669, "Tomas Cvancara", Position::ST, dec!(50.0), false),
        (40670, "Nathan Ngoumou", Position::RW, dec!(35.0), false),
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
