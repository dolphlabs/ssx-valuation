use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 15;
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id, "Brentford".to_string());
    
    club.intrinsic_value = dec!(480.00);
    
    // Rivals: Fulham (12), Chelsea (6)
    club.set_rival_factor(12, dec!(1.3));
    club.set_rival_factor(6, dec!(1.2));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1701, "Mark Flekken", Position::GK, dec!(35.0), false),
        (1702, "Bryan Mbeumo", Position::RW, dec!(75.0), false),
        (1703, "Yoane Wissa", Position::LW, dec!(60.0), false),
        (1704, "Christian Norgaard", Position::CDM, dec!(50.0), true),
        (1705, "Mathias Jensen", Position::CM, dec!(45.0), false),
        (1706, "Ethan Pinnock", Position::CB, dec!(50.0), false),
        (1707, "Nathan Collins", Position::CB, dec!(55.0), false),
        (1708, "Kristoffer Ajer", Position::CB, dec!(45.0), false),
        (1709, "Mikkel Damsgaard", Position::CAM, dec!(45.0), false),
        (1710, "Kevin Schade", Position::LW, dec!(40.0), false),
        (1711, "Igor Thiago", Position::ST, dec!(50.0), false),
        (1712, "Hakon Valdimarsson", Position::GK, dec!(15.0), false),
        (1713, "Sepp van den Berg", Position::CB, dec!(45.0), false),
        (1714, "Mads Roerslev", Position::RB, dec!(30.0), false),
        (1715, "Vitaly Janelt", Position::CDM, dec!(45.0), false),
        (1716, "Yunus Emre Konak", Position::CDM, dec!(25.0), false),
        (1717, "Fabio Carvalho", Position::CAM, dec!(55.0), false),
        (1718, "Keane Lewis-Potter", Position::LW, dec!(40.0), false),
        (1719, "Gustavo Nunes", Position::LW, dec!(35.0), false),
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
