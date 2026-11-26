//! Player ratings on the 0-100 scale people read ("82.4"). The engine's own
//! rating is open-ended (roughly a market value in €m) and keeps driving
//! everything; this is only how it's shown.
//!
//! Each player is placed against his own line - keepers, defenders,
//! midfielders, attackers - so the best keeper rates like the best striker
//! even though keepers are worth less in money. Within a line, the log of
//! the rating is turned into a z-score and mapped to
//! `CENTRE + HEADROOM * tanh(SPREAD * z / HEADROOM)`: a typical player is
//! 70, one standard deviation above is ~78, two ~85, and the very best land
//! in the low-to-mid 90s. The curve flattens towards the ends instead of
//! clamping, so the top players never tie at a ceiling. One decimal place.
use crate::transfer_impact::line;
use crate::Position;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Redis channel `ssx-backfill ratings apply` publishes a `RatingReset` on;
/// ssx-node sets each listed player's rating to the season replay's.
pub const RATING_RESET_CHANNEL: &str = "ssx:player_rating_reset";

/// One batch of replayed ratings (engine scale), by player id.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RatingReset {
    pub ratings: Vec<(u32, Decimal)>,
    pub note: String,
}

pub const CENTRE: f64 = 70.0;
/// Points per standard deviation near the middle.
pub const SPREAD: f64 = 8.0;
/// How far above or below the centre the curve can reach (never quite).
pub const HEADROOM: f64 = 29.0;
/// Ratings at or below this are treated as this, so a near-zero rating
/// can't throw the logs out.
const FLOOR: f64 = 0.5;

/// Each player's 0-100 rating, keyed by the id given with him.
pub fn display_ratings<K: Copy + Eq + std::hash::Hash>(players: &[(K, Position, f64)]) -> HashMap<K, f64> {
    let mut by_line: [Vec<(K, f64)>; 4] = Default::default();
    for &(id, position, rating) in players {
        by_line[line(position)].push((id, rating.max(FLOOR).ln()));
    }
    let mut out = HashMap::with_capacity(players.len());
    for group in by_line {
        if group.is_empty() {
            continue;
        }
        let n = group.len() as f64;
        let mean = group.iter().map(|(_, l)| l).sum::<f64>() / n;
        let sd = (group.iter().map(|(_, l)| (l - mean).powi(2)).sum::<f64>() / n).sqrt();
        for (id, l) in group {
            let z = if sd > 0.0 { (l - mean) / sd } else { 0.0 };
            let score = CENTRE + HEADROOM * (SPREAD * z / HEADROOM).tanh();
            out.insert(id, (score * 10.0).round() / 10.0);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use Position::*;

    #[test]
    fn ratings_sit_on_0_to_100_with_one_decimal_and_rank_within_each_line() {
        let mut players: Vec<(u32, Position, f64)> = (0..50).map(|i| (i, ST, 20.0 + i as f64 * 4.0)).collect();
        players.extend((100..150).map(|i| (i, GK, 5.0 + (i - 100) as f64)));
        let r = display_ratings(&players);
        for v in r.values() {
            assert!((1.0..=99.0).contains(v));
            assert_eq!((v * 10.0).round() / 10.0, *v);
        }
        // Best striker and best keeper rate alike, though worth very different amounts.
        assert!((r[&49] - r[&149]).abs() < 3.0, "{} vs {}", r[&49], r[&149]);
        assert!(r[&49] > r[&0]);
    }

    #[test]
    fn a_typical_player_is_seventy_and_the_top_never_ties_at_a_ceiling() {
        let players = vec![(1, CM, 40.0), (2, CM, 40.0), (3, CM, 40.0)];
        assert_eq!(display_ratings(&players)[&1], CENTRE);
        // Three stars far above a big pool stay in order, below 99.
        let pool: Vec<(u32, Position, f64)> = (0..400).map(|i| (i, ST, 10.0 * 1.01f64.powi(i as i32))).chain([(5001, ST, 450.0), (5002, ST, 500.0), (5003, ST, 550.0)]).collect();
        let r = display_ratings(&pool);
        assert!(r[&5001] < r[&5002] && r[&5002] < r[&5003], "{} {} {}", r[&5001], r[&5002], r[&5003]);
        assert!(r[&5003] < 99.0);
    }
}
