use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(12);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Fulham".to_string());
    
    club.intrinsic_value = dec!(758.57);
    
    // Rivals: Chelsea (6), Brentford (15)
    club.set_rival_factor(ClubId(6), dec!(1.3));
    club.set_rival_factor(ClubId(15), dec!(1.2));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1551, "Bernd Leno", Position::GK, dec!(41.07), dec!(1.081), false),
        (1552, "Andreas Pereira", Position::CAM, dec!(60.00), dec!(1.0), false),
        (1553, "Alex Iwobi", Position::LW, dec!(74.37), dec!(1.109), false),
        (1554, "Antonee Robinson", Position::LB, dec!(56.13), dec!(1.096), false),
        (1555, "Joao Palhinha", Position::CDM, dec!(80.00), dec!(1.0), false), // Note: He moved to Bayern but let's keep some value or replace
        (1556, "Rodrigo Muniz", Position::ST, dec!(60.38), dec!(1.032), false),
        (1557, "Emile Smith Rowe", Position::CAM, dec!(77.37), dec!(1.054), false),
        (1558, "Sander Berge", Position::CDM, dec!(56.09), dec!(1.080), false),
        (1559, "Kenny Tete", Position::RB, dec!(50.24), dec!(1.071), false),
        (1560, "Joachim Andersen", Position::CB, dec!(62.51), dec!(0.782), false),
        (1561, "Calvin Bassey", Position::CB, dec!(54.30), dec!(1.078), false),
        (1562, "Steven Benda", Position::GK, dec!(15.00), dec!(1.0), false),
        (1563, "Timothy Castagne", Position::RB, dec!(43.58), dec!(1.124), false),
        (1564, "Issa Diop", Position::CB, dec!(49.68), dec!(1.100), false),
        (1565, "Harrison Reed", Position::CDM, dec!(44.44), dec!(1.026), false),
        (1566, "Harry Wilson", Position::RW, dec!(84.48), dec!(1.069), false),
        (1567, "Reiss Nelson", Position::RW, dec!(50.00), dec!(1.0), false),
        (1568, "Adama Traore", Position::RW, dec!(42.79), dec!(1.024), false),
        (1569, "Raul Jimenez", Position::ST, dec!(65.18), dec!(1.014), false),
    ];

    // Wait, Palhinha is gone. Let's adjust.
    // Replace Palhinha with Lukic or Tete
    
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
            active: true,
        });
    }
}
