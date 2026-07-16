use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3005);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1189.72); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Napoli".to_string());

    let players = vec![
        (30201, "Alex Meret", Position::GK, dec!(40.62), dec!(0.996), false),
        (30202, "Giovanni Di Lorenzo", Position::RB, dec!(70.75), dec!(1.079), true),
        (30203, "Amir Rrahmani", Position::CB, dec!(54.33), dec!(1.065), false),
        (30204, "Alessandro Buongiorno", Position::CB, dec!(68.10), dec!(1.050), false),
        (30205, "Mathias Olivera", Position::LB, dec!(42.74), dec!(1.081), false),
        (30206, "Stanislav Lobotka", Position::CDM, dec!(66.93), dec!(1.032), false),
        (30207, "Frank Anguissa", Position::CM, dec!(90.83), dec!(1.030), false),
        (30208, "Scott McTominay", Position::CM, dec!(95.94), dec!(1.073), false),
        (30209, "Matteo Politano", Position::RW, dec!(60.88), dec!(0.957), false),
        (30210, "Khvicha Kvaratskhelia", Position::LW, dec!(100.00), dec!(1.0), false),
        (30211, "Romelu Lukaku", Position::ST, dec!(81.76), dec!(0.998), false),
        // Bench
        (30212, "Elia Caprile", Position::GK, dec!(25.00), dec!(1.0), false),
        (30213, "Rafa Marin", Position::CB, dec!(35.00), dec!(1.0), false),
        (30214, "Juan Jesus", Position::CB, dec!(11.53), dec!(1.053), false),
        (30215, "Pasquale Mazzocchi", Position::RB, dec!(20.91), dec!(1.019), false),
        (30216, "Billy Gilmour", Position::CM, dec!(43.80), dec!(1.064), false),
        (30217, "Cyril Ngonge", Position::RW, dec!(45.00), dec!(1.0), false),
        (30218, "David Neres", Position::RW, dec!(64.25), dec!(0.995), false),
        (30219, "Giacomo Raspadori", Position::ST, dec!(55.00), dec!(1.0), false),
        (30220, "Giovanni Simeone", Position::ST, dec!(30.00), dec!(1.0), false),
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
