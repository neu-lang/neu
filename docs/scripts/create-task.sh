#!/usr/bin/env sh
set -eu

usage() {
  echo "usage: docs/scripts/create-task.sh M#### short-slug \"Task title\"" >&2
  exit 2
}

[ "$#" -eq 3 ] || usage

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
repo_root="$(CDPATH= cd -- "${script_dir}/../.." && pwd)"
docs_dir="${repo_root}/docs"
tasks_dir="${docs_dir}/tasks"

milestone_id="$1"
slug="$2"
title="$3"

case "$milestone_id" in
  M[0-9][0-9][0-9][0-9]) ;;
  *) echo "error: milestone id must look like M0001" >&2; exit 2 ;;
esac

case "$slug" in
  *[!a-z0-9-]*|'') echo "error: slug must contain only lowercase letters, digits, and hyphens" >&2; exit 2 ;;
esac

[ -f "${docs_dir}/TASK_TEMPLATE.md" ] || { echo "error: docs/TASK_TEMPLATE.md not found" >&2; exit 1; }
[ -f "${docs_dir}/ROADMAP.md" ] || { echo "error: docs/ROADMAP.md not found" >&2; exit 1; }

milestone_file="$(find "${docs_dir}/milestones" -maxdepth 1 -type f -name "${milestone_id}-*.md" | sort | head -n 1)"
[ -n "$milestone_file" ] || { echo "error: milestone file not found for $milestone_id" >&2; exit 1; }

mkdir -p "$tasks_dir"

next_number="$(
  find "$tasks_dir" -maxdepth 1 -type f -name "${milestone_id}-[0-9][0-9][0-9]-*.md" |
    sed "s|.*/${milestone_id}-||; s|-.*||" |
    sort -n |
    tail -n 1
)"

if [ -z "$next_number" ]; then
  task_number="001"
else
  next_decimal="$(printf "%s" "$next_number" | sed 's/^0*//')"
  [ -n "$next_decimal" ] || next_decimal="0"
  task_number="$(printf "%03d" "$((next_decimal + 1))")"
fi

task_id="${milestone_id}-${task_number}"
task_file="${tasks_dir}/${task_id}-${slug}.md"

[ ! -e "$task_file" ] || { echo "error: task already exists: $task_file" >&2; exit 1; }

milestone_base="$(basename "$milestone_file")"
milestone_rel="docs/milestones/${milestone_base}"
today="$(date +%Y-%m-%d)"

sed \
  -e "s|<TASK_ID>|${task_id}|g" \
  -e "s|<Title>|${title}|g" \
  -e "s|<MILESTONE_ID>|${milestone_id}|g" \
  -e "s|docs/milestones/<MILESTONE_FILE>.md|${milestone_rel}|g" \
  -e "s|<MILESTONE_FILE>|${milestone_base}|g" \
  -e "s|YYYY-MM-DD|${today}|g" \
  -e "s|task/<TASK_ID>-<slug>|task/${task_id}-${slug}|g" \
  "${docs_dir}/TASK_TEMPLATE.md" > "$task_file"

echo "created ${task_file#"$repo_root"/}"
echo "milestone $milestone_rel"
