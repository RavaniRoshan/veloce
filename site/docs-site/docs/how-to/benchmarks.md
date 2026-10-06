---
title: Run the benchmarks
description: Recipe for reproducing every performance number.
---

# Run the benchmarks

**Problem:** reproduce the framework's performance claims on your machine.

**Steps:**

1. Run the criterion suite:

```bash
cargo bench -p veloce --bench frame
```

2. Read `frame_cost/children/{10,100,500,2000}` for per-frame render cost and
   `cold_boot_first_frame` for boot time.
3. Run the process probes:

```bash
cargo test -p veloce-runtime --test perf -- --nocapture
cargo test -p veloce-runtime --test dispatch -- --nocapture
```

4. Compare `COLD_BOOT_MS`, `IDLE_RSS_KIB`, and `KEY_LATENCY_P95_US` against
   the budgets in [Benchmarks](/reference/benchmarks).

**Note:** per-frame cost scales with child count (see the honest caveat on
the Benchmarks page) — record your machine's numbers before comparing.
