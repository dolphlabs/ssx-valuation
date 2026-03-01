use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 5001;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1100.00); 
    
    club.set_rival_factor(5008, dec!(1.5)); // Marseille (Le Classique)
    club.set_rival_factor(5002, dec!(1.3)); // Monaco

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Paris Saint-Germain".to_string());

    let players = vec![
        (50001, "Gianluigi Donnarumma", Position::GK, dec!(80.0), false),
        (50002, "Achraf Hakimi", Position::RB, dec!(85.0), false),
        (50003, "Marquinhos", Position::CB, dec!(75.0), true),
        (50004, "Willian Pacho", Position::CB, dec!(55.0), false),
        (50005, "Nuno Mendes", Position::LB, dec!(70.0), false),
        (50006, "Joao Neves", Position::CDM, dec!(75.0), false),
        (50007, "Vitinha", Position::CM, dec!(85.0), false),
        (50008, "Warren Zaïre-Emery", Position::CM, dec!(80.0), false),
        (50009, "Ousmane Dembélé", Position::RW, dec!(90.0), false),
        (50010, "Bradley Barcola", Position::LW, dec!(75.0), false),
        (50011, "Gonçalo Ramos", Position::ST, dec!(65.0), false),
        // Bench
        (50012, "Matvey Safonov", Position::GK, dec!(30.0), false),
        (50013, "Lucas Beraldo", Position::CB, dec!(45.0), false),
        (50014, "Milan Skriniar", Position::CB, dec!(45.0), false),
        (50015, "Fabian Ruiz", Position::CM, dec!(55.0), false),
        (50016, "Kang-in Lee", Position::CAM, dec!(60.0), false),
        (50017, "Marco Asensio", Position::RW, dec!(45.0), false),
        (50018, "Desire Doue", Position::LW, dec!(50.0), false),
        (50019, "Randal Kolo Muani", Position::ST, dec!(60.0), false),
        (50020, "Lucas Hernandez", Position::CB, dec!(60.0), false),
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
