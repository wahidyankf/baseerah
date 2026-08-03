#!/usr/bin/env bash
# The only state this test owns is its mktemp directory and Compose project.
set -euo pipefail

beaver_nest_compose=infra/dev/beaver-nest-app/docker-compose.yml
rg -q '^  beaver-nest-app:$' "$beaver_nest_compose"
! rg -q '^  beaver-nest-(be|fe):$|down -v|beaver-nest-be-e2e-data' "$beaver_nest_compose"
rg -Fq 'BEAVER_NEST_BE_HOST_DATA_DIRECTORY:-/tmp/beaver-nest-unconfigured-data' "$beaver_nest_compose"
rg -Fq 'create_host_path: false' "$beaver_nest_compose"
