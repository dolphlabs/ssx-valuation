use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4018);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(270.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Holstein Kiel".to_string());

    let players = vec![
        (40851, "Timon Weiner", Position::GK, dec!(20.0), dec!(1.0), false),
        (40852, "Timo Becker", Position::RB, dec!(25.0), dec!(1.0), false),
        (40853, "Carl Johansson", Position::CB, dec!(20.0), dec!(1.0), false),
        (40854, "Patrick Erras", Position::CB, dec!(20.0), dec!(1.0), false),
        (40855, "Tyger Puchacz", Position::LB, dec!(30.0), dec!(1.0), false),
        (40856, "Magnus Knudsen", Position::CM, dec!(25.0), dec!(1.0), false),
        (40857, "Lewis Holtby", Position::CM, dec!(25.0), dec!(1.0), true),
        (40858, "Finn Porath", Position::CAM, dec!(20.0), dec!(1.0), false),
        (40859, "Alexander Bernhardsson", Position::RW, dec!(30.0), dec!(1.0), false),
        (40860, "Shuto Machino", Position::ST, dec!(35.0), dec!(1.0), false),
        (40861, "Fiete Arp", Position::ST, dec!(25.0), dec!(1.0), false),
        // Bench
        (40862, "Thomas Dähne", Position::GK, dec!(10.0), dec!(1.0), false),
        (40863, "Colin Kleine-Bekel", Position::CB, dec!(30.0), dec!(1.0), false),
        (40864, "Max Geschwill", Position::CB, dec!(20.0), dec!(1.0), false),
        (40865, "Lasse Rosenboom", Position::RB, dec!(10.0), dec!(1.0), false),
        (40866, "Marvin Schulz", Position::CM, dec!(20.0), dec!(1.0), false),
        (40867, "Armin Gigovic", Position::CM, dec!(30.0), dec!(1.0), false),
        (40868, "Steven Skrzybski", Position::LW, dec!(30.0), dec!(1.0), false),
        (40869, "Benedikt Pichler", Position::ST, dec!(30.0), dec!(1.0), false),
        (40870, "Philipp Sander", Position::CM, dec!(30.0), dec!(1.0), false),
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
