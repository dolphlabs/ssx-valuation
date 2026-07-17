use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3003);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1353.94); 
    
     club.set_rival_factor(ClubId(3001), dec!(1.4)); // Inter (Derby d'Italia)
    club.set_rival_factor(ClubId(3002), dec!(1.3)); // AC Milan
    club.set_rival_factor(ClubId(3009), dec!(1.5)); // Torino (Derby della Mole)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Juventus".to_string());

    let players = vec![
        (30101, "Michele Di Gregorio", Position::GK, dec!(48.72), dec!(1.001), false),
        (30102, "Nicolo Savona", Position::RB, dec!(30.00), dec!(1.0), false),
        (30103, "Federico Gatti", Position::CB, dec!(54.72), dec!(1.043), false),
        (30104, "Bremer", Position::CB, dec!(103.91), dec!(1.061), false),
        (30105, "Andrea Cambiaso", Position::LB, dec!(43.06), dec!(1.056), false),
        (30106, "Douglas Luiz", Position::CDM, dec!(70.00), dec!(1.0), false),
        (30107, "Manuel Locatelli", Position::CDM, dec!(59.94), dec!(1.122), false),
        (30108, "Teun Koopmeiners", Position::CAM, dec!(81.64), dec!(1.018), false),
        (30109, "Kenan Yildiz", Position::LW, dec!(80.49), dec!(1.0), false),
        (30110, "Nico Gonzalez", Position::RW, dec!(58.30), dec!(1.020), false),
        (30111, "Dusan Vlahovic", Position::ST, dec!(132.02), dec!(1.0), false),
        // Bench
        (30112, "Mattia Perin", Position::GK, dec!(15.09), dec!(1.010), false),
        (30113, "Pierre Kalulu", Position::CB, dec!(45.71), dec!(1.032), false),
        (30114, "Juan Cabal", Position::LB, dec!(33.02), dec!(1.036), false),
        (30115, "Danilo", Position::CB, dec!(35.00), dec!(1.0), true),
        (30116, "Khephren Thuram", Position::CM, dec!(75.50), dec!(1.069), false),
        (30117, "Weston McKennie", Position::CM, dec!(51.83), dec!(1.042), false),
        (30118, "Timothy Weah", Position::RW, dec!(40.00), dec!(1.0), false),
        (30119, "Francisco Conceicao", Position::RW, dec!(62.43), dec!(1.058), false),
        (30120, "Arkadiusz Milik", Position::ST, dec!(30.03), dec!(1.002), false),
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
