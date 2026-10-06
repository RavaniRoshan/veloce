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

One honest caveat: per-frame cost is not flat as content grows (about
197 µs at 10 children rising to 1.44 ms at 2,000), because the element and
layout trees rebuild each frame. A memoized layout cache is the planned fix.
