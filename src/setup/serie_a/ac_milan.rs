use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3002);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(950.00); 
    
    club.set_rival_factor(ClubId(3001), dec!(1.5)); // Inter Milan
    club.set_rival_factor(ClubId(3003), dec!(1.3)); // Juventus

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "AC Milan".to_string());

    let players = vec![
        (30051, "Mike Maignan", Position::GK, dec!(75.0), false),
        (30052, "Emerson Royal", Position::RB, dec!(40.0), false),
        (30053, "Fikayo Tomori", Position::CB, dec!(60.0), false),
        (30054, "Strahinja Pavlovic", Position::CB, dec!(45.0), false),
        (30055, "Theo Hernandez", Position::LB, dec!(85.0), false),
        (30056, "Youssouf Fofana", Position::CDM, dec!(55.0), false),
        (30057, "Tijjani Reijnders", Position::CM, dec!(60.0), false),
        (30058, "Ruben Loftus-Cheek", Position::CAM, dec!(50.0), false),
        (30059, "Christian Pulisic", Position::RW, dec!(75.0), false),
        (30060, "Rafael Leao", Position::LW, dec!(110.0), false),
        (30061, "Alvaro Morata", Position::ST, dec!(70.0), false),
        // Bench
        (30062, "Marco Sportiello", Position::GK, dec!(15.0), false),
        (30063, "Matteo Gabbia", Position::CB, dec!(35.0), false),
        (30064, "Malick Thiaw", Position::CB, dec!(40.0), false),
        (30065, "Davide Calabria", Position::RB, dec!(40.0), true),
        (30066, "Ismael Bennacer", Position::CDM, dec!(50.0), false),
        (30067, "Yunus Musah", Position::CM, dec!(40.0), false),
        (30068, "Samuel Chukwueze", Position::RW, dec!(45.0), false),
        (30069, "Noah Okafor", Position::ST, dec!(40.0), false),
        (30070, "Tammy Abraham", Position::ST, dec!(45.0), false),
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
