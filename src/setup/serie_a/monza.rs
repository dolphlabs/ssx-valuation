use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3012;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(340.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Monza".to_string());

    let players = vec![
        (30551, "Stefano Turati", Position::GK, dec!(30.0), false),
        (30552, "Armando Izzo", Position::CB, dec!(25.0), false),
        (30553, "Pablo Mari", Position::CB, dec!(35.0), false),
        (30554, "Andrea Carboni", Position::CB, dec!(30.0), false),
        (30555, "Pedro Pereira", Position::RB, dec!(25.0), false),
        (30556, "Matteo Pessina", Position::CM, dec!(50.0), true),
        (30557, "Warren Bondo", Position::CM, dec!(35.0), false),
        (30558, "Andrea Kyriakopoulos", Position::LB, dec!(25.0), false),
        (30559, "Dany Mota", Position::LW, dec!(40.0), false),
        (30560, "Daniel Maldini", Position::CAM, dec!(45.0), false),
        (30561, "Milan Djuric", Position::ST, dec!(30.0), false),
        // Bench
        (30562, "Alessandro Sorrentino", Position::GK, dec!(10.0), false),
        (30563, "Danilo D'Ambrosio", Position::CB, dec!(15.0), false),
        (30564, "Luca Caldirola", Position::CB, dec!(10.0), false),
        (30565, "Samuele Birindelli", Position::RB, dec!(25.0), false),
        (30566, "Stefano Sensi", Position::CM, dec!(20.0), false),
        (30567, "Roberto Gagliardini", Position::CDM, dec!(25.0), false),
        (30568, "Gianluca Caprari", Position::LW, dec!(35.0), false),
        (30569, "Andrea Petagna", Position::ST, dec!(30.0), false),
        (30570, "Samuele Vignato", Position::CAM, dec!(20.0), false),
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
