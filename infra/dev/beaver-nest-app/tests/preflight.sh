#!/usr/bin/env bash
# Contract tests for production preflight. Every fixture is task-owned.
set -euo pipefail

beaver_nest_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd -P)
beaver_nest_fixture=$(mktemp -d)
trap 'rm -rf -- "$beaver_nest_fixture"' EXIT

install -d -m 0700 "$beaver_nest_fixture/data" "$beaver_nest_fixture/backups"

run_preflight() {
	env -i \
		PATH="$PATH" \
		HOME="$HOME" \
		BEAVER_NEST_BE_VPN_HOST_IP=127.0.0.1 \
		BEAVER_NEST_BE_ALLOW_LOOPBACK_CI=1 \
		BEAVER_NEST_BE_PUBLIC_PORT=19300 \
		BEAVER_NEST_BE_HOST_DATA_DIRECTORY="$beaver_nest_fixture/data" \
		BEAVER_NEST_BE_BACKUP_DIRECTORY="$beaver_nest_fixture/backups" \
		bash "$beaver_nest_root/infra/dev/beaver-nest-app/scripts/preflight.sh"
}

run_preflight

if env -i PATH="$PATH" HOME="$HOME" \
	BEAVER_NEST_BE_ALLOW_LOOPBACK_CI=1 \
	BEAVER_NEST_BE_HOST_DATA_DIRECTORY="$beaver_nest_fixture/data" \
	BEAVER_NEST_BE_BACKUP_DIRECTORY="$beaver_nest_fixture/backups" \
	bash "$beaver_nest_root/infra/dev/beaver-nest-app/scripts/preflight.sh" >/dev/null 2>&1; then
	printf '%s\n' 'FAIL: absent host address passed preflight' >&2
	exit 1
fi

chmod 0755 "$beaver_nest_fixture/data"
if run_preflight >/dev/null 2>&1; then
	printf '%s\n' 'FAIL: unsafe data-directory mode passed preflight' >&2
	exit 1
fi
chmod 0700 "$beaver_nest_fixture/data"

ln -s "$beaver_nest_fixture/data" "$beaver_nest_fixture/data-alias"
if env -i PATH="$PATH" HOME="$HOME" BEAVER_NEST_BE_VPN_HOST_IP=127.0.0.1 \
	BEAVER_NEST_BE_ALLOW_LOOPBACK_CI=1 BEAVER_NEST_BE_HOST_DATA_DIRECTORY="$beaver_nest_fixture/data-alias" \
	BEAVER_NEST_BE_BACKUP_DIRECTORY="$beaver_nest_fixture/backups" \
	bash "$beaver_nest_root/infra/dev/beaver-nest-app/scripts/preflight.sh" >/dev/null 2>&1; then
	printf '%s\n' 'FAIL: symlinked data directory passed preflight' >&2
	exit 1
fi
