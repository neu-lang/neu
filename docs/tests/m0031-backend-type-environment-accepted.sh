#!/usr/bin/env sh
set -eu
rg -q '^# ADR-0055: Bootstrap Type Environment Transport$' \
  docs/adr/ADR-0055-bootstrap-type-environment-transport.md
rg -q 'Status: Accepted' docs/adr/ADR-0055-bootstrap-type-environment-transport.md
rg -q 'owning module `TypeArena`' docs/SPEC.md
rg -q 'M0031-BACKEND-TYPE-ENVIRONMENT' docs/ambiguities/M0031-backend-type-environment.md
rg -q 'Status: `resolved`' docs/ambiguities/M0031-backend-type-environment.md
