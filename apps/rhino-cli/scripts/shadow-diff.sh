#!/usr/bin/env bash
# Shadow-diff harness — runs the Go and Rust rhino-cli binaries with the same
# arguments and exits 1 on any divergence in stdout, stderr, or exit code.
# Used during Phase 1+ migration to gate each command flip on byte-identical
# output for at least one week of CI runs before the matching Nx target
# switches to the Rust binary outright.
#
# Usage:   shadow-diff.sh <args-to-rhino-cli>
# Example: shadow-diff.sh test-coverage validate apps/rhino-cli/lcov.info 90
#          shadow-diff.sh --help
#
# Exit codes:
#   0 — outputs match byte-for-byte (PASS)
#   1 — divergence detected (FAIL — see /tmp/shadow-diff-*.txt for details)
#   2 — invocation/IO error

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
GO_DIR="${REPO_ROOT}/apps/rhino-cli"
RS_MANIFEST="${REPO_ROOT}/apps/rhino-cli/Cargo.toml"

if [[ ! -f "${GO_DIR}/main.go" ]]; then
  echo "shadow-diff: Go binary not found at ${GO_DIR}/main.go" >&2
  exit 2
fi
if [[ ! -f "${RS_MANIFEST}" ]]; then
  echo "shadow-diff: Rust manifest not found at ${RS_MANIFEST}" >&2
  exit 2
fi

TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT

GO_OUT="${TMP}/go.out"
GO_ERR="${TMP}/go.err"
GO_EXIT="${TMP}/go.exit"

RS_OUT="${TMP}/rs.out"
RS_ERR="${TMP}/rs.err"
RS_EXIT="${TMP}/rs.exit"

# Run Go binary (silently — exit code captured below).
set +e
( cd "${GO_DIR}" && CGO_ENABLED=0 go run main.go "$@" ) > "${GO_OUT}" 2> "${GO_ERR}"
echo "$?" > "${GO_EXIT}"

# Run Rust binary in release mode (cached after first build).
cargo run --release --quiet --manifest-path "${RS_MANIFEST}" -- "$@" > "${RS_OUT}" 2> "${RS_ERR}"
echo "$?" > "${RS_EXIT}"
set -e

# Compare results.
DIVERGED=0
if ! diff -q "${GO_OUT}" "${RS_OUT}" > /dev/null; then
  DIVERGED=1
  echo "shadow-diff: stdout DIVERGED" >&2
  diff -u "${GO_OUT}" "${RS_OUT}" | head -40 >&2 || true
fi
if ! diff -q "${GO_ERR}" "${RS_ERR}" > /dev/null; then
  DIVERGED=1
  echo "shadow-diff: stderr DIVERGED" >&2
  diff -u "${GO_ERR}" "${RS_ERR}" | head -40 >&2 || true
fi
GO_EXIT_VAL="$(cat "${GO_EXIT}")"
RS_EXIT_VAL="$(cat "${RS_EXIT}")"
if [[ "${GO_EXIT_VAL}" != "${RS_EXIT_VAL}" ]]; then
  DIVERGED=1
  echo "shadow-diff: exit code DIVERGED (go=${GO_EXIT_VAL} rs=${RS_EXIT_VAL})" >&2
fi

if [[ "${DIVERGED}" -eq 0 ]]; then
  echo "Shadow diff PASS (exit ${GO_EXIT_VAL})"
  exit 0
else
  echo "Shadow diff FAIL — see stderr above" >&2
  # Persist divergence artefacts for post-mortem (kept until next /tmp cleanup).
  ARTIFACT="/tmp/shadow-diff-$(date +%s)"
  cp -r "${TMP}" "${ARTIFACT}" || true
  echo "Divergence artefacts saved to: ${ARTIFACT}" >&2
  exit 1
fi
