use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(18);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Ipswich Town".to_string());
    
    club.intrinsic_value = dec!(280.00);
    
    // Rivals: Norwich (Not in PL)
    
    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1851, "Arijanet Muric", Position::GK, dec!(25.0), false),
        (1852, "Leif Davis", Position::LB, dec!(40.0), false),
        (1853, "Omari Hutchinson", Position::RW, dec!(45.0), false),
        (1854, "Liam Delap", Position::ST, dec!(40.0), false),
        (1855, "Sam Morsy", Position::CM, dec!(30.0), true),
        (1856, "Jacob Greaves", Position::CB, dec!(35.0), false),
        (1857, "Dara O'Shea", Position::CB, dec!(35.0), false),
        (1858, "Sammie Szmodics", Position::CAM, dec!(35.0), false),
        (1859, "Jack Clarke", Position::LW, dec!(40.0), false),
        (1860, "Kalvin Phillips", Position::CDM, dec!(40.0), false),
        (1861, "Chiedozie Ogbene", Position::RW, dec!(30.0), false),
        (1862, "Christian Walton", Position::GK, dec!(10.0), false),
        (1863, "Luke Woolfenden", Position::CB, dec!(20.0), false),
        (1864, "Ben Johnson", Position::RB, dec!(25.0), false),
        (1865, "Jens Cajuste", Position::CM, dec!(30.0), false),
        (1866, "Massimo Luongo", Position::CM, dec!(20.0), false),
        (1867, "Conor Chaplin", Position::CAM, dec!(25.0), false),
        (1868, "Wes Burns", Position::RW, dec!(20.0), false),
        (1869, "George Hirst", Position::ST, dec!(20.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.names.insert(id, name.to_string());
        engine.player_states.insert(PlayerId(id), PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight: dec!(1.0),
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![val; 5],
            position: pos,
            is_captain: captain,
        });
    }
}
