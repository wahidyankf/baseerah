# archived/ose-cli

This directory contains the archived Go source of `apps/ose-cli/` prior to its Rust
migration in 2026-05.

The Go implementation was replaced by a Rust rewrite that consumes `libs/rust-commons/`.
The Go library `libs/golang-link-commons/` (which this code depended on) is preserved in
the active workspace until the `ayokoding-cli` Rust migration completes.

Do not modify files in this directory. They are a historical snapshot only.
