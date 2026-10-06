---
title: Examples reference
description: The three runnable apps and what each covers.
---

# Examples reference

| Example | Run command | Covers |
|---|---|---|
| `reference_app` | `cargo run -p veloce --example reference_app` | Hydration, `d` deploys, streaming logs, `q` quits and restores. |
| `settings_log_viewer` | `cargo run -p veloce --example settings_log_viewer` | Two `TextInput` fields, `Tab` focus, hydrated `ScrollView`. |
| `agent_dashboard` | `cargo run -p veloce --example agent_dashboard` | Mock tool-execution stream; `r` runs, `Live` status flips. |

If you kill an example with `SIGKILL`, run `reset` — `SIGINT`, `SIGTERM`,
and panics are handled; `SIGKILL` cannot be.
