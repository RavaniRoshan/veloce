---
title: FAQ
description: Usage questions, answered with links.
---

# FAQ

## Do I write my own event loop?

No. `VeloceApp::run` owns the ingestion thread, the Tokio runtime, the
priority drain, and the 60 FPS flush. See [Architecture](/explanation/architecture).

## How do background tasks update the UI?

Through `cx.dispatcher()`. See [Update the UI from background tasks](/how-to/background-tasks).

## How do I handle keyboard input?

Implement `handle_key`; return `true` to consume. See
[Handle keys and move focus](/how-to/keys-focus).

## How does flexbox work on a cell grid?

taffy plus a quantization pass. See [Layout on a cell grid](/explanation/layout).

## What happens when my view panics?

The terminal is restored before the backtrace prints. See
[Terminal safety](/explanation/terminal-safety).

## Does it work on macOS and Windows?

Linux x86_64 is verified. macOS and Windows (WSL/Windows Terminal) use the
same portable paths but are not verified in this environment.

## Is it on crates.io?

Not yet. Releases will go leaf-crate first once the owner's token is configured.

## What is intentionally out of scope?

Hot-reload/dev-server, mouse-wheel scrolling, non-terminal backends, and a
memoized layout cache.
