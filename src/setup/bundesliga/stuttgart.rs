use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4003);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1071.83); // 2nd place last season
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "VfB Stuttgart".to_string());

    let players = vec![
        (40101, "Alexander Nubel", Position::GK, dec!(49.83), dec!(1.095), false),
        (40102, "Josha Vagnoman", Position::RB, dec!(38.52), dec!(1.016), false),
        (40103, "Anthony Rouault", Position::CB, dec!(30.00), dec!(1.0), false),
        (40104, "Jeff Chabot", Position::CB, dec!(45.82), dec!(1.097), false),
        (40105, "Maximilian Mittelstadt", Position::LB, dec!(72.28), dec!(1.071), false),
        (40106, "Atakan Karazor", Position::CDM, dec!(43.27), dec!(0.865), true),
        (40107, "Angelo Stiller", Position::CM, dec!(75.16), dec!(1.079), false),
        (40108, "Enzo Millot", Position::CAM, dec!(60.00), dec!(1.0), false),
        (40109, "Chris Fuhrich", Position::LW, dec!(84.68), dec!(1.085), false),
        (40110, "Jamie Leweling", Position::RW, dec!(71.80), dec!(1.043), false),
        (40111, "Ermedin Demirovic", Position::ST, dec!(79.79), dec!(1.0), false),
        // Bench
        (40112, "Fabian Bredlow", Position::GK, dec!(10.00), dec!(1.0), false),
        (40113, "Anrie Chase", Position::CB, dec!(20.00), dec!(1.0), false),
        (40114, "Ameen Al-Dakhil", Position::CB, dec!(36.86), dec!(1.038), false),
        (40115, "Pascal Stenzel", Position::RB, dec!(13.88), dec!(0.964), false),
        (40116, "Frans Kratzig", Position::LB, dec!(25.00), dec!(1.0), false),
        (40117, "Fabian Rieder", Position::CAM, dec!(40.00), dec!(1.0), false),
        (40118, "Yannik Keitel", Position::CDM, dec!(25.00), dec!(1.0), false),
        (40119, "Deniz Undav", Position::ST, dec!(134.09), dec!(1.070), false),
        (40120, "El Bilal Toure", Position::ST, dec!(45.00), dec!(1.0), false),
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
