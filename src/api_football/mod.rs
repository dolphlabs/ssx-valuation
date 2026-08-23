//! Shared glue between our domain model and API-Football's IDs/names.
//! Used by both `ssx-backfill` (one-off historical replay) and `ssx-node`'s
//! live transfers poller - kept here, not duplicated, so the team-id mapping
//! and player-name matching logic can never silently drift between the two.
pub mod player_match;
pub mod team_map;

/// `(api_football_league_id, short_name)` for the 5 tracked leagues. Shared
/// by `ssx-backfill` (historical replay) and `ssx-live-oracle` (real-fixture
/// discovery for Match Pools) so the two can never drift on which leagues
/// are actually tracked.
pub const LEAGUES: &[(u32, &str)] = &[
    (39, "pl"),
    (140, "la_liga"),
    (135, "serie_a"),
    (78, "bundesliga"),
    (61, "ligue_1"),
];
