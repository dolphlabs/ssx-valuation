use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(17);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Nottingham Forest".to_string());
    
    club.intrinsic_value = dec!(420.00);
    
    // Rivals: Leicester (19), Derby (Not in PL)
    club.set_rival_factor(ClubId(19), dec!(1.4));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1801, "Matz Sels", Position::GK, dec!(30.0), false),
        (1802, "Morgan Gibbs-White", Position::CAM, dec!(75.0), true),
        (1803, "Chris Wood", Position::ST, dec!(45.0), false),
        (1804, "Callum Hudson-Odoi", Position::LW, dec!(55.0), false),
        (1805, "Anthony Elanga", Position::RW, dec!(50.0), false),
        (1806, "Murillo", Position::CB, dec!(65.0), false),
        (1807, "Nikola Milenkovic", Position::CB, dec!(50.0), false),
        (1808, "Ola Aina", Position::RB, dec!(40.0), false),
        (1809, "Elliot Anderson", Position::CM, dec!(45.0), false),
        (1810, "Taiwo Awoniyi", Position::ST, dec!(50.0), false),
        (1811, "Danilo", Position::CDM, dec!(45.0), false),
        (1812, "Carlos Miguel", Position::GK, dec!(25.0), false),
        (1813, "Andrew Omobamidele", Position::CB, dec!(35.0), false),
        (1814, "Morrys", Position::CB, dec!(50.0), false), // Morato? let's use Morato
        (1815, "Neco Williams", Position::RB, dec!(45.0), false),
        (1816, "Ibrahim Sangare", Position::CDM, dec!(50.0), false),
        (1817, "James Ward-Prowse", Position::CM, dec!(55.0), false),
        (1818, "Nicolas Dominguez", Position::CM, dec!(45.0), false),
        (1819, "Ramon Sosa", Position::RW, dec!(40.0), false),
        (1820, "Jota Silva", Position::LW, dec!(45.0), false),
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
