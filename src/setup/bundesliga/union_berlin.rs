use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4015);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(360.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Union Berlin".to_string());

    let players = vec![
        (40701, "Frederik Ronnow", Position::GK, dec!(35.0), false),
        (40702, "Christopher Trimmel", Position::RB, dec!(15.0), true),
        (40703, "Danilho Doekhi", Position::CB, dec!(45.0), false),
        (40704, "Kevin Vogt", Position::CB, dec!(30.0), false),
        (40705, "Diogo Leite", Position::CB, dec!(40.0), false),
        (40706, "Tom Rothe", Position::LB, dec!(40.0), false),
        (40707, "Lucas Tousart", Position::CM, dec!(35.0), false),
        (40708, "Andras Schafer", Position::CM, dec!(35.0), false),
        (40709, "Woo-yeong Jeong", Position::CAM, dec!(30.0), false),
        (40710, "Yorbe Vertessen", Position::LW, dec!(40.0), false),
        (40711, "Jordan Siebatcheu", Position::ST, dec!(40.0), false),
        // Bench
        (40712, "Alexander Schwolow", Position::GK, dec!(10.0), false),
        (40713, "Leopold Querfeld", Position::CB, dec!(30.0), false),
        (40714, "Janik Haberer", Position::CM, dec!(25.0), false),
        (40715, "Aljoscha Kemlein", Position::CM, dec!(25.0), false),
        (40716, "Laszlo Benes", Position::CAM, dec!(40.0), false),
        (40717, "Benedict Hollerbach", Position::LW, dec!(35.0), false),
        (40718, "Tim Skarke", Position::RW, dec!(30.0), false),
        (40719, "Kevin Volland", Position::ST, dec!(40.0), false),
        (40720, "Ivan Prtajin", Position::ST, dec!(20.0), false),
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
