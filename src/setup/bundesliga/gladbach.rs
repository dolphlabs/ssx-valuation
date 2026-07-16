use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4014);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(692.28); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Borussia Monchengladbach".to_string());

    let players = vec![
        (40651, "Jonas Omlin", Position::GK, dec!(35.00), dec!(1.0), true),
        (40652, "Joe Scally", Position::RB, dec!(42.04), dec!(1.092), false),
        (40653, "Ko Itakura", Position::CB, dec!(45.00), dec!(1.0), false),
        (40654, "Nico Elvedi", Position::CB, dec!(47.00), dec!(1.155), false),
        (40655, "Luca Netz", Position::LB, dec!(44.56), dec!(1.040), false),
        (40656, "Julian Weigl", Position::CDM, dec!(40.00), dec!(1.0), false),
        (40657, "Rocco Reitz", Position::CM, dec!(46.79), dec!(1.115), false),
        (40658, "Kevin Stoger", Position::CAM, dec!(52.00), dec!(1.055), false),
        (40659, "Franck Honorat", Position::RW, dec!(65.88), dec!(1.028), false),
        (40660, "Alassane Plea", Position::LW, dec!(45.00), dec!(1.0), false),
        (40661, "Tim Kleindienst", Position::ST, dec!(50.75), dec!(1.015), false),
        // Bench
        (40662, "Moritz Nicolas", Position::GK, dec!(30.14), dec!(1.122), false),
        (40663, "Marvin Friedrich", Position::CB, dec!(26.85), dec!(1.047), false),
        (40664, "Fabio Chiarodia", Position::CB, dec!(22.22), dec!(1.024), false),
        (40665, "Stefan Lainer", Position::RB, dec!(15.00), dec!(1.0), false),
        (40666, "Philipp Sander", Position::CM, dec!(36.51), dec!(1.072), false),
        (40667, "Florian Neuhaus", Position::CAM, dec!(45.71), dec!(1.030), false),
        (40668, "Robin Hack", Position::LW, dec!(49.22), dec!(1.134), false),
        (40669, "Tomas Cvancara", Position::ST, dec!(50.00), dec!(1.0), false),
        (40670, "Nathan Ngoumou", Position::RW, dec!(35.00), dec!(1.0), false),
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
