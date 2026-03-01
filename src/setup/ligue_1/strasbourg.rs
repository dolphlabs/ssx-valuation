use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 5013;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(350.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "RC Strasbourg Alsace".to_string());

    let players = vec![
        (50601, "Djordje Petrovic", Position::GK, dec!(45.0), false),
        (50602, "Guela Doue", Position::RB, dec!(30.0), false),
        (50603, "Saidou Sow", Position::CB, dec!(25.0), false),
        (50604, "Abakar Sylla", Position::CB, dec!(35.0), false),
        (50605, "Diego Moreira", Position::LB, dec!(35.0), false),
        (50606, "Andrey Santos", Position::CDM, dec!(50.0), false),
        (50607, "Ismael Doukoure", Position::CDM, dec!(30.0), false),
        (50608, "Habib Diarra", Position::CM, dec!(45.0), true),
        (50609, "Sebastian Nanasi", Position::LW, dec!(40.0), false),
        (50610, "Dilane Bakwa", Position::RW, dec!(35.0), false),
        (50611, "Emanuel Emegha", Position::ST, dec!(40.0), false),
        // Bench
        (50612, "Karl-Johan Johnsson", Position::GK, dec!(10.0), false),
        (50613, "Mamadou Sarr", Position::CB, dec!(30.0), false),
        (50614, "Caleb Wiley", Position::LB, dec!(25.0), false),
        (50615, "Junior Mwanga", Position::CDM, dec!(30.0), false),
        (50616, "Felix Lemarechal", Position::CM, dec!(30.0), false),
        (50617, "Sékou Mara", Position::ST, dec!(35.0), false),
        (50618, "Marvin Senaya", Position::RB, dec!(25.0), false),
        (50619, "Oscar Perea", Position::LW, dec!(20.0), false),
        (50620, "Milosh Lukovic", Position::ST, dec!(25.0), false),
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
