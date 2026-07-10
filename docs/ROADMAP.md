# Compiler Implementation Roadmap

Status: Draft planning roadmap

Source of truth: `docs/SPEC.md` and `docs/adr/`. This roadmap does not modify language semantics.

## Project Phases

### Phase 0: Governance, Build, And Testing Foundations

- M0001: Source of Truth Alignment
- M0002: Rust Workspace And CI Skeleton
- M0003: Test Harness And Golden Fixture Layout
- M0004: Diagnostic Infrastructure Contract
- M0005: Source Database, Spans, And File Identity

### Phase 1: Lexical And Syntactic Frontend

- M0006: Token Model And Lexer Fixtures
- M0007: Lexer Implementation
- M0008: Grammar Authority And Syntax Ambiguity Ledger
- M0009: AST Data Model
- M0010: Parser Recovery Architecture
- M0011: Declaration Parser
- M0012: Type And Generic Syntax Parser
- M0013: Expression, Statement, And Pattern Parser

### Phase 2: Binding And Type Frontend

- M0014: Module, Package, And Visibility Model
- M0015: Symbol Interning And Name Tables
- M0016: Name Resolution Pass
- M0017: Type Representation
- M0018: Type Checking Core
- M0019: Nullability And Flow Typing
- M0020: Generic Parameter And Capability-Bound Representation
- M0021: Algebraic Data And Exhaustiveness

### Phase 3: Safety Semantics

- M0022: Ownership And Move Analysis
- M0023: Borrow And Lifetime Analysis
- M0024: Thread Safety Capability Analysis
- M0025: Coroutine Scope And Suspension Analysis
- M0026: Unsafe And FFI Boundary Analysis

### Phase 4: Executable Semantics And Intermediate Representations

- M0027: Executable Semantics Planning
- M0028: Executable Expression Frontend Completion
- M0029: HIR Design And Lowering
- M0030: MIR Design And Lowering

### Phase 5: Backend And Portability

- M0031: Cranelift Backend Smoke
- M0032: Object And Bundled Linker Pipeline
- M0033: Target Packs And Cross Compilation Smoke

### Phase 6: Release Hardening

- M0034: Milestone Release Hardening

## Milestone Ordering

1. M0001 Source of Truth Alignment
2. M0002 Rust Workspace And CI Skeleton
3. M0003 Test Harness And Golden Fixture Layout
4. M0004 Diagnostic Infrastructure Contract
5. M0005 Source Database, Spans, And File Identity
6. M0006 Token Model And Lexer Fixtures
7. M0007 Lexer Implementation
8. M0008 Grammar Authority And Syntax Ambiguity Ledger
9. M0009 AST Data Model
10. M0010 Parser Recovery Architecture
11. M0011 Declaration Parser
12. M0012 Type And Generic Syntax Parser
13. M0013 Expression, Statement, And Pattern Parser
14. M0014 Module, Package, And Visibility Model
15. M0015 Symbol Interning And Name Tables
16. M0016 Name Resolution Pass
17. M0017 Type Representation
18. M0018 Type Checking Core
19. M0019 Nullability And Flow Typing
20. M0020 Generic Parameter And Capability-Bound Representation
21. M0021 Algebraic Data And Exhaustiveness
22. M0022 Ownership And Move Analysis
23. M0023 Borrow And Lifetime Analysis
24. M0024 Thread Safety Capability Analysis
25. M0025 Coroutine Scope And Suspension Analysis
26. M0026 Unsafe And FFI Boundary Analysis
27. M0027 Executable Semantics Planning
28. M0028 Executable Expression Frontend Completion
29. M0029 HIR Design And Lowering
30. M0030 MIR Design And Lowering
31. M0031 Cranelift Backend Smoke
32. M0032 Object And Bundled Linker Pipeline
33. M0033 Target Packs And Cross Compilation Smoke
34. M0034 Milestone Release Hardening

## Dependency Graph

```text
M0001
  -> M0002
  -> M0003
  -> M0004
  -> M0005
  -> M0006
  -> M0007
  -> M0008
  -> M0009
  -> M0010
  -> M0011
  -> M0012
  -> M0013
  -> M0014
  -> M0015
  -> M0016
  -> M0017
  -> M0018
  -> M0019
  -> M0020
  -> M0021
  -> M0022
  -> M0023
  -> M0024
  -> M0025
  -> M0026
  -> M0027
  -> M0028
  -> M0029
  -> M0030
  -> M0031
  -> M0032
  -> M0033
  -> M0034
```

Each milestone may depend only on earlier milestones. No milestone depends on later work.

## Estimated Effort

| Phase | Milestones | Estimated effort |
| --- | --- | --- |
| Phase 0 | M0001-M0005 | 9-15 working days |
| Phase 1 | M0006-M0013 | 18-32 working days |
| Phase 2 | M0014-M0021 | 22-36 working days |
| Phase 3 | M0022-M0026 | 18-25 working days |
| Phase 4 | M0027-M0030 | 13-22 working days |
| Phase 5 | M0031-M0033 | 9-15 working days |
| Phase 6 | M0034 | 3-5 working days |

Total critical-path estimate: 92-150 working days for one serial implementation lane.

## Project Critical Path

The critical path is the full milestone chain from M0001 through M0034 because every phase builds on the prior phase's architecture and validation surface. Parallel work should be limited to review, test design, and ambiguity resolution for future milestones. Implementation should remain strictly ordered until the frontend contracts stabilize.

Critical path checkpoints:

- M0005 proves source identity and diagnostics can be trusted.
- M0013 proves the parser can produce AST for the accepted syntax subset.
- M0018 proves typed frontend behavior exists before safety passes.
- M0025 proves ownership, borrow, thread, and coroutine analyses can coexist.
- M0027 proves executable semantics are source-of-truth decisions, not backend
  inventions.
- M0028 proves the frontend accepts and rejects the first executable subset.
- M0030 proves backend-independent lowering exists.
- M0033 proves Go-like target-pack direction is viable.

## Optional Milestones

Optional milestones are not on the initial critical path:

- Documentation site generation for language reference drafts.
- Fuzzing farm integration beyond local fuzz smoke tests.
- IDE protocol integration.
- Incremental compilation cache.
- Formatter.
- Package manager.
- LLVM backend prototype.
- MIR optimization pipeline.

## Future Milestones Intentionally Deferred

- Production optimization passes.
- Stable ABI guarantees.
- Full standard library.
- Macro system or general metaprogramming beyond bounded compile-time evaluation.
- Higher-kinded types.
- Generic constraint enforcement until after M0024 capability semantics and a
  separate accepted ADR define satisfaction and diagnostics.
- Advanced coroutine scheduler implementation.
- Multi-backend backend abstraction for LLVM.
- Binary distribution signing and update channels.

## Global Roadmap Constraints

- No milestone may invent language semantics.
- Ambiguity in `docs/SPEC.md` or `docs/adr/` must become an explicit dependency.
- Test milestones and diagnostic expectations must precede implementation milestones.
- Existing implementation behavior is never source of truth.
- Cranelift is the initial backend.
- LLVM remains deferred unless a future ADR and roadmap revision approve it.
