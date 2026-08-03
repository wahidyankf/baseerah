#!/usr/bin/env bash
set -euo pipefail

beaver_nest_start=infra/dev/beaver-nest-app/scripts/start.sh
rg -Fq 'usage: start.sh --env-file PATH' "$beaver_nest_start"
rg -Fq 'scripts/preflight.sh' "$beaver_nest_start"
rg -Fq 'docker compose --env-file' "$beaver_nest_start"
! rg -q 'source .*\.env|\. .*\.env' "$beaver_nest_start"
