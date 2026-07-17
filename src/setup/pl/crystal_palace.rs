use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(14);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Crystal Palace".to_string());
    
    club.intrinsic_value = dec!(773.39);
    
    // Rivals: Brighton (10)
    club.set_rival_factor(ClubId(10), dec!(1.5));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1651, "Dean Henderson", Position::GK, dec!(47.10), dec!(1.064), false),
        (1652, "Marc Guehi", Position::CB, dec!(102.36), dec!(1.088), true),
        (1653, "Eberechi Eze", Position::CAM, dec!(105.39), dec!(1.039), false),
        (1654, "Jean-Philippe Mateta", Position::ST, dec!(112.53), dec!(1.103), false),
        (1655, "Adam Wharton", Position::CDM, dec!(74.91), dec!(1.037), false),
        (1656, "Tyrick Mitchell", Position::LB, dec!(59.67), dec!(1.051), false),
        (1657, "Daniel Munoz", Position::RB, dec!(71.03), dec!(1.018), false),
        (1658, "Maxence Lacroix", Position::CB, dec!(60.70), dec!(1.077), false),
        (1659, "Ismaila Sarr", Position::RW, dec!(83.41), dec!(1.042), false),
        (1660, "Daichi Kamada", Position::CAM, dec!(55.40), dec!(1.047), false),
        (1661, "Cheick Doucoure", Position::CDM, dec!(60.00), dec!(1.0), false),
        (1662, "Matt Turner", Position::GK, dec!(20.00), dec!(1.0), false),
        (1663, "Trevor Chalobah", Position::CB, dec!(50.00), dec!(1.0), false),
        (1664, "Chris Richards", Position::CB, dec!(57.04), dec!(0.984), false),
        (1665, "Joel Ward", Position::RB, dec!(20.00), dec!(1.0), false),
        (1666, "Will Hughes", Position::CM, dec!(35.61), dec!(1.080), false),
        (1667, "Jefferson Lerma", Position::CDM, dec!(46.86), dec!(1.049), false),
        (1668, "Matheus Franca", Position::CAM, dec!(40.00), dec!(1.0), false),
        (1669, "Eddie Nketiah", Position::ST, dec!(67.88), dec!(1.024), false),
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
