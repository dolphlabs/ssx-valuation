use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(20);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Southampton".to_string());
    
    club.intrinsic_value = dec!(290.00);
    
    // Rivals: Portsmouth (Not in PL), Bournemouth (13)
    club.set_rival_factor(ClubId(13), dec!(1.3));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1951, "Aaron Ramsdale", Position::GK, dec!(45.0), dec!(1.0), false),
        (1952, "Kyle Walker-Peters", Position::RB, dec!(40.0), dec!(1.0), false),
        (1953, "Jan Bednarek", Position::CB, dec!(30.0), dec!(1.0), false),
        (1954, "Adam Armstrong", Position::ST, dec!(35.0), dec!(1.0), false),
        (1955, "Flynn Downes", Position::CDM, dec!(40.0), dec!(1.0), true),
        (1956, "Tyler Dibling", Position::RW, dec!(40.0), dec!(1.0), false),
        (1957, "Mateus Fernandes", Position::CAM, dec!(45.0), dec!(1.0), false),
        (1958, "Taylor Harwood-Bellis", Position::CB, dec!(35.0), dec!(1.0), false),
        (1959, "Cameron Archer", Position::ST, dec!(35.0), dec!(1.0), false),
        (1960, "Ben Brereton Diaz", Position::LW, dec!(35.0), dec!(1.0), false),
        (1961, "Yukinari Sugawara", Position::RB, dec!(35.0), dec!(1.0), false),
        (1962, "Alex McCarthy", Position::GK, dec!(10.0), dec!(1.0), false),
        (1963, "Jack Stephens", Position::CB, dec!(20.0), dec!(1.0), false),
        (1964, "Nathan Wood", Position::CB, dec!(25.0), dec!(1.0), false),
        (1965, "Charlie Taylor", Position::LB, dec!(20.0), dec!(1.0), false),
        (1966, "Adam Lallana", Position::CM, dec!(15.0), dec!(1.0), false),
        (1967, "Joe Aribo", Position::CM, dec!(25.0), dec!(1.0), false),
        (1968, "Ryan Fraser", Position::LW, dec!(20.0), dec!(1.0), false),
        (1969, "Samuel Edozie", Position::RW, dec!(25.0), dec!(1.0), false),
        (1970, "Ross Stewart", Position::ST, dec!(20.0), dec!(1.0), false),
    ];

    for (id, name, pos, val, form_weight, captain) in players {
        engine.names.insert(id, name.to_string());
        engine.player_states.insert(PlayerId(id), PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight,
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![val; 5],
            position: pos,
            is_captain: captain,
        });
    }
}
