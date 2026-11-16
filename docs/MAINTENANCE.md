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

Record the platform and the numbers. Per-frame cost is currently linear in the
number of children because trees are rebuilt each frame; a memoized layout
cache is tracked as deferred work. Do not describe the frame cost as flat.

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
in this environment. Hot reload, mouse-wheel scrolling, non-terminal backends,
and the layout cache are intentionally out of scope for now.
