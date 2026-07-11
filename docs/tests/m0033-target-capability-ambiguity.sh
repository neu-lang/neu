#!/usr/bin/env bash
set -euo pipefail

grep -q 'M0033-TARGET-CAPABILITY-SCHEMA' docs/ambiguities/M0033-target-capability-schema.md
grep -q 'Status: `open`' docs/ambiguities/M0033-target-capability-schema.md
grep -q 'Task ID: `M0033-002`' docs/tasks/M0033-002-target-capability-schema.md
grep -q 'Status: `blocked`' docs/tasks/M0033-002-target-capability-schema.md
