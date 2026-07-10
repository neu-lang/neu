#!/usr/bin/env sh
set -eu

report="docs/ambiguities/M0019-nullability-and-flow-typing.md"
task="docs/tasks/M0019-001-nullability-flow-ambiguity-blocker.md"
milestone="docs/milestones/M0019-nullability-and-flow-typing.md"

[ -f "$task" ] || { echo "missing task: $task" >&2; exit 1; }
[ -f "$milestone" ] || { echo "missing milestone: $milestone" >&2; exit 1; }
[ -f "$report" ] || { echo "missing ambiguity report: $report" >&2; exit 1; }

grep -q 'Report ID: `M0019-nullability-and-flow-typing`' "$report"
grep -q 'Related Task: `M0019-001`' "$report"
grep -q 'Related Milestone: `M0019`' "$report"
grep -q 'Status: `resolved`' "$report"
grep -q 'Resolution Source: `docs/adr/ADR-0028-nullability-and-flow-typing.md`' "$report"
grep -q 'Required Owner: `main-task semantic design`' "$report"
grep -q 'nullable misuse' "$report"
grep -q 'smart-cast eligibility' "$report"
grep -qi 'mutation invalidation' "$report"
grep -q 'diagnostic' "$report"
grep -q 'Implementation may define nullability and flow typing only as specified by accepted ADR-0028' "$report"

grep -q 'Do not implement nullability checks.' "$task"
grep -q 'Do not implement flow tracking.' "$task"
grep -q 'Do not implement smart casts.' "$task"

echo "M0019 nullability and flow typing ambiguity blocker is recorded"
