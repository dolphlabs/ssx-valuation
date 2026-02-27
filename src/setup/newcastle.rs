use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 7;
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id, "Newcastle United".to_string());
    
    club.intrinsic_value = dec!(800.00);
    
    // Rivals: Sunderland (Not in PL), but let's add some strong competition factors
    club.set_rival_factor(1, dec!(1.2)); // Man City
    club.set_rival_factor(5, dec!(1.2)); // Liverpool

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1301, "Nick Pope", Position::GK, dec!(50.0), false),
        (1302, "Bruno Guimaraes", Position::CDM, dec!(95.0), true),
        (1303, "Alexander Isak", Position::ST, dec!(100.0), false),
        (1304, "Anthony Gordon", Position::LW, dec!(80.0), false),
        (1305, "Sandro Tonali", Position::CM, dec!(75.0), false),
        (1306, "Joelinton", Position::CM, dec!(70.0), false),
        (1307, "Fabian Schar", Position::CB, dec!(55.0), false),
        (1308, "Sven Botman", Position::CB, dec!(65.0), false),
        (1309, "Kieran Trippier", Position::RB, dec!(50.0), false),
        (1310, "Tino Livramento", Position::RB, dec!(55.0), false),
        (1311, "Harvey Barnes", Position::LW, dec!(60.0), false),
        (1312, "Odysseas Vlachodimos", Position::GK, dec!(25.0), false),
        (1313, "Lloyd Kelly", Position::CB, dec!(45.0), false),
        (1314, "Dan Burn", Position::CB, dec!(40.0), false),
        (1315, "Lewis Hall", Position::LB, dec!(50.0), false),
        (1316, "Sean Longstaff", Position::CM, dec!(45.0), false),
        (1317, "Joe Willock", Position::CM, dec!(50.0), false),
        (1318, "Jacob Murphy", Position::RW, dec!(45.0), false),
        (1319, "Miguel Almiron", Position::RW, dec!(45.0), false),
        (1320, "Callum Wilson", Position::ST, dec!(55.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.names.insert(id, name.to_string());
        engine.player_states.insert(id, PlayerValues {
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
