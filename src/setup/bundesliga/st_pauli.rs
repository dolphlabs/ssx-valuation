use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4017);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(290.00); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "FC St. Pauli".to_string());

    let players = vec![
        (40801, "Nikola Vasilj", Position::GK, dec!(25.0), false),
        (40802, "Eric Smith", Position::CB, dec!(40.0), false),
        (40803, "Hauke Wahl", Position::CB, dec!(25.0), false),
        (40804, "Karol Mets", Position::CB, dec!(20.0), false),
        (40805, "Manolis Saliakas", Position::RB, dec!(25.0), false),
        (40806, "Jackson Irvine", Position::CM, dec!(40.0), true),
        (40807, "Robert Wagner", Position::CM, dec!(30.0), false),
        (40808, "Leart Paqarada", Position::LB, dec!(30.0), false), // Is at Köln, let's use Philipp Treu
        (40809, "Philipp Treu", Position::LB, dec!(25.0), false),
        (40810, "Morgan Guilavogui", Position::RW, dec!(35.0), false),
        (40811, "Johannes Eggestein", Position::ST, dec!(35.0), false),
        // Bench
        (40812, "Ben Voll", Position::GK, dec!(10.0), false),
        (40813, "David Nemeth", Position::CB, dec!(20.0), false),
        (40814, "Adam Dzwigala", Position::CB, dec!(15.0), false),
        (40815, "Lars Ritzka", Position::LB, dec!(15.0), false),
        (40816, "Carlo Boukhalfa", Position::CM, dec!(20.0), false),
        (40817, "Connor Metcalfe", Position::CAM, dec!(30.0), false),
        (40818, "Oladapo Afolayan", Position::LW, dec!(40.0), false),
        (40819, "Elias Saad", Position::RW, dec!(40.0), false),
        (40820, "Andreas Albers", Position::ST, dec!(10.0), false),
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
