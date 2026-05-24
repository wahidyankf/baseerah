#!/usr/bin/env bash
# Pre-commit guard: reject staged .env* files except .env.example.
# Platform-agnostic — runs for all actors (agents and humans).
# Policy: guard-env-file-access
set -euo pipefail

offending="$(git diff --cached --name-only --diff-filter=AM \
  | grep -E '(^|/)\.env[^/]*$' \
  | grep -vE '(^|/)\.env\.example$' || true)"

if [ -n "$offending" ]; then
  echo "ERROR: refusing to commit real .env* files (policy: guard-env-file-access):"
  echo "$offending" | sed 's/^/  /'
  echo "Only .env.example may be committed."
  echo "Unstage with: git restore --staged <file>"
  exit 1
fi
