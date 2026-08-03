#!/usr/bin/env bash
set -euo pipefail

rg -Fq 'beaver-nest-be | beaver-nest-fe) echo "build-beaver-nest-be=true"' .github/workflows/publish-images.yml
rg -Fq 'Combined same-origin E2E' .github/workflows/_reusable-app-test-local-deploy-stag.yml
rg -Fq 'test:e2e:runner' .github/workflows/_reusable-app-test-local-deploy-stag.yml
rg -Fq 'BEAVER_NEST_BE_PUBLIC_PORT=19300' .github/workflows/_reusable-app-test-local-deploy-stag.yml
! test -e .github/workflows/beaver-nest-app-test-stag.yml
! rg -q 'stag-beaver-nest-fe|Vercel preview|localhost:19320' .github/workflows/beaver-nest-app-test-local-deploy-stag.yml .github/workflows/_reusable-app-test-local-deploy-stag.yml
