use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2001);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1668.39); // Champions League winners
    
    // Rivalries
    club.set_rival_factor(ClubId(2002), dec!(1.5)); // Barcelona
    club.set_rival_factor(ClubId(2003), dec!(1.3)); // Atletico Madrid

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Real Madrid".to_string());

    let players = vec![
        (20001, "Thibaut Courtois", Position::GK, dec!(86.30), dec!(0.957), false),
        (20002, "Dani Carvajal", Position::RB, dec!(67.56), dec!(1.060), false),
        (20003, "Eder Militao", Position::CB, dec!(88.01), dec!(1.075), false),
        (20004, "Antonio Rudiger", Position::CB, dec!(82.63), dec!(1.088), false),
        (20005, "Ferland Mendy", Position::LB, dec!(55.97), dec!(1.025), false),
        (20006, "Aurelien Tchouameni", Position::CDM, dec!(95.86), dec!(1.092), false),
        (20007, "Federico Valverde", Position::CM, dec!(116.81), dec!(1.073), false),
        (20008, "Jude Bellingham", Position::CAM, dec!(197.07), dec!(1.126), false),
        (20009, "Rodrygo", Position::RW, dec!(116.58), dec!(1.090), false),
        (20010, "Vinicius Junior", Position::LW, dec!(224.89), dec!(1.111), false),
        (20011, "Kylian Mbappe", Position::ST, dec!(310.09), dec!(1.145), false),
        // Bench
        (20012, "Andriy Lunin", Position::GK, dec!(12.03), dec!(1.054), false),
        (20013, "David Alaba", Position::CB, dec!(47.72), dec!(1.051), false),
        (20014, "Fran Garcia", Position::LB, dec!(50.15), dec!(1.090), false),
        (20015, "Lucas Vazquez", Position::RB, dec!(30.00), dec!(1.0), false),
        (20016, "Eduardo Camavinga", Position::CM, dec!(109.08), dec!(1.039), false),
        (20017, "Luka Modric", Position::CM, dec!(40.00), dec!(1.0), true),
        (20018, "Arda Guler", Position::CAM, dec!(82.18), dec!(1.035), false),
        (20019, "Brahim Diaz", Position::RW, dec!(59.59), dec!(1.102), false),
        (20020, "Endrick", Position::ST, dec!(24.43), dec!(0.510), false),
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
