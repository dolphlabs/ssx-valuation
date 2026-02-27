use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3014;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(340.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Udinese".to_string());

    let players = vec![
        (30651, "Maduka Okoye", Position::GK, dec!(30.0), false),
        (30652, "Christian Kabasele", Position::CB, dec!(25.0), false),
        (30653, "Jaka Bijol", Position::CB, dec!(45.0), false),
        (30654, "Thomas Kristensen", Position::CB, dec!(30.0), false),
        (30655, "Kingsley Ehizibue", Position::RM, dec!(25.0), false),
        (30656, "Sandi Lovric", Position::CM, dec!(40.0), false),
        (30657, "Jesper Karlstrom", Position::CDM, dec!(30.0), false),
        (30658, "Hassane Kamara", Position::LM, dec!(25.0), false),
        (30659, "Florian Thauvin", Position::CAM, dec!(50.0), true),
        (30660, "Brenner", Position::ST, dec!(35.0), false),
        (30661, "Lorenzo Lucca", Position::ST, dec!(50.0), false),
        // Bench
        (30662, "Razvan Sava", Position::GK, dec!(15.0), false),
        (30663, "Lautaro Giannetti", Position::CB, dec!(25.0), false),
        (30664, "Isaak Touré", Position::CB, dec!(35.0), false),
        (30665, "Enzo Ebosse", Position::LB, dec!(25.0), false),
        (30666, "Oier Zarraga", Position::CM, dec!(25.0), false),
        (30667, "Jurgen Ekkelenkamp", Position::CAM, dec!(40.0), false),
        (30668, "Iker Bravo", Position::ST, dec!(35.0), false),
        (30669, "Alexis Sanchez", Position::ST, dec!(40.0), false),
        (30670, "Rui Modesto", Position::RM, dec!(20.0), false),
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
