use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3014);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(607.83); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Udinese".to_string());

    let players = vec![
        (30651, "Maduka Okoye", Position::GK, dec!(27.07), dec!(1.098), false),
        (30652, "Christian Kabasele", Position::CB, dec!(32.18), dec!(0.666), false),
        (30653, "Jaka Bijol", Position::CB, dec!(45.00), dec!(1.0), false),
        (30654, "Thomas Kristensen", Position::CB, dec!(55.80), dec!(1.055), false),
        (30655, "Kingsley Ehizibue", Position::RM, dec!(29.44), dec!(1.011), false),
        (30656, "Sandi Lovric", Position::CM, dec!(40.00), dec!(1.0), false),
        (30657, "Jesper Karlstrom", Position::CDM, dec!(36.04), dec!(0.963), false),
        (30658, "Hassane Kamara", Position::LM, dec!(26.53), dec!(1.091), false),
        (30659, "Florian Thauvin", Position::CAM, dec!(50.00), dec!(1.0), true),
        (30660, "Brenner", Position::ST, dec!(35.00), dec!(1.0), false),
        (30661, "Lorenzo Lucca", Position::ST, dec!(50.00), dec!(1.0), false),
        // Bench
        (30662, "Razvan Sava", Position::GK, dec!(13.63), dec!(0.952), false),
        (30663, "Lautaro Giannetti", Position::CB, dec!(25.00), dec!(1.0), false),
        (30664, "Isaak Touré", Position::CB, dec!(35.00), dec!(1.0), false),
        (30665, "Enzo Ebosse", Position::LB, dec!(25.00), dec!(1.0), false),
        (30666, "Oier Zarraga", Position::CM, dec!(25.34), dec!(1.009), false),
        (30667, "Jurgen Ekkelenkamp", Position::CAM, dec!(64.27), dec!(1.085), false),
        (30668, "Iker Bravo", Position::ST, dec!(31.64), dec!(1.005), false),
        (30669, "Alexis Sanchez", Position::ST, dec!(40.00), dec!(1.0), false),
        (30670, "Rui Modesto", Position::RM, dec!(22.22), dec!(1.047), false),
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
