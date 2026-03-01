use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 5009;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(360.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Stade de Reims".to_string());

    let players = vec![
        (50401, "Yehvann Diouf", Position::GK, dec!(30.0), false),
        (50402, "Buta", Position::RB, dec!(30.0), false),
        (50403, "Emmanuel Agbadou", Position::CB, dec!(30.0), false),
        (50404, "Cedric Kipre", Position::CB, dec!(25.0), false),
        (50405, "Sergio Akieme", Position::LB, dec!(30.0), false),
        (50406, "Marshall Munetsi", Position::CDM, dec!(30.0), false),
        (50407, "Valentin Atangana", Position::CM, dec!(25.0), false),
        (50408, "Teddy Teuma", Position::CAM, dec!(35.0), true),
        (50409, "Junya Ito", Position::RW, dec!(45.0), false),
        (50410, "Keito Nakamura", Position::LW, dec!(40.0), false),
        (50411, "Oumar Diakite", Position::ST, dec!(40.0), false),
        // Bench
        (50412, "Alexandre Olliero", Position::GK, dec!(10.0), false),
        (50413, "Joseph Okumu", Position::CB, dec!(35.0), false),
        (50414, "Nuno Sangui", Position::LB, dec!(15.0), false),
        (50415, "Amadou Koné", Position::CM, dec!(20.0), false),
        (50416, "Yaya Fofana", Position::CM, dec!(25.0), false),
        (50417, "Reda Khadra", Position::CAM, dec!(35.0), false),
        (50418, "Cédric Kipré", Position::CB, dec!(25.0), false),
        (50419, "Mohamed Daramy", Position::LW, dec!(45.0), false),
        (50420, "Amine Salama", Position::ST, dec!(25.0), false),
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
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(id);
        }
    }
}
