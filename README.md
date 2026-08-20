# ssx-valuation

The event-driven pricing engine behind [Mirai Trade](https://miraitrade.xyz) - leveraged
perpetuals on real football clubs, priced by the sport itself.

This is a real mirror of the engine's actual development history (extracted from the
private application monorepo via `git subtree split`), not a curated snapshot written
for public consumption after the fact. What you see here - including its bugs and the
commits that fixed them - is what actually shipped.

## What this is

- **Event-driven valuation** (`src/lib.rs`) - goals, cards, transfers, and injuries
  replay through deterministic handlers the moment they happen, each scaled by a
  rivalry multiplier for genuine derbies.
- **Calibrated ambient volatility** (`heartbeat_multiplier`, `heartbeat_tick_half_width`)
  - a random walk that adds movement between real events without drifting or
  swamping real results. The `heartbeat_drift_tests` module documents a real bug this
  engine used to have (a systematic downward drift from Jensen's inequality, not
  hypothetical) and the fix, with the regression test that would catch it coming back.
- **The vAMM** (`src/trading/vamm.rs`) - constant-product (`x·y=k`) liquidity for every
  club pair, with periodic repeg toward the index price.
- **Risk math** (`src/trading/risk.rs`) - maintenance margin, liquidation triggers.
- **The full 96-club roster** (`src/setup/`) - Premier League, La Liga, Serie A,
  Bundesliga, Ligue 1, with real rivalry weightings, no hidden favoritism.

## What this deliberately isn't

This is the pricing engine only. It doesn't include (and never has, in this repo):
custody/wallet code, the trading API, admin tooling, database or Redis integration, or
anything that touches real funds. That code lives in a private repo alongside this
engine, for the reason you'd expect - security through actual access control, not
security through obscurity of the math, which is the whole point of open-sourcing this
part specifically.

## Verify it yourself

```sh
cargo build
cargo test
```

78 tests, standalone, no external services required - the same suite this engine has
to pass in the private monorepo before any change ships.

```sh
make visualize   # renders simulation_results.csv - a real calibration run's output
```

## License

MIT - see [LICENSE](./LICENSE).
