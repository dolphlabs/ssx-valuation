use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3019;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(350.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Como".to_string());

    let players = vec![
        (30901, "Pepe Reina", Position::GK, dec!(15.0), false),
        (30902, "Ignace Van der Brempt", Position::RB, dec!(30.0), false),
        (30903, "Alberto Dossena", Position::CB, dec!(35.0), false),
        (30904, "Marc-Oliver Kempf", Position::CB, dec!(30.0), false),
        (30905, "Alberto Moreno", Position::LB, dec!(25.0), false),
        (30906, "Maximo Perrone", Position::CDM, dec!(45.0), false),
        (30907, "Sergi Roberto", Position::CM, dec!(40.0), false),
        (30908, "Gabriel Strefezza", Position::RW, dec!(40.0), false),
        (30909, "Nico Paz", Position::CAM, dec!(55.0), false),
        (30910, "Alieu Fadera", Position::LW, dec!(35.0), false),
        (30911, "Patrick Cutrone", Position::ST, dec!(50.0), true),
        // Bench
        (30912, "Emil Audero", Position::GK, dec!(35.0), false),
        (30913, "Edoardo Goldaniga", Position::CB, dec!(20.0), false),
        (30914, "Filippo Terracciano", Position::RB, dec!(30.0), false),
        (30915, "Yannick Engelhardt", Position::CDM, dec!(35.0), false),
        (30916, "Luca Mazzitelli", Position::CM, dec!(30.0), false),
        (30917, "Lucas Da Cunha", Position::CAM, dec!(35.0), false),
        (30918, "Andrea Belotti", Position::ST, dec!(45.0), false),
        (30919, "Simone Verdi", Position::CAM, dec!(20.0), false),
        (30920, "Matthias Braunoder", Position::CM, dec!(25.0), false),
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
