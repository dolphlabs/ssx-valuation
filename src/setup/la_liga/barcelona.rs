use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2002);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1765.24); 
    
    // Rivalries
    club.set_rival_factor(ClubId(2001), dec!(1.5)); // Real Madrid
    club.set_rival_factor(ClubId(2020), dec!(1.2)); // Espanyol

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "FC Barcelona".to_string());

    let players = vec![
        (20051, "Marc-Andre ter Stegen", Position::GK, dec!(70.00), dec!(1.0), true),
        (20052, "Jules Kounde", Position::RB, dec!(71.07), dec!(0.977), false),
        (20053, "Pau Cubarsi", Position::CB, dec!(59.84), dec!(1.086), false),
        (20054, "Inigo Martinez", Position::CB, dec!(40.00), dec!(1.0), false),
        (20055, "Alejandro Balde", Position::LB, dec!(62.40), dec!(1.025), false),
        (20056, "Marc Casado", Position::CDM, dec!(43.90), dec!(1.012), false),
        (20057, "Pedri", Position::CM, dec!(100.10), dec!(1.089), false),
        (20058, "Dani Olmo", Position::CAM, dec!(121.63), dec!(1.027), false),
        (20059, "Lamine Yamal", Position::RW, dec!(254.58), dec!(1.164), false),
        (20060, "Raphinha", Position::LW, dec!(132.14), dec!(1.091), false),
        (20061, "Robert Lewandowski", Position::ST, dec!(130.75), dec!(1.095), false),
        // Bench
        (20062, "Inaki Pena", Position::GK, dec!(20.00), dec!(1.0), false),
        (20063, "Ronald Araujo", Position::CB, dec!(85.01), dec!(1.139), false),
        (20064, "Andreas Christensen", Position::CB, dec!(51.71), dec!(0.998), false),
        (20065, "Eric Garcia", Position::CB, dec!(39.03), dec!(1.100), false),
        (20066, "Frenkie de Jong", Position::CM, dec!(68.04), dec!(1.017), false),
        (20067, "Gavi", Position::CM, dec!(93.99), dec!(1.059), false),
        (20068, "Fermin Lopez", Position::CAM, dec!(67.72), dec!(1.044), false),
        (20069, "Ferran Torres", Position::LW, dec!(114.69), dec!(1.103), false),
        (20070, "Pau Victor", Position::ST, dec!(30.00), dec!(1.0), false),
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
