use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3007);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(723.31); 
    
    club.set_rival_factor(ClubId(3006), dec!(1.5)); // Roma (Derby della Capitale)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Lazio".to_string());

    let players = vec![
        (30301, "Ivan Provedel", Position::GK, dec!(53.26), dec!(1.011), false),
        (30302, "Manuel Lazzari", Position::RB, dec!(31.69), dec!(1.046), false),
        (30303, "Mario Gila", Position::CB, dec!(35.08), dec!(1.062), false),
        (30304, "Alessio Romagnoli", Position::CB, dec!(44.17), dec!(0.671), false),
        (30305, "Nuno Tavares", Position::LB, dec!(50.42), dec!(1.049), false),
        (30306, "Matteo Guendouzi", Position::CM, dec!(50.70), dec!(1.042), false),
        (30307, "Nicolo Rovella", Position::CDM, dec!(53.41), dec!(0.921), false),
        (30308, "Boulaye Dia", Position::CAM, dec!(62.71), dec!(1.029), false),
        (30309, "Gustav Isaksen", Position::RW, dec!(52.92), dec!(1.017), false),
        (30310, "Mattia Zaccagni", Position::LW, dec!(70.03), dec!(1.046), true),
        (30311, "Taty Castellanos", Position::ST, dec!(72.29), dec!(1.039), false),
        // Bench
        (30312, "Christos Mandas", Position::GK, dec!(20.00), dec!(1.0), false),
        (30313, "Samuel Gigot", Position::CB, dec!(30.00), dec!(1.0), false),
        (30314, "Adam Marusic", Position::RB, dec!(25.00), dec!(1.0), false),
        (30315, "Luca Pellegrini", Position::LB, dec!(24.77), dec!(0.917), false),
        (30316, "Matias Vecino", Position::CM, dec!(21.44), dec!(1.047), false),
        (30317, "Fisayo Dele-Bashiru", Position::CAM, dec!(32.51), dec!(1.026), false),
        (30318, "Tijjani Noslin", Position::RW, dec!(40.07), dec!(1.015), false),
        (30319, "Loum Tchaouna", Position::RW, dec!(35.00), dec!(1.0), false),
        (30320, "Gaetano Castrovilli", Position::CM, dec!(25.00), dec!(1.0), false),
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
