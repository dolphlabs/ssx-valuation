use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2006);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(520.00); 
    
    club.set_rival_factor(ClubId(2005), dec!(1.4)); // Athletic Club (Basque Derby)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Real Sociedad".to_string());

    let players = vec![
        (20251, "Alex Remiro", Position::GK, dec!(55.0), false),
        (20252, "Jon Aramburu", Position::RB, dec!(30.0), false),
        (20253, "Igor Zubeldia", Position::CB, dec!(45.0), false),
        (20254, "Nayef Aguerd", Position::CB, dec!(45.0), false),
        (20255, "Aihen Munoz", Position::LB, dec!(30.0), false),
        (20256, "Martin Zubimendi", Position::CDM, dec!(75.0), false),
        (20257, "Luka Sucic", Position::CM, dec!(45.0), false),
        (20258, "Brais Mendez", Position::CAM, dec!(60.0), false),
        (20259, "Takefusa Kubo", Position::RW, dec!(75.0), false),
        (20260, "Sheraldo Becker", Position::LW, dec!(40.0), false),
        (20261, "Mikel Oyarzabal", Position::ST, dec!(70.0), true),
        // Bench
        (20262, "Unai Marrero", Position::GK, dec!(10.0), false),
        (20263, "Aritz Elustondo", Position::CB, dec!(30.0), false),
        (20264, "Alvaro Odriozola", Position::RB, dec!(25.0), false),
        (20265, "Javi Lopez", Position::LB, dec!(35.0), false),
        (20266, "Sergio Gomez", Position::CM, dec!(40.0), false),
        (20267, "Turrientes", Position::CM, dec!(30.0), false),
        (20268, "Sadiq Umar", Position::ST, dec!(35.0), false),
        (20269, "Orri Oskarsson", Position::ST, dec!(35.0), false),
        (20270, "Barrenetxea", Position::LW, dec!(40.0), false),
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
