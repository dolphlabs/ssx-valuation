use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5007);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(480.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "RC Lens".to_string());

    let players = vec![
        (50301, "Brice Samba", Position::GK, dec!(45.0), true),
        (50302, "Przemyslaw Frankowski", Position::RM, dec!(35.0), false),
        (50303, "Jonathan Gradit", Position::CB, dec!(30.0), false),
        (50304, "Kevin Danso", Position::CB, dec!(65.0), false),
        (50305, "Deiver Machado", Position::LM, dec!(25.0), false),
        (50306, "Andy Diouf", Position::CDM, dec!(40.0), false),
        (50307, "Nampalys Mendy", Position::CDM, dec!(20.0), false),
        (50308, "Angelo Fulgini", Position::CAM, dec!(35.0), false),
        (50309, "Wesley Said", Position::ST, dec!(30.0), false),
        (50310, "Florian Sotoca", Position::RW, dec!(35.0), false),
        (50311, "M'Bala Nzola", Position::ST, dec!(45.0), false),
        // Bench
        (50312, "Herve Koffi", Position::GK, dec!(15.0), false),
        (50313, "Abdukodir Khusanov", Position::CB, dec!(35.0), false),
        (50314, "Malang Sarr", Position::CB, dec!(35.0), false),
        (50315, "Ruben Aguilar", Position::RB, dec!(30.0), false),
        (50316, "Sidi Bane", Position::CB, dec!(15.0), false),
        (50317, "Neil El Aynaoui", Position::CM, dec!(30.0), false),
        (50318, "Anass Zaroury", Position::CAM, dec!(35.0), false),
        (50319, "Martin Satriano", Position::ST, dec!(35.0), false),
        (50320, "David Pereira da Costa", Position::CAM, dec!(40.0), false),
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
