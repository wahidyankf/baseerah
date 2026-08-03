#!/usr/bin/env bash
set -euo pipefail

rg -Fq 'apps/beaver-nest-be-e2e/scripts/run-playwright.sh' apps/beaver-nest-be/scripts/run-e2e.sh
rg -Fq 'apps/beaver-nest-fe-e2e/scripts/run-playwright.sh' apps/beaver-nest-be/scripts/run-e2e.sh
! rg -q 'down -v' apps/beaver-nest-be/scripts/run-e2e.sh
rg -Fq 'scripts/run-playwright.sh' apps/beaver-nest-be-e2e/project.json
rg -Fq 'scripts/run-playwright.sh' apps/beaver-nest-fe-e2e/project.json
! rg -q 'docker compose|run-e2e.sh' apps/beaver-nest-{be,fe}-e2e/scripts/run-playwright.sh
