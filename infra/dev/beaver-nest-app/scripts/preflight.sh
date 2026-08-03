#!/usr/bin/env bash
# Validate explicit production inputs before Docker Compose can render or start.
set -euo pipefail

beaver_nest_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd -P)
# shellcheck source=infra/dev/beaver-nest-app/scripts/lib.sh
source "$beaver_nest_root/infra/dev/beaver-nest-app/scripts/lib.sh"

: "${BEAVER_NEST_BE_VPN_HOST_IP:?BEAVER_NEST_BE_VPN_HOST_IP is required}"
: "${BEAVER_NEST_BE_HOST_DATA_DIRECTORY:?BEAVER_NEST_BE_HOST_DATA_DIRECTORY is required}"
: "${BEAVER_NEST_BE_BACKUP_DIRECTORY:?BEAVER_NEST_BE_BACKUP_DIRECTORY is required}"

beaver_nest_data_directory=$(beaver_nest_validate_safe_directory \
	'BEAVER_NEST_BE_HOST_DATA_DIRECTORY' "$BEAVER_NEST_BE_HOST_DATA_DIRECTORY" "$beaver_nest_root")
beaver_nest_backup_directory=$(beaver_nest_validate_safe_directory \
	'BEAVER_NEST_BE_BACKUP_DIRECTORY' "$BEAVER_NEST_BE_BACKUP_DIRECTORY" "$beaver_nest_root")
[[ "$beaver_nest_data_directory" != "$beaver_nest_backup_directory" ]] ||
	beaver_nest_fail 'data and backup directories must be distinct'
beaver_nest_validate_directory_mode 'BEAVER_NEST_BE_HOST_DATA_DIRECTORY' "$beaver_nest_data_directory"
beaver_nest_validate_directory_mode 'BEAVER_NEST_BE_BACKUP_DIRECTORY' "$beaver_nest_backup_directory"

if [[ "$BEAVER_NEST_BE_VPN_HOST_IP" == 127.0.0.1 ]]; then
	[[ "${BEAVER_NEST_BE_ALLOW_LOOPBACK_CI:-}" == 1 ]] ||
		beaver_nest_fail 'loopback publication is limited to explicit CI fixtures'
elif ! { command -v ip >/dev/null && ip -o addr show | awk '{print $4}' | cut -d/ -f1 | grep -Fxq "$BEAVER_NEST_BE_VPN_HOST_IP"; } &&
	! { command -v ifconfig >/dev/null && ifconfig | awk '/inet / {print $2}' | grep -Fxq "$BEAVER_NEST_BE_VPN_HOST_IP"; }; then
	beaver_nest_fail 'BEAVER_NEST_BE_VPN_HOST_IP is not configured on this host'
fi
