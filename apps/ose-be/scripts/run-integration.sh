#!/usr/bin/env bash
# Integration test runner for ose-be.
# Brings up a fresh PostgreSQL container, runs the EF/DbUp integration suite
# against it from the host, then tears the container down.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
COMPOSE_FILE="${ROOT}/apps/ose-be/docker-compose.integration.yml"
PROJECT_NAME="ose-be-integration"

cleanup() {
  docker compose -p "${PROJECT_NAME}" -f "${COMPOSE_FILE}" down -v >/dev/null 2>&1 || true
}
trap cleanup EXIT

cleanup
docker compose -p "${PROJECT_NAME}" -f "${COMPOSE_FILE}" up -d --wait

export DATABASE_URL="Host=localhost;Port=5433;Database=ose_app_be_test;Username=ose_app_be;Password=ose_app_be"

dotnet test "${ROOT}/apps/ose-be/tests/integration/OseBe.IntegrationTests.fsproj" --logger "console;verbosity=normal"
