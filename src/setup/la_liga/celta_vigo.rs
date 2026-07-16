use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2013);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(700.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "RC Celta".to_string());

    let players = vec![
        (20601, "Vicente Guaita", Position::GK, dec!(25.00), dec!(1.0), false),
        (20602, "Oscar Mingueza", Position::RB, dec!(48.59), dec!(1.035), false),
        (20603, "Carl Starfelt", Position::CB, dec!(31.61), dec!(1.073), false),
        (20604, "Jalil Jailer", Position::CB, dec!(30.00), dec!(1.0), false), // Jailson? let's use Carl Starfelt + Marcos Alonso
        (20605, "Marcos Alonso", Position::LB, dec!(39.99), dec!(1.093), false),
        (20606, "Fran Beltran", Position::CDM, dec!(43.70), dec!(1.034), false),
        (20607, "Hugo Sotelo", Position::CM, dec!(36.49), dec!(1.021), false),
        (20608, "Iago Aspas", Position::CAM, dec!(70.97), dec!(1.032), true),
        (20609, "Jonathan Bamba", Position::LW, dec!(45.00), dec!(1.0), false),
        (20610, "Williot Swedberg", Position::LW, dec!(52.47), dec!(1.026), false),
        (20611, "Borja Iglesias", Position::ST, dec!(78.79), dec!(1.005), false),
        // Bench
        (20612, "Ivan Villar", Position::GK, dec!(15.00), dec!(1.0), false),
        (20613, "Carlos Dominguez", Position::CB, dec!(30.54), dec!(1.034), false),
        (20614, "Javi Rodriguez", Position::RB, dec!(23.36), dec!(1.086), false),
        (20615, "Ilaix Moriba", Position::CM, dec!(38.04), dec!(1.128), false),
        (20616, "Luca de la Torre", Position::CM, dec!(30.00), dec!(1.0), false),
        (20617, "Alfon Gonzalez", Position::RW, dec!(20.02), dec!(1.005), false),
        (20618, "Tadeo Allende", Position::RW, dec!(35.00), dec!(1.0), false),
        (20619, "Tasos Douvikas", Position::ST, dec!(40.00), dec!(1.0), false),
        (20620, "Franco Cervi", Position::LW, dec!(25.00), dec!(1.0), false),
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
