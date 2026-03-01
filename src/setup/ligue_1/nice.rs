use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5005);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(520.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "OGC Nice".to_string());

    let players = vec![
        (50201, "Marcin Bulka", Position::GK, dec!(55.0), false),
        (50202, "Jonathan Clauss", Position::RB, dec!(45.0), false),
        (50203, "Jean-Clair Todibo", Position::CB, dec!(60.0), false), // Is at West Ham, let's use Moise Bombito
        (50204, "Moise Bombito", Position::CB, dec!(35.0), false),
        (50205, "Dante", Position::CB, dec!(20.0), true),
        (50206, "Melvin Bard", Position::LB, dec!(35.0), false),
        (50207, "Pablo Rosario", Position::CDM, dec!(35.0), false),
        (50208, "Tanguy Ndombele", Position::CM, dec!(35.0), false),
        (50209, "Youssoufa Moukoko", Position::ST, dec!(60.0), false),
        (50210, "Jeremie Boga", Position::LW, dec!(45.0), false),
        (50211, "Evann Guessand", Position::ST, dec!(35.0), false),
        // Bench
        (50212, "Maxime Dupé", Position::GK, dec!(15.0), false),
        (50213, "Mohamed Abdelmonem", Position::CB, dec!(35.0), false),
        (50214, "Ali Abdi", Position::LB, dec!(25.0), false),
        (50215, "Hicham Boudaoui", Position::CM, dec!(40.0), false),
        (50216, "Morgan Sanson", Position::CM, dec!(35.0), false),
        (50217, "Sofiane Diop", Position::CAM, dec!(35.0), false),
        (50218, "Gaetan Laborde", Position::ST, dec!(40.0), false),
        (50219, "Momo Cho", Position::RW, dec!(35.0), false),
        (50220, "Badredine Bouanani", Position::RW, dec!(20.0), false),
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
