use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 4001;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(980.00); // Unbeaten champions
    
    club.set_rival_factor(4002, dec!(1.3)); // Bayern Munich
    club.set_rival_factor(4005, dec!(1.25)); // Dortmund

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Bayer Leverkusen".to_string());

    let players = vec![
        (40001, "Lukas Hradecky", Position::GK, dec!(45.0), true),
        (40002, "Edmond Tapsoba", Position::CB, dec!(65.0), false),
        (40003, "Jonathan Tah", Position::CB, dec!(55.0), false),
        (40004, "Piero Hincapie", Position::CB, dec!(60.0), false),
        (40005, "Jeremie Frimpong", Position::RM, dec!(80.0), false),
        (40006, "Granit Xhaka", Position::CDM, dec!(75.0), false),
        (40007, "Aleix Garcia", Position::CM, dec!(50.0), false),
        (40008, "Alejandro Grimaldo", Position::LM, dec!(70.0), false),
        (40009, "Florian Wirtz", Position::CAM, dec!(130.0), false),
        (40010, "Martin Terrier", Position::LW, dec!(45.0), false),
        (40011, "Victor Boniface", Position::ST, dec!(75.0), false),
        // Bench
        (40012, "Matej Kovar", Position::GK, dec!(35.0), false),
        (40013, "Jeanuel Belocian", Position::CB, dec!(30.0), false),
        (40014, "Nordi Mukiele", Position::RB, dec!(40.0), false),
        (40015, "Robert Andrich", Position::CDM, dec!(45.0), false),
        (40016, "Exequiel Palacios", Position::CM, dec!(55.0), false),
        (40017, "Jonas Hofmann", Position::CAM, dec!(40.0), false),
        (40018, "Amine Adli", Position::RW, dec!(55.0), false),
        (40019, "Patrik Schick", Position::ST, dec!(45.0), false),
        (40020, "Nathan Tella", Position::RM, dec!(40.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.player_states.insert(id, PlayerValues {
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
            c.player_ids.push(id);
        }
    }
}
