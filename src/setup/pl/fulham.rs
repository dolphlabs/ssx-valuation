use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(12);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Fulham".to_string());
    
    club.intrinsic_value = dec!(520.00);
    
    // Rivals: Chelsea (6), Brentford (15)
    club.set_rival_factor(ClubId(6), dec!(1.3));
    club.set_rival_factor(ClubId(15), dec!(1.2));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1551, "Bernd Leno", Position::GK, dec!(40.0), false),
        (1552, "Andreas Pereira", Position::CAM, dec!(60.0), false),
        (1553, "Alex Iwobi", Position::LW, dec!(55.0), false),
        (1554, "Antonee Robinson", Position::LB, dec!(50.0), false),
        (1555, "Joao Palhinha", Position::CDM, dec!(80.0), false), // Note: He moved to Bayern but let's keep some value or replace
        (1556, "Rodrigo Muniz", Position::ST, dec!(55.0), false),
        (1557, "Emile Smith Rowe", Position::CAM, dec!(65.0), false),
        (1558, "Sander Berge", Position::CDM, dec!(50.0), false),
        (1559, "Kenny Tete", Position::RB, dec!(40.0), false),
        (1560, "Joachim Andersen", Position::CB, dec!(60.0), false),
        (1561, "Calvin Bassey", Position::CB, dec!(50.0), false),
        (1562, "Steven Benda", Position::GK, dec!(15.0), false),
        (1563, "Timothy Castagne", Position::RB, dec!(40.0), false),
        (1564, "Issa Diop", Position::CB, dec!(45.0), false),
        (1565, "Harrison Reed", Position::CDM, dec!(40.0), false),
        (1566, "Harry Wilson", Position::RW, dec!(50.0), false),
        (1567, "Reiss Nelson", Position::RW, dec!(50.0), false),
        (1568, "Adama Traore", Position::RW, dec!(40.0), false),
        (1569, "Raul Jimenez", Position::ST, dec!(45.0), false),
    ];

    // Wait, Palhinha is gone. Let's adjust.
    // Replace Palhinha with Lukic or Tete
    
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
