use rust_decimal::Decimal;

/// Unrealized PnL in USD for a position. `size_signed` is base-club units
/// (positive = long, negative = short); `entry_price` is quote-per-base at
/// fill time; `base_intrinsic_now`/`quote_intrinsic_now` are each club's
/// *current* `ClubState::intrinsic_value`, which this codebase already
/// treats as an absolute USD-denominated numéraire (starting at $100, see
/// `ClubState::new`).
///
/// This deliberately always uses the independently-computed index price
/// (derived from each club's own intrinsic_value), never a vAMM pool's own
/// post-trade price — otherwise a trader could push the (intentionally
/// thin) vAMM price to trigger favorable liquidations against themselves or
/// others. Only a *new* trade's fill price should ever come from the vAMM.
pub fn unrealized_pnl_usd(
    size_signed: Decimal,
    entry_price: Decimal,
    base_intrinsic_now: Decimal,
    quote_intrinsic_now: Decimal,
) -> Decimal {
    size_signed * (base_intrinsic_now - entry_price * quote_intrinsic_now)
}

/// Current USD notional of a position's base leg. The quote leg cancels out
/// of this calculation (it only matters for PnL, not notional size).
pub fn notional_usd_now(size_signed: Decimal, base_intrinsic_now: Decimal) -> Decimal {
    size_signed.abs() * base_intrinsic_now
}

/// equity / notional — the standard perp margin-health ratio. Returns `None`
/// when there's no notional to divide by (should not happen for a real open
/// position, but callers must not assume this is infallible).
pub fn margin_ratio(equity_usd: Decimal, notional_usd_now: Decimal) -> Option<Decimal> {
    if notional_usd_now.is_zero() {
        return None;
    }
    Some(equity_usd / notional_usd_now)
}

/// A position is liquidatable once its equity (margin + unrealized PnL) falls
/// to or below `maintenance_margin_ratio` of its current notional.
pub fn is_liquidatable(
    margin_usd: Decimal,
    unrealized_pnl_usd: Decimal,
    notional_usd_now: Decimal,
    maintenance_margin_ratio: Decimal,
) -> bool {
    match margin_ratio(margin_usd + unrealized_pnl_usd, notional_usd_now) {
        Some(ratio) => ratio <= maintenance_margin_ratio,
        None => false,
    }
}

/// Whether a take-profit at `tp_price` (an index price, quote-per-base, same
/// units as `entry_price`) should fire right now. For a long, TP is above
/// entry and fires once the index price rises to meet or exceed it; for a
/// short it's the mirror image.
pub fn take_profit_triggered(size_signed: Decimal, index_price: Decimal, tp_price: Decimal) -> bool {
    if size_signed > Decimal::ZERO {
        index_price >= tp_price
    } else {
        index_price <= tp_price
    }
}

/// Whether a stop-loss at `sl_price` should fire right now - the mirror of
/// `take_profit_triggered`: for a long it fires once price falls to or below
/// it, for a short once price rises to or above it.
pub fn stop_loss_triggered(size_signed: Decimal, index_price: Decimal, sl_price: Decimal) -> bool {
    if size_signed > Decimal::ZERO {
        index_price <= sl_price
    } else {
        index_price >= sl_price
    }
}

/// Flat per-fill trading fee, charged on the *executed* notional (post-
/// slippage), not the requested one - matches how `fill_price`/`entry_price`
/// already reflect the real fill rather than the request.
pub fn trade_fee_usd(executed_notional_usd: Decimal, fee_rate: Decimal) -> Decimal {
    executed_notional_usd.abs() * fee_rate
}

/// Funding rate for one interval: the vAMM price's premium (or discount)
/// over the index price, clamped. Positive means the vAMM is trading above
/// the index (longs pushed it there) - so longs pay shorts; negative is the
/// mirror. This is the market-driven complement to `VammPool::repeg` - repeg
/// forcibly resets the pool, funding gives traders an economic incentive to
/// correct it themselves instead.
pub fn funding_rate(vamm_price: Decimal, index_price: Decimal, cap: Decimal) -> Decimal {
    if index_price.is_zero() {
        return Decimal::ZERO;
    }
    ((vamm_price - index_price) / index_price).clamp(-cap, cap)
}

/// One position's funding settlement for this interval. Positive means the
/// position pays (debit its margin); negative means it receives (credit its
/// margin) - same sign convention as `unrealized_pnl_usd`'s caller-facing
/// arithmetic. A long pays when `funding_rate` is positive (vAMM above
/// index); a short pays when it's negative.
pub fn funding_payment_usd(size_signed: Decimal, notional_usd_now: Decimal, funding_rate: Decimal) -> Decimal {
    if size_signed.is_zero() {
        return Decimal::ZERO;
    }
    let sign = if size_signed > Decimal::ZERO { Decimal::ONE } else { -Decimal::ONE };
    sign * notional_usd_now * funding_rate
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn long_pnl_positive_when_base_appreciates() {
        // Entered long 10 units at entry_price=2.0 (quote intrinsic was 100 at entry).
        // Base intrinsic rises from 200 to 250, quote stays at 100.
        let pnl = unrealized_pnl_usd(dec!(10), dec!(2.0), dec!(250), dec!(100));
        // (250 - 2.0*100) * 10 = (250-200)*10 = 500
        assert_eq!(pnl, dec!(500));
    }

    #[test]
    fn short_pnl_positive_when_base_depreciates() {
        // Short 10 units at entry_price=2.0, quote intrinsic 100 at entry.
        // Base intrinsic falls from 200 to 150.
        let pnl = unrealized_pnl_usd(dec!(-10), dec!(2.0), dec!(150), dec!(100));
        // (150-200)*-10 = 500
        assert_eq!(pnl, dec!(500));
    }

    #[test]
    fn pnl_accounts_for_quote_leg_movement() {
        // Long 10 units, entry_price=2.0, quote intrinsic at entry implied 100.
        // Base unchanged at 200, but quote intrinsic rises to 120 (quote
        // appreciated against base) - the position should show a loss even
        // though the base leg alone didn't move.
        let pnl = unrealized_pnl_usd(dec!(10), dec!(2.0), dec!(200), dec!(120));
        // (200 - 2.0*120)*10 = (200-240)*10 = -400
        assert_eq!(pnl, dec!(-400));
    }

    #[test]
    fn notional_uses_absolute_size() {
        assert_eq!(notional_usd_now(dec!(-10), dec!(200)), dec!(2000));
        assert_eq!(notional_usd_now(dec!(10), dec!(200)), dec!(2000));
    }

    #[test]
    fn margin_ratio_none_when_no_notional() {
        assert_eq!(margin_ratio(dec!(100), dec!(0)), None);
    }

    #[test]
    fn is_liquidatable_true_once_equity_breaches_maintenance() {
        // margin=1000, notional=20000 (20x leverage), maintenance=0.5%.
        // Maintenance equity floor = 0.005 * 20000 = 100.
        let notional = dec!(20000);
        let maintenance = dec!(0.005);

        // Equity still comfortably above the floor (margin 1000, pnl -500 -> equity 500).
        assert!(!is_liquidatable(dec!(1000), dec!(-500), notional, maintenance));

        // Equity right at the floor (margin 1000, pnl -900 -> equity 100).
        assert!(is_liquidatable(dec!(1000), dec!(-900), notional, maintenance));

        // Equity below the floor.
        assert!(is_liquidatable(dec!(1000), dec!(-950), notional, maintenance));
    }

    #[test]
    fn is_liquidatable_false_for_healthy_position() {
        assert!(!is_liquidatable(dec!(1000), dec!(200), dec!(20000), dec!(0.005)));
    }

    #[test]
    fn take_profit_triggers_for_long_when_price_rises_to_target() {
        assert!(!take_profit_triggered(dec!(10), dec!(1.9), dec!(2.0)));
        assert!(take_profit_triggered(dec!(10), dec!(2.0), dec!(2.0)));
        assert!(take_profit_triggered(dec!(10), dec!(2.1), dec!(2.0)));
    }

    #[test]
    fn take_profit_triggers_for_short_when_price_falls_to_target() {
        assert!(!take_profit_triggered(dec!(-10), dec!(2.1), dec!(2.0)));
        assert!(take_profit_triggered(dec!(-10), dec!(2.0), dec!(2.0)));
        assert!(take_profit_triggered(dec!(-10), dec!(1.9), dec!(2.0)));
    }

    #[test]
    fn stop_loss_triggers_for_long_when_price_falls_to_target() {
        assert!(!stop_loss_triggered(dec!(10), dec!(2.1), dec!(2.0)));
        assert!(stop_loss_triggered(dec!(10), dec!(2.0), dec!(2.0)));
        assert!(stop_loss_triggered(dec!(10), dec!(1.9), dec!(2.0)));
    }

    #[test]
    fn stop_loss_triggers_for_short_when_price_rises_to_target() {
        assert!(!stop_loss_triggered(dec!(-10), dec!(1.9), dec!(2.0)));
        assert!(stop_loss_triggered(dec!(-10), dec!(2.0), dec!(2.0)));
        assert!(stop_loss_triggered(dec!(-10), dec!(2.1), dec!(2.0)));
    }

    #[test]
    fn trade_fee_is_notional_times_rate_regardless_of_side() {
        assert_eq!(trade_fee_usd(dec!(1000), dec!(0.0005)), dec!(0.5));
        // Fee doesn't care whether notional came from a long or a short -
        // callers always pass an already-absolute executed notional, but
        // this guards against a negative slipping through unnoticed.
        assert_eq!(trade_fee_usd(dec!(-1000), dec!(0.0005)), dec!(0.5));
    }

    #[test]
    fn funding_rate_is_zero_when_vamm_matches_index() {
        assert_eq!(funding_rate(dec!(2.0), dec!(2.0), dec!(0.005)), dec!(0));
    }

    #[test]
    fn funding_rate_positive_when_vamm_trades_above_index() {
        // 1% premium, capped at 0.5% - clamps to the cap.
        let rate = funding_rate(dec!(2.02), dec!(2.0), dec!(0.005));
        assert_eq!(rate, dec!(0.005));
    }

    #[test]
    fn funding_rate_negative_when_vamm_trades_below_index() {
        let rate = funding_rate(dec!(1.98), dec!(2.0), dec!(0.005));
        assert_eq!(rate, dec!(-0.005));
    }

    #[test]
    fn funding_rate_uncapped_within_bounds() {
        // 0.1% premium, well under the 0.5% cap - passes through un-clamped.
        let rate = funding_rate(dec!(2.002), dec!(2.0), dec!(0.005));
        assert_eq!(rate, dec!(0.001));
    }

    #[test]
    fn funding_rate_zero_when_index_price_is_zero() {
        assert_eq!(funding_rate(dec!(2.0), dec!(0), dec!(0.005)), dec!(0));
    }

    #[test]
    fn long_pays_funding_when_rate_is_positive() {
        // 10-unit long, $20000 notional, 0.1% funding rate -> pays $20.
        let payment = funding_payment_usd(dec!(10), dec!(20000), dec!(0.001));
        assert_eq!(payment, dec!(20));
    }

    #[test]
    fn short_receives_funding_when_rate_is_positive() {
        // Mirror of the long case: same magnitude, opposite sign (receives).
        let payment = funding_payment_usd(dec!(-10), dec!(20000), dec!(0.001));
        assert_eq!(payment, dec!(-20));
    }

    #[test]
    fn short_pays_funding_when_rate_is_negative() {
        let payment = funding_payment_usd(dec!(-10), dec!(20000), dec!(-0.001));
        assert_eq!(payment, dec!(20));
    }

    #[test]
    fn funding_payment_zero_for_zero_size() {
        assert_eq!(funding_payment_usd(dec!(0), dec!(20000), dec!(0.001)), dec!(0));
    }
}
