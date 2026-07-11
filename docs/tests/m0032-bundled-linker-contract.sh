#!/usr/bin/env sh
set -eu
rg -q 'Status: `blocked`' docs/tasks/M0032-003-bundled-linker-contract.md
rg -q 'Status: `open`' docs/ambiguities/M0032-bundled-linker-contract.md
rg -q 'host.*ld|clang' docs/ambiguities/M0032-bundled-linker-contract.md
rg -q '\[ \] Bundled linker path works for host smoke\.' docs/milestones/M0032-object-and-bundled-linker-pipeline.md
rg -q '\[ \] Hidden host dependencies are documented or eliminated\.' docs/milestones/M0032-object-and-bundled-linker-pipeline.md
