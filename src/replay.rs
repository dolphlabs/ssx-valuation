use crate::{ValuationEngine, MatchEvent};
use rust_decimal::Decimal;
use tracing::{info, info_span, warn};
use std::time::Instant;

pub struct HistoricalReplay<'a> {
    engine: &'a mut ValuationEngine,
}

impl<'a> HistoricalReplay<'a> {
    pub fn new(engine: &'a mut ValuationEngine) -> Self {
        Self { engine }
    }

    /// Processes events in chronological order and returns a TimeSeries of the Exchange Rate for a specific ClubPair.
    pub fn replay(
        &mut self,
        mut events: Vec<(u64, MatchEvent)>,
        base_id: u32,
        quote_id: u32,
    ) -> Vec<(u64, Decimal)> {
        // Ensure chronological order
        events.sort_by_key(|(ts, _)| *ts);

        let mut time_series = Vec::with_capacity(events.len());

        for (ts, event) in events {
            let span = info_span!("process_event", timestamp = ts);
            let _enter = span.enter();
            let start = Instant::now();

            self.engine.process_event(event, ts);
            
            let rate = self.engine.get_club_pair_exchange_rate(base_id, quote_id);
            time_series.push((ts, rate));

            let duration = start.elapsed();
            info!(target: "valuation_replay", duration_ms = duration.as_millis(), "Event processed");
            
            if duration.as_millis() >= 500 {
                warn!("LATENCY_CRITICAL: Event processing exceeded 500ms: {:?}", duration);
            }
        }

        time_series
    }

    /// Verifies the final calculated intrinsic value against a target 'Point Score'.
    pub fn verify_calibration(&self, club_id: u32, target_score: Decimal) -> bool {
        if let Some(club) = self.engine.club_states.get(&club_id) {
            let result = club.intrinsic_value;
            info!("Calibration check for club {}: calculated={}, target={}", club_id, result, target_score);
            // Allow for small floating point / decimal rounding differences if any, 
            // though Decimal should be exact.
            result == target_score
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClubState, Position};
    use rust_decimal_macros::dec;
    use std::collections::BTreeMap;

    #[test]
    fn test_historical_replay_exchange_rate() {
        let mut engine = ValuationEngine::new();
        engine.club_states.insert(1, ClubState {
            id: 1,
            intrinsic_value: dec!(100.0),
            last_match_update: 0,
            top_oppositions: BTreeMap::new(),
            rivals: vec![],
            player_ids: vec![],
        });
        engine.club_states.insert(2, ClubState {
            id: 2,
            intrinsic_value: dec!(50.0),
            last_match_update: 0,
            top_oppositions: BTreeMap::new(),
            rivals: vec![],
            player_ids: vec![],
        });

        let events = vec![
            (10, MatchEvent::Goal { team_id: 1, opponent_id: 2, player_id: 0, minute: 10 }),
            (20, MatchEvent::Goal { team_id: 1, opponent_id: 2, player_id: 0, minute: 20 }),
        ];

        let mut replay = HistoricalReplay::new(&mut engine);
        let ts = replay.replay(events, 1, 2);

        assert_eq!(ts.len(), 2);
        // Initial 100/50 = 2.0
        // After first goal (min 10): impact = 5 + 10 * 10/90 = 5 + 1.11... = 6.11...
        // New rate > 2.0
        assert!(ts[0].1 > dec!(2.0));
        assert!(ts[1].1 > ts[0].1);
    }
}
