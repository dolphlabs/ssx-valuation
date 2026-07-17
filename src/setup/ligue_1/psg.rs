use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5001);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1701.78); 
    
    club.set_rival_factor(ClubId(5008), dec!(1.5)); // Marseille (Le Classique)
    club.set_rival_factor(ClubId(5002), dec!(1.3)); // Monaco

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Paris Saint-Germain".to_string());

    let players = vec![
        (50001, "Gianluigi Donnarumma", Position::GK, dec!(80.00), dec!(1.0), false),
        (50002, "Achraf Hakimi", Position::RB, dec!(78.80), dec!(1.056), false),
        (50003, "Marquinhos", Position::CB, dec!(88.45), dec!(1.111), true),
        (50004, "Willian Pacho", Position::CB, dec!(65.24), dec!(1.087), false),
        (50005, "Nuno Mendes", Position::LB, dec!(94.71), dec!(1.094), false),
        (50006, "Joao Neves", Position::CDM, dec!(118.01), dec!(1.070), false),
        (50007, "Vitinha", Position::CM, dec!(123.24), dec!(1.072), false),
        (50008, "Warren Zaïre-Emery", Position::CM, dec!(99.78), dec!(1.024), false),
        (50009, "Ousmane Dembélé", Position::RW, dec!(149.94), dec!(1.108), false),
        (50010, "Bradley Barcola", Position::LW, dec!(131.42), dec!(1.028), false),
        (50011, "Gonçalo Ramos", Position::ST, dec!(73.95), dec!(1.013), false),
        // Bench
        (50012, "Matvey Safonov", Position::GK, dec!(31.40), dec!(1.143), false),
        (50013, "Lucas Beraldo", Position::CB, dec!(56.88), dec!(1.082), false),
        (50014, "Milan Skriniar", Position::CB, dec!(45.00), dec!(1.0), false),
        (50015, "Fabian Ruiz", Position::CM, dec!(67.57), dec!(1.089), false),
        (50016, "Kang-in Lee", Position::CAM, dec!(65.57), dec!(1.054), false),
        (50017, "Marco Asensio", Position::RW, dec!(45.00), dec!(1.0), false),
        (50018, "Desire Doue", Position::LW, dec!(68.36), dec!(1.136), false),
        (50019, "Randal Kolo Muani", Position::ST, dec!(60.00), dec!(1.0), false),
        (50020, "Lucas Hernandez", Position::CB, dec!(65.30), dec!(1.095), false),
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
