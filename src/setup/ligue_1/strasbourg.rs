use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5013);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(715.22); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "RC Strasbourg Alsace".to_string());

    let players = vec![
        (50601, "Djordje Petrovic", Position::GK, dec!(45.00), dec!(1.0), false),
        (50602, "Guela Doue", Position::RB, dec!(36.78), dec!(1.073), false),
        (50603, "Saidou Sow", Position::CB, dec!(25.00), dec!(1.0), false),
        (50604, "Abakar Sylla", Position::CB, dec!(35.00), dec!(1.0), false),
        (50605, "Diego Moreira", Position::LB, dec!(49.37), dec!(1.175), false),
        (50606, "Andrey Santos", Position::CDM, dec!(50.00), dec!(1.0), false),
        (50607, "Ismael Doukoure", Position::CDM, dec!(21.91), dec!(1.031), false),
        (50608, "Habib Diarra", Position::CM, dec!(45.00), dec!(1.0), true),
        (50609, "Sebastian Nanasi", Position::LW, dec!(60.73), dec!(1.195), false),
        (50610, "Dilane Bakwa", Position::RW, dec!(60.14), dec!(1.134), false),
        (50611, "Emanuel Emegha", Position::ST, dec!(61.73), dec!(1.014), false),
        // Bench
        (50612, "Karl-Johan Johnsson", Position::GK, dec!(11.72), dec!(1.060), false),
        (50613, "Mamadou Sarr", Position::CB, dec!(34.88), dec!(1.044), false),
        (50614, "Caleb Wiley", Position::LB, dec!(25.00), dec!(1.0), false),
        (50615, "Junior Mwanga", Position::CDM, dec!(34.85), dec!(1.043), false),
        (50616, "Felix Lemarechal", Position::CM, dec!(32.43), dec!(1.020), false),
        (50617, "Sékou Mara", Position::ST, dec!(35.00), dec!(1.0), false),
        (50618, "Marvin Senaya", Position::RB, dec!(25.00), dec!(1.0), false),
        (50619, "Oscar Perea", Position::LW, dec!(20.00), dec!(1.0), false),
        (50620, "Milosh Lukovic", Position::ST, dec!(25.00), dec!(1.0), false),
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
            active: true,
        });
        engine.names.insert(id, name.to_string());
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(PlayerId(id));
        }
    }
}
