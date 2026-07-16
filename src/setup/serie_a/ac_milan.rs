use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3002);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1284.33); 
    
    club.set_rival_factor(ClubId(3001), dec!(1.5)); // Inter Milan
    club.set_rival_factor(ClubId(3003), dec!(1.3)); // Juventus

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "AC Milan".to_string());

    let players = vec![
        (30051, "Mike Maignan", Position::GK, dec!(87.79), dec!(1.151), false),
        (30052, "Emerson Royal", Position::RB, dec!(40.00), dec!(1.0), false),
        (30053, "Fikayo Tomori", Position::CB, dec!(63.26), dec!(1.020), false),
        (30054, "Strahinja Pavlovic", Position::CB, dec!(51.82), dec!(1.0), false),
        (30055, "Theo Hernandez", Position::LB, dec!(85.00), dec!(1.0), false),
        (30056, "Youssouf Fofana", Position::CDM, dec!(64.37), dec!(1.021), false),
        (30057, "Tijjani Reijnders", Position::CM, dec!(60.00), dec!(1.0), false),
        (30058, "Ruben Loftus-Cheek", Position::CAM, dec!(75.38), dec!(1.026), false),
        (30059, "Christian Pulisic", Position::RW, dec!(116.01), dec!(1.0), false),
        (30060, "Rafael Leao", Position::LW, dec!(137.60), dec!(1.040), false),
        (30061, "Alvaro Morata", Position::ST, dec!(70.00), dec!(1.0), false),
        // Bench
        (30062, "Marco Sportiello", Position::GK, dec!(15.00), dec!(1.0), false),
        (30063, "Matteo Gabbia", Position::CB, dec!(38.11), dec!(1.073), false),
        (30064, "Malick Thiaw", Position::CB, dec!(40.00), dec!(1.0), false),
        (30065, "Davide Calabria", Position::RB, dec!(40.00), dec!(1.0), true),
        (30066, "Ismael Bennacer", Position::CDM, dec!(50.00), dec!(1.0), false),
        (30067, "Yunus Musah", Position::CM, dec!(45.35), dec!(1.042), false),
        (30068, "Samuel Chukwueze", Position::RW, dec!(45.00), dec!(1.0), false),
        (30069, "Noah Okafor", Position::ST, dec!(40.00), dec!(1.0), false),
        (30070, "Tammy Abraham", Position::ST, dec!(45.00), dec!(1.0), false),
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
