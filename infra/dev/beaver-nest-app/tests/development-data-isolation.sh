#!/usr/bin/env bash
set -euo pipefail

beaver_nest_development_script=apps/beaver-nest-be/scripts/start-development.sh
rg -Fq 'BEAVER_NEST_BE_DEVELOPMENT_DATA_DIRECTORY' "$beaver_nest_development_script"
rg -Fq 'export BEAVER_NEST_BE_DATA_DIRECTORY="$beaver_nest_canonical_directory"' "$beaver_nest_development_script"
rg -Fq 'unset BEAVER_NEST_BE_HOST_DATA_DIRECTORY' "$beaver_nest_development_script"
! rg -q 'BEAVER_NEST_BE_DEVELOPMENT_DATA_DIRECTORY' infra/dev/beaver-nest-app/docker-compose.yml
