use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2003;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(850.00); 
    
    club.set_rival_factor(2001, dec!(1.3)); // Real Madrid
    club.set_rival_factor(2002, dec!(1.15)); // Barcelona

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Atletico Madrid".to_string());

    let players = vec![
        (20101, "Jan Oblak", Position::GK, dec!(60.0), false),
        (20102, "Nahuel Molina", Position::RB, dec!(45.0), false),
        (20103, "Robin Le Normand", Position::CB, dec!(55.0), false),
        (20104, "Jose Maria Gimenez", Position::CB, dec!(50.0), false),
        (20105, "Cesar Azpilicueta", Position::LB, dec!(25.0), false),
        (20106, "Koke", Position::CDM, dec!(55.0), true),
        (20107, "Rodrigo de Paul", Position::CM, dec!(65.0), false),
        (20108, "Conor Gallagher", Position::CM, dec!(60.0), false),
        (20109, "Marcos Llorente", Position::RM, dec!(60.0), false),
        (20110, "Antoine Griezmann", Position::ST, dec!(95.0), false),
        (20111, "Julian Alvarez", Position::ST, dec!(120.0), false),
        // Bench
        (20112, "Juan Musso", Position::GK, dec!(25.0), false),
        (20113, "Axel Witsel", Position::CB, dec!(30.0), false),
        (20114, "Reinildo Mandava", Position::LB, dec!(35.0), false),
        (20115, "Samuel Lino", Position::LM, dec!(60.0), false),
        (20116, "Pablo Barrios", Position::CM, dec!(40.0), false),
        (20117, "Thomas Lemar", Position::LM, dec!(30.0), false),
        (20118, "Rodrigo Riquelme", Position::LM, dec!(45.0), false),
        (20119, "Angel Correa", Position::ST, dec!(55.0), false),
        (20120, "Alexander Sorloth", Position::ST, dec!(65.0), false),
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
