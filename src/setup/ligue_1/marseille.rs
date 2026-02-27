use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 5008;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(620.00); 
    
    club.set_rival_factor(5001, dec!(1.5)); // PSG (Le Classique)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Olympique de Marseille".to_string());

    let players = vec![
        (50351, "Geronimo Rulli", Position::GK, dec!(35.0), false),
        (50352, "Michael Murillo", Position::RB, dec!(25.0), false),
        (50353, "Leonardo Balerdi", Position::CB, dec!(45.0), true),
        (50354, "Derek Cornelius", Position::CB, dec!(30.0), false),
        (50355, "Quentin Merlin", Position::LB, dec!(35.0), false),
        (50356, "Pierre-Emile Højbjerg", Position::CDM, dec!(60.0), false),
        (50357, "Geoffrey Kondogbia", Position::CDM, dec!(40.0), false),
        (50358, "Amine Harit", Position::CAM, dec!(45.0), false),
        (50359, "Mason Greenwood", Position::RW, dec!(75.0), false),
        (50360, "Luis Henrique", Position::LW, dec!(35.0), false),
        (50361, "Elye Wahi", Position::ST, dec!(60.0), false),
        // Bench
        (50362, "Jeffrey de Lange", Position::GK, dec!(20.0), false),
        (50363, "Lilian Brassier", Position::CB, dec!(35.0), false),
        (50364, "Pol Lirola", Position::RB, dec!(20.0), false),
        (50365, "Ulisses Garcia", Position::LB, dec!(25.0), false),
        (50366, "Adrien Rabiot", Position::CM, dec!(70.0), false),
        (50367, "Valentin Rongier", Position::CM, dec!(40.0), false),
        (50368, "Ismaël Koné", Position::CM, dec!(40.0), false),
        (50369, "Jonathan Rowe", Position::LW, dec!(35.0), false),
        (50370, "Neal Maupay", Position::ST, dec!(50.0), false),
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
