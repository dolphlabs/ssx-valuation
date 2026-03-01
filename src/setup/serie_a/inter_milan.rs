use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(3001);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(1050.00); // Serie A champions
    
    club.set_rival_factor(ClubId(3002), dec!(1.5)); // AC Milan (Derby della Madonnina)
    club.set_rival_factor(ClubId(3003), dec!(1.4)); // Juventus (Derby d'Italia)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Inter Milan".to_string());

    let players = vec![
        (30001, "Yann Sommer", Position::GK, dec!(60.0), false),
        (30002, "Benjamin Pavard", Position::CB, dec!(65.0), false),
        (30003, "Francesco Acerbi", Position::CB, dec!(40.0), false),
        (30004, "Alessandro Bastoni", Position::CB, dec!(80.0), false),
        (30005, "Denzel Dumfries", Position::RB, dec!(55.0), false),
        (30006, "Hakan Calhanoglu", Position::CDM, dec!(85.0), false),
        (30007, "Nicolo Barella", Position::CM, dec!(95.0), false),
        (30008, "Henrikh Mkhitaryan", Position::CM, dec!(45.0), false),
        (30009, "Federico Dimarco", Position::LB, dec!(70.0), false),
        (30010, "Marcus Thuram", Position::ST, dec!(85.0), false),
        (30011, "Lautaro Martinez", Position::ST, dec!(120.0), true),
        // Bench
        (30012, "Josep Martinez", Position::GK, dec!(30.0), false),
        (30013, "Stefan de Vrij", Position::CB, dec!(40.0), false),
        (30014, "Yann Bisseck", Position::CB, dec!(45.0), false),
        (30015, "Matteo Darmian", Position::RB, dec!(30.0), false),
        (30016, "Piotr Zielinski", Position::CM, dec!(55.0), false),
        (30017, "Davide Frattesi", Position::CM, dec!(50.0), false),
        (30018, "Carlos Augusto", Position::LB, dec!(40.0), false),
        (30019, "Mehdi Taremi", Position::ST, dec!(45.0), false),
        (30020, "Marko Arnautovic", Position::ST, dec!(25.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.player_states.insert(PlayerId(id), PlayerValues {
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
            c.player_ids.push(PlayerId(id));
        }
    }
}
