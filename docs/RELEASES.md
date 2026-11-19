# Release checklist

Veloce is published as a set of crates. The facade crate `veloce` depends on
the internal crates, so releases go leaf-crate first.

## Order

1. `veloce-core`
2. `veloce-layout`
3. `veloce-render`
4. `veloce-router`
5. `veloce-runtime`
6. `veloce`

Each crate must be package-validated before it is published. The owner's
crates.io token is required, so publishing is a manual step.

## Before publishing

- [ ] `cargo test --workspace` passes
- [ ] `cargo fmt --all -- --check` is clean
- [ ] `cargo clippy --all-targets -- -D warnings` has no warnings
- [ ] `cargo package --allow-dirty --list` shows only intended files for each crate
- [ ] README quickstart compiles against the version being released
- [ ] `CHANGELOG.md` has an entry for the release
- [ ] Benchmarks are recorded with the platform they were measured on

## After publishing

- [ ] Tag the release in git
- [ ] Update the docs site if a public API changed
- [ ] Confirm `veloce = "0.1"` resolves from a clean Cargo project

Until the crates.io release happens, the manifests are package-validated but
the published crate path is intentionally absent. The FAQ says this plainly.
