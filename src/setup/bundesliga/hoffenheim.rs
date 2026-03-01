use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4007);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(400.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "TSG Hoffenheim".to_string());

    let players = vec![
        (40301, "Oliver Baumann", Position::GK, dec!(30.0), true),
        (40302, "Pavel Kaderabek", Position::RM, dec!(20.0), false),
        (40303, "Anton Stach", Position::CB, dec!(45.0), false),
        (40304, "Kevin Akpoguma", Position::CB, dec!(20.0), false),
        (40305, "Alexander Prass", Position::LM, dec!(35.0), false),
        (40306, "Florian Grillitsch", Position::CDM, dec!(30.0), false),
        (40307, "Tom Bischof", Position::CM, dec!(25.0), false),
        (40308, "Andrej Kramaric", Position::CAM, dec!(45.0), false),
        (40309, "Adam Hlozek", Position::LW, dec!(45.0), false),
        (40310, "Marius Bulter", Position::RW, dec!(25.0), false),
        (40311, "Mergim Berisha", Position::ST, dec!(35.0), false),
        // Bench
        (40312, "Luca Philipp", Position::GK, dec!(10.0), false),
        (40313, "Arthur Chaves", Position::CB, dec!(25.0), false),
        (40314, "Valentin Gendrey", Position::RB, dec!(25.0), false),
        (40315, "David Jurasek", Position::LB, dec!(20.0), false),
        (40316, "Umut Tohumcu", Position::CM, dec!(25.0), false),
        (40317, "Dennis Geiger", Position::CM, dec!(20.0), false),
        (40318, "Jacob Bruun Larsen", Position::LW, dec!(30.0), false),
        (40319, "Haris Tabakovic", Position::ST, dec!(25.0), false),
        (40320, "Ihlas Bebou", Position::ST, dec!(30.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.player_states.insert(PlayerId(id), PlayerValues {
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
            c.player_ids.push(PlayerId(id));
        }
    }
}
