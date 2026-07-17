use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(13);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Bournemouth".to_string());
    
    club.intrinsic_value = dec!(840.33);
    
    // Rivals: Southampton (20)
    club.set_rival_factor(ClubId(20), dec!(1.3));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1601, "Kepa Arrizabalaga", Position::GK, dec!(35.00), dec!(1.0), false),
        (1602, "Evanilson", Position::ST, dec!(90.80), dec!(1.060), false),
        (1603, "Antoine Semenyo", Position::RW, dec!(91.45), dec!(1.088), false),
        (1604, "Justin Kluivert", Position::LW, dec!(59.50), dec!(1.022), false),
        (1605, "Lewis Cook", Position::CM, dec!(50.80), dec!(1.014), true),
        (1606, "Ryan Christie", Position::CM, dec!(53.37), dec!(0.773), false),
        (1607, "Illia Zabarnyi", Position::CB, dec!(55.00), dec!(1.0), false),
        (1608, "Marcos Senesi", Position::CB, dec!(56.72), dec!(1.120), false),
        (1609, "Milos Kerkez", Position::LB, dec!(50.00), dec!(1.0), false),
        (1610, "Adam Smith", Position::RB, dec!(30.50), dec!(1.078), false),
        (1611, "Marcus Tavernier", Position::LM, dec!(69.17), dec!(1.152), false),
        (1612, "Mark Travers", Position::GK, dec!(25.00), dec!(1.0), false),
        (1613, "Julian Araujo", Position::RB, dec!(40.00), dec!(1.0), false),
        (1614, "Dean Huijsen", Position::CB, dec!(45.00), dec!(1.0), false),
        (1615, "Alex Scott", Position::CM, dec!(67.44), dec!(1.035), false),
        (1616, "Tyler Adams", Position::CDM, dec!(59.15), dec!(1.081), false),
        (1617, "Luis Sinisterra", Position::LW, dec!(55.00), dec!(1.0), false),
        (1618, "Dango Ouattara", Position::RW, dec!(50.00), dec!(1.0), false),
        (1619, "David Brooks", Position::RW, dec!(44.75), dec!(1.035), false),
        (1620, "Enes Unal", Position::ST, dec!(52.65), dec!(1.008), false),
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
