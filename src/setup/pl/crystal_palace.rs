use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(14);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Crystal Palace".to_string());
    
    club.intrinsic_value = dec!(550.00);
    
    // Rivals: Brighton (10)
    club.set_rival_factor(ClubId(10), dec!(1.5));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1651, "Dean Henderson", Position::GK, dec!(45.0), false),
        (1652, "Marc Guehi", Position::CB, dec!(80.0), true),
        (1653, "Eberechi Eze", Position::CAM, dec!(95.0), false),
        (1654, "Jean-Philippe Mateta", Position::ST, dec!(70.0), false),
        (1655, "Adam Wharton", Position::CDM, dec!(65.0), false),
        (1656, "Tyrick Mitchell", Position::LB, dec!(50.0), false),
        (1657, "Daniel Munoz", Position::RB, dec!(55.0), false),
        (1658, "Maxence Lacroix", Position::CB, dec!(55.0), false),
        (1659, "Ismaila Sarr", Position::RW, dec!(50.0), false),
        (1660, "Daichi Kamada", Position::CAM, dec!(50.0), false),
        (1661, "Cheick Doucoure", Position::CDM, dec!(60.0), false),
        (1662, "Matt Turner", Position::GK, dec!(20.0), false),
        (1663, "Trevor Chalobah", Position::CB, dec!(50.0), false),
        (1664, "Chris Richards", Position::CB, dec!(45.0), false),
        (1665, "Joel Ward", Position::RB, dec!(20.0), false),
        (1666, "Will Hughes", Position::CM, dec!(35.0), false),
        (1667, "Jefferson Lerma", Position::CDM, dec!(45.0), false),
        (1668, "Matheus Franca", Position::CAM, dec!(40.0), false),
        (1669, "Eddie Nketiah", Position::ST, dec!(60.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.names.insert(id, name.to_string());
        engine.player_states.insert(PlayerId(id), PlayerValues {
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
