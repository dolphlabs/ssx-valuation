use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3009;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(480.00); 
    
    club.set_rival_factor(3003, dec!(1.5)); // Juventus (Derby della Mole)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Torino".to_string());

    let players = vec![
        (30401, "Vanja Milinkovic-Savic", Position::GK, dec!(40.0), false),
        (30402, "Saul Coco", Position::CB, dec!(35.0), false),
        (30403, "Guillermo Maripan", Position::CB, dec!(30.0), false),
        (30404, "Adam Masina", Position::CB, dec!(25.0), false),
        (30405, "Marcus Pedersen", Position::RB, dec!(30.0), false),
        (30406, "Samuele Ricci", Position::CDM, dec!(65.0), false),
        (30407, "Ivan Ilic", Position::CM, dec!(55.0), false),
        (30408, "Karol Linetty", Position::CM, dec!(25.0), true),
        (30409, "Valentino Lazaro", Position::LB, dec!(30.0), false),
        (30410, "Antonio Sanabria", Position::ST, dec!(45.0), false),
        (30411, "Che Adams", Position::ST, dec!(45.0), false),
        // Bench
        (30412, "Alberto Paleari", Position::GK, dec!(10.0), false),
        (30413, "Walukiewicz", Position::CB, dec!(30.0), false),
        (30414, "Borna Sosa", Position::LB, dec!(35.0), false),
        (30415, "Vojvoda", Position::RB, dec!(25.0), false),
        (30416, "Adrien Tameze", Position::CDM, dec!(30.0), false),
        (30417, "Saba Sazonov", Position::CB, dec!(20.0), false),
        (30418, "Nikola Vlasic", Position::CAM, dec!(50.0), false),
        (30419, "Duvan Zapata", Position::ST, dec!(55.0), false),
        (30420, "Karamoh", Position::LW, dec!(20.0), false),
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
        if let Some(c) = engine.club_states.get_mut(&club_id) {
            c.player_ids.push(id);
        }
    }
}
