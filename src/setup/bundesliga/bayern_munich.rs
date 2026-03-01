use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4002);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1150.00); 
    
    club.set_rival_factor(ClubId(4005), dec!(1.5)); // Dortmund (Der Klassiker)
    club.set_rival_factor(ClubId(4001), dec!(1.3)); // Leverkusen

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Bayern Munich".to_string());

    let players = vec![
        (40051, "Manuel Neuer", Position::GK, dec!(60.0), true),
        (40052, "Sacha Boey", Position::RB, dec!(45.0), false),
        (40053, "Dayot Upamecano", Position::CB, dec!(65.0), false),
        (40054, "Kim Min-jae", Position::CB, dec!(70.0), false),
        (40055, "Alphonso Davies", Position::LB, dec!(80.0), false),
        (40056, "Joshua Kimmich", Position::CDM, dec!(95.0), false),
        (40057, "Joao Palhinha", Position::CDM, dec!(80.0), false),
        (40058, "Jamal Musiala", Position::CAM, dec!(120.0), false),
        (40059, "Michael Olise", Position::RW, dec!(75.0), false),
        (40060, "Leroy Sane", Position::LW, dec!(85.0), false),
        (40061, "Harry Kane", Position::ST, dec!(150.0), false),
        // Bench
        (40062, "Sven Ulreich", Position::GK, dec!(15.0), false),
        (40063, "Eric Dier", Position::CB, dec!(30.0), false),
        (40064, "Raphael Guerreiro", Position::LB, dec!(40.0), false),
        (40065, "Konrad Laimer", Position::CM, dec!(45.0), false),
        (40066, "Aleksandar Pavlovic", Position::CM, dec!(45.0), false),
        (40067, "Thomas Muller", Position::CAM, dec!(50.0), false),
        (40068, "Serge Gnabry", Position::LW, dec!(55.0), false),
        (40069, "Kingsley Coman", Position::RW, dec!(65.0), false),
        (40070, "Mathys Tel", Position::ST, dec!(50.0), false),
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
