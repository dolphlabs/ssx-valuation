use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(7);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Newcastle United".to_string());
    
    club.intrinsic_value = dec!(1095.69);
    
    // Rivals: Sunderland (Not in PL), but let's add some strong competition factors
    club.set_rival_factor(ClubId(1), dec!(1.2)); // Man City
    club.set_rival_factor(ClubId(5), dec!(1.2)); // Liverpool

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1301, "Nick Pope", Position::GK, dec!(54.66), dec!(1.019), false),
        (1302, "Bruno Guimaraes", Position::CDM, dec!(135.05), dec!(1.040), true),
        (1303, "Alexander Isak", Position::ST, dec!(100.00), dec!(1.0), false),
        (1304, "Anthony Gordon", Position::LW, dec!(87.89), dec!(1.062), false),
        (1305, "Sandro Tonali", Position::CM, dec!(86.54), dec!(1.046), false),
        (1306, "Joelinton", Position::CM, dec!(69.95), dec!(1.076), false),
        (1307, "Fabian Schar", Position::CB, dec!(65.37), dec!(1.068), false),
        (1308, "Sven Botman", Position::CB, dec!(77.25), dec!(1.108), false),
        (1309, "Kieran Trippier", Position::RB, dec!(53.76), dec!(1.011), false),
        (1310, "Tino Livramento", Position::RB, dec!(66.19), dec!(1.079), false),
        (1311, "Harvey Barnes", Position::LW, dec!(81.31), dec!(1.027), false),
        (1312, "Odysseas Vlachodimos", Position::GK, dec!(25.00), dec!(1.0), false),
        (1313, "Lloyd Kelly", Position::CB, dec!(45.00), dec!(1.0), false),
        (1314, "Dan Burn", Position::CB, dec!(38.75), dec!(1.061), false),
        (1315, "Lewis Hall", Position::LB, dec!(60.24), dec!(1.057), false),
        (1316, "Sean Longstaff", Position::CM, dec!(45.00), dec!(1.0), false),
        (1317, "Joe Willock", Position::CM, dec!(52.30), dec!(1.052), false),
        (1318, "Jacob Murphy", Position::RW, dec!(58.88), dec!(1.022), false),
        (1319, "Miguel Almiron", Position::RW, dec!(45.00), dec!(1.0), false),
        (1320, "Callum Wilson", Position::ST, dec!(55.00), dec!(1.0), false),
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
