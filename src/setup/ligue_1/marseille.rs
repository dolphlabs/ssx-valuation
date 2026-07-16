use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5008);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(905.47); 
    
    club.set_rival_factor(ClubId(5001), dec!(1.5)); // PSG (Le Classique)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Olympique de Marseille".to_string());

    let players = vec![
        (50351, "Geronimo Rulli", Position::GK, dec!(40.29), dec!(1.085), false),
        (50352, "Michael Murillo", Position::RB, dec!(32.56), dec!(1.084), false),
        (50353, "Leonardo Balerdi", Position::CB, dec!(44.17), dec!(1.111), true),
        (50354, "Derek Cornelius", Position::CB, dec!(32.71), dec!(1.033), false),
        (50355, "Quentin Merlin", Position::LB, dec!(35.00), dec!(1.0), false),
        (50356, "Pierre-Emile Højbjerg", Position::CDM, dec!(69.26), dec!(1.185), false),
        (50357, "Geoffrey Kondogbia", Position::CDM, dec!(40.47), dec!(1.051), false),
        (50358, "Amine Harit", Position::CAM, dec!(45.00), dec!(1.0), false),
        (50359, "Mason Greenwood", Position::RW, dec!(127.18), dec!(1.196), false),
        (50360, "Luis Henrique", Position::LW, dec!(35.00), dec!(1.0), false),
        (50361, "Elye Wahi", Position::ST, dec!(60.00), dec!(1.0), false),
        // Bench
        (50362, "Jeffrey de Lange", Position::GK, dec!(21.03), dec!(0.987), false),
        (50363, "Lilian Brassier", Position::CB, dec!(35.00), dec!(1.0), false),
        (50364, "Pol Lirola", Position::RB, dec!(20.99), dec!(1.024), false),
        (50365, "Ulisses Garcia", Position::LB, dec!(12.47), dec!(0.659), false),
        (50366, "Adrien Rabiot", Position::CM, dec!(56.03), dec!(0.936), false),
        (50367, "Valentin Rongier", Position::CM, dec!(40.00), dec!(1.0), false),
        (50368, "Ismaël Koné", Position::CM, dec!(40.00), dec!(1.0), false),
        (50369, "Jonathan Rowe", Position::LW, dec!(35.39), dec!(1.004), false),
        (50370, "Neal Maupay", Position::ST, dec!(50.00), dec!(1.0), false),
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
