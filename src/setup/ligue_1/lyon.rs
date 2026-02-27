use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 5006;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(600.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Olympique Lyonnais".to_string());

    let players = vec![
        (50251, "Lucas Perri", Position::GK, dec!(35.0), false),
        (50252, "Ainsley Maitland-Niles", Position::RB, dec!(35.0), false),
        (50253, "Duje Caleta-Car", Position::CB, dec!(35.0), false),
        (50254, "Moussa Niakhate", Position::CB, dec!(45.0), false),
        (50255, "Nicolas Tagliafico", Position::LB, dec!(35.0), false),
        (50256, "Nemanja Matic", Position::CDM, dec!(30.0), false),
        (50257, "Maxence Caqueret", Position::CM, dec!(55.0), false),
        (50258, "Corentin Tolisso", Position::CM, dec!(40.0), false),
        (50259, "Said Benrahma", Position::LW, dec!(50.0), false),
        (50260, "Ernest Nuamah", Position::RW, dec!(50.0), false),
        (50261, "Alexandre Lacazette", Position::ST, dec!(55.0), true),
        // Bench
        (50262, "Anthony Lopes", Position::GK, dec!(25.0), false),
        (50263, "Clinton Mata", Position::CB, dec!(30.0), false),
        (50264, "Abner", Position::LB, dec!(25.0), false),
        (50265, "Tanner Tessmann", Position::CDM, dec!(40.0), false),
        (50266, "Jordan Veretout", Position::CM, dec!(40.0), false),
        (50267, "Rayan Cherki", Position::CAM, dec!(65.0), false),
        (50268, "Georges Mikautadze", Position::ST, dec!(55.0), false),
        (50269, "Malick Fofana", Position::LW, dec!(45.0), false),
        (50270, "Wilfried Zaha", Position::LW, dec!(40.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.player_states.insert(id, PlayerValues {
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
        if let Some(c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(id);
        }
    }
}
