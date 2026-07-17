//! Live injury-news wire format. Deliberately separate from `transfers.rs`'s
//! `TransferFact`/`TransferEvent` split even though the shape rhymes: an
//! injury never creates/deactivates a player or changes their club, it's
//! purely a one-time proportional haircut to `intrinsic_value` (see
//! `ValuationEngine::process_event`'s existing `MatchEvent::Injury` arm,
//! unused until this module's subscriber started calling it). Published by
//! the standalone `ssx-injuries` process (same isolation principle as
//! `ssx-transfers`/`ssx-live-oracle` - if it or API-Football is down, only
//! injury-driven price movement pauses, nothing else is affected) onto
//! `ssx:injury_broadcast`.
use crate::ClubId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjuryFact {
    pub api_player_id: u32,
    pub player_name: String,
    /// The club that reported the injury - used only to scope the
    /// subscriber's name-match (`PlayerIndex::find`), never resolved here.
    pub club_id: ClubId,
    /// 1-100, how much of the player's `intrinsic_value` to cut. See
    /// `ssx-injuries::classify::classify_severity` for how this is derived
    /// from API-Football's free-text `reason` field - a coarse, best-effort
    /// estimate, not a medical assessment.
    pub severity: u32,
}
