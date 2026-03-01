use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2013);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(360.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "RC Celta".to_string());

    let players = vec![
        (20601, "Vicente Guaita", Position::GK, dec!(25.0), false),
        (20602, "Oscar Mingueza", Position::RB, dec!(45.0), false),
        (20603, "Carl Starfelt", Position::CB, dec!(35.0), false),
        (20604, "Jalil Jailer", Position::CB, dec!(30.0), false), // Jailson? let's use Carl Starfelt + Marcos Alonso
        (20605, "Marcos Alonso", Position::LB, dec!(35.0), false),
        (20606, "Fran Beltran", Position::CDM, dec!(40.0), false),
        (20607, "Hugo Sotelo", Position::CM, dec!(35.0), false),
        (20608, "Iago Aspas", Position::CAM, dec!(60.0), true),
        (20609, "Jonathan Bamba", Position::LW, dec!(45.0), false),
        (20610, "Williot Swedberg", Position::LW, dec!(40.0), false),
        (20611, "Borja Iglesias", Position::ST, dec!(50.0), false),
        // Bench
        (20612, "Ivan Villar", Position::GK, dec!(15.0), false),
        (20613, "Carlos Dominguez", Position::CB, dec!(30.0), false),
        (20614, "Javi Rodriguez", Position::RB, dec!(25.0), false),
        (20615, "Ilaix Moriba", Position::CM, dec!(35.0), false),
        (20616, "Luca de la Torre", Position::CM, dec!(30.0), false),
        (20617, "Alfon Gonzalez", Position::RW, dec!(20.0), false),
        (20618, "Tadeo Allende", Position::RW, dec!(35.0), false),
        (20619, "Tasos Douvikas", Position::ST, dec!(40.0), false),
        (20620, "Franco Cervi", Position::LW, dec!(25.0), false),
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
