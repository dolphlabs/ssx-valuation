use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5011);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(506.89); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Toulouse FC".to_string());

    let players = vec![
        (50501, "Guillaume Restes", Position::GK, dec!(55.91), dec!(1.029), false),
        (50502, "Djibril Sidibe", Position::RB, dec!(46.59), dec!(1.027), false),
        (50503, "Rasmus Nicolaisen", Position::CB, dec!(39.70), dec!(1.089), false),
        (50504, "Charlie Cresswell", Position::CB, dec!(41.39), dec!(1.078), false),
        (50505, "Gabriel Suazo", Position::LB, dec!(30.00), dec!(1.0), false),
        (50506, "Cristian Casseres Jr.", Position::CDM, dec!(37.77), dec!(1.080), false),
        (50507, "Vincent Sierro", Position::CM, dec!(30.00), dec!(1.0), true),
        (50508, "Yann Gboho", Position::CAM, dec!(57.51), dec!(1.125), false),
        (50509, "Zakaria Aboukhlal", Position::RW, dec!(45.00), dec!(1.0), false),
        (50510, "Aron Donnum", Position::LW, dec!(35.84), dec!(1.0), false),
        (50511, "Frank Magri", Position::ST, dec!(61.53), dec!(1.027), false),
        // Bench
        (50512, "Alex Dominguez", Position::GK, dec!(15.00), dec!(1.0), false),
        (50513, "Mark McKenzie", Position::CB, dec!(33.78), dec!(1.051), false),
        (50514, "Warren Kamanzi", Position::RB, dec!(27.21), dec!(1.052), false),
        (50515, "Miha Blazic", Position::CB, dec!(20.00), dec!(1.0), false),
        (50516, "Niklas Schmidt", Position::CM, dec!(35.00), dec!(1.0), false),
        (50517, "Shavy Babicka", Position::RW, dec!(25.00), dec!(1.0), false),
        (50518, "Joshua King", Position::ST, dec!(30.00), dec!(1.0), false),
        (50519, "Kingsteh Kamanzi", Position::RB, dec!(25.00), dec!(1.0), false),
        (50520, "Rafik Messali", Position::CM, dec!(16.01), dec!(1.069), false),
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
