use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5016);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(290.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "AJ Auxerre".to_string());

    let players = vec![
        (50751, "Donovan Leon", Position::GK, dec!(20.0), false),
        (50752, "Paul Joly", Position::RB, dec!(30.0), false),
        (50753, "Jubal", Position::CB, dec!(25.0), true),
        (50754, "Gabriel Osho", Position::CB, dec!(25.0), false),
        (50755, "Gideon Mensah", Position::LB, dec!(25.0), false),
        (50756, "Elisha Owusu", Position::CDM, dec!(30.0), false),
        (50757, "Kevin Danois", Position::CM, dec!(25.0), false),
        (50758, "Lassine Sinayoko", Position::RW, dec!(35.0), false),
        (50759, "Gaetan Perrin", Position::CAM, dec!(30.0), false),
        (50760, "Ado Onaiwu", Position::LW, dec!(25.0), false),
        (50761, "Theoson Siebatcheu", Position::ST, dec!(35.0), false), // Is at Union Berlin, let's use Bair
        (50762, "Theo Bair", Position::ST, dec!(30.0), false),
        // Bench
        (50763, "Theo De Percin", Position::GK, dec!(10.0), false),
        (50764, "Sinaly Diomande", Position::CB, dec!(30.0), false),
        (50765, "Ki-Jana Hoever", Position::RB, dec!(35.0), false),
        (50766, "Clement Akpa", Position::CB, dec!(20.0), false),
        (50767, "Rayan Raveloson", Position::CM, dec!(30.0), false),
        (50768, "Hamidou Diallo", Position::LW, dec!(25.0), false),
        (50769, "Florian Aye", Position::ST, dec!(25.0), false),
        (50770, "Eros Maddy", Position::RW, dec!(15.0), false),
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
