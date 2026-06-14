#!/usr/bin/env bash
# cargo-deny check with a TEMPORARY pinned advisory database.
#
# WHY: as of 2026-06-14 the upstream RustSec advisory-db HEAD ships a malformed
# advisory (crates/libcrux-chacha20poly1305/RUSTSEC-2026-0124.md — TOML parse
# error at line 8) that makes `cargo deny check` fail to load the whole database.
# cargo-deny cannot skip a single broken advisory file, so we pin the advisory
# database to the last known-good revision and run the advisories check offline
# against it. Full advisory gating is preserved (1-day-old snapshot); bans,
# licenses, and sources run online as normal.
#
# REVERT: once upstream fixes RUSTSEC-2026-0124.md, restore the single
# `cargo deny --manifest-path apps/rhino-cli/Cargo.toml check` command in
# apps/rhino-cli/project.json and delete this script. Tracked by a plan follow-on.
set -euo pipefail

PIN="09735e12749564d6f364ecab7d723caf52ada026"
DB_DIR="${CARGO_HOME:-$HOME/.cargo}/advisory-dbs/advisory-db-3157b0e258782691"
MANIFEST="apps/rhino-cli/Cargo.toml"

if [ ! -d "$DB_DIR/.git" ]; then
  mkdir -p "$(dirname "$DB_DIR")"
  git clone --quiet https://github.com/rustsec/advisory-db "$DB_DIR"
fi
git -C "$DB_DIR" fetch --quiet origin || true
git -C "$DB_DIR" checkout --quiet "$PIN"

# Advisories: offline against the pinned good DB (avoids the corrupt HEAD).
# (--offline is a global flag and must precede the `check` subcommand.)
cargo deny --manifest-path "$MANIFEST" --offline check advisories
# Bans / licenses / sources: online as usual (need the crates index).
cargo deny --manifest-path "$MANIFEST" check bans licenses sources
