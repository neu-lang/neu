# ADR-0049 Simplicity Review

## Metadata

- Proposal: `ADR-0049`
- Milestone: `M0028`
- Review: `main-task simplicity review`
- Date: `2026-07-11`
- Decision: `approve`

## Review

The source-or-external-input rule is the minimum contract needed for a
package-level missing-entry diagnostic. It adds neither a general diagnostics
framework nor module-loading behavior.
