use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3016);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(335.78); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Lecce".to_string());

    let players = vec![
        (30751, "Wladimiro Falcone", Position::GK, dec!(37.88), dec!(1.068), false),
        (30752, "Frederic Guilbert", Position::RB, dec!(25.00), dec!(1.0), false),
        (30753, "Kialonda Gaspar", Position::CB, dec!(24.14), dec!(0.984), false),
        (30754, "Federico Baschirotto", Position::CB, dec!(30.00), dec!(1.0), true),
        (30755, "Antonino Gallo", Position::LB, dec!(40.45), dec!(1.065), false),
        (30756, "Ylber Ramadani", Position::CDM, dec!(38.59), dec!(1.034), false),
        (30757, "Lassana Coulibaly", Position::CM, dec!(46.72), dec!(1.025), false),
        (30758, "Filip Marchwinski", Position::CAM, dec!(40.00), dec!(1.0), false),
        (30759, "Patrick Dorgu", Position::RW, dec!(50.00), dec!(1.0), false),
        (30760, "Lameck Banda", Position::LW, dec!(37.84), dec!(1.114), false),
        (30761, "Nikola Krstovic", Position::ST, dec!(50.00), dec!(1.0), false),
        // Bench
        (30762, "Christian Früchtl", Position::GK, dec!(20.00), dec!(1.0), false),
        (30763, "Gaby Jean", Position::CB, dec!(25.04), dec!(1.011), false),
        (30764, "Andy Pelmard", Position::CB, dec!(25.00), dec!(1.0), false),
        (30765, "Hamza Rafia", Position::CM, dec!(25.00), dec!(1.0), false),
        (30766, "Balthazar Pierret", Position::CDM, dec!(32.24), dec!(1.027), false),
        (30767, "Rémi Oudin", Position::CAM, dec!(30.00), dec!(1.0), false),
        (30768, "Tete Morente", Position::LW, dec!(25.19), dec!(1.020), false),
        (30769, "Santiago Pierotti", Position::RW, dec!(20.72), dec!(1.062), false),
        (30770, "Ante Rebic", Position::ST, dec!(40.00), dec!(1.0), false),
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
