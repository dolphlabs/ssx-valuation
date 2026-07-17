use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2014);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(566.78); 
    
    club.set_rival_factor(ClubId(2007), dec!(1.5)); // Real Betis (Seville Derby)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Sevilla FC".to_string());

    let players = vec![
        (20651, "Orjan Nyland", Position::GK, dec!(27.25), dec!(1.008), false),
        (20652, "Jose Angel Carmona", Position::RB, dec!(34.56), dec!(1.085), false),
        (20653, "Loic Bade", Position::CB, dec!(50.00), dec!(1.0), false),
        (20654, "Marcao", Position::CB, dec!(23.13), dec!(0.736), false),
        (20655, "Valentin Barco", Position::LB, dec!(45.00), dec!(1.0), false),
        (20656, "Nemanja Gudelj", Position::CDM, dec!(37.30), dec!(1.024), true),
        (20657, "Albert Sambi Lokonga", Position::CM, dec!(45.00), dec!(1.0), false),
        (20658, "Saul Niguez", Position::CM, dec!(40.00), dec!(1.0), false),
        (20659, "Dodi Lukebakio", Position::RW, dec!(73.53), dec!(1.043), false),
        (20660, "Chidera Ejuke", Position::LW, dec!(46.75), dec!(1.042), false),
        (20661, "Isaac Romero", Position::ST, dec!(47.35), dec!(1.017), false),
        // Bench
        (20662, "Alvaro Fernandez", Position::GK, dec!(16.64), dec!(1.025), false),
        (20663, "Tanguy Nianzou", Position::CB, dec!(32.08), dec!(0.991), false),
        (20664, "Gonzalo Montiel", Position::RB, dec!(30.00), dec!(1.0), false),
        (20665, "Adria Pedrosa", Position::LB, dec!(31.28), dec!(1.015), false),
        (20666, "Lucien Agoume", Position::CDM, dec!(49.91), dec!(1.034), false),
        (20667, "Djibril Sow", Position::CM, dec!(54.17), dec!(1.054), false),
        (20668, "Sus", Position::CAM, dec!(30.00), dec!(1.0), false), // Suso
        (20669, "Peque", Position::CAM, dec!(37.61), dec!(1.0), false),
        (20670, "Kelechi Iheanacho", Position::ST, dec!(45.00), dec!(1.0), false),
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
