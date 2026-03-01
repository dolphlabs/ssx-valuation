use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 4013;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(340.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Mainz 05".to_string());

    let players = vec![
        (40601, "Robin Zentner", Position::GK, dec!(30.0), false),
        (40602, "Dominik Kohr", Position::CB, dec!(30.0), false),
        (40603, "Stefan Bell", Position::CB, dec!(15.0), false),
        (40604, "Maxim Leitsch", Position::CB, dec!(25.0), false),
        (40605, "Anthony Caci", Position::RB, dec!(35.0), false),
        (40606, "Kaishu Sano", Position::CDM, dec!(30.0), false),
        (40607, "Nadiem Amiri", Position::CM, dec!(50.0), false),
        (40608, "Phillipp Mwene", Position::LB, dec!(25.0), false),
        (40609, "Lee Jae-sung", Position::CAM, dec!(40.0), false),
        (40610, "Paul Nebel", Position::LW, dec!(35.0), false),
        (40611, "Jonathan Burkardt", Position::ST, dec!(55.0), true),
        // Bench
        (40612, "Lasse Rieß", Position::GK, dec!(10.0), false),
        (40613, "Moritz Jenz", Position::CB, dec!(35.0), false),
        (40614, "Silvan Widmer", Position::RB, dec!(25.0), false),
        (40615, "Danny da Costa", Position::RB, dec!(15.0), false),
        (40616, "Hong Hyun-seok", Position::CM, dec!(35.0), false),
        (40617, "Armindo Sieb", Position::CAM, dec!(40.0), false),
        (40618, "Karim Onisiwo", Position::RW, dec!(25.0), false),
        (40619, "Nelson Weiper", Position::ST, dec!(30.0), false),
        (40620, "Gabriel Vidovic", Position::LW, dec!(40.0), false),
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
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(id);
        }
    }
}
