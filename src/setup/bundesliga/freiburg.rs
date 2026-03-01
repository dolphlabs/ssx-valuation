use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 4010;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(440.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "SC Freiburg".to_string());

    let players = vec![
        (40451, "Noah Atubolu", Position::GK, dec!(40.0), false),
        (40452, "Lukas Kubler", Position::RB, dec!(25.0), false),
        (40453, "Matthias Ginter", Position::CB, dec!(45.0), false),
        (40454, "Philipp Lienhart", Position::CB, dec!(40.0), false),
        (40455, "Christian Gunter", Position::LB, dec!(35.0), true),
        (40456, "Maximilian Eggestein", Position::CDM, dec!(40.0), false),
        (40457, "Nicolas Hofler", Position::CDM, dec!(25.0), false),
        (40458, "Ritsu Doan", Position::RW, dec!(55.0), false),
        (40459, "Vincenzo Grifo", Position::LW, dec!(60.0), false),
        (40460, "Eren Dinkci", Position::RW, dec!(45.0), false),
        (40461, "Junior Adamu", Position::ST, dec!(35.0), false),
        // Bench
        (40462, "Florian Muller", Position::GK, dec!(15.0), false),
        (40463, "Max Rosenfelder", Position::CB, dec!(20.0), false),
        (40464, "Jordy Makengo", Position::LB, dec!(25.0), false),
        (40465, "Patrick Osterhage", Position::CM, dec!(40.0), false),
        (40466, "Merlin Rohl", Position::CM, dec!(40.0), false),
        (40467, "Noah Weisshaupt", Position::LW, dec!(30.0), false),
        (40468, "Michael Gregoritsch", Position::ST, dec!(50.0), false),
        (40469, "Lucas Holer", Position::ST, dec!(40.0), false),
        (40470, "Florent Muslija", Position::CAM, dec!(30.0), false),
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
