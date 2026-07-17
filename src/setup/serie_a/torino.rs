use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3009);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(739.00); 
    
    club.set_rival_factor(ClubId(3003), dec!(1.5)); // Juventus (Derby della Mole)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Torino".to_string());

    let players = vec![
        (30401, "Vanja Milinkovic-Savic", Position::GK, dec!(40.00), dec!(1.0), false),
        (30402, "Saul Coco", Position::CB, dec!(44.89), dec!(1.045), false),
        (30403, "Guillermo Maripan", Position::CB, dec!(36.94), dec!(0.944), false),
        (30404, "Adam Masina", Position::CB, dec!(27.45), dec!(1.042), false),
        (30405, "Marcus Pedersen", Position::RB, dec!(32.85), dec!(1.038), false),
        (30406, "Samuele Ricci", Position::CDM, dec!(65.00), dec!(1.0), false),
        (30407, "Ivan Ilic", Position::CM, dec!(55.00), dec!(1.0), false),
        (30408, "Karol Linetty", Position::CM, dec!(25.00), dec!(1.0), true),
        (30409, "Valentino Lazaro", Position::LB, dec!(31.77), dec!(1.040), false),
        (30410, "Antonio Sanabria", Position::ST, dec!(45.00), dec!(1.0), false),
        (30411, "Che Adams", Position::ST, dec!(62.34), dec!(1.079), false),
        // Bench
        (30412, "Alberto Paleari", Position::GK, dec!(10.34), dec!(0.994), false),
        (30413, "Walukiewicz", Position::CB, dec!(30.00), dec!(1.0), false),
        (30414, "Borna Sosa", Position::LB, dec!(35.00), dec!(1.0), false),
        (30415, "Vojvoda", Position::RB, dec!(25.00), dec!(1.0), false),
        (30416, "Adrien Tameze", Position::CDM, dec!(28.59), dec!(0.926), false),
        (30417, "Saba Sazonov", Position::CB, dec!(20.00), dec!(1.0), false),
        (30418, "Nikola Vlasic", Position::CAM, dec!(64.27), dec!(1.0), false),
        (30419, "Duvan Zapata", Position::ST, dec!(67.54), dec!(1.032), false),
        (30420, "Karamoh", Position::LW, dec!(20.00), dec!(1.0), false),
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
