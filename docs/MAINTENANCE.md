# Maintenance checklist

Routine checks for the Veloce workspace. Run these after dependency bumps and
before cutting a release.

## Workspace

```sh
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

All three must be clean. The test suite includes real-PTY end-to-end cases, so
a passing run is evidence that the terminal is restored on normal exit, panic,
SIGINT, and SIGTERM.

## Layout and runtime

```sh
cargo bench -p veloce --bench frame
```

Record the platform and the numbers. `frame_cost_cached` measures steady-state
frames that reuse the layout cache; `frame_cost` measures cold frames. At 2,000
children they were 160 µs and 2.25 ms respectively on Linux x86_64. Do not
present the cold number as the steady-state cost.

## Sites

```sh
cd site && npm ci && npm run build
cd ../site/docs-site && npm ci && npm run build
```

The marketing site uses self-hosted fonts in `site/public/fonts/`. Verify that
the font files are present in the built output after a build.

## Known boundaries

Linux x86_64 is verified directly. macOS and Windows through WSL / Windows
Terminal use the same portable crossterm and ratatui paths but are not verified
in this environment. Hot reload, mouse-wheel scrolling, and non-terminal
backends are intentionally out of scope for now.
