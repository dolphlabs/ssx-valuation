use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(2006);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(820.98); 
    
    club.set_rival_factor(ClubId(2005), dec!(1.4)); // Athletic Club (Basque Derby)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Real Sociedad".to_string());

    let players = vec![
        (20251, "Alex Remiro", Position::GK, dec!(57.32), dec!(0.875), false),
        (20252, "Jon Aramburu", Position::RB, dec!(31.54), dec!(1.075), false),
        (20253, "Igor Zubeldia", Position::CB, dec!(42.32), dec!(1.065), false),
        (20254, "Nayef Aguerd", Position::CB, dec!(45.00), dec!(1.0), false),
        (20255, "Aihen Munoz", Position::LB, dec!(32.54), dec!(1.040), false),
        (20256, "Martin Zubimendi", Position::CDM, dec!(75.00), dec!(1.0), false),
        (20257, "Luka Sucic", Position::CM, dec!(50.77), dec!(1.0), false),
        (20258, "Brais Mendez", Position::CAM, dec!(66.96), dec!(1.080), false),
        (20259, "Takefusa Kubo", Position::RW, dec!(106.14), dec!(1.026), false),
        (20260, "Sheraldo Becker", Position::LW, dec!(40.00), dec!(1.0), false),
        (20261, "Mikel Oyarzabal", Position::ST, dec!(124.10), dec!(1.046), true),
        // Bench
        (20262, "Unai Marrero", Position::GK, dec!(10.02), dec!(1.141), false),
        (20263, "Aritz Elustondo", Position::CB, dec!(32.57), dec!(1.038), false),
        (20264, "Alvaro Odriozola", Position::RB, dec!(30.89), dec!(1.046), false),
        (20265, "Javi Lopez", Position::LB, dec!(35.00), dec!(1.0), false),
        (20266, "Sergio Gomez", Position::CM, dec!(39.92), dec!(1.063), false),
        (20267, "Turrientes", Position::CM, dec!(28.20), dec!(1.058), false),
        (20268, "Sadiq Umar", Position::ST, dec!(35.00), dec!(1.0), false),
        (20269, "Orri Oskarsson", Position::ST, dec!(52.21), dec!(1.0), false),
        (20270, "Barrenetxea", Position::LW, dec!(59.16), dec!(0.942), false),
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
