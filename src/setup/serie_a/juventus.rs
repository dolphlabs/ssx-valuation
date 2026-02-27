use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3003;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1000.00); 
    
     club.set_rival_factor(3001, dec!(1.4)); // Inter (Derby d'Italia)
    club.set_rival_factor(3002, dec!(1.3)); // AC Milan
    club.set_rival_factor(3009, dec!(1.5)); // Torino (Derby della Mole)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Juventus".to_string());

    let players = vec![
        (30101, "Michele Di Gregorio", Position::GK, dec!(45.0), false),
        (30102, "Nicolo Savona", Position::RB, dec!(30.0), false),
        (30103, "Federico Gatti", Position::CB, dec!(45.0), false),
        (30104, "Bremer", Position::CB, dec!(80.0), false),
        (30105, "Andrea Cambiaso", Position::LB, dec!(55.0), false),
        (30106, "Douglas Luiz", Position::CDM, dec!(70.0), false),
        (30107, "Manuel Locatelli", Position::CDM, dec!(50.0), false),
        (30108, "Teun Koopmeiners", Position::CAM, dec!(85.0), false),
        (30109, "Kenan Yildiz", Position::LW, dec!(60.0), false),
        (30110, "Nico Gonzalez", Position::RW, dec!(55.0), false),
        (30111, "Dusan Vlahovic", Position::ST, dec!(100.0), false),
        // Bench
        (30112, "Mattia Perin", Position::GK, dec!(15.0), false),
        (30113, "Pierre Kalulu", Position::CB, dec!(45.0), false),
        (30114, "Juan Cabal", Position::LB, dec!(25.0), false),
        (30115, "Danilo", Position::CB, dec!(35.0), true),
        (30116, "Khephren Thuram", Position::CM, dec!(55.0), false),
        (30117, "Weston McKennie", Position::CM, dec!(40.0), false),
        (30118, "Timothy Weah", Position::RW, dec!(40.0), false),
        (30119, "Francisco Conceicao", Position::RW, dec!(45.0), false),
        (30120, "Arkadiusz Milik", Position::ST, dec!(30.0), false),
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
