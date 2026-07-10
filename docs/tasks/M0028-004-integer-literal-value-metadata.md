# Task: M0028-004 Integer Literal Value Metadata

## Task Metadata

- Task ID: `M0028-004`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task test work`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADR: `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`
- Project Rules: `AGENTS.md`

## Objective

Preserve parser metadata for valid decimal, binary, and hexadecimal integer
literals so later M0028 static diagnostics can evaluate bootstrap `Int` facts.

## Authority Extract

- ADR-0043, “Recommended Choice”: signed 64-bit `Int`, range diagnostics, and
  statically provable arithmetic failures.
- `docs/SPEC.md`, “ADR-0043: Bootstrap Integer Runtime Semantics”.
- `crates/compiler/src/lexer.rs` integer-token categories.
- `crates/compiler/src/parser.rs` literal metadata.

## Scope

- Add integer-literal parser metadata associated with its expression and span.
- Decode decimal, binary, and hexadecimal literal digits after underscores.
- Represent every value that fits `u64`, including `9223372036854775808` needed
  to recognize the valid unary spelling of minimum bootstrap `Int`.
- Represent an unbounded-source literal that exceeds `u64` as unavailable value
  metadata rather than truncating or wrapping.

## Out Of Scope

- `integer_literal_out_of_range` diagnostics.
- Constant-expression evaluation and arithmetic diagnostics.
- Type-check core integration, runtime behavior, or code generation.

## Required Tests

- Decimal, binary, hexadecimal, and underscore-separated values preserve their
  decoded `u64` values.
- The unsigned magnitude of minimum `Int` is preserved.
- A literal above `u64` has no decoded value and is not truncated.

## Test-First Gate

- Test files to create before implementation:
  - `crates/compiler/tests/parser.rs`
  - `docs/tests/m0028-integer-literal-value-metadata.sh`
- Expected pre-implementation result: `fail`.
- Failure reason: integer literal values are not present in parser output.
- Reviewer approval required to modify/delete failing tests: `yes`

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Radix and underscore forms decode without changing source spans.
- [x] Values do not wrap or truncate.
- [x] Later diagnostic work can distinguish `u64`-representable literals from larger values.
- [x] Ordinary tests, adversarial check, review, and CI pass.

## Execution Commands

- Generate tests: `cargo test -p compiler --test parser`.
- Verify tests fail: `cargo test -p compiler --test parser`.
- Ordinary tests: `cargo test --workspace --all-targets`.
- Adversarial tests: `sh docs/scripts/adversarial-check.sh docs/tasks/M0028-004-integer-literal-value-metadata.md`.
- Review: `docs/scripts/review-task.sh docs/tasks/M0028-004-integer-literal-value-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets`.

## Files Expected To Change

- Tests: `crates/compiler/tests/parser.rs` and
  `docs/tests/m0028-integer-literal-value-metadata.sh`.
- Implementation: `crates/compiler/src/parser.rs`.
- Documentation: this task file.

## Forbidden Changes

- Do not modify `docs/SPEC.md` or accepted ADRs.
- Do not add range or arithmetic diagnostics in this metadata-only task.
- Do not weaken or delete failing tests without reviewer approval.

## Execution Log

- 2026-07-11 agent=Main phase=create-task result=pass evidence=bounded integer literal metadata task created from ADR-0043. handoff=Test
- 2026-07-11 agent=Main phase=test-first result=pass evidence=parser test failed because ParseOutput lacked integer_literals. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=parser retains optional u64 values for decimal, binary, and hexadecimal literals. handoff=Validation
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=cargo test --workspace --all-targets passed 275 tests in 14 suites. handoff=Adversarial
- 2026-07-11 agent=Main phase=adversarial result=pass evidence=radix values, minimum Int magnitude, and too-large literals cannot truncate; report in docs/tasks/soundness/M0028-004-soundness.md. handoff=Review
- 2026-07-11 agent=Main phase=review result=pass evidence=scope and ADR-0043 metadata compliance approved; report in docs/tasks/reviews/M0028-004-review.md. handoff=CI
- 2026-07-11 agent=Main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; M0028 metadata validator; agent-configs. handoff=Commit

## Handoff

- Next main task: `main-task implementation`.
- Reason: the tests must first show that parser output lacks literal values.
