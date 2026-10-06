---
title: Examples
description: Three runnable apps that cover the framework.
---

# Examples

Each example is a complete app. Run it, then read it — they are the best
documentation for how pieces compose.

## Reference dashboard

Hydration, key-triggered deploys, and a streaming log view.

```bash
cargo run -p veloce --example reference_app
```

Press `d` to deploy, `q` to quit.

## Settings form and log viewer

Two `TextInput` fields with `Tab` focus plus a hydrated `ScrollView` of logs.

```bash
cargo run -p veloce --example settings_log_viewer
```

## Agent harness dashboard

Streams mock tool executions into a live dashboard. Press `r` to run.

```bash
cargo run -p veloce --example agent_dashboard
```

If you kill an example with `SIGKILL`, run `reset` — `SIGINT`, `SIGTERM`,
and panics are handled; `SIGKILL` cannot be.
