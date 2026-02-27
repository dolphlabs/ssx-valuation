use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2012;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(340.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Getafe CF".to_string());

    let players = vec![
        (20551, "David Soria", Position::GK, dec!(30.0), false),
        (20552, "Juan Iglesias", Position::RB, dec!(25.0), false),
        (20553, "Djené", Position::CB, dec!(35.0), true),
        (20554, "Omar Alderete", Position::CB, dec!(40.0), false),
        (20555, "Diego Rico", Position::LB, dec!(25.0), false),
        (20556, "Mauro Arambarri", Position::CDM, dec!(40.0), false),
        (20557, "Luis Milla", Position::CM, dec!(45.0), false),
        (20558, "Carles Aleñá", Position::CAM, dec!(35.0), false),
        (20559, "Alex Sola", Position::RW, dec!(30.0), false),
        (20560, "Bertuğ Yıldırım", Position::ST, dec!(35.0), false),
        (20561, "Borja Mayoral", Position::ST, dec!(60.0), false),
        // Bench
        (20562, "Jiří Letáček", Position::GK, dec!(15.0), false),
        (20563, "Nabil Aberdin", Position::CB, dec!(10.0), false),
        (20564, "Fabrizio Angileri", Position::LB, dec!(20.0), false),
        (20565, "Yellu Santiago", Position::CM, dec!(25.0), false),
        (20566, "Chrisantus Uche", Position::CAM, dec!(30.0), false),
        (20567, "Peter Federico", Position::RW, dec!(25.0), false),
        (20568, "Álvaro Rodríguez", Position::ST, dec!(30.0), false),
        (20569, "Juan Latasa", Position::ST, dec!(30.0), false),
        (20570, "Carles Pérez", Position::RW, dec!(35.0), false),
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
