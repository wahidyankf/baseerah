#!/usr/bin/env bash
# Sole production entrypoint: explicit env file, then preflight, then one Compose service.
set -euo pipefail

beaver_nest_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd -P)

[[ $# -eq 2 && "$1" == --env-file ]] || {
	printf '%s\n' 'usage: start.sh --env-file PATH' >&2
	exit 1
}
beaver_nest_env_file=$2
[[ -f "$beaver_nest_env_file" && ! -L "$beaver_nest_env_file" ]] || {
	printf '%s\n' 'environment file must be a regular non-symlink file' >&2
	exit 1
}

beaver_nest_env_value() {
	awk -F= -v beaver_nest_key="$1" '$1 == beaver_nest_key { print substr($0, length(beaver_nest_key) + 2); exit }' "$beaver_nest_env_file"
}

BEAVER_NEST_BE_VPN_HOST_IP=$(beaver_nest_env_value BEAVER_NEST_BE_VPN_HOST_IP)
BEAVER_NEST_BE_PUBLIC_PORT=$(beaver_nest_env_value BEAVER_NEST_BE_PUBLIC_PORT)
BEAVER_NEST_BE_HOST_DATA_DIRECTORY=$(beaver_nest_env_value BEAVER_NEST_BE_HOST_DATA_DIRECTORY)
BEAVER_NEST_BE_BACKUP_DIRECTORY=$(beaver_nest_env_value BEAVER_NEST_BE_BACKUP_DIRECTORY)
export BEAVER_NEST_BE_VPN_HOST_IP BEAVER_NEST_BE_PUBLIC_PORT
export BEAVER_NEST_BE_HOST_DATA_DIRECTORY BEAVER_NEST_BE_BACKUP_DIRECTORY
bash "$beaver_nest_root/infra/dev/beaver-nest-app/scripts/preflight.sh"

exec docker compose --env-file "$beaver_nest_env_file" \
	-f "$beaver_nest_root/infra/dev/beaver-nest-app/docker-compose.yml" \
	up -d --build beaver-nest-app
