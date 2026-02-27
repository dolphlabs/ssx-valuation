use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3005;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(880.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Napoli".to_string());

    let players = vec![
        (30201, "Alex Meret", Position::GK, dec!(45.0), false),
        (30202, "Giovanni Di Lorenzo", Position::RB, dec!(55.0), true),
        (30203, "Amir Rrahmani", Position::CB, dec!(45.0), false),
        (30204, "Alessandro Buongiorno", Position::CB, dec!(60.0), false),
        (30205, "Mathias Olivera", Position::LB, dec!(40.0), false),
        (30206, "Stanislav Lobotka", Position::CDM, dec!(60.0), false),
        (30207, "Frank Anguissa", Position::CM, dec!(55.0), false),
        (30208, "Scott McTominay", Position::CM, dec!(60.0), false),
        (30209, "Matteo Politano", Position::RW, dec!(50.0), false),
        (30210, "Khvicha Kvaratskhelia", Position::LW, dec!(100.0), false),
        (30211, "Romelu Lukaku", Position::ST, dec!(80.0), false),
        // Bench
        (30212, "Elia Caprile", Position::GK, dec!(25.0), false),
        (30213, "Rafa Marin", Position::CB, dec!(35.0), false),
        (30214, "Juan Jesus", Position::CB, dec!(15.0), false),
        (30215, "Pasquale Mazzocchi", Position::RB, dec!(25.0), false),
        (30216, "Billy Gilmour", Position::CM, dec!(40.0), false),
        (30217, "Cyril Ngonge", Position::RW, dec!(45.0), false),
        (30218, "David Neres", Position::RW, dec!(55.0), false),
        (30219, "Giacomo Raspadori", Position::ST, dec!(55.0), false),
        (30220, "Giovanni Simeone", Position::ST, dec!(30.0), false),
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
