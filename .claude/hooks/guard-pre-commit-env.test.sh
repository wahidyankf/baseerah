#!/usr/bin/env bash
# Self-test for the pre-commit staged-.env* guard (scripts/check-no-env-staged.sh).
# Run: bash .claude/hooks/guard-pre-commit-env.test.sh
# Red:  exits non-zero when guard script not yet created
# Green: exits 0 after guard is implemented
set -euo pipefail

GUARD="scripts/check-no-env-staged.sh"
PASS=0
FAIL=0

cleanup() {
  git restore --staged local-temp/.env.local 2>/dev/null || true
  git restore --staged local-temp/.env.example 2>/dev/null || true
  rm -f local-temp/.env.local local-temp/.env.example
}
trap cleanup EXIT

mkdir -p local-temp

echo "=== Case 1: staged .env.local must be rejected ==="
printf 'SECRET=test\n' > local-temp/.env.local
git add -f local-temp/.env.local
guard_output="$(bash "$GUARD" 2>&1 || true)"
if printf '%s' "$guard_output" | grep -q "ERROR"; then
  echo "PASS [DENY] staged .env.local rejected"
  PASS=$((PASS + 1))
else
  echo "FAIL [DENY] staged .env.local was not rejected (guard missing or did not output ERROR)"
  FAIL=$((FAIL + 1))
fi
git restore --staged local-temp/.env.local
rm -f local-temp/.env.local

echo "=== Case 2: staged .env.example must be allowed ==="
printf 'SECRET=changeme\n' > local-temp/.env.example
git add -f local-temp/.env.example
if bash "$GUARD"; then
  echo "PASS [ALLOW] staged .env.example allowed"
  PASS=$((PASS + 1))
else
  echo "FAIL [ALLOW] staged .env.example was rejected"
  FAIL=$((FAIL + 1))
fi
git restore --staged local-temp/.env.example
rm -f local-temp/.env.example

echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
[ "$FAIL" -eq 0 ]
