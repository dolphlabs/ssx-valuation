use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2004;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(450.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Girona FC".to_string());

    let players = vec![
        (20151, "Paulo Gazzaniga", Position::GK, dec!(35.0), false),
        (20152, "Alejandro Frances", Position::RB, dec!(30.0), false),
        (20153, "Daley Blind", Position::CB, dec!(40.0), false),
        (20154, "David Lopez", Position::CB, dec!(25.0), false),
        (20155, "Miguel Gutierrez", Position::LB, dec!(55.0), false),
        (20156, "Oriol Romeu", Position::CDM, dec!(35.0), false),
        (20157, "Yangel Herrera", Position::CM, dec!(45.0), false),
        (20158, "Donny van de Beek", Position::CM, dec!(35.0), false),
        (20159, "Viktor Tsygankov", Position::RW, dec!(55.0), false),
        (20160, "Bryan Gil", Position::LW, dec!(45.0), false),
        (20161, "Abel Ruiz", Position::ST, dec!(40.0), false),
        // Bench
        (20162, "Pau Lopez", Position::GK, dec!(25.0), false),
        (20163, "Ladislav Krejci", Position::CB, dec!(40.0), false),
        (20164, "Arnau Martinez", Position::RB, dec!(35.0), false),
        (20165, "Jhon Solis", Position::CM, dec!(25.0), false),
        (20166, "Ivan Martin", Position::CM, dec!(45.0), false),
        (20167, "Portu", Position::RW, dec!(30.0), false),
        (20168, "Arnaut Danjuma", Position::LW, dec!(45.0), false),
        (20169, "Mojan Miovski", Position::ST, dec!(35.0), false),
        (20170, "Cristhian Stuani", Position::ST, dec!(20.0), true),
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
