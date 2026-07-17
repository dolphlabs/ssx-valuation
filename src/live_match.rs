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
    YellowCard { team_id: ClubId, opponent_id: ClubId, player_name: Option<String> },
    RedCard { team_id: ClubId, opponent_id: ClubId, player_name: Option<String> },
    WhistleEnd { team_a_id: ClubId, team_b_id: ClubId },
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
            LiveMatchFact::YellowCard { team_id, opponent_id, player_name } => MatchEvent::YellowCard {
                team_id: *team_id,
                opponent_id: *opponent_id,
                player_id: resolve_name(*team_id, player_name),
            },
            LiveMatchFact::RedCard { team_id, opponent_id, player_name } => MatchEvent::RedCard {
                team_id: *team_id,
                opponent_id: *opponent_id,
                player_id: resolve_name(*team_id, player_name),
            },
            LiveMatchFact::WhistleEnd { team_a_id, team_b_id } => {
                MatchEvent::WhistleEnd { team_a_id: *team_a_id, team_b_id: *team_b_id }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let unmatched = LiveMatchFact::YellowCard { team_id: ClubId(4), opponent_id: ClubId(6), player_name: Some("Nobody Here".into()) };
        assert!(matches!(unmatched.resolve(&index), MatchEvent::YellowCard { player_id: PlayerId(0), .. }));

        let missing = LiveMatchFact::RedCard { team_id: ClubId(4), opponent_id: ClubId(6), player_name: None };
        assert!(matches!(missing.resolve(&index), MatchEvent::RedCard { player_id: PlayerId(0), .. }));
    }

    #[test]
    fn whistle_end_passes_through_untouched() {
        let index = PlayerIndex::build(&ValuationEngine::new());
        let fact = LiveMatchFact::WhistleEnd { team_a_id: ClubId(4), team_b_id: ClubId(6) };
        assert!(matches!(fact.resolve(&index), MatchEvent::WhistleEnd { team_a_id: ClubId(4), team_b_id: ClubId(6) }));
    }
}
