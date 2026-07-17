use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2010);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(470.11); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Deportivo Alaves".to_string());

    let players = vec![
        (20451, "Antonio Sivera", Position::GK, dec!(26.24), dec!(1.057), false),
        (20452, "Nahuel Tenaglia", Position::RB, dec!(30.54), dec!(1.030), false),
        (20453, "Abdel Abqar", Position::CB, dec!(30.00), dec!(1.0), false),
        (20454, "Aleksandar Sedlar", Position::CB, dec!(15.00), dec!(1.0), false),
        (20455, "Manu Sanchez", Position::LB, dec!(25.00), dec!(1.0), false),
        (20456, "Ander Guevara", Position::CDM, dec!(30.51), dec!(1.009), false),
        (20457, "Jon Guridi", Position::CAM, dec!(31.38), dec!(1.074), false),
        (20458, "Antonio Blanco", Position::CM, dec!(39.67), dec!(1.061), false),
        (20459, "Carlos Vicente", Position::RW, dec!(61.48), dec!(1.083), false),
        (20460, "Tomas Conechny", Position::LW, dec!(30.00), dec!(1.0), false),
        (20461, "Kike Garcia", Position::ST, dec!(20.00), dec!(1.0), true),
        // Bench
        (20462, "Jesus Owono", Position::GK, dec!(10.00), dec!(1.0), false),
        (20463, "Santiago Mourino", Position::CB, dec!(25.00), dec!(1.0), false),
        (20464, "Hugo Novoa", Position::RB, dec!(25.00), dec!(1.0), false),
        (20465, "Joan Jordan", Position::CM, dec!(30.00), dec!(1.0), false),
        (20466, "Carlos Martin", Position::ST, dec!(25.00), dec!(1.0), false),
        (20467, "Luka Romero", Position::RW, dec!(30.00), dec!(1.0), false),
        (20468, "Stoichkov", Position::LW, dec!(25.00), dec!(1.0), false),
        (20469, "Toni Martinez", Position::ST, dec!(60.26), dec!(1.131), false),
        (20470, "Asier Villalibre", Position::ST, dec!(25.00), dec!(1.0), false),
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
