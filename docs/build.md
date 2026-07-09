# Build And CI

This project uses a Rust workspace for the compiler implementation. The current workspace is a skeleton only and contains no compiler features.

## Required Local Gates

Run these commands before review:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
```

## CI

CI runs the same gates:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --all-targets`

## Current Scope

The workspace may contain placeholder crates needed to validate build and CI plumbing. It must not contain lexer, parser, AST, semantic analysis, backend, target-pack, or language-runtime behavior until later milestones authorize those areas.

