#!/usr/bin/env bash
# E2E test runner for beaver-nest-be.
# Brings up the beaver-nest-be dev container via infra/dev/beaver-nest-app, waits for
# GET /api/v1/health to respond, runs the beaver-nest-be-e2e Playwright suite
# against it, then tears the stack down.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"

# Fail fast (before paying for docker-compose startup) on any unconditional
# test.skip() left in the e2e suite. test.skip(condition, reason) - the
# documented Playwright environment-guard form - is intentionally allowed through.
if grep -rn -E --include='*.ts' --exclude-dir=node_modules --exclude-dir=.features-gen --exclude-dir=test-results --exclude-dir=playwright-report '\$?test\.skip\([^,)]*\)' "${ROOT}/apps/beaver-nest-be-e2e"; then
	echo "ERROR: unconditional test.skip() found in test files above - use test.skip(condition, reason) for legitimate environment guards, or remove" >&2
	exit 1
fi

COMPOSE_FILES=(-f "${ROOT}/infra/dev/beaver-nest-app/docker-compose.yml" -f "${ROOT}/infra/dev/beaver-nest-app/docker-compose.ci.yml")
PROJECT_NAME="beaver-nest-be-e2e"
PORT=19320

cleanup() {
	docker compose -p "${PROJECT_NAME}" "${COMPOSE_FILES[@]}" down -v >/dev/null 2>&1 || true
}
trap cleanup EXIT

# Start the backend
docker compose -p "${PROJECT_NAME}" "${COMPOSE_FILES[@]}" down -v >/dev/null 2>&1 || true
docker compose -p "${PROJECT_NAME}" "${COMPOSE_FILES[@]}" up -d --build beaver-nest-be

# Wait until the health endpoint responds (up to 120s - cold NuGet restore in-container)
echo "Waiting for beaver-nest-be on port ${PORT}..."
for i in $(seq 1 120); do
	if curl -sf "http://localhost:${PORT}/api/v1/health" >/dev/null 2>&1; then
		echo "beaver-nest-be is ready (${i}s)"
		break
	fi
	if [[ "${i}" -eq 120 ]]; then
		echo "ERROR: beaver-nest-be did not start within 120 seconds" >&2
		exit 1
	fi
	sleep 1
done

# Run the Playwright e2e suite
cd "${ROOT}/apps/beaver-nest-be-e2e"
npx bddgen && npx playwright test
