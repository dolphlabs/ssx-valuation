//! Wire format for a one-time admin correction to a club's `intrinsic_value`
//! - published by `ssx-executor`'s `POST /v1/admin/club-valuations/correct`
//! onto `ssx:admin_correction_broadcast`, applied by `ssx-node`'s subscriber
//! (see `ssx_node::admin_correction`). Same isolation principle as every
//! other fact type here: `ssx-executor` only ever publishes a fact, it never
//! calls `ValuationEngine::process_event` itself - `ssx-node` remains the
//! only thing that resolves and mutates engine state.
//!
//! Exists for exactly one reason so far: the Heartbeat drift/magnitude bugs
//! (see `heartbeat_tick_half_width`'s doc comment and AGENTS.md's "Known
//! gotchas") pushed real clubs' values down on pure noise, unrelated to
//! actual results. This is the audited, engine-native way to correct that -
//! not a direct Redis edit (which would bypass `ssx-node`'s own mutation
//! path and get silently overwritten by the next real event's stale
//! `resting_value` anyway).
use crate::ClubId;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminCorrectionFact {
    pub club_id: ClubId,
    /// Applied multiplicatively to whatever the club's live `intrinsic_value`
    /// is *at the moment `ssx-node` processes this* - not a snapshot value
    /// computed when the admin submitted the request, which could already be
    /// stale by the time this is actually applied (the whole point of a
    /// percentage delta over an absolute target).
    pub delta_pct: Decimal,
    /// Why - surfaced in `ssx-node`'s own log line and already recorded in
    /// `ssx-executor`'s `club_valuation_corrections` table before this fact
    /// is even published, so this field is for operational visibility here,
    /// not the durable audit record.
    pub note: String,
}
