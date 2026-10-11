//! Live match event processing. Wire format published by the standalone
//! `ssx-live-oracle` process (fetches real in-play fixtures/events from
//! API-Football, same isolation principle as `ssx-transfers` - if the API or
//! this process is down, nothing else in the system is affected, prices just
//! stop reacting to match events until it recovers) onto
//! `ssx:live_match_broadcast`.
//!
//! Deliberately not published as an already-resolved `MatchEvent`: only the
//! subscriber (`ssx-node`, which holds the live `ValuationEngine`) can
//! resolve a real player's name against the roster as it actually stands
//! right now (post-transfers) - same division of responsibility as
//! `transfers.rs`'s `TransferFact` vs `TransferEvent`.
use crate::api_football::player_match::PlayerIndex;
use crate::{ClubId, MatchEvent, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiveMatchFact {
    Goal { team_id: ClubId, opponent_id: ClubId, player_name: Option<String>, minute: u32 },
    // `minute` defaults to None so a payload from an ssx-live-oracle build
    // that predates it still deserializes - it's display-only (the terminal's
    // event chips), the card's price effect never depended on it.
    YellowCard {
        team_id: ClubId,
        opponent_id: ClubId,
        player_name: Option<String>,
        #[serde(default)]
        minute: Option<u32>,
    },
    RedCard {
        team_id: ClubId,
        opponent_id: ClubId,
        player_name: Option<String>,
        #[serde(default)]
        minute: Option<u32>,
    },
    WhistleEnd { team_a_id: ClubId, team_b_id: ClubId },
    /// A `Goal` VAR took back - carries the original goal's fields so the
    /// engine can undo exactly what that goal moved.
    GoalCancelled { team_id: ClubId, opponent_id: ClubId, player_name: Option<String>, minute: u32 },
}

impl LiveMatchFact {
    /// Resolves the player name (if any) against `index`, scoped to the
    /// acting club, and produces the `MatchEvent` `process_event` actually
    /// consumes. A miss (unmatched name, or no name reported by the API)
    /// resolves to `PlayerId(0)` - never present in `player_states`, so the
    /// event's player-side effect is a harmless no-op while the club-side
    /// effect still fires. Same sentinel and reasoning as the historical
    /// backfill's `convert_fixture`.
    pub fn resolve(&self, index: &PlayerIndex) -> MatchEvent {
        let resolve_name = |club: ClubId, name: &Option<String>| -> PlayerId {
            name.as_deref().and_then(|n| index.find(club, n)).unwrap_or(PlayerId(0))
        };
        match self {
            LiveMatchFact::Goal { team_id, opponent_id, player_name, minute } => MatchEvent::Goal {
                team_id: *team_id,
                opponent_id: *opponent_id,
                player_id: resolve_name(*team_id, player_name),
                minute: *minute,
            },
            LiveMatchFact::YellowCard { team_id, opponent_id, player_name, .. } => MatchEvent::YellowCard {
                team_id: *team_id,
                opponent_id: *opponent_id,
                player_id: resolve_name(*team_id, player_name),
            },
            LiveMatchFact::RedCard { team_id, opponent_id, player_name, .. } => MatchEvent::RedCard {
                team_id: *team_id,
                opponent_id: *opponent_id,
                player_id: resolve_name(*team_id, player_name),
            },
            LiveMatchFact::WhistleEnd { team_a_id, team_b_id } => {
                MatchEvent::WhistleEnd { team_a_id: *team_a_id, team_b_id: *team_b_id }
            }
            // Resolves the scorer the same way the goal did, so the player
            // bump is reversed on the same player.
            LiveMatchFact::GoalCancelled { team_id, opponent_id, player_name, minute } => MatchEvent::GoalCancelled {
                team_id: *team_id,
                opponent_id: *opponent_id,
                player_id: resolve_name(*team_id, player_name),
                minute: *minute,
            },
        }
    }
}

/// What `ssx-node` publishes on `ssx:match_events` the moment it has applied
/// a live fact to the engine - the terminal's event chips, feed and
/// timeline are built from these. `ts` is the engine's own timestamp for
/// the application (the same `current_ts` the resulting `EngineUpdate::Club`
/// ticks carry), so a consumer can read the pair's value just before and
/// just after it straight out of `valuations`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppliedMatchEvent {
    /// "goal", "yellow", "red", "full_time" or "goal_cancelled".
    pub kind: String,
    /// The scoring/carded club; for "full_time", the first listed side.
    pub club_id: ClubId,
    pub opponent_id: ClubId,
    pub minute: Option<u32>,
    pub ts: u64,
}

/// Redis list `ssx-live-oracle` pushes a `FeedLagSample` onto for every
/// goal and card it applies; `ssx-executor`'s admin feed-lag route reads it.
pub const FEED_LAG_KEY: &str = "ssx:feed_lag";
/// Newest samples kept - several seasons of goals and cards.
pub const FEED_LAG_KEEP: isize = 5000;

/// How far behind the real match one event reached us. `estimated_at_ms`
/// comes from API-Football's own half kick-off timestamps plus the event's
/// minute, so it's only good to about ±30s; the median over many events is
/// what to read, not any single sample.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeedLagSample {
    pub fixture_id: u64,
    /// "goal", "yellow" or "red".
    pub kind: String,
    pub club_id: ClubId,
    pub minute: i32,
    pub extra: Option<i32>,
    /// When our oracle first saw the event.
    pub seen_at_ms: i64,
    pub estimated_at_ms: i64,
    pub lag_ms: i64,
    /// Time since the previous poll of this fixture: the share of the lag
    /// that's our own polling rather than the provider's.
    pub since_last_poll_ms: Option<i64>,
}

/// When an event at `minute` (+`extra` stoppage) most likely happened,
/// given each half's real kick-off (unix seconds). Football minutes round
/// up - "23′" is 22:00-22:59 into the half - so the estimate is the middle
/// of that minute. Second-half minutes count from 46. Extra time and
/// shoot-outs (past 90′) aren't estimated.
pub fn estimated_event_ms(minute: i32, extra: Option<i32>, first_half: Option<i64>, second_half: Option<i64>) -> Option<i64> {
    let (start, offset) = match minute {
        m if m <= 45 => (first_half?, m - 1),
        m if m <= 90 => (second_half?, m - 46),
        _ => return None,
    };
    let offset_min = (offset + extra.unwrap_or(0)).max(0) as i64;
    Some(start * 1000 + offset_min * 60_000 + 30_000)
}

impl LiveMatchFact {
    pub fn applied(&self, ts: u64) -> AppliedMatchEvent {
        let (kind, club_id, opponent_id, minute) = match self {
            LiveMatchFact::Goal { team_id, opponent_id, minute, .. } => ("goal", *team_id, *opponent_id, Some(*minute)),
            LiveMatchFact::YellowCard { team_id, opponent_id, minute, .. } => ("yellow", *team_id, *opponent_id, *minute),
            LiveMatchFact::RedCard { team_id, opponent_id, minute, .. } => ("red", *team_id, *opponent_id, *minute),
            LiveMatchFact::WhistleEnd { team_a_id, team_b_id } => ("full_time", *team_a_id, *team_b_id, None),
            LiveMatchFact::GoalCancelled { team_id, opponent_id, minute, .. } => ("goal_cancelled", *team_id, *opponent_id, Some(*minute)),
        };
        AppliedMatchEvent { kind: kind.to_string(), club_id, opponent_id, minute, ts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_time_estimates_sit_mid_minute_from_each_half_kick_off() {
        // First half kicked off at t=1000s, second at t=5000s.
        assert_eq!(estimated_event_ms(1, None, Some(1000), Some(5000)), Some(1_030_000));
        assert_eq!(estimated_event_ms(23, None, Some(1000), Some(5000)), Some(1_000_000 + 22 * 60_000 + 30_000));
        assert_eq!(estimated_event_ms(45, Some(2), Some(1000), Some(5000)), Some(1_000_000 + 46 * 60_000 + 30_000));
        assert_eq!(estimated_event_ms(46, None, Some(1000), Some(5000)), Some(5_030_000));
        assert_eq!(estimated_event_ms(90, Some(4), Some(1000), Some(5000)), Some(5_000_000 + 48 * 60_000 + 30_000));
        assert_eq!(estimated_event_ms(60, None, Some(1000), None), None, "no second-half kick-off yet");
        assert_eq!(estimated_event_ms(105, None, Some(1000), Some(5000)), None, "extra time isn't estimated");
    }
    use crate::{PlayerValues, Position, ValuationEngine};
    use rust_decimal_macros::dec;

    fn engine_with(club_id: u32, players: &[(u32, &str)]) -> ValuationEngine {
        let engine = ValuationEngine::new();
        for (id, name) in players {
            engine.names.insert(*id, name.to_string());
            engine.player_states.insert(
                PlayerId(*id),
                PlayerValues {
                    team_id: ClubId(club_id),
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
        }
        engine
    }

    #[test]
    fn goal_resolves_matched_player() {
        let engine = engine_with(4, &[(1155, "Bukayo Saka")]);
        let index = PlayerIndex::build(&engine);
        let fact = LiveMatchFact::Goal {
            team_id: ClubId(4),
            opponent_id: ClubId(6),
            player_name: Some("Bukayo Saka".to_string()),
            minute: 23,
        };
        match fact.resolve(&index) {
            MatchEvent::Goal { team_id, opponent_id, player_id, minute } => {
                assert_eq!(team_id, ClubId(4));
                assert_eq!(opponent_id, ClubId(6));
                assert_eq!(player_id, PlayerId(1155));
                assert_eq!(minute, 23);
            }
            other => panic!("expected Goal, got {other:?}"),
        }
    }

    #[test]
    fn unmatched_or_missing_name_resolves_to_sentinel_player_id() {
        let engine = engine_with(4, &[(1155, "Bukayo Saka")]);
        let index = PlayerIndex::build(&engine);

        let unmatched = LiveMatchFact::YellowCard { team_id: ClubId(4), opponent_id: ClubId(6), player_name: Some("Nobody Here".into()), minute: Some(12) };
        assert!(matches!(unmatched.resolve(&index), MatchEvent::YellowCard { player_id: PlayerId(0), .. }));

        let missing = LiveMatchFact::RedCard { team_id: ClubId(4), opponent_id: ClubId(6), player_name: None, minute: None };
        assert!(matches!(missing.resolve(&index), MatchEvent::RedCard { player_id: PlayerId(0), .. }));
    }

    #[test]
    fn whistle_end_passes_through_untouched() {
        let index = PlayerIndex::build(&ValuationEngine::new());
        let fact = LiveMatchFact::WhistleEnd { team_a_id: ClubId(4), team_b_id: ClubId(6) };
        assert!(matches!(fact.resolve(&index), MatchEvent::WhistleEnd { team_a_id: ClubId(4), team_b_id: ClubId(6) }));
    }
}
