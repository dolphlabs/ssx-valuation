use rust_decimal::{Decimal, MathematicalOps};
use serde::{Deserialize, Serialize};

/// A lazily-seeded constant-product (x*y=k) virtual liquidity pool for one
/// club pair. `reserve_quote / reserve_base` is the pool's price, quoted as
/// quote-per-base (the same convention as `ValuationEngine::get_club_pair_exchange_rate`).
/// Reserves are virtual — not backed by real capital — purely to give trades
/// mechanical price impact instead of filling at a single flat index price.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VammPool {
    pub reserve_base: Decimal,
    pub reserve_quote: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VammError {
    InvalidNotional,
    InsufficientDepth,
}

impl std::fmt::Display for VammError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VammError::InvalidNotional => write!(f, "trade notional must be positive"),
            VammError::InsufficientDepth => write!(f, "insufficient vAMM depth for this trade size"),
        }
    }
}

impl std::error::Error for VammError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwapResult {
    /// Average execution price for this fill, quote-per-base.
    pub fill_price: Decimal,
    /// Signed change in the trader's base-club-denominated position size:
    /// positive for a long (base gained), negative for a short (base owed).
    pub size_delta: Decimal,
    pub pool_after: VammPool,
}

impl VammPool {
    /// Seeds a new pool so its price matches `index_price` exactly, sized by
    /// `virtual_depth_quote` (the quote-denominated reserve) — bigger depth
    /// means less price impact per unit of trade notional.
    pub fn seed(index_price: Decimal, virtual_depth_quote: Decimal) -> Self {
        Self {
            reserve_quote: virtual_depth_quote,
            reserve_base: virtual_depth_quote / index_price,
        }
    }

    pub fn price(&self) -> Decimal {
        self.reserve_quote / self.reserve_base
    }

    pub fn k(&self) -> Decimal {
        self.reserve_base * self.reserve_quote
    }

    /// Opens/adds to a long: `quote_notional_in` (quote-club units) is swapped
    /// into the pool, base flows out to the trader.
    pub fn open_long(&self, quote_notional_in: Decimal) -> Result<SwapResult, VammError> {
        if quote_notional_in <= Decimal::ZERO {
            return Err(VammError::InvalidNotional);
        }
        let k = self.k();
        let new_reserve_quote = self.reserve_quote + quote_notional_in;
        let new_reserve_base = k / new_reserve_quote;
        let base_out = self.reserve_base - new_reserve_base;
        if base_out <= Decimal::ZERO || base_out >= self.reserve_base {
            return Err(VammError::InsufficientDepth);
        }
        Ok(SwapResult {
            fill_price: quote_notional_in / base_out,
            size_delta: base_out,
            pool_after: VammPool {
                reserve_base: new_reserve_base,
                reserve_quote: new_reserve_quote,
            },
        })
    }

    /// Opens/adds to a short: `quote_notional_out` (quote-club units) flows
    /// out of the pool to the trader, base flows in.
    pub fn open_short(&self, quote_notional_out: Decimal) -> Result<SwapResult, VammError> {
        if quote_notional_out <= Decimal::ZERO {
            return Err(VammError::InvalidNotional);
        }
        if quote_notional_out >= self.reserve_quote {
            return Err(VammError::InsufficientDepth);
        }
        let k = self.k();
        let new_reserve_quote = self.reserve_quote - quote_notional_out;
        let new_reserve_base = k / new_reserve_quote;
        let base_in = new_reserve_base - self.reserve_base;
        if base_in <= Decimal::ZERO {
            return Err(VammError::InsufficientDepth);
        }
        Ok(SwapResult {
            fill_price: quote_notional_out / base_in,
            size_delta: -base_in,
            pool_after: VammPool {
                reserve_base: new_reserve_base,
                reserve_quote: new_reserve_quote,
            },
        })
    }

    /// Closes (fully) a position of `position_size_signed` base-club units
    /// (positive = closing a long by selling base back into the pool,
    /// negative = closing a short by buying base back from the pool) against
    /// this pool. This is the exact inverse of `open_long`/`open_short`,
    /// parameterized by base size (what a position actually holds) instead
    /// of quote notional (what opening a new position is sized by).
    pub fn close(&self, position_size_signed: Decimal) -> Result<SwapResult, VammError> {
        if position_size_signed == Decimal::ZERO {
            return Err(VammError::InvalidNotional);
        }
        let k = self.k();
        if position_size_signed > Decimal::ZERO {
            // Closing a long: sell `position_size_signed` base into the pool.
            let new_reserve_base = self.reserve_base + position_size_signed;
            let new_reserve_quote = k / new_reserve_base;
            let quote_out = self.reserve_quote - new_reserve_quote;
            if quote_out <= Decimal::ZERO {
                return Err(VammError::InsufficientDepth);
            }
            Ok(SwapResult {
                fill_price: quote_out / position_size_signed,
                size_delta: -position_size_signed,
                pool_after: VammPool { reserve_base: new_reserve_base, reserve_quote: new_reserve_quote },
            })
        } else {
            // Closing a short: buy back |position_size_signed| base from the pool.
            let base_amount = position_size_signed.abs();
            if base_amount >= self.reserve_base {
                return Err(VammError::InsufficientDepth);
            }
            let new_reserve_base = self.reserve_base - base_amount;
            let new_reserve_quote = k / new_reserve_base;
            let quote_in = new_reserve_quote - self.reserve_quote;
            if quote_in <= Decimal::ZERO {
                return Err(VammError::InsufficientDepth);
            }
            Ok(SwapResult {
                fill_price: quote_in / base_amount,
                size_delta: base_amount,
                pool_after: VammPool { reserve_base: new_reserve_base, reserve_quote: new_reserve_quote },
            })
        }
    }

    /// Resets the pool's price back to `index_price` while preserving `k`,
    /// so its depth/slippage character stays consistent. Without this, a
    /// pool drifts arbitrarily far from fundamentals over time since each
    /// club's `intrinsic_value` random-walks independently of the pool.
    pub fn repeg(&self, index_price: Decimal) -> Option<Self> {
        if index_price <= Decimal::ZERO {
            return None;
        }
        let k = self.k();
        // reserve_base^2 = k / index_price, given reserve_quote/reserve_base = index_price
        let reserve_base = (k / index_price).sqrt()?;
        if reserve_base <= Decimal::ZERO {
            return None;
        }
        let reserve_quote = k / reserve_base;
        Some(Self {
            reserve_base,
            reserve_quote,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn approx_eq(a: Decimal, b: Decimal, tolerance: Decimal) -> bool {
        (a - b).abs() <= tolerance
    }

    #[test]
    fn seed_matches_index_price_exactly() {
        let pool = VammPool::seed(dec!(2.0), dec!(100000));
        assert_eq!(pool.price(), dec!(2.0));
        assert_eq!(pool.reserve_quote, dec!(100000));
    }

    #[test]
    fn open_long_moves_price_up_and_preserves_k() {
        let pool = VammPool::seed(dec!(2.0), dec!(100000));
        let k_before = pool.k();
        let result = pool.open_long(dec!(1000)).unwrap();
        assert!(result.pool_after.price() > pool.price());
        assert!(result.size_delta > Decimal::ZERO);
        assert!(approx_eq(result.pool_after.k(), k_before, dec!(0.0001)));
        // Fill price should be worse (higher) than the pre-trade price due to slippage.
        assert!(result.fill_price > pool.price());
    }

    #[test]
    fn open_short_moves_price_down_and_preserves_k() {
        let pool = VammPool::seed(dec!(2.0), dec!(100000));
        let k_before = pool.k();
        let result = pool.open_short(dec!(1000)).unwrap();
        assert!(result.pool_after.price() < pool.price());
        assert!(result.size_delta < Decimal::ZERO);
        assert!(approx_eq(result.pool_after.k(), k_before, dec!(0.0001)));
        assert!(result.fill_price < pool.price());
    }

    #[test]
    fn larger_trades_incur_more_slippage() {
        let pool = VammPool::seed(dec!(2.0), dec!(100000));
        let small = pool.open_long(dec!(100)).unwrap();
        let large = pool.open_long(dec!(10000)).unwrap();
        let small_slippage = small.fill_price - pool.price();
        let large_slippage = large.fill_price - pool.price();
        assert!(large_slippage > small_slippage);
    }

    #[test]
    fn open_short_rejects_notional_exceeding_depth() {
        let pool = VammPool::seed(dec!(2.0), dec!(100000));
        assert_eq!(pool.open_short(dec!(200000)), Err(VammError::InsufficientDepth));
    }

    #[test]
    fn rejects_non_positive_notional() {
        let pool = VammPool::seed(dec!(2.0), dec!(100000));
        assert_eq!(pool.open_long(dec!(0)), Err(VammError::InvalidNotional));
        assert_eq!(pool.open_long(dec!(-5)), Err(VammError::InvalidNotional));
        assert_eq!(pool.open_short(dec!(0)), Err(VammError::InvalidNotional));
    }

    #[test]
    fn repeg_restores_index_price_and_preserves_k() {
        let pool = VammPool::seed(dec!(2.0), dec!(100000));
        let k_before = pool.k();
        let drifted = pool.open_long(dec!(5000)).unwrap().pool_after;
        assert_ne!(drifted.price(), dec!(2.0));

        let repegged = drifted.repeg(dec!(2.0)).unwrap();
        assert!(approx_eq(repegged.price(), dec!(2.0), dec!(0.0000001)));
        assert!(approx_eq(repegged.k(), k_before, dec!(0.01)));
    }

    #[test]
    fn repeg_tracks_a_new_index_price_after_fundamentals_move() {
        let pool = VammPool::seed(dec!(2.0), dec!(100000));
        let repegged = pool.repeg(dec!(3.5)).unwrap();
        assert!(approx_eq(repegged.price(), dec!(3.5), dec!(0.0000001)));
    }

    #[test]
    fn closing_a_long_fully_unwinds_the_position() {
        let pool = VammPool::seed(dec!(2.0), dec!(100000));
        let open = pool.open_long(dec!(1000)).unwrap();
        let closed = open.pool_after.close(open.size_delta).unwrap();
        // Closing fully must exactly zero out the position.
        assert_eq!(open.size_delta + closed.size_delta, Decimal::ZERO);
        // Round-tripping open+close through slippage should lose value, not gain it.
        assert!(closed.pool_after.price() < open.pool_after.price());
    }

    #[test]
    fn closing_a_short_fully_unwinds_the_position() {
        let pool = VammPool::seed(dec!(2.0), dec!(100000));
        let open = pool.open_short(dec!(1000)).unwrap();
        assert!(open.size_delta < Decimal::ZERO);
        let closed = open.pool_after.close(open.size_delta).unwrap();
        assert_eq!(open.size_delta + closed.size_delta, Decimal::ZERO);
        assert!(closed.pool_after.price() > open.pool_after.price());
    }

    #[test]
    fn close_rejects_zero_size() {
        let pool = VammPool::seed(dec!(2.0), dec!(100000));
        assert_eq!(pool.close(dec!(0)), Err(VammError::InvalidNotional));
    }
}
