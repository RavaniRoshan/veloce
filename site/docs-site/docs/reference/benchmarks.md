---
title: Benchmarks
description: Measured numbers against the budgets.
---

# Benchmarks

Measured on Linux x86_64 with criterion and process probes. Reproduce them
with [Run the benchmarks](/how-to/benchmarks).

| Budget | Measured | Status |
| --- | --- | --- |
| Cold boot to first frame < 15 ms | 3.55 ms (criterion) | Pass |
| Idle memory < 12 MiB | 4 MiB VmRSS | Pass |
| Key echo under 1,000 actions/s | p95 10 µs | Pass |
| 10,000 interleaved events, no deadlock | Pass | Pass |

Repeated frames reuse the last layout tree while the element tree and terminal
size are unchanged. That costs 94 µs at 10 children and 160 µs at 2,000. A cold
frame after a change still pays full layout, 180 µs to 2.25 ms at the same
sizes. Cache equivalence is covered by regression tests for text, scroll, and
resize changes.
