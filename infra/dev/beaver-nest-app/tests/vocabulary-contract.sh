#!/usr/bin/env bash
set -euo pipefail

jq -e '.tags | index("platform:vite")' apps/beaver-nest-fe/project.json >/dev/null
rg -Fq 'Static `dist/` production build' apps/beaver-nest-fe/README.md
rg -Fq 'BeaverNest Vite CSR client' docs/reference/monorepo-structure.md
! rg -q 'beaver-nest-fe.*Next\.js|beaver-nest-fe.*\.next' AGENTS.md apps/beaver-nest-fe docs/reference/monorepo-structure.md repo-governance/development/infra/nx-targets.md
