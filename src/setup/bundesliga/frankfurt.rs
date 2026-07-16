use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4006);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(978.56); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Eintracht Frankfurt".to_string());

    let players = vec![
        (40251, "Kevin Trapp", Position::GK, dec!(35.00), dec!(1.0), true),
        (40252, "Rasmus Kristensen", Position::RB, dec!(48.69), dec!(1.021), false),
        (40253, "Tuta", Position::CB, dec!(35.00), dec!(1.0), false),
        (40254, "Robin Koch", Position::CB, dec!(67.73), dec!(1.065), false),
        (40255, "Arthur Theate", Position::LB, dec!(42.31), dec!(1.060), false),
        (40256, "Hugo Larsson", Position::CM, dec!(62.62), dec!(1.081), false),
        (40257, "Ellyes Skhiri", Position::CDM, dec!(42.80), dec!(1.103), false),
        (40258, "Mario Gotze", Position::CAM, dec!(37.61), dec!(1.022), false),
        (40259, "Fares Chaibi", Position::LW, dec!(54.86), dec!(1.034), false),
        (40260, "Hugo Ekitike", Position::ST, dec!(60.00), dec!(1.0), false),
        (40261, "Omar Marmoush", Position::ST, dec!(65.00), dec!(1.0), false),
        // Bench
        (40262, "Kaua Santos", Position::GK, dec!(14.80), dec!(1.125), false),
        (40263, "Aurele Amenda", Position::CB, dec!(25.26), dec!(1.046), false),
        (40264, "Niels Nkounkou", Position::LB, dec!(35.00), dec!(1.0), false),
        (40265, "Mo Dahoud", Position::CM, dec!(35.08), dec!(1.028), false),
        (40266, "Eric Junior Dina Ebimbe", Position::RM, dec!(35.00), dec!(1.0), false),
        (40267, "Ansgar Knauff", Position::RW, dec!(67.92), dec!(1.013), false),
        (40268, "Can Uzun", Position::CAM, dec!(93.35), dec!(1.110), false),
        (40269, "Igor Matanovic", Position::ST, dec!(30.00), dec!(1.0), false),
        (40270, "Timothy Chandler", Position::RB, dec!(10.02), dec!(1.043), false),
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
