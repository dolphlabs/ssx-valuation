use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 4005;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(920.00); // Champions League finalists
    
    club.set_rival_factor(4002, dec!(1.5)); // Bayern Munich (Der Klassiker)
    club.set_rival_factor(4012, dec!(1.2)); // Wolfsburg? let's use Schalke if they were in BL, but they aren't.

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Borussia Dortmund".to_string());

    let players = vec![
        (40201, "Gregor Kobel", Position::GK, dec!(75.0), false),
        (40202, "Yan Couto", Position::RB, dec!(55.0), false),
        (40203, "Waldemar Anton", Position::CB, dec!(45.0), false),
        (40204, "Nico Schlotterbeck", Position::CB, dec!(75.0), false),
        (40205, "Julian Ryerson", Position::LB, dec!(40.0), false),
        (40206, "Emre Can", Position::CDM, dec!(50.0), true),
        (40207, "Pascal Gross", Position::CM, dec!(45.0), false),
        (40208, "Julian Brandt", Position::CAM, dec!(75.0), false),
        (40209, "Karim Adeyemi", Position::LW, dec!(60.0), false),
        (40210, "Donyell Malen", Position::RW, dec!(60.0), false),
        (40211, "Serhou Guirassy", Position::ST, dec!(70.0), false),
        // Bench
        (40212, "Alexander Meyer", Position::GK, dec!(15.0), false),
        (40213, "Niklas Sule", Position::CB, dec!(45.0), false),
        (40214, "Ramy Bensebaini", Position::LB, dec!(35.0), false),
        (40215, "Felix Nmecha", Position::CM, dec!(30.0), false),
        (40216, "Marcel Sabitzer", Position::CM, dec!(55.0), false),
        (40217, "Gio Reyna", Position::CAM, dec!(40.0), false),
        (40218, "Jamie Gittens", Position::LW, dec!(50.0), false),
        (40219, "Maximilian Beier", Position::ST, dec!(55.0), false),
        (40220, "Cole Campbell", Position::RW, dec!(15.0), false),
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
