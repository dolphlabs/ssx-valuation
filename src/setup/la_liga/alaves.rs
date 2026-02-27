use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2010;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(320.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Deportivo Alaves".to_string());

    let players = vec![
        (20451, "Antonio Sivera", Position::GK, dec!(25.0), false),
        (20452, "Nahuel Tenaglia", Position::RB, dec!(25.0), false),
        (20453, "Abdel Abqar", Position::CB, dec!(30.0), false),
        (20454, "Aleksandar Sedlar", Position::CB, dec!(15.0), false),
        (20455, "Manu Sanchez", Position::LB, dec!(25.0), false),
        (20456, "Ander Guevara", Position::CDM, dec!(35.0), false),
        (20457, "Jon Guridi", Position::CAM, dec!(30.0), false),
        (20458, "Antonio Blanco", Position::CM, dec!(35.0), false),
        (20459, "Carlos Vicente", Position::RW, dec!(35.0), false),
        (20460, "Tomas Conechny", Position::LW, dec!(30.0), false),
        (20461, "Kike Garcia", Position::ST, dec!(20.0), true),
        // Bench
        (20462, "Jesus Owono", Position::GK, dec!(10.0), false),
        (20463, "Santiago Mourino", Position::CB, dec!(25.0), false),
        (20464, "Hugo Novoa", Position::RB, dec!(25.0), false),
        (20465, "Joan Jordan", Position::CM, dec!(30.0), false),
        (20466, "Carlos Martin", Position::ST, dec!(25.0), false),
        (20467, "Luka Romero", Position::RW, dec!(30.0), false),
        (20468, "Stoichkov", Position::LW, dec!(25.0), false),
        (20469, "Toni Martinez", Position::ST, dec!(30.0), false),
        (20470, "Asier Villalibre", Position::ST, dec!(25.0), false),
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
