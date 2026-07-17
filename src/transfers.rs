//! Confirmed-transfer processing. Deliberately not a `MatchEvent` variant:
//! a transfer doesn't happen *in* a match (no opponent/minute), and it can
//! create or deactivate a player entirely, which nothing in `process_event`
//! does. Resolving *which* `PlayerId` (if any) a transfer refers to is the
//! caller's job (it needs a persisted api-player-id map, which lives in
//! Redis on the `ssx-node` side, not here) - this module only knows what to
//! do once that resolution has already happened.
use crate::{ClubId, EngineUpdate, PlayerId, PlayerValues, Position, ValuationEngine};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};

/// First id handed out to a player created by a transfer arriving from
/// outside our tracked universe. Every seed file's ids sit well under
/// 100,000 (highest today is ~50,020 for Ligue 1) - a wide gap so this can
/// never collide with the static roster, even if it grows.
pub const DYNAMIC_PLAYER_ID_BASE: u32 = 9_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionBucket {
    Gk,
    Def,
    Mid,
    Att,
}

impl PositionBucket {
    /// API-Football only ever reports one of these four coarse buckets for
    /// a player, never our finer CB/LB/RB/etc split - this picks one
    /// representative variant per bucket for a brand-new signing. An
    /// explicit approximation, not a claim of positional precision; nothing
    /// downstream depends on exact position for a dynamically-created
    /// player beyond `base_weight()`, which only varies mildly within a
    /// bucket anyway.
    pub fn representative_position(self) -> Position {
        match self {
            PositionBucket::Gk => Position::GK,
            PositionBucket::Def => Position::CB,
            PositionBucket::Mid => Position::CM,
            PositionBucket::Att => Position::ST,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TransferKind {
    Permanent { fee: Option<Decimal> },
    Loan,
    Free,
}

/// Parses API-Football's `type` field on a transfer record: a money amount
/// ("€45M", "€800K"), "Free" (Bosman-style), "Loan", or "N/A" when the
/// provider doesn't have the fee.
pub fn parse_transfer_kind(raw: &str) -> TransferKind {
    let trimmed = raw.trim();
    if trimmed.eq_ignore_ascii_case("loan") {
        return TransferKind::Loan;
    }
    if trimmed.eq_ignore_ascii_case("free") || trimmed.eq_ignore_ascii_case("free agent") {
        return TransferKind::Free;
    }
    if trimmed.eq_ignore_ascii_case("n/a") || trimmed.eq_ignore_ascii_case("transfer") || trimmed.is_empty() {
        return TransferKind::Permanent { fee: None };
    }
    TransferKind::Permanent { fee: parse_fee_in_value_units(trimmed) }
}

/// Converts a disclosed fee straight into our internal valuation-unit
/// scale: "€45M" -> 45, "€800K" -> 0.8. Chosen because it lines up with the
/// real-data backfill's own player values (e.g. Havertz backfilled to ~103
/// on a real transfer fee of ~€75M, Saka to ~210 against a real market value
/// in the ~€120-150M range) - close enough that a real fee reads as a
/// same-scale re-rating signal rather than needing its own conversion
/// constant tuned separately.
fn parse_fee_in_value_units(raw: &str) -> Option<Decimal> {
    let last_char = raw.chars().last()?;
    let (numeric_str, multiplier): (&str, Decimal) = if last_char.eq_ignore_ascii_case(&'m') {
        (&raw[..raw.len() - last_char.len_utf8()], dec!(1))
    } else if last_char.eq_ignore_ascii_case(&'k') {
        (&raw[..raw.len() - last_char.len_utf8()], dec!(0.001))
    } else {
        (raw, dec!(0.000001)) // bare euro amount - rare, but handle it rather than silently dropping the fee
    };
    let digits: String = numeric_str.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse::<Decimal>().ok().map(|n| n * multiplier)
}

/// A confirmed, real-world transfer. `from`/`to` are `None` when that side
/// isn't one of our tracked clubs (a move to/from outside the five tracked
/// leagues). `existing_player_id` is `Some` when the caller has already
/// resolved this player against our records (active or previously
/// deactivated) - via a persisted api-player-id map, or, on first sighting
/// of an already-seeded player, a name match scoped to their `from` club.
pub struct TransferEvent {
    pub existing_player_id: Option<PlayerId>,
    pub player_name: String,
    pub position_bucket: PositionBucket,
    pub from: Option<ClubId>,
    pub to: Option<ClubId>,
    pub kind: TransferKind,
}

/// The wire format published by the standalone `ssx-transfers` poller to
/// `ssx:transfer_broadcast` and consumed by `ssx-node`'s subscriber - same
/// role as `EngineUpdate`/`MatchEvent` play for the oracle feed. Everything
/// here is derivable from API-Football alone (team-id mapping, position
/// bucket, fee parsing); resolving it against *our* records (does this
/// player already exist, are they returning) needs live engine access, so
/// it deliberately isn't done here - that's `TransferEvent`'s job, built by
/// the subscriber after resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferFact {
    pub api_player_id: u32,
    pub player_name: String,
    pub position_bucket: PositionBucket,
    pub from: Option<ClubId>,
    pub to: Option<ClubId>,
    pub kind: TransferKind,
}

const PLAYER_REBLEND_RETAIN: Decimal = dec!(0.5);
const PLAYER_REBLEND_ADOPT: Decimal = dec!(0.5);
const LOAN_DAMPING: Decimal = dec!(0.4);
const NEW_SIGNING_DEFAULT_VALUE: Decimal = dec!(20.0); // fallback for a brand-new player with no fee signal (undisclosed/free/loan) - roughly a fringe-squad-player level in the backfilled data
const CLUB_FREE_OR_UNKNOWN_IMPACT: Decimal = dec!(5.0); // no fee signal - same order of magnitude as a card, not a goal

impl ValuationEngine {
    /// Applies a confirmed transfer. Club-level effects fire independent of
    /// whether the player side resolves, mirroring `process_event`'s
    /// Goal/Card philosophy. Returns the `PlayerId` this transfer ended up
    /// referring to (existing, newly created, or `PlayerId(0)` if neither
    /// side was anything we track), so the caller can persist an
    /// api-player-id mapping for next time.
    pub fn process_transfer(&self, event: TransferEvent, current_ts: u64) -> PlayerId {
        let fee = match event.kind {
            TransferKind::Permanent { fee } => fee,
            _ => None,
        };
        let damping = if event.kind == TransferKind::Loan { LOAN_DAMPING } else { dec!(1.0) };
        let club_delta = fee.map(|f| f * damping).unwrap_or(CLUB_FREE_OR_UNKNOWN_IMPACT * damping);

        if let Some(to_club) = event.to {
            if let Some(mut club) = self.club_states.get_mut(&to_club) {
                club.intrinsic_value += club_delta;
                club.last_match_update = current_ts;
                self.notify(EngineUpdate::Club { id: to_club, state: club.clone() });
            }
        }
        if let Some(from_club) = event.from {
            if let Some(mut club) = self.club_states.get_mut(&from_club) {
                club.intrinsic_value -= club_delta;
                club.last_match_update = current_ts;
                self.notify(EngineUpdate::Club { id: from_club, state: club.clone() });
            }
        }

        match (event.existing_player_id, event.to) {
            // Known player (possibly a returning, previously-deactivated one) moving to a tracked club.
            (Some(pid), Some(to_club)) => {
                if let Some(mut player) = self.player_states.get_mut(&pid) {
                    player.team_id = to_club;
                    player.active = true;
                    player.performance_history.clear();
                    if let Some(estimate) = fee {
                        player.intrinsic_value =
                            player.intrinsic_value * PLAYER_REBLEND_RETAIN + estimate * PLAYER_REBLEND_ADOPT;
                    }
                    let value = player.intrinsic_value;
                    player.performance_history.push(value);
                    self.notify(EngineUpdate::Player { id: pid, state: player.clone() });
                }
                pid
            }
            // Known player leaving our tracked universe entirely - soft delete, never a hard removal.
            (Some(pid), None) => {
                if let Some(mut player) = self.player_states.get_mut(&pid) {
                    player.active = false;
                    self.notify(EngineUpdate::Player { id: pid, state: player.clone() });
                }
                pid
            }
            // Brand-new arrival: not previously tracked, moving to one of our clubs.
            (None, Some(to_club)) => {
                let new_id = self.allocate_dynamic_player_id();
                let initial_value = fee.unwrap_or(NEW_SIGNING_DEFAULT_VALUE);
                self.names.insert(new_id.0, event.player_name.clone());
                let player = PlayerValues {
                    team_id: to_club,
                    intrinsic_value: initial_value,
                    form_weight: dec!(1.0),
                    sentiment_score: dec!(1.0),
                    volatility_factor: dec!(1.0),
                    performance_history: vec![initial_value],
                    position: event.position_bucket.representative_position(),
                    is_captain: false,
                    active: true,
                };
                self.player_states.insert(new_id, player.clone());
                self.notify(EngineUpdate::Player { id: new_id, state: player });
                new_id
            }
            // Neither side is anything we track or can create against - nothing to do.
            (None, None) => PlayerId(0),
        }
    }

    fn allocate_dynamic_player_id(&self) -> PlayerId {
        PlayerId(self.next_dynamic_player_id.fetch_add(1, Ordering::Relaxed))
    }

    /// Called once at `ssx-node` startup, after rehydrating any
    /// dynamically-created players from Redis, so newly allocated ids never
    /// collide with ones created before a restart.
    pub fn resume_dynamic_player_ids_from(&self, highest_seen: u32) {
        let next = (highest_seen + 1).max(DYNAMIC_PLAYER_ID_BASE);
        self.next_dynamic_player_id.store(next, Ordering::Relaxed);
    }
}

pub(crate) fn new_dynamic_player_id_counter() -> AtomicU32 {
    AtomicU32::new(DYNAMIC_PLAYER_ID_BASE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClubState, Position};

    fn engine_with_two_clubs() -> (ValuationEngine, ClubId, ClubId) {
        let engine = ValuationEngine::new();
        let seller = ClubId(1);
        let buyer = ClubId(2);
        engine.club_states.insert(seller, ClubState::new(seller));
        engine.club_states.insert(buyer, ClubState::new(buyer));
        (engine, seller, buyer)
    }

    #[test]
    fn fee_parsing_handles_millions_thousands_and_words() {
        assert_eq!(parse_transfer_kind("€45M"), TransferKind::Permanent { fee: Some(dec!(45)) });
        assert_eq!(parse_transfer_kind("€800K"), TransferKind::Permanent { fee: Some(dec!(0.8)) });
        assert_eq!(parse_transfer_kind("Free"), TransferKind::Free);
        assert_eq!(parse_transfer_kind("Free agent"), TransferKind::Free); // confirmed live variant
        assert_eq!(parse_transfer_kind("Loan"), TransferKind::Loan);
        assert_eq!(parse_transfer_kind("N/A"), TransferKind::Permanent { fee: None });
        assert_eq!(parse_transfer_kind("Transfer"), TransferKind::Permanent { fee: None }); // confirmed live: undisclosed-fee label
        assert_eq!(parse_transfer_kind("€ 500K"), TransferKind::Permanent { fee: Some(dec!(0.5)) }); // confirmed live: space after symbol
    }

    #[test]
    fn intra_universe_move_reassigns_club_and_reblends_value_toward_fee() {
        let (engine, seller, buyer) = engine_with_two_clubs();
        let pid = PlayerId(100);
        engine.player_states.insert(
            pid,
            PlayerValues {
                team_id: seller,
                intrinsic_value: dec!(50.0),
                form_weight: dec!(1.2),
                sentiment_score: dec!(1.0),
                volatility_factor: dec!(1.0),
                performance_history: vec![dec!(50.0); 5],
                position: Position::ST,
                is_captain: false,
                active: true,
            },
        );

        let event = TransferEvent {
            existing_player_id: Some(pid),
            player_name: "Test Player".into(),
            position_bucket: PositionBucket::Att,
            from: Some(seller),
            to: Some(buyer),
            kind: TransferKind::Permanent { fee: Some(dec!(70.0)) },
        };
        engine.process_transfer(event, 0);

        let player = engine.player_states.get(&pid).unwrap();
        assert_eq!(player.team_id, buyer);
        assert!(player.active);
        assert_eq!(player.intrinsic_value, dec!(60.0)); // 0.5*50 + 0.5*70
        assert_eq!(player.performance_history.len(), 1); // cleared, fresh club context
        assert_eq!(player.form_weight, dec!(1.2)); // form carries over unchanged

        assert!(engine.club_states.get(&buyer).unwrap().intrinsic_value > dec!(100.0));
        assert!(engine.club_states.get(&seller).unwrap().intrinsic_value < dec!(100.0));
    }

    #[test]
    fn leaving_tracked_universe_deactivates_without_deleting() {
        let (engine, seller, _buyer) = engine_with_two_clubs();
        let pid = PlayerId(101);
        engine.player_states.insert(
            pid,
            PlayerValues {
                team_id: seller,
                intrinsic_value: dec!(50.0),
                form_weight: dec!(1.0),
                sentiment_score: dec!(1.0),
                volatility_factor: dec!(1.0),
                performance_history: vec![dec!(50.0); 5],
                position: Position::ST,
                is_captain: false,
                active: true,
            },
        );

        let event = TransferEvent {
            existing_player_id: Some(pid),
            player_name: "Test Player".into(),
            position_bucket: PositionBucket::Att,
            from: Some(seller),
            to: None,
            kind: TransferKind::Permanent { fee: Some(dec!(20.0)) },
        };
        engine.process_transfer(event, 0);

        let player = engine.player_states.get(&pid).unwrap();
        assert!(!player.active);
        assert_eq!(player.team_id, seller); // last known club preserved, not cleared
        assert_eq!(player.intrinsic_value, dec!(50.0)); // no destination to re-rate against
        assert!(engine.club_states.get(&seller).unwrap().intrinsic_value < dec!(100.0));
    }

    #[test]
    fn arriving_from_outside_creates_a_new_player_with_fee_derived_value() {
        let (engine, _seller, buyer) = engine_with_two_clubs();
        let event = TransferEvent {
            existing_player_id: None,
            player_name: "Brand New Signing".into(),
            position_bucket: PositionBucket::Mid,
            from: None,
            to: Some(buyer),
            kind: TransferKind::Permanent { fee: Some(dec!(30.0)) },
        };
        let new_id = engine.process_transfer(event, 0);

        assert!(new_id.0 >= DYNAMIC_PLAYER_ID_BASE, "expected an id in the reserved dynamic range, got {}", new_id.0);
        let player = engine.player_states.get(&new_id).unwrap();
        assert_eq!(player.team_id, buyer);
        assert!(player.active);
        assert_eq!(player.intrinsic_value, dec!(30.0));
        assert_eq!(player.position, Position::CM);
        assert_eq!(engine.names.get(&new_id.0).map(|n| n.clone()), Some("Brand New Signing".to_string()));
    }

    #[test]
    fn arriving_from_outside_with_no_fee_signal_uses_default_value() {
        let (engine, _seller, buyer) = engine_with_two_clubs();
        let event = TransferEvent {
            existing_player_id: None,
            player_name: "Unknown Fee Signing".into(),
            position_bucket: PositionBucket::Def,
            from: None,
            to: Some(buyer),
            kind: TransferKind::Free,
        };
        let new_id = engine.process_transfer(event, 0);
        let player = engine.player_states.get(&new_id).unwrap();
        assert_eq!(player.intrinsic_value, NEW_SIGNING_DEFAULT_VALUE);
    }

    #[test]
    fn reactivating_a_returning_player_does_not_create_a_duplicate() {
        let (engine, seller, buyer) = engine_with_two_clubs();
        let pid = PlayerId(102);
        engine.player_states.insert(
            pid,
            PlayerValues {
                team_id: seller,
                intrinsic_value: dec!(40.0),
                form_weight: dec!(1.0),
                sentiment_score: dec!(1.0),
                volatility_factor: dec!(1.0),
                performance_history: vec![dec!(40.0); 5],
                position: Position::CB,
                is_captain: false,
                active: false, // previously left our tracked universe
            },
        );

        let event = TransferEvent {
            existing_player_id: Some(pid), // caller resolved this via the persisted api-player-id map
            player_name: "Returning Player".into(),
            position_bucket: PositionBucket::Def,
            from: None,
            to: Some(buyer),
            kind: TransferKind::Free,
        };
        let resolved_id = engine.process_transfer(event, 0);

        assert_eq!(resolved_id, pid); // reactivated the existing record, not a new one
        let player = engine.player_states.get(&pid).unwrap();
        assert!(player.active);
        assert_eq!(player.team_id, buyer);
        assert_eq!(engine.player_states.len(), 1);
    }

    #[test]
    fn loan_moves_are_damped_relative_to_permanent_moves() {
        let (engine_permanent, seller, buyer) = engine_with_two_clubs();
        let pid = PlayerId(200);
        let make_player = || PlayerValues {
            team_id: seller,
            intrinsic_value: dec!(50.0),
            form_weight: dec!(1.0),
            sentiment_score: dec!(1.0),
            volatility_factor: dec!(1.0),
            performance_history: vec![dec!(50.0); 5],
            position: Position::ST,
            is_captain: false,
            active: true,
        };
        engine_permanent.player_states.insert(pid, make_player());
        engine_permanent.process_transfer(
            TransferEvent {
                existing_player_id: Some(pid),
                player_name: "X".into(),
                position_bucket: PositionBucket::Att,
                from: Some(seller),
                to: Some(buyer),
                kind: TransferKind::Permanent { fee: Some(dec!(50.0)) },
            },
            0,
        );
        let permanent_buyer_delta = engine_permanent.club_states.get(&buyer).unwrap().intrinsic_value - dec!(100.0);

        let (engine_loan, seller2, buyer2) = engine_with_two_clubs();
        engine_loan.player_states.insert(pid, make_player());
        engine_loan.process_transfer(
            TransferEvent {
                existing_player_id: Some(pid),
                player_name: "X".into(),
                position_bucket: PositionBucket::Att,
                from: Some(seller2),
                to: Some(buyer2),
                kind: TransferKind::Loan,
            },
            0,
        );
        let loan_buyer_delta = engine_loan.club_states.get(&buyer2).unwrap().intrinsic_value - dec!(100.0);

        assert!(loan_buyer_delta < permanent_buyer_delta, "expected a loan to move club value less than a permanent transfer");
        assert!(loan_buyer_delta > dec!(0.0));
    }
}
