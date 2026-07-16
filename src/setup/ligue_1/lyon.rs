use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5006);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(837.06); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Olympique Lyonnais".to_string());

    let players = vec![
        (50251, "Lucas Perri", Position::GK, dec!(35.00), dec!(1.0), false),
        (50252, "Ainsley Maitland-Niles", Position::RB, dec!(42.88), dec!(1.010), false),
        (50253, "Duje Caleta-Car", Position::CB, dec!(35.00), dec!(1.0), false),
        (50254, "Moussa Niakhate", Position::CB, dec!(54.43), dec!(1.049), false),
        (50255, "Nicolas Tagliafico", Position::LB, dec!(32.01), dec!(0.920), false),
        (50256, "Nemanja Matic", Position::CDM, dec!(30.00), dec!(1.0), false),
        (50257, "Maxence Caqueret", Position::CM, dec!(55.00), dec!(1.0), false),
        (50258, "Corentin Tolisso", Position::CM, dec!(67.72), dec!(1.083), false),
        (50259, "Said Benrahma", Position::LW, dec!(50.00), dec!(1.0), false),
        (50260, "Ernest Nuamah", Position::RW, dec!(50.18), dec!(1.057), false),
        (50261, "Alexandre Lacazette", Position::ST, dec!(55.00), dec!(1.0), true),
        // Bench
        (50262, "Anthony Lopes", Position::GK, dec!(25.00), dec!(1.0), false),
        (50263, "Clinton Mata", Position::CB, dec!(33.01), dec!(1.004), false),
        (50264, "Abner", Position::LB, dec!(25.00), dec!(1.0), false),
        (50265, "Tanner Tessmann", Position::CDM, dec!(50.25), dec!(1.022), false),
        (50266, "Jordan Veretout", Position::CM, dec!(40.00), dec!(1.0), false),
        (50267, "Rayan Cherki", Position::CAM, dec!(65.00), dec!(1.0), false),
        (50268, "Georges Mikautadze", Position::ST, dec!(77.94), dec!(1.045), false),
        (50269, "Malick Fofana", Position::LW, dec!(55.64), dec!(1.058), false),
        (50270, "Wilfried Zaha", Position::LW, dec!(40.00), dec!(1.0), false),
    ];

    for (id, name, pos, val, form_weight, captain) in players {
        engine.player_states.insert(PlayerId(id), PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight,
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![],
            position: pos,
            is_captain: captain,
        });
        engine.names.insert(id, name.to_string());
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(PlayerId(id));
        }
    }
}
