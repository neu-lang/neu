# `stdlib/core`

This directory contains the dependency-free foundational Neu library. The
types here are ordinary Neu declarations; they do not rely on compiler-private
runtime or allocation APIs.

The initial implementation provides the closed foundational enums specified by
ADR-0107 and ADR-0109:

- `Option<T>`: `Some(T)` and `None`;
- `Result<T, E>`: `Ok(T)` and `Err(E)`;
- `Ordering`: `Less`, `Equal`, and `Greater`.
- explicit `Int` utilities: `min`, `max`, `clamp`, `abs`, and `sign`.

Operations are added only when their ownership, generic, error, and runtime
contracts are accepted in `docs/SPEC.md` and the corresponding ADRs.
