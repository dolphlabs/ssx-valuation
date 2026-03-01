use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5004);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(400.00); // Surprise 3rd place
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Stade Brestois 29".to_string());

    let players = vec![
        (50151, "Marco Bizot", Position::GK, dec!(35.0), false),
        (50152, "Kenny Lala", Position::RB, dec!(30.0), false),
        (50153, "Brendan Chardonnet", Position::CB, dec!(30.0), true),
        (50154, "Soumaila Coulibaly", Position::CB, dec!(35.0), false),
        (50155, "Jordan Amavi", Position::LB, dec!(20.0), false),
        (50156, "Pierre Lees-Melou", Position::CDM, dec!(40.0), false),
        (50157, "Mahdi Camara", Position::CM, dec!(35.0), false),
        (50158, "Hugo Magnetti", Position::CM, dec!(30.0), false),
        (50159, "Romain Del Castillo", Position::RW, dec!(45.0), false),
        (50160, "Abdallah Sima", Position::LW, dec!(40.0), false),
        (50161, "Ludovic Ajorque", Position::ST, dec!(35.0), false),
        // Bench
        (50162, "Gregoire Coudert", Position::GK, dec!(10.0), false),
        (50163, "Julien Le Cardinal", Position::CB, dec!(25.0), false),
        (50164, "Massadio Haidara", Position::LB, dec!(20.0), false),
        (50165, "Edimilson Fernandes", Position::CDM, dec!(30.0), false),
        (50166, "Kamory Doumbia", Position::CAM, dec!(40.0), false),
        (50167, "Romain Faivre", Position::CAM, dec!(50.0), false),
        (50168, "Mama Baldé", Position::ST, dec!(30.0), false),
        (50169, "Ibrahim Salah", Position::LW, dec!(25.0), false),
        (50170, "Luc Zogbe", Position::RB, dec!(15.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.player_states.insert(PlayerId(id), PlayerValues {
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
            c.player_ids.push(PlayerId(id));
        }
    }
}
