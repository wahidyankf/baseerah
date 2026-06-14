#!/usr/bin/env bash
# cargo-deny check with the advisories check TEMPORARILY skipped.
#
# WHY: as of 2026-06-14 the upstream RustSec advisory-db HEAD ships a malformed
# advisory (crates/libcrux-chacha20poly1305/RUSTSEC-2026-0124.md — TOML parse
# error at line 8) that makes the `advisories` check fail to load the whole
# database. cargo-deny cannot skip a single broken advisory file, and pinning the
# advisory database to a known-good revision does NOT hold: cargo-deny
# re-materializes the upstream HEAD on open regardless of a pre-seeded pin,
# `git reset --hard`, removing the origin remote, `--offline`, or
# `--disable-fetch` (all five were verified to fail in CI). The only reliable
# mitigation until upstream fixes the advisory is to skip the advisories check.
#
# We still run bans, licenses, and sources, so supply-chain/license/source
# gating is fully preserved; only known-vulnerability advisory scanning is
# paused.
#
# REVERT: once upstream fixes RUSTSEC-2026-0124.md, restore the single
# `cargo deny --manifest-path apps/rhino-cli/Cargo.toml check` command in
# apps/rhino-cli/project.json and delete this script. Tracked by a plan follow-on.
set -euo pipefail

MANIFEST="apps/rhino-cli/Cargo.toml"

echo "NOTE: advisories check skipped (upstream RUSTSEC-2026-0124 advisory-db corruption); bans/licenses/sources still enforced."
cargo deny --manifest-path "$MANIFEST" check bans licenses sources
