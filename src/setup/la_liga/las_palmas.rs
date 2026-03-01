use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2016;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(310.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "UD Las Palmas".to_string());

    let players = vec![
        (20751, "Jasper Cillessen", Position::GK, dec!(25.0), false),
        (20752, "Alex Suarez", Position::RB, dec!(25.0), false),
        (20753, "Mika Marmol", Position::CB, dec!(40.0), false),
        (20754, "Scott McKenna", Position::CB, dec!(25.0), false),
        (20755, "Alex Munoz", Position::LB, dec!(25.0), false),
        (20756, "Dario Essugo", Position::CDM, dec!(30.0), false),
        (20757, "Kirian Rodriguez", Position::CM, dec!(40.0), true),
        (20758, "Javi Munoz", Position::CM, dec!(30.0), false),
        (20759, "Alberto Moleiro", Position::CAM, dec!(60.0), false),
        (20760, "Sandro Ramirez", Position::ST, dec!(35.0), false),
        (20761, "Oliver McBurnie", Position::ST, dec!(35.0), false),
        // Bench
        (20762, "Dinko Horkas", Position::GK, dec!(15.0), false),
        (20763, "Juanma Herzog", Position::CB, dec!(25.0), false),
        (20764, "Viti Rozada", Position::RB, dec!(20.0), false),
        (20765, "Jose Campana", Position::CM, dec!(25.0), false),
        (20766, "Fabio Silva", Position::ST, dec!(45.0), false),
        (20767, "Sory Kaba", Position::ST, dec!(25.0), false),
        (20768, "Marc Cardona", Position::ST, dec!(20.0), false),
        (20769, "Adnan Januzaj", Position::RW, dec!(35.0), false),
        (20770, "Manu Fuster", Position::LW, dec!(25.0), false),
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
