#!/usr/bin/env bash
# Pure runner: CI and the lifecycle wrapper supply an already-running runtime.
set -euo pipefail

beaver_nest_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd -P)
cd "$beaver_nest_root/apps/beaver-nest-fe-e2e"
npx bddgen
exec npx playwright test
