use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4011);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(360.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "FC Augsburg".to_string());

    let players = vec![
        (40501, "Nediljko Labrovic", Position::GK, dec!(30.0), false),
        (40502, "Marius Wolf", Position::RB, dec!(35.0), false),
        (40503, "Jeffrey Gouweleeuw", Position::CB, dec!(25.0), true),
        (40504, "Kven Schlotterbeck", Position::CB, dec!(35.0), false),
        (40505, "Dimitris Giannoulis", Position::LB, dec!(30.0), false),
        (40506, "Frank Onyeka", Position::CDM, dec!(40.0), false),
        (40507, "Kristijan Jakic", Position::CDM, dec!(35.0), false),
        (40508, "Elvis Rexhbecaj", Position::CM, dec!(30.0), false),
        (40509, "Arne Maier", Position::CM, dec!(35.0), false),
        (40510, "Ruben Vargas", Position::CAM, dec!(45.0), false),
        (40511, "Samuel Essende", Position::ST, dec!(40.0), false),
        // Bench
        (40512, "Finn Dahmen", Position::GK, dec!(25.0), false),
        (40513, "Chrislain Matsima", Position::CB, dec!(35.0), false),
        (40514, "Mads Pedersen", Position::LB, dec!(20.0), false),
        (40515, "Mert Komur", Position::CM, dec!(15.0), false),
        (40516, "Fredrik Jensen", Position::CAM, dec!(25.0), false),
        (40517, "Alexis Claude-Maurice", Position::CAM, dec!(30.0), false),
        (40518, "Masaya Okugawa", Position::RW, dec!(20.0), false),
        (40519, "Phillip Tietz", Position::ST, dec!(35.0), false),
        (40520, "Steve Mounie", Position::ST, dec!(30.0), false),
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
