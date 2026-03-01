use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 2007;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(480.00); 
    
    club.set_rival_factor(2014, dec!(1.5)); // Sevilla (Seville Derby)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Real Betis".to_string());

    let players = vec![
        (20301, "Rui Silva", Position::GK, dec!(35.0), false),
        (20302, "Hector Bellerin", Position::RB, dec!(30.0), false),
        (20303, "Diego Llorente", Position::CB, dec!(40.0), false),
        (20304, "Natan", Position::CB, dec!(35.0), false),
        (20305, "Romain Perraud", Position::LB, dec!(30.0), false),
        (20306, "Marc Roca", Position::CDM, dec!(40.0), false),
        (20307, "Johnny Cardoso", Position::CDM, dec!(45.0), false),
        (20308, "Pablo Fornals", Position::RM, dec!(50.0), false),
        (20309, "Giovani Lo Celso", Position::CAM, dec!(55.0), false),
        (20310, "Abde Ezzalzouli", Position::LM, dec!(45.0), false),
        (20311, "Vitor Roque", Position::ST, dec!(60.0), false),
        // Bench
        (20312, "Adrian", Position::GK, dec!(10.0), false),
        (20313, "Marc Bartra", Position::CB, dec!(25.0), false),
        (20314, "Ricardo Rodriguez", Position::LB, dec!(25.0), false),
        (20315, "Youssouf Sabaly", Position::RB, dec!(25.0), false),
        (20316, "William Carvalho", Position::CM, dec!(30.0), false),
        (20317, "Assane Diao", Position::RW, dec!(40.0), false),
        (20318, "Iker Losada", Position::CAM, dec!(20.0), false),
        (20319, "Chimy Avila", Position::ST, dec!(35.0), false),
        (20320, "Cedric Bakambu", Position::ST, dec!(30.0), false),
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
