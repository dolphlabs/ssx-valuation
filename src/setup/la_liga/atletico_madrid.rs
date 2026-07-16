use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2003);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1214.48); 
    
    club.set_rival_factor(ClubId(2001), dec!(1.3)); // Real Madrid
    club.set_rival_factor(ClubId(2002), dec!(1.15)); // Barcelona

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Atletico Madrid".to_string());

    let players = vec![
        (20101, "Jan Oblak", Position::GK, dec!(64.22), dec!(1.195), false),
        (20102, "Nahuel Molina", Position::RB, dec!(55.29), dec!(1.042), false),
        (20103, "Robin Le Normand", Position::CB, dec!(60.66), dec!(1.086), false),
        (20104, "Jose Maria Gimenez", Position::CB, dec!(52.42), dec!(1.068), false),
        (20105, "Cesar Azpilicueta", Position::LB, dec!(25.00), dec!(1.0), false),
        (20106, "Koke", Position::CDM, dec!(63.08), dec!(1.060), true),
        (20107, "Rodrigo de Paul", Position::CM, dec!(65.00), dec!(1.0), false),
        (20108, "Conor Gallagher", Position::CM, dec!(68.90), dec!(1.026), false),
        (20109, "Marcos Llorente", Position::RM, dec!(57.13), dec!(0.737), false),
        (20110, "Antoine Griezmann", Position::ST, dec!(122.12), dec!(1.092), false),
        (20111, "Julian Alvarez", Position::ST, dec!(179.10), dec!(1.060), false),
        // Bench
        (20112, "Juan Musso", Position::GK, dec!(22.84), dec!(0.962), false),
        (20113, "Axel Witsel", Position::CB, dec!(30.00), dec!(1.0), false),
        (20114, "Reinildo Mandava", Position::LB, dec!(35.00), dec!(1.0), false),
        (20115, "Samuel Lino", Position::LM, dec!(60.00), dec!(1.0), false),
        (20116, "Pablo Barrios", Position::CM, dec!(58.12), dec!(1.090), false),
        (20117, "Thomas Lemar", Position::LM, dec!(30.00), dec!(1.0), false),
        (20118, "Rodrigo Riquelme", Position::LM, dec!(45.00), dec!(1.0), false),
        (20119, "Angel Correa", Position::ST, dec!(55.00), dec!(1.0), false),
        (20120, "Alexander Sorloth", Position::ST, dec!(92.09), dec!(1.068), false),
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
