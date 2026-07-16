use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3001);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1649.26); // Serie A champions
    
    club.set_rival_factor(ClubId(3002), dec!(1.5)); // AC Milan (Derby della Madonnina)
    club.set_rival_factor(ClubId(3003), dec!(1.4)); // Juventus (Derby d'Italia)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Inter Milan".to_string());

    let players = vec![
        (30001, "Yann Sommer", Position::GK, dec!(67.14), dec!(1.043), false),
        (30002, "Benjamin Pavard", Position::CB, dec!(71.82), dec!(1.033), false),
        (30003, "Francesco Acerbi", Position::CB, dec!(43.73), dec!(1.019), false),
        (30004, "Alessandro Bastoni", Position::CB, dec!(96.86), dec!(1.054), false),
        (30005, "Denzel Dumfries", Position::RB, dec!(68.90), dec!(1.045), false),
        (30006, "Hakan Calhanoglu", Position::CDM, dec!(121.25), dec!(1.050), false),
        (30007, "Nicolo Barella", Position::CM, dec!(109.94), dec!(1.055), false),
        (30008, "Henrikh Mkhitaryan", Position::CM, dec!(52.39), dec!(0.967), false),
        (30009, "Federico Dimarco", Position::LB, dec!(99.15), dec!(1.082), false),
        (30010, "Marcus Thuram", Position::ST, dec!(173.19), dec!(1.100), false),
        (30011, "Lautaro Martinez", Position::ST, dec!(138.85), dec!(1.004), true),
        // Bench
        (30012, "Josep Martinez", Position::GK, dec!(30.95), dec!(1.016), false),
        (30013, "Stefan de Vrij", Position::CB, dec!(42.14), dec!(1.073), false),
        (30014, "Yann Bisseck", Position::CB, dec!(57.53), dec!(1.078), false),
        (30015, "Matteo Darmian", Position::RB, dec!(30.15), dec!(1.033), false),
        (30016, "Piotr Zielinski", Position::CM, dec!(67.07), dec!(1.053), false),
        (30017, "Davide Frattesi", Position::CM, dec!(53.60), dec!(1.034), false),
        (30018, "Carlos Augusto", Position::LB, dec!(43.46), dec!(1.049), false),
        (30019, "Mehdi Taremi", Position::ST, dec!(45.00), dec!(1.0), false),
        (30020, "Marko Arnautovic", Position::ST, dec!(25.00), dec!(1.0), false),
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
