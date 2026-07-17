use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(1);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Manchester City".to_string());
    
    club.intrinsic_value = dec!(1749.45);
    
    // Rivals: Man Utd (2), Liverpool (5), Arsenal (4)
    club.set_rival_factor(ClubId(2), dec!(1.5));
    club.set_rival_factor(ClubId(5), dec!(1.3));
    club.set_rival_factor(ClubId(4), dec!(1.4));
    
    // Bogey Teams: Tottenham (3)
    club.set_opposition_factor(ClubId(3), dec!(0.85));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1001, "Ederson", Position::GK, dec!(70.00), dec!(1.0), false),
        (1002, "Ruben Dias", Position::CB, dec!(97.60), dec!(1.047), false),
        (1003, "Rodri", Position::CDM, dec!(183.39), dec!(1.039), false),
        (1004, "Kevin De Bruyne", Position::CAM, dec!(140.00), dec!(1.0), false),
        (1005, "Erling Haaland", Position::ST, dec!(319.81), dec!(1.115), false),
        (1006, "Phil Foden", Position::CAM, dec!(164.61), dec!(1.045), false),
        (1007, "Bernardo Silva", Position::CM, dec!(106.52), dec!(1.066), false),
        (1008, "Kyle Walker", Position::RB, dec!(60.00), dec!(1.0), true),
        (1009, "Josko Gvardiol", Position::CB, dec!(96.98), dec!(1.032), false),
        (1010, "Jeremy Doku", Position::LW, dec!(110.74), dec!(1.040), false),
        (1011, "Jack Grealish", Position::LW, dec!(80.00), dec!(1.0), false),
        (1012, "Stefan Ortega", Position::GK, dec!(30.00), dec!(1.0), false),
        (1013, "Nathan Ake", Position::CB, dec!(74.77), dec!(1.035), false),
        (1014, "Manuel Akanji", Position::CB, dec!(70.00), dec!(1.0), false),
        (1015, "Mateo Kovacic", Position::CM, dec!(60.00), dec!(1.0), false),
        (1016, "Ilkay Gundogan", Position::CM, dec!(70.00), dec!(1.0), false),
        (1017, "Matheus Nunes", Position::CM, dec!(57.52), dec!(1.069), false),
        (1018, "Savinho", Position::RW, dec!(72.49), dec!(1.045), false),
        (1019, "Rico Lewis", Position::RB, dec!(56.56), dec!(1.004), false),
        (1020, "James McAtee", Position::CAM, dec!(35.00), dec!(1.0), false),
    ];

    for (id, name, pos, val, form_weight, captain) in players {
        engine.names.insert(id, name.to_string());
        engine.player_states.insert(PlayerId(id), PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight,
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![val; 5],
            position: pos,
            is_captain: captain,
            active: true,
        });
    }
}
