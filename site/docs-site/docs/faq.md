---
title: FAQ
description: Usage questions, answered.
---

# FAQ

## Do I write my own event loop?

No. `VeloceApp::run` owns the ingestion thread, the Tokio runtime, the
priority drain, and the 60 FPS flush. You implement `View` and declare routes.

## How do background tasks update the UI?

Through `cx.dispatcher()` — a cloneable typed sink. Move it into any future;
dispatched actions land in `update()` on the UI thread.

## How do I handle keyboard input?

Implement `handle_key` — it runs before global routing. Return `true` to
consume the event. `q` quits via `VeloceApp::run`.

## How does flexbox work on a cell grid?

taffy computes continuous layout; the framework rounds absolute edges once,
clamps children inside parents, and forces a 1-cell minimum on positive
sizes. Text wraps via a measure function, so wrapping participates in layout.

## What happens when my view panics?

A panic hook restores the terminal before the backtrace prints. `SIGINT` and
`SIGTERM` are trapped the same way.

## Does it work on macOS and Windows?

Linux x86_64 is verified. macOS and Windows (WSL/Windows Terminal) use the
same portable paths but are not verified in this environment.

## Is it on crates.io?

Not yet. Releases will go leaf-crate first once the owner's token is configured.
