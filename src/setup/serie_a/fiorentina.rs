use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3008);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(712.78); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Fiorentina".to_string());

    let players = vec![
        (30351, "David De Gea", Position::GK, dec!(44.74), dec!(1.188), false),
        (30352, "Dodô", Position::RB, dec!(39.83), dec!(1.071), false),
        (30353, "Lucas Martinez Quarta", Position::CB, dec!(40.00), dec!(1.0), false),
        (30354, "Luca Ranieri", Position::CB, dec!(35.08), dec!(0.929), false),
        (30355, "Cristiano Biraghi", Position::LB, dec!(30.00), dec!(1.0), true),
        (30356, "Danilo Cataldi", Position::CDM, dec!(35.00), dec!(1.0), false),
        (30357, "Edoardo Bove", Position::CM, dec!(40.00), dec!(1.0), false),
        (30358, "Andrea Colpani", Position::CAM, dec!(45.00), dec!(1.0), false),
        (30359, "Albert Gudmundsson", Position::LW, dec!(71.03), dec!(1.0), false),
        (30360, "Riccardo Sottil", Position::LW, dec!(30.00), dec!(1.0), false),
        (30361, "Moise Kean", Position::ST, dec!(75.80), dec!(0.997), false),
        // Bench
        (30362, "Pietro Terracciano", Position::GK, dec!(20.00), dec!(1.0), false),
        (30363, "Marin Pongracic", Position::CB, dec!(35.00), dec!(1.0), false),
        (30364, "Pietro Comuzzo", Position::CB, dec!(30.11), dec!(1.036), false),
        (30365, "Michael Kayode", Position::RB, dec!(40.00), dec!(1.0), false),
        (30366, "Yacine Adli", Position::CM, dec!(40.00), dec!(1.0), false),
        (30367, "Rolando Mandragora", Position::CM, dec!(63.18), dec!(1.094), false),
        (30368, "Jonathan Ikoné", Position::RW, dec!(35.00), dec!(1.0), false),
        (30369, "Lucas Beltrán", Position::ST, dec!(45.00), dec!(1.0), false),
        (30370, "Christian Kouame", Position::ST, dec!(30.26), dec!(1.006), false),
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
