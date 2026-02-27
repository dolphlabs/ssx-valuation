use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 1;
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id, "Manchester City".to_string());
    
    club.intrinsic_value = dec!(1200.00);
    
    // Rivals: Man Utd (2), Liverpool (5), Arsenal (4)
    club.set_rival_factor(2, dec!(1.5));
    club.set_rival_factor(5, dec!(1.3));
    club.set_rival_factor(4, dec!(1.4));
    
    // Bogey Teams: Tottenham (3)
    club.set_opposition_factor(3, dec!(0.85));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1001, "Ederson", Position::GK, dec!(70.0), false),
        (1002, "Ruben Dias", Position::CB, dec!(85.0), false),
        (1003, "Rodri", Position::CDM, dec!(150.0), false),
        (1004, "Kevin De Bruyne", Position::CAM, dec!(140.0), false),
        (1005, "Erling Haaland", Position::ST, dec!(180.0), false),
        (1006, "Phil Foden", Position::CAM, dec!(130.0), false),
        (1007, "Bernardo Silva", Position::CM, dec!(95.0), false),
        (1008, "Kyle Walker", Position::RB, dec!(60.0), true),
        (1009, "Josko Gvardiol", Position::CB, dec!(80.0), false),
        (1010, "Jeremy Doku", Position::LW, dec!(75.0), false),
        (1011, "Jack Grealish", Position::LW, dec!(80.0), false),
        (1012, "Stefan Ortega", Position::GK, dec!(30.0), false),
        (1013, "Nathan Ake", Position::CB, dec!(65.0), false),
        (1014, "Manuel Akanji", Position::CB, dec!(70.0), false),
        (1015, "Mateo Kovacic", Position::CM, dec!(60.0), false),
        (1016, "Ilkay Gundogan", Position::CM, dec!(70.0), false),
        (1017, "Matheus Nunes", Position::CM, dec!(50.0), false),
        (1018, "Savinho", Position::RW, dec!(65.0), false),
        (1019, "Rico Lewis", Position::RB, dec!(55.0), false),
        (1020, "James McAtee", Position::CAM, dec!(35.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.names.insert(id, name.to_string());
        engine.player_states.insert(id, PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight: dec!(1.0),
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![val; 5],
            position: pos,
            is_captain: captain,
        });
    }
}
