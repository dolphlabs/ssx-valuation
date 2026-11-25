use crate::{ValuationEngine, ClubState, ClubId};
use rust_decimal_macros::dec;

/// Promoted for 2026/27, so there's no top-flight season to have computed a
/// baseline from: every promoted club starts at the same 300, in line with
/// the earlier promoted seeds (270-340), and the squad is filled in by the
/// transfer poller as signings arrive.
pub fn seed(engine: &mut ValuationEngine) {
    let club_id = ClubId(5020);
    let mut club = ClubState::new(club_id);
    club.intrinsic_value = dec!(300.00);
    club.set_rival_factor(ClubId(5010), dec!(1.2)); // Stade Rennais FC (Breton derby)

    engine.club_states.insert(club_id, club);
    engine.names.insert(club_id.0, "FC Lorient".to_string());
}
