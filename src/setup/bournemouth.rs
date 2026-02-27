use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 13;
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id, "Bournemouth".to_string());
    
    club.intrinsic_value = dec!(500.00);
    
    // Rivals: Southampton (20)
    club.set_rival_factor(20, dec!(1.3));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1601, "Kepa Arrizabalaga", Position::GK, dec!(35.0), false),
        (1602, "Evanilson", Position::ST, dec!(70.0), false),
        (1603, "Antoine Semenyo", Position::RW, dec!(55.0), false),
        (1604, "Justin Kluivert", Position::LW, dec!(50.0), false),
        (1605, "Lewis Cook", Position::CM, dec!(45.0), true),
        (1606, "Ryan Christie", Position::CM, dec!(45.0), false),
        (1607, "Illia Zabarnyi", Position::CB, dec!(55.0), false),
        (1608, "Marcos Senesi", Position::CB, dec!(50.0), false),
        (1609, "Milos Kerkez", Position::LB, dec!(50.0), false),
        (1610, "Adam Smith", Position::RB, dec!(30.0), false),
        (1611, "Marcus Tavernier", Position::LM, dec!(45.0), false),
        (1612, "Mark Travers", Position::GK, dec!(25.0), false),
        (1613, "Julian Araujo", Position::RB, dec!(40.0), false),
        (1614, "Dean Huijsen", Position::CB, dec!(45.0), false),
        (1615, "Alex Scott", Position::CM, dec!(50.0), false),
        (1616, "Tyler Adams", Position::CDM, dec!(55.0), false),
        (1617, "Luis Sinisterra", Position::LW, dec!(55.0), false),
        (1618, "Dango Ouattara", Position::RW, dec!(50.0), false),
        (1619, "David Brooks", Position::RW, dec!(40.0), false),
        (1620, "Enes Unal", Position::ST, dec!(50.0), false),
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
