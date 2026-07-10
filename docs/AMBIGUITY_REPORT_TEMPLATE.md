# Ambiguity Report: <ID>

## Metadata

- Report ID: `<AMBIGUITY_ID>`
- Related Task: `<TASK_ID or none>`
- Related Milestone: `<MILESTONE_ID>`
- Filed By: `<main task>`
- Date: `YYYY-MM-DD`
- Status: `open | resolved | superseded`
- Required Owner: `main-task language review | main-task semantic design | main task`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/<ADR>.md`
- Milestone:
  - `docs/milestones/<MILESTONE_FILE>.md`

## Exact Ambiguous Text Or Missing Rule

```text
<quote exact text, or state the missing rule>
```

## Competing Interpretations

1. <Interpretation A>
2. <Interpretation B>
3. <Interpretation C, if applicable>

## Why Guessing Is Unsafe

- <soundness risk, diagnostic risk, architecture risk, or roadmap risk>

## Affected Work

- Tasks blocked:
  - `<TASK_ID>`
- Milestones affected:
  - `<MILESTONE_ID>`
- Tests blocked:
  - `<path or fixture category>`
- Implementation areas blocked:
  - `<path or subsystem>`

## Recommended Resolution Path

- [ ] main-task language review determines whether existing text resolves it.
- [ ] main-task semantic design drafts ADR or spec revision if new semantics are required.
- [ ] main-task adversarial check reviews soundness risk.
- [ ] main-task diagnostics check reviews diagnostic consequences.
- [ ] main-task simplicity check reviews complexity.
- [ ] main task approves final resolution.

## Temporary Rule

No implementation may proceed on the ambiguous behavior until the source of truth is updated or the ambiguity is ruled non-blocking by main task.

## Resolution

- Decision:
  - <decision>
- Source of truth updated:
  - `<path>`
- Date resolved:
  - `YYYY-MM-DD`

