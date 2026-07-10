# Task Execution System

Tasks are the executable unit of milestone work. One milestone becomes many small task files. Every task references exactly one milestone.

## Directory Rules

- Task files live in `docs/tasks/`.
- Task file names use `M####-NNN-short-slug.md`.
- Each task must be created from `TASK_TEMPLATE.md`.
- Each task must reference exactly one milestone under `docs/milestones/`.
- Each task must define scope, out-of-scope, tests, acceptance criteria, and execution commands.
- Task reports, ambiguity reports, and soundness reports may be stored under task-specific subdirectories when needed.

## Source Of Truth

- `docs/SPEC.md`
- `docs/adr/`
- `docs/ROADMAP.md`
- `docs/milestones/`
- `main task rules`
- `AGENTS.md`

If the spec or ADRs are ambiguous, main tasks must file an ambiguity report instead of guessing.

## Exact Execution Loop

1. select next milestone
2. decompose into tasks
3. create first task
4. generate tests
5. verify tests fail
6. implement smallest passing change
7. run ordinary tests
8. run adversarial tests
9. run main-task review
10. update milestone checklist
11. commit

## Gate Rules

- Tests must be generated before implementation.
- Verification must record that tests fail before implementation for the expected reason.
- Implementation may not modify or delete failing tests unless main-task review approval is recorded.
- Ordinary tests must pass before adversarial tests run.
- Adversarial check must run after ordinary tests.
- main-task review must compare task output against `docs/SPEC.md` and the milestone.
- CI is the final gate.

## Creating A Task

```sh
docs/scripts/create-task.sh M0001 source-truth-paths "Align source of truth paths"
```

The script creates the next numbered task file for the milestone using `TASK_TEMPLATE.md`.

## Running A Task

```sh
docs/scripts/run-task.sh docs/tasks/M0001-001-source-truth-paths.md
```

The script validates the task file and prints the required execution loop. It does not invent project-specific compiler commands.

## Reviewing A Task

```sh
docs/scripts/review-task.sh docs/tasks/M0001-001-source-truth-paths.md
```

The script validates required review inputs and creates a review report from `REVIEW_TEMPLATE.md`.

## Adversarial Check

```sh
docs/scripts/adversarial-check.sh docs/tasks/M0001-001-source-truth-paths.md
```

The script creates a soundness report from `SOUNDNESS_REPORT_TEMPLATE.md` and enforces that ordinary tests must be recorded before adversarial review.
