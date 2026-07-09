#!/usr/bin/env sh
set -eu

usage() {
  echo "usage: docs/scripts/adversarial-check.sh docs/tasks/M####-NNN-slug.md" >&2
  exit 2
}

[ "$#" -eq 1 ] || usage

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
repo_root="$(CDPATH= cd -- "${script_dir}/../.." && pwd)"
docs_dir="${repo_root}/docs"
cd "$repo_root"

task_file="$1"
[ -f "$task_file" ] || { echo "error: task file not found: $task_file" >&2; exit 1; }
[ -f "${docs_dir}/SOUNDNESS_REPORT_TEMPLATE.md" ] || { echo "error: docs/SOUNDNESS_REPORT_TEMPLATE.md not found" >&2; exit 1; }

if ! grep -Eq 'phase=ordinary-tests result=pass|Ordinary tests: .*pass|ordinary tests.*pass' "$task_file"; then
  echo "error: ordinary tests must be recorded as passing before adversarial check" >&2
  echo "hint: append an Execution Log entry like: YYYY-MM-DD agent=<agent> phase=ordinary-tests result=pass notes=<notes>" >&2
  exit 1
fi

task_base="$(basename "$task_file" .md)"
task_id="$(printf "%s" "$task_base" | sed 's/^\(M[0-9][0-9][0-9][0-9]-[0-9][0-9][0-9]\).*/\1/')"
milestone_id="$(grep -E '^- Milestone: `M[0-9][0-9][0-9][0-9]`$' "$task_file" | sed 's/^- Milestone: `//; s/`$//' | head -n 1)"

[ -n "$milestone_id" ] || { echo "error: milestone metadata missing" >&2; exit 1; }

milestone_file="$(find docs/milestones -maxdepth 1 -type f -name "${milestone_id}-*.md" | sort | head -n 1)"
[ -n "$milestone_file" ] || { echo "error: milestone file not found for $milestone_id" >&2; exit 1; }

mkdir -p "docs/tasks/soundness"
report_file="docs/tasks/soundness/${task_id}-soundness.md"
today="$(date +%Y-%m-%d)"
milestone_base="$(basename "$milestone_file")"

sed \
  -e "s|<TASK_ID>|${task_id}|g" \
  -e "s|<MILESTONE_ID>|${milestone_id}|g" \
  -e "s|tasks/<TASK_ID>-<slug>.md|${task_file}|g" \
  -e "s|docs/milestones/<MILESTONE_FILE>.md|${milestone_file}|g" \
  -e "s|<MILESTONE_FILE>|${milestone_base}|g" \
  -e "s|YYYY-MM-DD|${today}|g" \
  "${docs_dir}/SOUNDNESS_REPORT_TEMPLATE.md" > "$report_file"

echo "created $report_file"
echo "adversarial check is after ordinary tests by construction"
