use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5014);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(310.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "FC Nantes".to_string());

    let players = vec![
        (50651, "Alban Lafont", Position::GK, dec!(40.0), false),
        (50652, "Kelvin Amian", Position::RB, dec!(30.0), false),
        (50653, "Jean-Charles Castelletto", Position::CB, dec!(30.0), false),
        (50654, "Nathan Zeze", Position::CB, dec!(35.0), false),
        (50655, "Jean-Kevin Duverne", Position::LB, dec!(20.0), false),
        (50656, "Douglas Augusto", Position::CDM, dec!(35.0), false),
        (50657, "Pedro Chirivella", Position::CM, dec!(35.0), true),
        (50658, "Johann Lepenant", Position::CM, dec!(35.0), false),
        (50659, "Moses Simon", Position::LW, dec!(35.0), false),
        (50660, "Tino Kadewere", Position::RW, dec!(20.0), false),
        (50661, "Mostafa Mohamed", Position::ST, dec!(40.0), false),
        // Bench
        (50662, "Patrik Carlgren", Position::GK, dec!(10.0), false),
        (50663, "Nicolas Cozza", Position::LB, dec!(25.0), false),
        (50664, "Nicolas Pallois", Position::CB, dec!(10.0), false),
        (50665, "Marcus Coco", Position::RB, dec!(20.0), false),
        (50666, "Sorba Thomas", Position::RW, dec!(35.0), false),
        (50667, "Florent Mollet", Position::CAM, dec!(25.0), false),
        (50668, "Matthis Abline", Position::ST, dec!(45.0), false),
        (50669, "Ignatius Ganago", Position::ST, dec!(25.0), false),
        (50670, "Bahereba Guirassy", Position::LW, dec!(15.0), false),
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
