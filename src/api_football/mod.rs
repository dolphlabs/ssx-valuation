//! Shared glue between our domain model and API-Football's IDs/names.
//! Used by both `ssx-backfill` (one-off historical replay) and `ssx-node`'s
//! live transfers poller - kept here, not duplicated, so the team-id mapping
//! and player-name matching logic can never silently drift between the two.
pub mod player_match;
pub mod team_map;
