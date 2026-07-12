# Neu

Neu is an experimental systems programming language and compiler project.
The language is designed around Kotlin-like syntax and ergonomics, compile-time
memory and thread safety, inferred ownership effects, structured concurrency,
and a compiler implemented in Rust.

> **Weekend-project disclaimer:** This project is largely vibe coded and began
> as an attempt to make a boring weekend substantially less boring. Treat the
> specification, compiler, and roadmap accordingly.

The compiler currently uses a Cranelift backend and supports host-only system
linking. It emits a host object and links it with `cc`; `NEU_LINKER` can override
the linker command. Non-host targets are rejected explicitly. The repository
itself is the compiler project, not a Neu application module.

## Requirements

- Rust toolchain with Rust 2024 support
- A host C compiler and linker available as `cc`

## Build The CLI

```sh
cargo build --workspace
```

The CLI binary is `target/debug/neu`.

## Build An Example Project

Neu application projects use a `neu.json` manifest. The repository includes a
small manifest project under `examples/projects/manifest_app`:

```sh
cargo run -p neu -- build examples/projects/manifest_app
examples/projects/manifest_app/target/manifest.app
echo $?
```

The example exits with status `7`. An explicit output path is also supported:

```sh
cargo run -p neu -- build examples/projects/manifest_app \
  --output /tmp/manifest_app
```

The compiler leaves an intermediate object file beside the requested output.

## Examples

Runnable language examples are in [`examples/current`](examples/current),
including arithmetic, control flow, arrays, strings, functions, classes,
interfaces, enums, and ownership behavior. Some files intentionally document
accepted or deferred surfaces and are exercised by compiler tests rather than
being standalone projects.

## Validation

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
git diff --check
```

## Scope

Neu has no garbage collector or user-visible manual allocation API. Printing,
standard-library modules, FFI, slices, dynamic cross-compilation, and other
deferred features are not part of the current bootstrap project contract.

The language specification is [`docs/SPEC.md`](docs/SPEC.md). Accepted
architecture decisions are in [`docs/adr`](docs/adr).
