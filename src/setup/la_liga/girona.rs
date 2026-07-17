use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2004);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(619.39); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Girona FC".to_string());

    let players = vec![
        (20151, "Paulo Gazzaniga", Position::GK, dec!(21.43), dec!(1.043), false),
        (20152, "Alejandro Frances", Position::RB, dec!(24.00), dec!(1.096), false),
        (20153, "Daley Blind", Position::CB, dec!(48.06), dec!(1.068), false),
        (20154, "David Lopez", Position::CB, dec!(27.86), dec!(1.024), false),
        (20155, "Miguel Gutierrez", Position::LB, dec!(55.00), dec!(1.0), false),
        (20156, "Oriol Romeu", Position::CDM, dec!(35.00), dec!(1.0), false),
        (20157, "Yangel Herrera", Position::CM, dec!(48.26), dec!(1.024), false),
        (20158, "Donny van de Beek", Position::CM, dec!(36.49), dec!(1.017), false),
        (20159, "Viktor Tsygankov", Position::RW, dec!(81.75), dec!(1.054), false),
        (20160, "Bryan Gil", Position::LW, dec!(46.65), dec!(0.998), false),
        (20161, "Abel Ruiz", Position::ST, dec!(41.07), dec!(1.021), false),
        // Bench
        (20162, "Pau Lopez", Position::GK, dec!(25.00), dec!(1.0), false),
        (20163, "Ladislav Krejci", Position::CB, dec!(40.00), dec!(1.0), false),
        (20164, "Arnau Martinez", Position::RB, dec!(40.46), dec!(1.070), false),
        (20165, "Jhon Solis", Position::CM, dec!(26.85), dec!(1.028), false),
        (20166, "Ivan Martin", Position::CM, dec!(39.78), dec!(1.082), false),
        (20167, "Portu", Position::RW, dec!(28.78), dec!(0.981), false),
        (20168, "Arnaut Danjuma", Position::LW, dec!(45.00), dec!(1.0), false),
        (20169, "Mojan Miovski", Position::ST, dec!(35.60), dec!(1.006), false),
        (20170, "Cristhian Stuani", Position::ST, dec!(25.54), dec!(1.012), true),
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
