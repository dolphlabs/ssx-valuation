use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(10);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Brighton & Hove Albion".to_string());
    
    club.intrinsic_value = dec!(962.36);
    
    // Rivals: Crystal Palace (14)
    club.set_rival_factor(ClubId(14), dec!(1.5));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1451, "Bart Verbruggen", Position::GK, dec!(42.78), dec!(0.990), false),
        (1452, "Lewis Dunk", Position::CB, dec!(59.85), dec!(1.081), true),
        (1453, "Kaoru Mitoma", Position::LW, dec!(98.30), dec!(0.982), false),
        (1454, "Joao Pedro", Position::ST, dec!(75.00), dec!(1.0), false),
        (1455, "Carlos Baleba", Position::CDM, dec!(63.35), dec!(1.076), false),
        (1456, "Pervis Estupinan", Position::LB, dec!(55.00), dec!(1.0), false),
        (1457, "Jan Paul van Hecke", Position::CB, dec!(74.04), dec!(1.094), false),
        (1458, "Simon Adingra", Position::RW, dec!(60.00), dec!(1.0), false),
        (1459, "Yankuba Minteh", Position::RW, dec!(80.52), dec!(1.028), false),
        (1460, "Georginio Rutter", Position::ST, dec!(77.84), dec!(1.023), false),
        (1461, "Jack Hinshelwood", Position::CM, dec!(59.06), dec!(1.026), false),
        (1462, "Jason Steele", Position::GK, dec!(20.00), dec!(1.0), false),
        (1463, "Joel Veltman", Position::RB, dec!(39.71), dec!(1.009), false),
        (1464, "Igor Julio", Position::CB, dec!(40.00), dec!(1.0), false),
        (1465, "Ferdi Kadioglu", Position::LB, dec!(56.80), dec!(1.0), false),
        (1466, "Mats Wieffer", Position::CDM, dec!(56.59), dec!(1.044), false),
        (1467, "Matt O'Riley", Position::CM, dec!(57.77), dec!(1.024), false),
        (1468, "Yasin Ayari", Position::CM, dec!(41.58), dec!(1.021), false),
        (1469, "Brajan Gruda", Position::RW, dec!(53.59), dec!(1.056), false),
        (1470, "Evan Ferguson", Position::ST, dec!(60.00), dec!(1.0), false),
        (1471, "Danny Welbeck", Position::ST, dec!(74.09), dec!(1.005), false),
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
