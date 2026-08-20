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

## Where the starting numbers in `src/setup/` come from

Every club and player's starting `intrinsic_value` is a **real, computed baseline** -
not hand-picked, not arbitrary, and not invented by this engine out of nothing.

The starting point traces back to real transfer fees and market values (see the
conversion comment in `transfers.rs` - a disclosed fee like "€75M" maps directly onto
the engine's internal valuation-unit scale, chosen specifically to line up with
real-data-backfilled player values already on that same scale). From there, the
numbers you see in `src/setup/` are produced by `ssx-backfill` - a one-off tool in the
private application repo (not included here, since it writes directly to Postgres,
QuestDB, and Redis) - which:

1. Seeds this exact engine from its previous baseline.
2. Fetches every real fixture, match event (goals, cards), and per-player match
   statistic for the completed season across all five leagues from API-Football.
3. Replays those real events **chronologically, through this exact engine** -
   deterministically, with the ambient heartbeat noise turned off - so a real goal
   moves a club's value by precisely the same formula that runs live in production,
   nothing softer or hand-tuned for the backfill.
4. Updates each player's `form_weight` from real per-match performance stats.
5. Mechanically rewrites just the two numbers per club/player (`intrinsic_value`,
   `form_weight`) in these seed files to the resulting end-of-season state. Names,
   positions, captain flags, and rivalry weightings are never touched by this process -
   set once, independently, and left alone.

This is a repeatable process, not a one-time guess: re-running
`ssx-backfill fetch|replay|apply` is how the baseline gets refreshed going forward.

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
