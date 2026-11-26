//! What a transfer does to the two clubs' values. Both sides move by a
//! percentage of their own value, made of two parts:
//!
//! - **Talent** (the main part): the change in the club's squad strength -
//!   its best XI by position plus a lightly weighted bench - so depth is
//!   built in. Selling an 88 whose backup is a 75 costs the XI 13 points;
//!   selling one of several 84s costs almost nothing, because the next man
//!   up is about as good. A signing only counts if it improves the XI or
//!   the bench.
//! - **Business** (weighted `BUSINESS_WEIGHT` of talent): the fee against
//!   the player's rating, scaled by how big a part of the squad he was. A
//!   good sale lifts the seller; the same overpayment drags the buyer.
//!   Only a disclosed permanent fee carries a business signal.
//!
//! Loans count at `LOAN_SHARE` of a permanent move. Each side is capped at
//! `MAX_TRANSFER_IMPACT` either way, per transfer. Ratings are only ever
//! compared as ratios, so the formula doesn't depend on their scale.
use crate::Position;

/// Business counts this much of talent's weight.
pub const BUSINESS_WEIGHT: f64 = 0.4;
/// No single transfer moves a club more than this, up or down.
pub const MAX_TRANSFER_IMPACT: f64 = 0.25;
/// A loan moves a club this share of what the same permanent move would.
pub const LOAN_SHARE: f64 = 0.4;
/// Players per line in the XI: GK, defence, midfield, attack (1-4-3-3).
const XI_SLOTS: [usize; 4] = [1, 4, 3, 3];
/// Bench players counted after the XI, and what each is worth relative to a starter.
const BENCH_SIZE: usize = 7;
const BENCH_WEIGHT: f64 = 0.2;

/// Which line of the XI a position plays in.
pub fn line(position: Position) -> usize {
    use Position::*;
    match position {
        GK => 0,
        CB | LB | RB | LWB | RWB => 1,
        CDM | CM | CAM | LM | RM => 2,
        ST | CF | LW | RW => 3,
    }
}

/// The best XI by line plus a weighted bench. An empty slot counts as zero,
/// so selling a club's only keeper hurts.
pub fn squad_strength(squad: &[(Position, f64)]) -> f64 {
    let mut by_line: [Vec<f64>; 4] = Default::default();
    for &(position, rating) in squad {
        by_line[line(position)].push(rating.max(0.0));
    }
    let mut xi = 0.0;
    let mut bench: Vec<f64> = Vec::new();
    for (players, slots) in by_line.iter_mut().zip(XI_SLOTS) {
        players.sort_by(|a, b| b.total_cmp(a));
        xi += players.iter().take(slots).sum::<f64>();
        bench.extend(players.iter().skip(slots));
    }
    bench.sort_by(|a, b| b.total_cmp(a));
    xi + BENCH_WEIGHT * bench.iter().take(BENCH_SIZE).sum::<f64>()
}

/// The move being priced, as far as the formula cares.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub position: Position,
    /// The player's rating before the move: what he was worth to the seller,
    /// and the fair value a fee is judged against.
    pub rating: f64,
    /// His rating once he arrives (a transfer re-rates him toward the fee).
    pub rating_after: f64,
    /// A disclosed permanent fee, on the rating scale.
    pub fee: Option<f64>,
    pub loan: bool,
}

/// How far the fee was from the player's rating, as a share of the rating,
/// clipped to ±1: +1 means the seller got at least double his worth.
pub fn fee_gap(fee: f64, rating: f64) -> f64 {
    ((fee - rating) / rating.max(1.0)).clamp(-1.0, 1.0)
}

fn finish(talent: f64, business: f64, loan: bool) -> f64 {
    let raw = talent + BUSINESS_WEIGHT * business;
    let scaled = if loan { raw * LOAN_SHARE } else { raw };
    if scaled.is_finite() { scaled.clamp(-MAX_TRANSFER_IMPACT, MAX_TRANSFER_IMPACT) } else { 0.0 }
}

/// The seller's percentage move (as a fraction, -0.08 = -8%). `squad` is
/// the seller's squad *including* the player leaving.
pub fn seller_impact(squad: &[(Position, f64)], leaving_index: usize, mv: &Move) -> f64 {
    let before = squad_strength(squad);
    if before <= 0.0 {
        return 0.0;
    }
    let after: Vec<(Position, f64)> = squad.iter().enumerate().filter(|(i, _)| *i != leaving_index).map(|(_, p)| *p).collect();
    let talent = (squad_strength(&after) - before) / before;
    let business = mv.fee.map(|fee| fee_gap(fee, mv.rating) * (mv.rating / before)).unwrap_or(0.0);
    finish(talent, business, mv.loan)
}

/// The buyer's percentage move. `squad` is the buyer's squad *before* the
/// player arrives.
pub fn buyer_impact(squad: &[(Position, f64)], mv: &Move) -> f64 {
    let before = squad_strength(squad);
    if before <= 0.0 {
        return 0.0;
    }
    let mut after = squad.to_vec();
    after.push((mv.position, mv.rating_after));
    let talent = (squad_strength(&after) - before) / before;
    let business = mv.fee.map(|fee| -fee_gap(fee, mv.rating) * (mv.rating / before)).unwrap_or(0.0);
    finish(talent, business, mv.loan)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Position::*;

    /// A plausible 18-man squad: every slot filled at `level`, plus a bench.
    fn squad(level: f64) -> Vec<(Position, f64)> {
        let mut s = vec![(GK, level), (CB, level), (CB, level), (LB, level), (RB, level), (CM, level), (CDM, level), (CAM, level), (ST, level), (LW, level), (RW, level)];
        s.extend([(GK, level * 0.6), (CB, level * 0.7), (CM, level * 0.7), (CM, level * 0.6), (ST, level * 0.7), (RW, level * 0.6), (LB, level * 0.6)]);
        s
    }

    fn sale(rating: f64, fee: Option<f64>) -> Move {
        Move { position: ST, rating, rating_after: rating, fee, loan: false }
    }

    #[test]
    fn strength_counts_the_xi_by_line_and_a_light_bench() {
        let s = vec![(GK, 50.0), (GK, 40.0), (ST, 80.0), (ST, 70.0), (ST, 60.0), (ST, 10.0)];
        // XI: GK 50 + ST 80+70+60; bench: GK 40, ST 10 at 0.2.
        assert!((squad_strength(&s) - (260.0 + 0.2 * 50.0)).abs() < 1e-9);
    }

    #[test]
    fn losing_a_star_with_a_weak_backup_hurts_far_more_than_losing_one_of_many() {
        // Club A: an 88 striker, next best striker 75 among 60s.
        let mut a = squad(60.0);
        a[8] = (ST, 88.0);
        a[15] = (ST, 75.0);
        // Club B: 84s everywhere, so an 84 striker's replacement is about as good.
        let mut b = squad(84.0);
        b[15] = (ST, 82.0);
        let a_hit = seller_impact(&a, 8, &sale(88.0, None));
        let b_hit = seller_impact(&b, 8, &sale(84.0, None));
        assert!(a_hit < 0.0 && b_hit < 0.0);
        assert!(a_hit < 2.0 * b_hit, "the thin squad should lose well over twice as much: {a_hit} vs {b_hit}");
    }

    #[test]
    fn a_good_sale_softens_the_loss_and_an_overpayment_drags_the_buyer() {
        let seller = squad(60.0);
        let cheap = seller_impact(&seller, 8, &sale(60.0, Some(30.0)));
        let fair = seller_impact(&seller, 8, &sale(60.0, Some(60.0)));
        let premium = seller_impact(&seller, 8, &sale(60.0, Some(120.0)));
        assert!(cheap < fair && fair < premium, "{cheap} < {fair} < {premium}");

        let buyer = squad(50.0);
        let mv = |fee| Move { position: ST, rating: 60.0, rating_after: 60.0, fee: Some(fee), loan: false };
        assert!(buyer_impact(&buyer, &mv(120.0)) < buyer_impact(&buyer, &mv(60.0)));
    }

    #[test]
    fn talent_outweighs_business() {
        // Selling the best player cheaply is still a loss even at twice his
        // worth: business tops out at 40% of what he meant to the squad.
        let mut s = squad(40.0);
        s[8] = (ST, 90.0);
        assert!(seller_impact(&s, 8, &sale(90.0, Some(1000.0))) < 0.0);
    }

    #[test]
    fn a_signing_who_wouldnt_start_or_make_the_bench_adds_nothing() {
        let buyer = squad(80.0);
        let fourth_keeper = Move { position: GK, rating: 5.0, rating_after: 5.0, fee: None, loan: false };
        let mut with_two_more = buyer.clone();
        with_two_more.extend([(GK, 50.0), (GK, 50.0), (CB, 50.0), (CB, 50.0), (CM, 50.0), (ST, 50.0), (RW, 50.0)]);
        assert_eq!(buyer_impact(&with_two_more, &fourth_keeper), 0.0);
    }

    #[test]
    fn every_move_is_capped_at_25_percent_and_loans_count_less() {
        // A one-man club selling its only player.
        let lonely = vec![(ST, 100.0)];
        assert_eq!(seller_impact(&lonely, 0, &sale(100.0, None)), -MAX_TRANSFER_IMPACT);
        let empty_buyer = vec![(GK, 1.0)];
        let star = Move { position: ST, rating: 300.0, rating_after: 300.0, fee: Some(10.0), loan: false };
        assert_eq!(buyer_impact(&empty_buyer, &star), MAX_TRANSFER_IMPACT);

        let s = squad(60.0);
        let permanent = seller_impact(&s, 8, &sale(60.0, None));
        let loan = seller_impact(&s, 8, &Move { loan: true, ..sale(60.0, None) });
        assert!((loan - permanent * LOAN_SHARE).abs() < 1e-12);
    }

    #[test]
    fn the_scale_of_ratings_doesnt_matter() {
        let small = seller_impact(&squad(60.0), 8, &sale(60.0, Some(90.0)));
        let big: Vec<(Position, f64)> = squad(60.0).into_iter().map(|(p, r)| (p, r * 10.0)).collect();
        let scaled = seller_impact(&big, 8, &sale(600.0, Some(900.0)));
        assert!((small - scaled).abs() < 1e-12);
    }

    #[test]
    fn nothing_to_measure_means_no_move() {
        assert_eq!(seller_impact(&[], 0, &sale(60.0, Some(60.0))), 0.0);
        assert_eq!(buyer_impact(&[], &sale(60.0, Some(60.0))), 0.0);
    }
}
