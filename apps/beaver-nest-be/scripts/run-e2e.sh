#!/usr/bin/env bash
# Own one disposable combined runtime, then invoke exactly one pure Playwright runner.
set -euo pipefail

beaver_nest_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd -P)
beaver_nest_suite=backend
if [[ "${1:-}" == --frontend ]]; then
	beaver_nest_suite=frontend
	shift
fi
[[ $# -eq 0 ]] || {
	printf '%s\n' 'usage: run-e2e.sh [--frontend]' >&2
	exit 1
}

beaver_nest_api_base_url=${API_BASE_URL:-}
beaver_nest_web_base_url=${WEB_BASE_URL:-}
if [[ -n "$beaver_nest_api_base_url$beaver_nest_web_base_url" ]]; then
	if [[ "$beaver_nest_suite" == backend ]]; then
		[[ -n "$beaver_nest_api_base_url" ]] || {
			printf '%s\n' 'API_BASE_URL is required for an existing runtime' >&2
			exit 1
		}
		API_BASE_URL="$beaver_nest_api_base_url" bash "$beaver_nest_root/apps/beaver-nest-be-e2e/scripts/run-playwright.sh"
	else
		[[ -n "$beaver_nest_web_base_url" ]] || {
			printf '%s\n' 'WEB_BASE_URL is required for an existing runtime' >&2
			exit 1
		}
		WEB_BASE_URL="$beaver_nest_web_base_url" bash "$beaver_nest_root/apps/beaver-nest-fe-e2e/scripts/run-playwright.sh"
	fi
	exit 0
fi

beaver_nest_fixture_root=$(mktemp -d)
beaver_nest_project="beaver-nest-e2e-${RANDOM}-${RANDOM}"
beaver_nest_compose=(docker compose --env-file /dev/null -p "$beaver_nest_project"
	-f "$beaver_nest_root/infra/dev/beaver-nest-app/docker-compose.yml"
	-f "$beaver_nest_root/infra/dev/beaver-nest-app/docker-compose.ci.yml")

cleanup() {
	"${beaver_nest_compose[@]}" down --remove-orphans >/dev/null 2>&1 || true
	rm -rf -- "$beaver_nest_fixture_root"
}
trap cleanup EXIT

install -d -m 0700 "$beaver_nest_fixture_root/data" "$beaver_nest_fixture_root/backups"
export BEAVER_NEST_BE_VPN_HOST_IP=127.0.0.1
export BEAVER_NEST_BE_PUBLIC_PORT=19300
export BEAVER_NEST_BE_HOST_DATA_DIRECTORY="$beaver_nest_fixture_root/data"
export BEAVER_NEST_BE_BACKUP_DIRECTORY="$beaver_nest_fixture_root/backups"

"${beaver_nest_compose[@]}" build beaver-nest-app
"${beaver_nest_compose[@]}" run --rm --no-deps --user 0:0 --entrypoint sh beaver-nest-app -ceu \
	'chown 10001:10001 /var/lib/beaver-nest && chmod 0700 /var/lib/beaver-nest'
"${beaver_nest_compose[@]}" up -d beaver-nest-app

beaver_nest_api_base_url=http://127.0.0.1:19300
for beaver_nest_attempt in $(seq 1 120); do
	if curl -fsS "$beaver_nest_api_base_url/api/v1/readiness" >/dev/null 2>&1; then
		break
	fi
	[[ "$beaver_nest_attempt" -lt 120 ]] || {
		printf '%s\n' 'combined runtime did not become ready' >&2
		exit 1
	}
	sleep 1
done

if [[ "$beaver_nest_suite" == backend ]]; then
	API_BASE_URL="$beaver_nest_api_base_url" \
		BEAVER_NEST_BE_E2E_COMPOSE_PROJECT="$beaver_nest_project" \
		bash "$beaver_nest_root/apps/beaver-nest-be-e2e/scripts/run-playwright.sh"
else
	WEB_BASE_URL="$beaver_nest_api_base_url" bash "$beaver_nest_root/apps/beaver-nest-fe-e2e/scripts/run-playwright.sh"
fi
