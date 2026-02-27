use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3008;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(550.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Fiorentina".to_string());

    let players = vec![
        (30351, "David De Gea", Position::GK, dec!(40.0), false),
        (30352, "Dodô", Position::RB, dec!(35.0), false),
        (30353, "Lucas Martinez Quarta", Position::CB, dec!(40.0), false),
        (30354, "Luca Ranieri", Position::CB, dec!(30.0), false),
        (30355, "Cristiano Biraghi", Position::LB, dec!(30.0), true),
        (30356, "Danilo Cataldi", Position::CDM, dec!(35.0), false),
        (30357, "Edoardo Bove", Position::CM, dec!(40.0), false),
        (30358, "Andrea Colpani", Position::CAM, dec!(45.0), false),
        (30359, "Albert Gudmundsson", Position::LW, dec!(60.0), false),
        (30360, "Riccardo Sottil", Position::LW, dec!(30.0), false),
        (30361, "Moise Kean", Position::ST, dec!(55.0), false),
        // Bench
        (30362, "Pietro Terracciano", Position::GK, dec!(20.0), false),
        (30363, "Marin Pongracic", Position::CB, dec!(35.0), false),
        (30364, "Pietro Comuzzo", Position::CB, dec!(25.0), false),
        (30365, "Michael Kayode", Position::RB, dec!(40.0), false),
        (30366, "Yacine Adli", Position::CM, dec!(40.0), false),
        (30367, "Rolando Mandragora", Position::CM, dec!(30.0), false),
        (30368, "Jonathan Ikoné", Position::RW, dec!(35.0), false),
        (30369, "Lucas Beltrán", Position::ST, dec!(45.0), false),
        (30370, "Christian Kouame", Position::ST, dec!(30.0), false),
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
