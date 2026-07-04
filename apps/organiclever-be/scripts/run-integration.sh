#!/usr/bin/env bash
# Integration test runner for organiclever-be.
# Brings up a fresh PostgreSQL container, runs the EF/DbUp integration suite
# against it from the host, then tears the container down.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"

# Fail fast (before paying for docker-compose startup) on any xunit Skip= attribute
# left in the integration suite.
if grep -rn -E --include='*.fs' 'Skip\s*=' "${ROOT}/apps/organiclever-be/tests/integration"; then
  echo "ERROR: xunit Skip= attribute found in test files above" >&2
  exit 1
fi

COMPOSE_FILE="${ROOT}/apps/organiclever-be/docker-compose.integration.yml"
PROJECT_NAME="organiclever-be-integration"

cleanup() {
  docker compose -p "${PROJECT_NAME}" -f "${COMPOSE_FILE}" down -v >/dev/null 2>&1 || true
}
trap cleanup EXIT

cleanup
docker compose -p "${PROJECT_NAME}" -f "${COMPOSE_FILE}" up -d --wait

export DATABASE_URL="Host=localhost;Port=5434;Database=organiclever_be_test;Username=organiclever_be;Password=organiclever_be"

dotnet test "${ROOT}/apps/organiclever-be/tests/integration/OrganicleverBe.IntegrationTests.fsproj" --logger "console;verbosity=normal"
