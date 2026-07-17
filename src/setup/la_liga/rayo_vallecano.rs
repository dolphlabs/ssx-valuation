use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2017);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(395.39); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Rayo Vallecano".to_string());

    let players = vec![
        (20801, "Augusto Batalla", Position::GK, dec!(24.42), dec!(1.095), false),
        (20802, "Andrei Ratiu", Position::RB, dec!(35.00), dec!(1.0), false),
        (20803, "Abdul Mumin", Position::CB, dec!(30.12), dec!(1.029), false),
        (20804, "Florian Lejeune", Position::CB, dec!(36.51), dec!(1.110), false),
        (20805, "Pep Chavarria", Position::LB, dec!(22.01), dec!(1.079), false),
        (20806, "Oscar Valentin", Position::CDM, dec!(32.49), dec!(1.096), true),
        (20807, "Unai Lopez", Position::CM, dec!(20.13), dec!(1.082), false),
        (20808, "Isi Palazon", Position::RW, dec!(55.95), dec!(0.608), false),
        (20809, "James Rodriguez", Position::CAM, dec!(55.00), dec!(1.0), false),
        (20810, "Alvaro Garcia", Position::LW, dec!(58.04), dec!(1.020), false),
        (20811, "Sergio Camello", Position::ST, dec!(44.91), dec!(1.170), false),
        // Bench
        (20812, "Dani Cardenas", Position::GK, dec!(15.27), dec!(1.203), false),
        (20813, "Aridane Hernandez", Position::CB, dec!(20.00), dec!(1.0), false),
        (20814, "Ivan Balliu", Position::RB, dec!(25.46), dec!(1.069), false),
        (20815, "Pathé Ciss", Position::CDM, dec!(26.13), dec!(1.056), false),
        (20816, "Gerard Gumbau", Position::CM, dec!(24.08), dec!(1.043), false),
        (20817, "Oscar Trejo", Position::CAM, dec!(17.12), dec!(1.123), false),
        (20818, "Randy Nteka", Position::ST, dec!(28.16), dec!(1.147), false),
        (20819, "Raul de Tomas", Position::ST, dec!(40.00), dec!(1.0), false),
        (20820, "Adri Embarba", Position::RW, dec!(30.00), dec!(1.0), false),
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
