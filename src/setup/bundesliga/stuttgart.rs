use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 4003;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(550.00); // 2nd place last season
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "VfB Stuttgart".to_string());

    let players = vec![
        (40101, "Alexander Nubel", Position::GK, dec!(45.0), false),
        (40102, "Josha Vagnoman", Position::RB, dec!(35.0), false),
        (40103, "Anthony Rouault", Position::CB, dec!(30.0), false),
        (40104, "Jeff Chabot", Position::CB, dec!(35.0), false),
        (40105, "Maximilian Mittelstadt", Position::LB, dec!(45.0), false),
        (40106, "Atakan Karazor", Position::CDM, dec!(40.0), true),
        (40107, "Angelo Stiller", Position::CM, dec!(55.0), false),
        (40108, "Enzo Millot", Position::CAM, dec!(60.0), false),
        (40109, "Chris Fuhrich", Position::LW, dec!(55.0), false),
        (40110, "Jamie Leweling", Position::RW, dec!(40.0), false),
        (40111, "Ermedin Demirovic", Position::ST, dec!(55.0), false),
        // Bench
        (40112, "Fabian Bredlow", Position::GK, dec!(10.0), false),
        (40113, "Anrie Chase", Position::CB, dec!(20.0), false),
        (40114, "Ameen Al-Dakhil", Position::CB, dec!(35.0), false),
        (40115, "Pascal Stenzel", Position::RB, dec!(15.0), false),
        (40116, "Frans Kratzig", Position::LB, dec!(25.0), false),
        (40117, "Fabian Rieder", Position::CAM, dec!(40.0), false),
        (40118, "Yannik Keitel", Position::CDM, dec!(25.0), false),
        (40119, "Deniz Undav", Position::ST, dec!(65.0), false),
        (40120, "El Bilal Toure", Position::ST, dec!(45.0), false),
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
