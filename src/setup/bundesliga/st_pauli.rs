use crate::{ValuationEngine, ClubState, PlayerValues, Position, ClubId, PlayerId};
use rust_decimal_macros::dec;

pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(4017);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(440.50); 
    
    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "FC St. Pauli".to_string());

    let players = vec![
        (40801, "Nikola Vasilj", Position::GK, dec!(27.70), dec!(1.129), false),
        (40802, "Eric Smith", Position::CB, dec!(58.27), dec!(1.045), false),
        (40803, "Hauke Wahl", Position::CB, dec!(37.09), dec!(1.025), false),
        (40804, "Karol Mets", Position::CB, dec!(21.49), dec!(0.975), false),
        (40805, "Manolis Saliakas", Position::RB, dec!(32.19), dec!(1.058), false),
        (40806, "Jackson Irvine", Position::CM, dec!(39.87), dec!(1.046), true),
        (40807, "Robert Wagner", Position::CM, dec!(30.00), dec!(1.0), false),
        (40808, "Leart Paqarada", Position::LB, dec!(30.00), dec!(1.0), false), // Is at Köln, let's use Philipp Treu
        (40809, "Philipp Treu", Position::LB, dec!(25.00), dec!(1.0), false),
        (40810, "Morgan Guilavogui", Position::RW, dec!(35.00), dec!(1.0), false),
        (40811, "Johannes Eggestein", Position::ST, dec!(35.00), dec!(1.0), false),
        // Bench
        (40812, "Ben Voll", Position::GK, dec!(10.00), dec!(1.0), false),
        (40813, "David Nemeth", Position::CB, dec!(20.00), dec!(1.0), false),
        (40814, "Adam Dzwigala", Position::CB, dec!(15.00), dec!(1.0), false),
        (40815, "Lars Ritzka", Position::LB, dec!(16.66), dec!(1.047), false),
        (40816, "Carlo Boukhalfa", Position::CM, dec!(20.00), dec!(1.0), false),
        (40817, "Connor Metcalfe", Position::CAM, dec!(31.56), dec!(1.112), false),
        (40818, "Oladapo Afolayan", Position::LW, dec!(42.78), dec!(1.038), false),
        (40819, "Elias Saad", Position::RW, dec!(40.00), dec!(1.0), false),
        (40820, "Andreas Albers", Position::ST, dec!(10.00), dec!(1.0), false),
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
