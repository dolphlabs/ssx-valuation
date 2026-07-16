use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4005);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1466.83); // Champions League finalists
    
    club.set_rival_factor(ClubId(4002), dec!(1.5)); // Bayern Munich (Der Klassiker)
    club.set_rival_factor(ClubId(4012), dec!(1.2)); // Wolfsburg? let's use Schalke if they were in BL, but they aren't.

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Borussia Dortmund".to_string());

    let players = vec![
        (40201, "Gregor Kobel", Position::GK, dec!(84.36), dec!(1.102), false),
        (40202, "Yan Couto", Position::RB, dec!(65.38), dec!(1.081), false),
        (40203, "Waldemar Anton", Position::CB, dec!(78.58), dec!(1.115), false),
        (40204, "Nico Schlotterbeck", Position::CB, dec!(101.55), dec!(1.131), false),
        (40205, "Julian Ryerson", Position::LB, dec!(44.75), dec!(1.069), false),
        (40206, "Emre Can", Position::CDM, dec!(64.78), dec!(1.083), true),
        (40207, "Pascal Gross", Position::CM, dec!(45.00), dec!(1.0), false),
        (40208, "Julian Brandt", Position::CAM, dec!(134.97), dec!(1.047), false),
        (40209, "Karim Adeyemi", Position::LW, dec!(99.27), dec!(1.069), false),
        (40210, "Donyell Malen", Position::RW, dec!(60.00), dec!(1.0), false),
        (40211, "Serhou Guirassy", Position::ST, dec!(157.70), dec!(1.111), false),
        // Bench
        (40212, "Alexander Meyer", Position::GK, dec!(15.00), dec!(1.0), false),
        (40213, "Niklas Sule", Position::CB, dec!(43.96), dec!(1.020), false),
        (40214, "Ramy Bensebaini", Position::LB, dec!(48.14), dec!(1.128), false),
        (40215, "Felix Nmecha", Position::CM, dec!(49.61), dec!(1.085), false),
        (40216, "Marcel Sabitzer", Position::CM, dec!(62.04), dec!(1.067), false),
        (40217, "Gio Reyna", Position::CAM, dec!(40.00), dec!(1.0), false),
        (40218, "Jamie Gittens", Position::LW, dec!(50.00), dec!(1.0), false),
        (40219, "Maximilian Beier", Position::ST, dec!(86.42), dec!(1.020), false),
        (40220, "Cole Campbell", Position::RW, dec!(15.22), dec!(1.006), false),
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
