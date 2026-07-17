use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5004);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(569.78); // Surprise 3rd place
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Stade Brestois 29".to_string());

    let players = vec![
        (50151, "Marco Bizot", Position::GK, dec!(35.00), dec!(1.0), false),
        (50152, "Kenny Lala", Position::RB, dec!(32.99), dec!(1.071), false),
        (50153, "Brendan Chardonnet", Position::CB, dec!(33.90), dec!(1.062), true),
        (50154, "Soumaila Coulibaly", Position::CB, dec!(36.49), dec!(1.048), false),
        (50155, "Jordan Amavi", Position::LB, dec!(20.00), dec!(1.0), false),
        (50156, "Pierre Lees-Melou", Position::CDM, dec!(50.42), dec!(1.082), false),
        (50157, "Mahdi Camara", Position::CM, dec!(35.00), dec!(1.0), false),
        (50158, "Hugo Magnetti", Position::CM, dec!(35.35), dec!(1.029), false),
        (50159, "Romain Del Castillo", Position::RW, dec!(64.91), dec!(1.114), false),
        (50160, "Abdallah Sima", Position::LW, dec!(40.00), dec!(1.0), false),
        (50161, "Ludovic Ajorque", Position::ST, dec!(45.89), dec!(1.043), false),
        // Bench
        (50162, "Gregoire Coudert", Position::GK, dec!(10.70), dec!(1.021), false),
        (50163, "Julien Le Cardinal", Position::CB, dec!(48.90), dec!(1.071), false),
        (50164, "Massadio Haidara", Position::LB, dec!(20.00), dec!(1.0), false),
        (50165, "Edimilson Fernandes", Position::CDM, dec!(30.00), dec!(1.0), false),
        (50166, "Kamory Doumbia", Position::CAM, dec!(80.34), dec!(1.077), false),
        (50167, "Romain Faivre", Position::CAM, dec!(50.00), dec!(1.0), false),
        (50168, "Mama Baldé", Position::ST, dec!(44.34), dec!(1.021), false),
        (50169, "Ibrahim Salah", Position::LW, dec!(25.00), dec!(1.0), false),
        (50170, "Luc Zogbe", Position::RB, dec!(15.98), dec!(1.045), false),
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
