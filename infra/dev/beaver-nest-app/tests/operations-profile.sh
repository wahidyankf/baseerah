#!/usr/bin/env bash
set -euo pipefail

beaver_nest_compose=infra/dev/beaver-nest-app/docker-compose.yml
rg -Fq 'profiles: ["operations"]' "$beaver_nest_compose"
rg -Fq 'command: ["backup", "--name", "${BEAVER_NEST_BE_OPERATION_NAME:-operation.sqlite3}"]' "$beaver_nest_compose"
rg -Fq 'command: ["restore", "--name", "${BEAVER_NEST_BE_OPERATION_NAME:-operation.sqlite3}"]' "$beaver_nest_compose"
rg -Fq 'command: ["integrity"]' "$beaver_nest_compose"
[[ $(rg -c 'target: /var/backups/beaver-nest' "$beaver_nest_compose") -eq 2 ]]
rg -Fq 'beaver_nest_validate_operation_name' infra/dev/beaver-nest-app/scripts/operations.sh
rg -Fq 'restore refused while beaver-nest-app is running' infra/dev/beaver-nest-app/scripts/operations.sh
