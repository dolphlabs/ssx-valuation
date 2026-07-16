use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5003);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(760.89); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Lille OSC".to_string());

    let players = vec![
        (50101, "Lucas Chevalier", Position::GK, dec!(60.00), dec!(1.0), false),
        (50102, "Tiago Santos", Position::RB, dec!(48.53), dec!(1.036), false),
        (50103, "Bafode Diakite", Position::CB, dec!(35.00), dec!(1.0), false),
        (50104, "Alexsandro", Position::CB, dec!(30.00), dec!(1.0), false),
        (50105, "Gabriel Gudmundsson", Position::LB, dec!(30.00), dec!(1.0), false),
        (50106, "Benjamin Andre", Position::CDM, dec!(36.09), dec!(1.072), true),
        (50107, "Angel Gomes", Position::CM, dec!(55.00), dec!(1.0), false),
        (50108, "Edon Zhegrova", Position::RW, dec!(65.00), dec!(1.0), false),
        (50109, "Remy Cabella", Position::CAM, dec!(25.00), dec!(1.0), false),
        (50110, "Osame Sahraoui", Position::LW, dec!(42.99), dec!(1.020), false),
        (50111, "Jonathan David", Position::ST, dec!(95.00), dec!(1.0), false),
        // Bench
        (50112, "Vito Mannone", Position::GK, dec!(10.00), dec!(1.0), false),
        (50113, "Aissa Mandi", Position::CB, dec!(27.88), dec!(1.046), false),
        (50114, "Thomas Meunier", Position::RB, dec!(37.86), dec!(1.123), false),
        (50115, "Mitchel Bakker", Position::LB, dec!(30.00), dec!(1.0), false),
        (50116, "Ngal'ayel Mukau", Position::CM, dec!(43.87), dec!(1.021), false),
        (50117, "Andre Gomes", Position::CM, dec!(35.97), dec!(1.021), false),
        (50118, "Hakon Arnar Haraldsson", Position::CAM, dec!(75.49), dec!(1.036), false),
        (50119, "Mohamed Bayo", Position::ST, dec!(35.00), dec!(1.0), false),
        (50120, "Matias Fernandez-Pardo", Position::LW, dec!(39.85), dec!(1.059), false),
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
