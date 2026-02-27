use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2001;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1250.00); // Champions League winners
    
    // Rivalries
    club.set_rival_factor(2002, dec!(1.5)); // Barcelona
    club.set_rival_factor(2003, dec!(1.3)); // Atletico Madrid

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Real Madrid".to_string());

    let players = vec![
        (20001, "Thibaut Courtois", Position::GK, dec!(80.0), false),
        (20002, "Dani Carvajal", Position::RB, dec!(60.0), false),
        (20003, "Eder Militao", Position::CB, dec!(70.0), false),
        (20004, "Antonio Rudiger", Position::CB, dec!(75.0), false),
        (20005, "Ferland Mendy", Position::LB, dec!(55.0), false),
        (20006, "Aurelien Tchouameni", Position::CDM, dec!(90.0), false),
        (20007, "Federico Valverde", Position::CM, dec!(100.0), false),
        (20008, "Jude Bellingham", Position::CAM, dec!(150.0), false),
        (20009, "Rodrygo", Position::RW, dec!(100.0), false),
        (20010, "Vinicius Junior", Position::LW, dec!(150.0), false),
        (20011, "Kylian Mbappe", Position::ST, dec!(180.0), false),
        // Bench
        (20012, "Andriy Lunin", Position::GK, dec!(30.0), false),
        (20013, "David Alaba", Position::CB, dec!(45.0), false),
        (20014, "Fran Garcia", Position::LB, dec!(35.0), false),
        (20015, "Lucas Vazquez", Position::RB, dec!(30.0), false),
        (20016, "Eduardo Camavinga", Position::CM, dec!(90.0), false),
        (20017, "Luka Modric", Position::CM, dec!(40.0), true),
        (20018, "Arda Guler", Position::CAM, dec!(50.0), false),
        (20019, "Brahim Diaz", Position::RW, dec!(55.0), false),
        (20020, "Endrick", Position::ST, dec!(60.0), false),
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
