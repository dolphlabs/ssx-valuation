use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4013);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(599.56); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Mainz 05".to_string());

    let players = vec![
        (40601, "Robin Zentner", Position::GK, dec!(26.64), dec!(1.023), false),
        (40602, "Dominik Kohr", Position::CB, dec!(29.17), dec!(1.085), false),
        (40603, "Stefan Bell", Position::CB, dec!(19.14), dec!(1.074), false),
        (40604, "Maxim Leitsch", Position::CB, dec!(29.05), dec!(1.068), false),
        (40605, "Anthony Caci", Position::RB, dec!(41.42), dec!(1.018), false),
        (40606, "Kaishu Sano", Position::CDM, dec!(44.43), dec!(1.056), false),
        (40607, "Nadiem Amiri", Position::CM, dec!(111.80), dec!(1.169), false),
        (40608, "Phillipp Mwene", Position::LB, dec!(28.41), dec!(1.097), false),
        (40609, "Lee Jae-sung", Position::CAM, dec!(52.44), dec!(1.0), false),
        (40610, "Paul Nebel", Position::LW, dec!(48.30), dec!(1.025), false),
        (40611, "Jonathan Burkardt", Position::ST, dec!(55.00), dec!(1.0), true),
        // Bench
        (40612, "Lasse Rieß", Position::GK, dec!(10.00), dec!(1.0), false),
        (40613, "Moritz Jenz", Position::CB, dec!(35.00), dec!(1.0), false),
        (40614, "Silvan Widmer", Position::RB, dec!(32.46), dec!(1.094), false),
        (40615, "Danny da Costa", Position::RB, dec!(21.90), dec!(1.064), false),
        (40616, "Hong Hyun-seok", Position::CM, dec!(35.00), dec!(1.0), false),
        (40617, "Armindo Sieb", Position::CAM, dec!(49.91), dec!(1.023), false),
        (40618, "Karim Onisiwo", Position::RW, dec!(25.00), dec!(1.0), false),
        (40619, "Nelson Weiper", Position::ST, dec!(31.71), dec!(1.019), false),
        (40620, "Gabriel Vidovic", Position::LW, dec!(40.00), dec!(1.0), false),
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
