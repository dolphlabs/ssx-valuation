use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(19);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Leicester City".to_string());
    
    club.intrinsic_value = dec!(320.00);
    
    // Rivals: Nottingham Forest (17), Aston Villa (8)
    club.set_rival_factor(ClubId(17), dec!(1.4));
    club.set_rival_factor(ClubId(8), dec!(1.1));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1901, "Mads Hermansen", Position::GK, dec!(30.0), dec!(1.0), false),
        (1902, "Jamie Vardy", Position::ST, dec!(35.0), dec!(1.0), true),
        (1903, "Harry Winks", Position::CM, dec!(40.0), dec!(1.0), false),
        (1904, "Stephy Mavididi", Position::LW, dec!(40.0), dec!(1.0), false),
        (1905, "Wilfred Ndidi", Position::CDM, dec!(45.0), dec!(1.0), false),
        (1906, "Wout Faes", Position::CB, dec!(40.0), dec!(1.0), false),
        (1907, "Jannik Vestergaard", Position::CB, dec!(30.0), dec!(1.0), false),
        (1908, "James Justin", Position::RB, dec!(35.0), dec!(1.0), false),
        (1909, "Abdul Fatawu", Position::RW, dec!(45.0), dec!(1.0), false),
        (1910, "Facundo Buonanotte", Position::CAM, dec!(45.0), dec!(1.0), false),
        (1911, "Jordan Ayew", Position::ST, dec!(30.0), dec!(1.0), false),
        (1912, "Danny Ward", Position::GK, dec!(10.0), dec!(1.0), false),
        (1913, "Caleb Okoli", Position::CB, dec!(35.0), dec!(1.0), false),
        (1914, "Conor Coady", Position::CB, dec!(25.0), dec!(1.0), false),
        (1915, "Victor Kristiansen", Position::LB, dec!(35.0), dec!(1.0), false),
        (1916, "Oliver Skipp", Position::CDM, dec!(40.0), dec!(1.0), false),
        (1917, "Boubakary Soumare", Position::CM, dec!(30.0), dec!(1.0), false),
        (1918, "Bilal El Khannouss", Position::CAM, dec!(45.0), dec!(1.0), false),
        (1919, "Kasey McAteer", Position::LW, dec!(25.0), dec!(1.0), false),
        (1920, "Odsonne Edouard", Position::ST, dec!(40.0), dec!(1.0), false),
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
