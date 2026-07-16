use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4010);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(778.83); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "SC Freiburg".to_string());

    let players = vec![
        (40451, "Noah Atubolu", Position::GK, dec!(43.58), dec!(1.129), false),
        (40452, "Lukas Kubler", Position::RB, dec!(35.53), dec!(1.061), false),
        (40453, "Matthias Ginter", Position::CB, dec!(57.78), dec!(1.120), false),
        (40454, "Philipp Lienhart", Position::CB, dec!(39.14), dec!(1.016), false),
        (40455, "Christian Gunter", Position::LB, dec!(34.53), dec!(1.028), true),
        (40456, "Maximilian Eggestein", Position::CDM, dec!(50.23), dec!(1.054), false),
        (40457, "Nicolas Hofler", Position::CDM, dec!(26.51), dec!(1.051), false),
        (40458, "Ritsu Doan", Position::RW, dec!(55.00), dec!(1.0), false),
        (40459, "Vincenzo Grifo", Position::LW, dec!(110.52), dec!(1.043), false),
        (40460, "Eren Dinkci", Position::RW, dec!(45.88), dec!(1.032), false),
        (40461, "Junior Adamu", Position::ST, dec!(42.97), dec!(1.025), false),
        // Bench
        (40462, "Florian Muller", Position::GK, dec!(15.00), dec!(1.0), false),
        (40463, "Max Rosenfelder", Position::CB, dec!(21.51), dec!(1.037), false),
        (40464, "Jordy Makengo", Position::LB, dec!(27.21), dec!(1.119), false),
        (40465, "Patrick Osterhage", Position::CM, dec!(43.35), dec!(1.030), false),
        (40466, "Merlin Rohl", Position::CM, dec!(42.63), dec!(1.022), false),
        (40467, "Noah Weisshaupt", Position::LW, dec!(30.00), dec!(1.0), false),
        (40468, "Michael Gregoritsch", Position::ST, dec!(50.00), dec!(1.0), false),
        (40469, "Lucas Holer", Position::ST, dec!(50.34), dec!(1.024), false),
        (40470, "Florent Muslija", Position::CAM, dec!(30.00), dec!(1.0), false),
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
        });
        engine.names.insert(id, name.to_string());
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(PlayerId(id));
        }
    }
}
