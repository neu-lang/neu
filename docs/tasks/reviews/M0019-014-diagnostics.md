Role: main-task diagnostics check
Diagnostic area: M0019-014 immutable-local `const` migration.

Inputs read: main-task language review ruling; task Authority Extract and log; `docs/SPEC.md` ADR-0022/ADR-0029 sections; accepted ADR-0022 and ADR-0029 diagnostic sections; parser diff and tests; main task instructions.

Expected error condition: `fun const();` receives ordinary `MissingDeclarationName`; removed-introducer `val removed = 1;` receives ordinary `UnexpectedTokenInStatement` and no binding or compatibility recovery.

Required message qualities: ADR-0022 controls `MissingDeclarationName`, whose primary span is the declaration keyword `fun`. The removed-`val` diagnostic spans the unexpected `val`. Neither path uses migration wording, a fix-it, or compile-time terminology.

Tests/snapshots: Parser tests (35), M0019 migration validator, M0013 fixture validator, formatting, and `git diff --check` pass. The task log records broader validation passing.

Findings: Prior lower-authority conclusion requiring the `const` span is withdrawn. Current implementation and assertions match ADR-0022 and ADR-0029. No residual diagnostic risk found.

Decision: APPROVE.

Files changed: `docs/tasks/reviews/M0019-014-diagnostics.md` only. Open questions: none. Blockers: none. Handoff: main-task review for final task closure.
