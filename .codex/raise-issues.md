# Raising Issues

File issues at [neu-lang/neu](https://github.com/neu-lang/neu/issues) only after checking for an existing report. Keep each issue focused on one bug, missing behavior, or proposed change.

Use a concise, descriptive title that names the affected area and outcome. In the body, include:

- A short summary of the problem or request.
- The expected behavior and the actual behavior.
- Minimal reproduction steps, source, command, and complete diagnostic output for bugs.
- Relevant environment details, such as the commit, host target, and toolchain version.
- Scope, acceptance criteria, and any known constraints or related issues.

Link the relevant specification or ADR when the report concerns Neu language behavior. Do not claim a semantic decision is required unless the specification or accepted ADR leaves the behavior ambiguous; describe the ambiguity and competing interpretations instead. Avoid bundling unrelated work, speculative implementation plans, or sensitive information.

## Priority

Set the repository-level `Priority` Issue Field for every issue; do not use
priority labels. Choose the value that matches the report's impact:

- `High` for substantial compiler cost or a broadly blocking problem.
- `Medium` for a meaningful but bounded cost or improvement.
- `Low` for an isolated or inspection-only improvement.

Use GitHub's issue-field value API when automation is needed. Confirm the
field ID and the available single-select option names from an existing issue
before setting values, then verify the field value after the update.
