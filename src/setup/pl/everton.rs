use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(16);
    let mut club = ClubState::new(club_id);
    engine.names.insert(club_id.0, "Everton".to_string());
    
    club.intrinsic_value = dec!(400.00);
    
    // Rivals: Liverpool (5)
    club.set_rival_factor(ClubId(5), dec!(1.7));

    engine.club_states.insert(club_id, club);

    // Players
    let players = vec![
        (1751, "Jordan Pickford", Position::GK, dec!(55.0), false),
        (1752, "James Tarkowski", Position::CB, dec!(50.0), true),
        (1753, "Dwight McNeil", Position::LW, dec!(55.0), false),
        (1754, "Dominic Calvert-Lewin", Position::ST, dec!(55.0), false),
        (1755, "Abdoulaye Doucoure", Position::CM, dec!(45.0), false),
        (1756, "Idrissa Gueye", Position::CDM, dec!(40.0), false),
        (1757, "Vitaliy Mykolenko", Position::LB, dec!(45.0), false),
        (1758, "Jarrad Branthwaite", Position::CB, dec!(75.0), false),
        (1759, "Iliman Ndiaye", Position::CAM, dec!(55.0), false),
        (1760, "Jack Harrison", Position::RW, dec!(45.0), false),
        (1761, "Tim Iroegbunam", Position::CM, dec!(35.0), false),
        (1762, "Joao Virginia", Position::GK, dec!(15.0), false),
        (1763, "Michael Keane", Position::CB, dec!(30.0), false),
        (1764, "Jake O'Brien", Position::CB, dec!(40.0), false),
        (1765, "Nathan Patterson", Position::RB, dec!(35.0), false),
        (1766, "Ashley Young", Position::RB, dec!(10.0), false),
        (1767, "Orel Mangala", Position::CDM, dec!(45.0), false),
        (1768, "James Garner", Position::CM, dec!(50.0), false),
        (1769, "Beto", Position::ST, dec!(45.0), false),
        (1770, "Youssef Chermiti", Position::ST, dec!(35.0), false),
    ];

    for (id, name, pos, val, captain) in players {
        engine.names.insert(id, name.to_string());
        engine.player_states.insert(PlayerId(id), PlayerValues {
            team_id: club_id,
            intrinsic_value: val,
            form_weight: dec!(1.0),
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![val; 5],
            position: pos,
            is_captain: captain,
        });
    }
}
