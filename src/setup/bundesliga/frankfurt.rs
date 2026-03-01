use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4006);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(580.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Eintracht Frankfurt".to_string());

    let players = vec![
        (40251, "Kevin Trapp", Position::GK, dec!(35.0), true),
        (40252, "Rasmus Kristensen", Position::RB, dec!(35.0), false),
        (40253, "Tuta", Position::CB, dec!(35.0), false),
        (40254, "Robin Koch", Position::CB, dec!(45.0), false),
        (40255, "Arthur Theate", Position::LB, dec!(40.0), false),
        (40256, "Hugo Larsson", Position::CM, dec!(50.0), false),
        (40257, "Ellyes Skhiri", Position::CDM, dec!(40.0), false),
        (40258, "Mario Gotze", Position::CAM, dec!(35.0), false),
        (40259, "Fares Chaibi", Position::LW, dec!(40.0), false),
        (40260, "Hugo Ekitike", Position::ST, dec!(60.0), false),
        (40261, "Omar Marmoush", Position::ST, dec!(65.0), false),
        // Bench
        (40262, "Kaua Santos", Position::GK, dec!(15.0), false),
        (40263, "Aurele Amenda", Position::CB, dec!(25.0), false),
        (40264, "Niels Nkounkou", Position::LB, dec!(35.0), false),
        (40265, "Mo Dahoud", Position::CM, dec!(30.0), false),
        (40266, "Eric Junior Dina Ebimbe", Position::RM, dec!(35.0), false),
        (40267, "Ansgar Knauff", Position::RW, dec!(35.0), false),
        (40268, "Can Uzun", Position::CAM, dec!(35.0), false),
        (40269, "Igor Matanovic", Position::ST, dec!(30.0), false),
        (40270, "Timothy Chandler", Position::RB, dec!(10.0), false),
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
