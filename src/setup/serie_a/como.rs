use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3019);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(674.61); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Como".to_string());

    let players = vec![
        (30901, "Pepe Reina", Position::GK, dec!(15.00), dec!(1.0), false),
        (30902, "Ignace Van der Brempt", Position::RB, dec!(31.88), dec!(1.085), false),
        (30903, "Alberto Dossena", Position::CB, dec!(35.00), dec!(1.0), false),
        (30904, "Marc-Oliver Kempf", Position::CB, dec!(47.36), dec!(1.080), false),
        (30905, "Alberto Moreno", Position::LB, dec!(27.71), dec!(1.089), false),
        (30906, "Maximo Perrone", Position::CDM, dec!(51.81), dec!(1.061), false),
        (30907, "Sergi Roberto", Position::CM, dec!(35.13), dec!(1.036), false),
        (30908, "Gabriel Strefezza", Position::RW, dec!(40.00), dec!(1.0), false),
        (30909, "Nico Paz", Position::CAM, dec!(108.62), dec!(1.094), false),
        (30910, "Alieu Fadera", Position::LW, dec!(35.00), dec!(1.0), false),
        (30911, "Patrick Cutrone", Position::ST, dec!(50.00), dec!(1.0), true),
        // Bench
        (30912, "Emil Audero", Position::GK, dec!(35.00), dec!(1.0), false),
        (30913, "Edoardo Goldaniga", Position::CB, dec!(20.00), dec!(1.0), false),
        (30914, "Filippo Terracciano", Position::RB, dec!(30.00), dec!(1.0), false),
        (30915, "Yannick Engelhardt", Position::CDM, dec!(35.00), dec!(1.0), false),
        (30916, "Luca Mazzitelli", Position::CM, dec!(30.00), dec!(1.0), false),
        (30917, "Lucas Da Cunha", Position::CAM, dec!(54.82), dec!(1.094), false),
        (30918, "Andrea Belotti", Position::ST, dec!(45.00), dec!(1.0), false),
        (30919, "Simone Verdi", Position::CAM, dec!(20.00), dec!(1.0), false),
        (30920, "Matthias Braunoder", Position::CM, dec!(25.00), dec!(1.0), false),
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
