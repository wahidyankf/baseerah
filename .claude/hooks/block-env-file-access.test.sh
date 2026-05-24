#!/usr/bin/env bash
# Test suite for block-env-file-access.sh
# Run: bash .claude/hooks/block-env-file-access.test.sh
# Each assertion exits non-zero on failure (set -e propagates).
set -euo pipefail

HOOK=".claude/hooks/block-env-file-access.sh"
PASS=0
FAIL=0

assert_deny() {
  local desc="$1"
  local input="$2"
  local result
  result="$(printf '%s' "$input" | bash "$HOOK" 2>/dev/null || true)"
  if printf '%s' "$result" | jq -e '.hookSpecificOutput.permissionDecision=="deny"' > /dev/null 2>&1; then
    echo "PASS [DENY] $desc"
    PASS=$((PASS + 1))
  else
    echo "FAIL [DENY] $desc — got: $result"
    FAIL=$((FAIL + 1))
  fi
}

assert_allow() {
  local desc="$1"
  local input="$2"
  local result
  result="$(printf '%s' "$input" | bash "$HOOK" 2>/dev/null || true)"
  if [ -z "$result" ]; then
    echo "PASS [ALLOW] $desc"
    PASS=$((PASS + 1))
  else
    echo "FAIL [ALLOW] $desc — expected empty output, got: $result"
    FAIL=$((FAIL + 1))
  fi
}

echo "=== File-tool branch ==="

# DENY: Read on real .env file
assert_deny "Read .env.local" \
  '{"tool_name":"Read","tool_input":{"file_path":".env.local"}}'

# DENY: Write on nested real .env file
assert_deny "Write apps/organiclever-web/.env.local" \
  '{"tool_name":"Write","tool_input":{"file_path":"apps/organiclever-web/.env.local"}}'

# DENY: Edit on .env.production
assert_deny "Edit .env.production" \
  '{"tool_name":"Edit","tool_input":{"file_path":".env.production"}}'

# DENY: Write on arbitrary .env name
assert_deny "Write .env.whatever" \
  '{"tool_name":"Write","tool_input":{"file_path":".env.whatever"}}'

# ALLOW: Read on .env.example
assert_allow "Read .env.example" \
  '{"tool_name":"Read","tool_input":{"file_path":".env.example"}}'

# ALLOW: Write on nested .env.example
assert_allow "Write infra/dev/ose-web/.env.example" \
  '{"tool_name":"Write","tool_input":{"file_path":"infra/dev/ose-web/.env.example"}}'

echo ""
echo "=== Bash branch ==="

# DENY: cat .env.local
assert_deny "Bash: cat .env.local" \
  '{"tool_name":"Bash","tool_input":{"command":"cat .env.local"}}'

# DENY: echo redirect to .env.local
assert_deny "Bash: echo X > .env.local" \
  '{"tool_name":"Bash","tool_input":{"command":"echo X > .env.local"}}'

# DENY: git add .env.local
assert_deny "Bash: git add .env.local" \
  '{"tool_name":"Bash","tool_input":{"command":"git add .env.local"}}'

# ALLOW: cat .env.example
assert_allow "Bash: cat .env.example" \
  '{"tool_name":"Bash","tool_input":{"command":"cat .env.example"}}'

# ALLOW: project script invocation
assert_allow "Bash: bash scripts/setup-env.sh" \
  '{"tool_name":"Bash","tool_input":{"command":"bash scripts/setup-env.sh"}}'

# ALLOW: node script under apps/
assert_allow "Bash: node apps/foo/seed-env.js" \
  '{"tool_name":"Bash","tool_input":{"command":"node apps/foo/seed-env.js"}}'

# ALLOW: npm runner
assert_allow "Bash: npm run setup:env" \
  '{"tool_name":"Bash","tool_input":{"command":"npm run setup:env"}}'

echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
[ "$FAIL" -eq 0 ]
