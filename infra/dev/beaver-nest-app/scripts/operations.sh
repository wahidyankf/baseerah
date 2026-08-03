#!/usr/bin/env bash
# Run one guarded database operation using an explicitly supplied environment file.
set -euo pipefail

beaver_nest_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd -P)
# shellcheck source=infra/dev/beaver-nest-app/scripts/lib.sh
source "$beaver_nest_root/infra/dev/beaver-nest-app/scripts/lib.sh"

[[ $# -ge 3 ]] || beaver_nest_fail 'usage: operations.sh {backup|integrity|restore} --env-file PATH [--name NAME.sqlite3]'
beaver_nest_operation=$1
[[ "$2" == --env-file ]] || beaver_nest_fail 'the explicit --env-file argument is required'
beaver_nest_env_file=$3
shift 3
[[ -f "$beaver_nest_env_file" && ! -L "$beaver_nest_env_file" ]] || beaver_nest_fail 'environment file must be a regular non-symlink file'

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

case "$beaver_nest_operation" in
backup | restore)
	[[ $# -eq 2 && "$1" == --name ]] || beaver_nest_fail 'backup and restore require --name NAME.sqlite3'
	beaver_nest_validate_operation_name "$2"
	export BEAVER_NEST_BE_OPERATION_NAME=$2
	;;
integrity) [[ $# -eq 0 ]] || beaver_nest_fail 'integrity does not accept a name' ;;
*) beaver_nest_fail 'operation must be backup, integrity, or restore' ;;
esac

beaver_nest_compose=(docker compose --env-file "$beaver_nest_env_file" -f "$beaver_nest_root/infra/dev/beaver-nest-app/docker-compose.yml")
if [[ "$beaver_nest_operation" == restore ]] && "${beaver_nest_compose[@]}" ps --services --status running | grep -Fxq beaver-nest-app; then
	beaver_nest_fail 'restore refused while beaver-nest-app is running'
fi

case "$beaver_nest_operation" in
backup) "${beaver_nest_compose[@]}" run --rm beaver-nest-backup ;;
integrity) "${beaver_nest_compose[@]}" run --rm beaver-nest-integrity ;;
restore) "${beaver_nest_compose[@]}" run --rm beaver-nest-restore ;;
esac
