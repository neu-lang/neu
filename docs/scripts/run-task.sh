#!/usr/bin/env sh
set -eu

usage() {
  echo "usage: docs/scripts/run-task.sh docs/tasks/M####-NNN-slug.md" >&2
  exit 2
}

[ "$#" -eq 1 ] || usage

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
repo_root="$(CDPATH= cd -- "${script_dir}/../.." && pwd)"
cd "$repo_root"

task_file="$1"
[ -f "$task_file" ] || { echo "error: task file not found: $task_file" >&2; exit 1; }

require_line() {
  pattern="$1"
  message="$2"
  if ! grep -Eq "$pattern" "$task_file"; then
    echo "error: $message" >&2
    exit 1
  fi
}

require_section() {
  section="$1"
  require_line "^## ${section}$" "missing section: ${section}"
}

require_section "Task Metadata"
require_section "Source Of Truth"
require_section "Scope"
require_section "Out Of Scope"
require_section "Required Tests"
require_section "Test-First Gate"
require_section "Acceptance Criteria"
require_section "Execution Commands"
require_section "Execution Log"

milestone_count="$(grep -E '^- Milestone: `M[0-9][0-9][0-9][0-9]`$' "$task_file" | wc -l | tr -d ' ')"
[ "$milestone_count" = "1" ] || {
  echo "error: task must reference exactly one milestone in Task Metadata" >&2
  exit 1
}

require_line '^- Specification: `docs/SPEC\.md`$' "task must reference docs/SPEC.md"
require_line '^- Reviewer approval required to modify/delete failing tests: `yes`$' "task must require reviewer approval before modifying/deleting failing tests"
require_line '^- Generate tests: ' "missing Generate tests command"
require_line '^- Verify tests fail: ' "missing Verify tests fail command"
require_line '^- Ordinary tests: ' "missing Ordinary tests command"
require_line '^- Adversarial tests: ' "missing Adversarial tests command"
require_line '^- Review: `docs/scripts/review-task\.sh <task-file>`$|^- Review: `docs/scripts/review-task\.sh ' "missing review command"
require_line '^- CI: ' "missing CI command"

echo "validated task: $task_file"
echo
echo "Required execution loop:"
echo "1. select next milestone"
echo "2. decompose into tasks"
echo "3. create first task"
echo "4. generate tests"
echo "5. verify tests fail"
echo "6. implement smallest passing change"
echo "7. run ordinary tests"
echo "8. run adversarial tests"
echo "9. run reviewer"
echo "10. update milestone checklist"
echo "11. commit"
echo
echo "Gate order enforced by policy:"
echo "- tests before implementation"
echo "- ordinary tests before adversarial tests"
echo "- reviewer before milestone checklist update"
echo "- CI as final gate"
