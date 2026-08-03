#!/usr/bin/env bash
set -euo pipefail

beaver_nest_compose=infra/dev/beaver-nest-app/docker-compose.yml
rg -Fq '${BEAVER_NEST_BE_VPN_HOST_IP:-127.0.0.1}:${BEAVER_NEST_BE_PUBLIC_PORT:-19300}:19300' "$beaver_nest_compose"
! rg -q '0\.0\.0\.0:|:19310:|:19320:' "$beaver_nest_compose"
[[ $(rg -c 'target: /var/lib/beaver-nest' "$beaver_nest_compose") -eq 4 ]]
