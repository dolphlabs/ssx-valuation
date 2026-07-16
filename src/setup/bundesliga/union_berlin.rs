use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4015);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(629.56); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "Union Berlin".to_string());

    let players = vec![
        (40701, "Frederik Ronnow", Position::GK, dec!(34.82), dec!(0.974), false),
        (40702, "Christopher Trimmel", Position::RB, dec!(16.81), dec!(1.090), true),
        (40703, "Danilho Doekhi", Position::CB, dec!(67.62), dec!(1.042), false),
        (40704, "Kevin Vogt", Position::CB, dec!(30.00), dec!(1.0), false),
        (40705, "Diogo Leite", Position::CB, dec!(44.13), dec!(1.106), false),
        (40706, "Tom Rothe", Position::LB, dec!(48.38), dec!(1.105), false),
        (40707, "Lucas Tousart", Position::CM, dec!(35.00), dec!(1.0), false),
        (40708, "Andras Schafer", Position::CM, dec!(35.70), dec!(1.207), false),
        (40709, "Woo-yeong Jeong", Position::CAM, dec!(33.57), dec!(1.103), false),
        (40710, "Yorbe Vertessen", Position::LW, dec!(40.00), dec!(1.0), false),
        (40711, "Jordan Siebatcheu", Position::ST, dec!(40.00), dec!(1.0), false),
        // Bench
        (40712, "Alexander Schwolow", Position::GK, dec!(10.00), dec!(1.0), false),
        (40713, "Leopold Querfeld", Position::CB, dec!(38.85), dec!(1.104), false),
        (40714, "Janik Haberer", Position::CM, dec!(25.60), dec!(1.040), false),
        (40715, "Aljoscha Kemlein", Position::CM, dec!(27.86), dec!(1.072), false),
        (40716, "Laszlo Benes", Position::CAM, dec!(40.00), dec!(1.0), false),
        (40717, "Benedict Hollerbach", Position::LW, dec!(35.00), dec!(1.0), false),
        (40718, "Tim Skarke", Position::RW, dec!(35.44), dec!(1.034), false),
        (40719, "Kevin Volland", Position::ST, dec!(40.00), dec!(1.0), false),
        (40720, "Ivan Prtajin", Position::ST, dec!(20.00), dec!(1.0), false),
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
