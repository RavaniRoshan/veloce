# Contributing to Veloce

Thanks for helping improve Veloce. This document covers the local workflow and
the bar every change has to meet before it is merged.

## Setup

```sh
rustup default stable
cargo test --workspace
```

The marketing site and the Blume docs site live in `site/` and
`site/docs-site/`. Each is a Vite app with its own `package.json`.

## The bar for a change

A claim is only true when an automated test that drives the real code passes.
Reading the code is not proof. Add or update a test alongside the change:

- unit tests for pure logic
- `insta` snapshots for rendered output
- `proptest` for layout invariants on the cell grid
- real-PTY end-to-end tests for terminal restore on exit, panic, SIGINT, and SIGTERM

## Checks before opening a PR

```sh
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

For layout or runtime work, also run the criterion baselines and record the
machine used, since timing numbers are only meaningful with context.

## Docs and site

User-facing documentation lives on the Blume docs site. Process notes about
implementation phases are not published there. When you change a public API,
update the docs page and the README example together.

## Pull requests

Keep one concern per PR. Describe what the change does, name the test that
proves it, and note any measured numbers with the platform they came from.
