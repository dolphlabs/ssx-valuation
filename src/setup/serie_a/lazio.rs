use crate::{ValuationEngine, ClubState, PlayerValues, Position};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = 3007;
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(580.00); 
    
    club.set_rival_factor(3006, dec!(1.5)); // Roma (Derby della Capitale)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id, "Lazio".to_string());

    let players = vec![
        (30301, "Ivan Provedel", Position::GK, dec!(45.0), false),
        (30302, "Manuel Lazzari", Position::RB, dec!(30.0), false),
        (30303, "Mario Gila", Position::CB, dec!(35.0), false),
        (30304, "Alessio Romagnoli", Position::CB, dec!(45.0), false),
        (30305, "Nuno Tavares", Position::LB, dec!(45.0), false),
        (30306, "Matteo Guendouzi", Position::CM, dec!(55.0), false),
        (30307, "Nicolo Rovella", Position::CDM, dec!(40.0), false),
        (30308, "Boulaye Dia", Position::CAM, dec!(50.0), false),
        (30309, "Gustav Isaksen", Position::RW, dec!(35.0), false),
        (30310, "Mattia Zaccagni", Position::LW, dec!(65.0), true),
        (30311, "Taty Castellanos", Position::ST, dec!(55.0), false),
        // Bench
        (30312, "Christos Mandas", Position::GK, dec!(20.0), false),
        (30313, "Samuel Gigot", Position::CB, dec!(30.0), false),
        (30314, "Adam Marusic", Position::RB, dec!(25.0), false),
        (30315, "Luca Pellegrini", Position::LB, dec!(25.0), false),
        (30316, "Matias Vecino", Position::CM, dec!(20.0), false),
        (30317, "Fisayo Dele-Bashiru", Position::CAM, dec!(30.0), false),
        (30318, "Tijjani Noslin", Position::RW, dec!(40.0), false),
        (30319, "Loum Tchaouna", Position::RW, dec!(35.0), false),
        (30320, "Gaetano Castrovilli", Position::CM, dec!(25.0), false),
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
