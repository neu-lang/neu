# ADR-0049 Adversarial Review

## Metadata

- Proposal: `ADR-0049`
- Milestone: `M0028`
- Review: `main-task adversarial review`
- Date: `2026-07-11`
- Decision: `approve`

## Review

Cross-package and cross-source `main` declarations cannot satisfy, hide, or
misattribute the selected-package entry contract. Every duplicate candidate is
diagnosed at its own source declaration, and no source node is fabricated when
the candidate set is empty.
