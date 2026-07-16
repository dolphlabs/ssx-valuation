use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3013);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(335.28); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Hellas Verona".to_string());

    let players = vec![
        (30601, "Lorenzo Montipò", Position::GK, dec!(29.48), dec!(1.105), false),
        (30602, "Jackson Tchatchoua", Position::RB, dec!(35.00), dec!(1.0), false),
        (30603, "Pawel Dawidowicz", Position::CB, dec!(30.00), dec!(1.0), true),
        (30604, "Diego Coppola", Position::CB, dec!(30.00), dec!(1.0), false),
        (30605, "Martin Frese", Position::LB, dec!(32.41), dec!(1.063), false),
        (30606, "Suat Serdar", Position::CM, dec!(66.39), dec!(1.022), false),
        (30607, "Ondrej Duda", Position::CM, dec!(35.00), dec!(1.0), false),
        (30608, "Tomas Suslov", Position::CAM, dec!(37.43), dec!(1.021), false),
        (30609, "Darko Lazovic", Position::LW, dec!(25.00), dec!(1.0), false),
        (30610, "Casper Tengstedt", Position::ST, dec!(40.00), dec!(1.0), false),
        (30611, "Daniel Mosquera", Position::ST, dec!(31.15), dec!(1.019), false),
        // Bench
        (30612, "Simone Perilli", Position::GK, dec!(9.67), dec!(0.969), false),
        (30613, "Flavius Daniliuc", Position::CB, dec!(25.00), dec!(1.0), false),
        (30614, "Dani Silva", Position::CM, dec!(25.00), dec!(1.0), false),
        (30615, "Reda Belahyane", Position::CDM, dec!(30.00), dec!(1.0), false),
        (30616, "Grigoris Kastanos", Position::CAM, dec!(32.18), dec!(1.026), false),
        (30617, "Abdou Harroui", Position::LW, dec!(38.30), dec!(1.037), false),
        (30618, "Dailon Livramento", Position::ST, dec!(25.00), dec!(1.0), false),
        (30619, "Faride Alidou", Position::RW, dec!(30.00), dec!(1.0), false),
        (30620, "Amin Sarr", Position::ST, dec!(23.46), dec!(1.029), false),
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
