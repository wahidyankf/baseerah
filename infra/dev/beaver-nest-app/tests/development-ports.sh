#!/usr/bin/env bash

set -euo pipefail

jq -e '.targets.dev.options.command == "scripts/start-development.sh"' apps/beaver-nest-be/project.json >/dev/null
jq -e '.targets.dev.options.command | contains("--host 127.0.0.1 --port 19310")' apps/beaver-nest-fe/project.json >/dev/null
rg -Fq 'BEAVER_NEST_BE_HTTP_LISTEN_ADDRESS=127.0.0.1' apps/beaver-nest-be/scripts/start-development.sh
rg -Fq 'BEAVER_NEST_BE_HTTP_LISTEN_PORT=19320' apps/beaver-nest-be/scripts/start-development.sh
! jq -r '.scripts["beaver-nest:dev"]' package.json | rg -q 'docker compose|down -v'
