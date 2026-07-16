use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2012);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(371.28); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Getafe CF".to_string());

    let players = vec![
        (20551, "David Soria", Position::GK, dec!(31.11), dec!(1.165), false),
        (20552, "Juan Iglesias", Position::RB, dec!(27.82), dec!(1.081), false),
        (20553, "Djené", Position::CB, dec!(27.74), dec!(0.951), true),
        (20554, "Omar Alderete", Position::CB, dec!(40.00), dec!(1.0), false),
        (20555, "Diego Rico", Position::LB, dec!(26.26), dec!(1.028), false),
        (20556, "Mauro Arambarri", Position::CDM, dec!(56.26), dec!(1.046), false),
        (20557, "Luis Milla", Position::CM, dec!(44.29), dec!(1.103), false),
        (20558, "Carles Aleñá", Position::CAM, dec!(35.00), dec!(1.0), false),
        (20559, "Alex Sola", Position::RW, dec!(30.81), dec!(1.009), false),
        (20560, "Bertuğ Yıldırım", Position::ST, dec!(35.00), dec!(1.0), false),
        (20561, "Borja Mayoral", Position::ST, dec!(76.87), dec!(1.028), false),
        // Bench
        (20562, "Jiří Letáček", Position::GK, dec!(15.00), dec!(1.0), false),
        (20563, "Nabil Aberdin", Position::CB, dec!(10.00), dec!(1.0), false),
        (20564, "Fabrizio Angileri", Position::LB, dec!(20.00), dec!(1.0), false),
        (20565, "Yellu Santiago", Position::CM, dec!(25.00), dec!(1.0), false),
        (20566, "Chrisantus Uche", Position::CAM, dec!(43.65), dec!(1.026), false),
        (20567, "Peter Federico", Position::RW, dec!(25.00), dec!(1.0), false),
        (20568, "Álvaro Rodríguez", Position::ST, dec!(30.00), dec!(1.0), false),
        (20569, "Juan Latasa", Position::ST, dec!(30.00), dec!(1.0), false),
        (20570, "Carles Pérez", Position::RW, dec!(35.67), dec!(1.014), false),
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
