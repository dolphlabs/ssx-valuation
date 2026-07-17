use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5018);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(331.28); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "AS Saint-Etienne".to_string());

    let players = vec![
        (50851, "Gautier Larsonneur", Position::GK, dec!(30.03), dec!(1.006), false),
        (50852, "Dennis Appiah", Position::RB, dec!(20.03), dec!(1.032), false),
        (50853, "Yunis Abdelhamid", Position::CB, dec!(25.00), dec!(1.0), true),
        (50854, "Dylan Batubinsika", Position::CB, dec!(30.00), dec!(1.0), false),
        (50855, "Pierre Cornud", Position::LB, dec!(30.00), dec!(1.0), false),
        (50856, "Pierre Ekwah", Position::CDM, dec!(45.00), dec!(1.0), false),
        (50857, "Aimen Moueffek", Position::CM, dec!(35.00), dec!(1.010), false),
        (50858, "Benold Old", Position::CAM, dec!(25.00), dec!(1.0), false),
        (50859, "Mathieu Cafaro", Position::RW, dec!(30.00), dec!(1.0), false),
        (50860, "Ibrahim Sissoko", Position::ST, dec!(35.00), dec!(1.0), false),
        (50861, "Lucas Stassin", Position::ST, dec!(45.05), dec!(0.998), false),
        // Bench
        (50862, "Brice Maubleu", Position::GK, dec!(15.00), dec!(1.0), false),
        (50863, "Mickael Nade", Position::CB, dec!(25.00), dec!(0.999), false),
        (50864, "Yvann Maçon", Position::RB, dec!(25.00), dec!(1.0), false),
        (50865, "Florian Tardieu", Position::CDM, dec!(20.03), dec!(1.016), false),
        (50866, "Louis Mouton", Position::CM, dec!(15.00), dec!(1.0), false),
        (50867, "Mathis Amougou", Position::CM, dec!(40.00), dec!(1.0), false),
        (50868, "Zuriko Davitashvili", Position::LW, dec!(52.62), dec!(1.046), false),
        (50869, "Augustine Boakye", Position::RW, dec!(30.09), dec!(1.015), false),
        (50870, "Ibrahima Wadji", Position::ST, dec!(25.00), dec!(1.0), false),
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
