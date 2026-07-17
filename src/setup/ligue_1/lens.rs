use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5007);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(866.67); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "RC Lens".to_string());

    let players = vec![
        (50301, "Brice Samba", Position::GK, dec!(45.00), dec!(1.0), true),
        (50302, "Przemyslaw Frankowski", Position::RM, dec!(35.00), dec!(1.0), false),
        (50303, "Jonathan Gradit", Position::CB, dec!(21.78), dec!(0.845), false),
        (50304, "Kevin Danso", Position::CB, dec!(65.00), dec!(1.0), false),
        (50305, "Deiver Machado", Position::LM, dec!(27.54), dec!(1.038), false),
        (50306, "Andy Diouf", Position::CDM, dec!(43.38), dec!(1.031), false),
        (50307, "Nampalys Mendy", Position::CDM, dec!(20.00), dec!(1.0), false),
        (50308, "Angelo Fulgini", Position::CAM, dec!(35.00), dec!(1.0), false),
        (50309, "Wesley Said", Position::ST, dec!(67.49), dec!(1.187), false),
        (50310, "Florian Sotoca", Position::RW, dec!(35.68), dec!(1.144), false),
        (50311, "M'Bala Nzola", Position::ST, dec!(45.00), dec!(1.0), false),
        // Bench
        (50312, "Herve Koffi", Position::GK, dec!(15.00), dec!(1.0), false),
        (50313, "Abdukodir Khusanov", Position::CB, dec!(35.00), dec!(1.0), false),
        (50314, "Malang Sarr", Position::CB, dec!(39.15), dec!(1.064), false),
        (50315, "Ruben Aguilar", Position::RB, dec!(29.18), dec!(1.020), false),
        (50316, "Sidi Bane", Position::CB, dec!(15.00), dec!(1.0), false),
        (50317, "Neil El Aynaoui", Position::CM, dec!(30.00), dec!(1.0), false),
        (50318, "Anass Zaroury", Position::CAM, dec!(35.00), dec!(1.0), false),
        (50319, "Martin Satriano", Position::ST, dec!(35.00), dec!(1.0), false),
        (50320, "David Pereira da Costa", Position::CAM, dec!(40.00), dec!(1.0), false),
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
            active: true,
        });
        engine.names.insert(id, name.to_string());
        if let Some(mut c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(PlayerId(id));
        }
    }
}
