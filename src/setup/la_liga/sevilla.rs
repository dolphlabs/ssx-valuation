use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2014);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(400.00); 
    
    club.set_rival_factor(ClubId(2007), dec!(1.5)); // Real Betis (Seville Derby)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Sevilla FC".to_string());

    let players = vec![
        (20651, "Orjan Nyland", Position::GK, dec!(30.0), false),
        (20652, "Jose Angel Carmona", Position::RB, dec!(35.0), false),
        (20653, "Loic Bade", Position::CB, dec!(50.0), false),
        (20654, "Marcao", Position::CB, dec!(35.0), false),
        (20655, "Valentin Barco", Position::LB, dec!(45.0), false),
        (20656, "Nemanja Gudelj", Position::CDM, dec!(35.0), true),
        (20657, "Albert Sambi Lokonga", Position::CM, dec!(45.0), false),
        (20658, "Saul Niguez", Position::CM, dec!(40.0), false),
        (20659, "Dodi Lukebakio", Position::RW, dec!(50.0), false),
        (20660, "Chidera Ejuke", Position::LW, dec!(40.0), false),
        (20661, "Isaac Romero", Position::ST, dec!(45.0), false),
        // Bench
        (20662, "Alvaro Fernandez", Position::GK, dec!(20.0), false),
        (20663, "Tanguy Nianzou", Position::CB, dec!(35.0), false),
        (20664, "Gonzalo Montiel", Position::RB, dec!(30.0), false),
        (20665, "Adria Pedrosa", Position::LB, dec!(30.0), false),
        (20666, "Lucien Agoume", Position::CDM, dec!(40.0), false),
        (20667, "Djibril Sow", Position::CM, dec!(40.0), false),
        (20668, "Sus", Position::CAM, dec!(30.0), false), // Suso
        (20669, "Peque", Position::CAM, dec!(35.0), false),
        (20670, "Kelechi Iheanacho", Position::ST, dec!(45.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.player_states.insert(PlayerId(id), PlayerValues {
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
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(PlayerId(id));
        }
    }
}
