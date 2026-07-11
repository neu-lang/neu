# ADR-0057 Simplicity Review

Decision: approve. A pack-owned linker and startup object with one explicit
resolver boundary is smaller than a compiler-owned native linker and avoids a
parallel implicit host-tool discovery path.
