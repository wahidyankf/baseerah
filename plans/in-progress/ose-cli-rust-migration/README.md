# ose-cli Rust Migration

Migrate `apps/ose-cli/` from Go to Rust, and extract a new shared Rust library `libs/rust-commons/` that encapsulates the link-checking logic currently implemented in `libs/golang-link-commons/`.

## Context

`apps/ose-cli/` is a Go CLI tool that validates internal links in `apps/ose-web/content/`. It depends on `libs/golang-link-commons/` for link-walking and output logic.

`apps/rhino-cli/` completed a successful Go-to-Rust port in 2026-05. This plan follows the same patterns (Cargo.toml lints, Nx target shapes, `rust-toolchain.toml`, `deny.toml`, smoke tests) established by that port.

Phase 0 creates `libs/rust-commons/`, a new Rust library crate that ports the Go link-checking logic from `libs/golang-link-commons/`. This library will also be consumed by the sibling `ayokoding-cli-rust-migration` plan (executed independently in parallel). Phase 1 rewrites `apps/ose-cli/` in Rust consuming `libs/rust-commons/`. Phase 2 archives the Go source.

The Go library `libs/golang-link-commons/` is **not deleted** — it remains until the `ayokoding-cli-rust-migration` plan completes its own migration.

## Scope

**In scope:**

- New `libs/rust-commons/` Rust library crate (link-check types, walker, output functions)
- Rust rewrite of `apps/ose-cli/` (binary + lib) consuming `libs/rust-commons/`
- Archive Go source to `archived/ose-cli/` [_New directory_]
- Remove Go build artifacts from `apps/ose-cli/`
- Update `apps/ose-cli/project.json` tags to `lang:rust`

**Out of scope:**

- Deleting `libs/golang-link-commons/` (blocked by `ayokoding-cli` migration)
- `ayokoding-cli` Rust migration (separate plan, executed in parallel)
- `organiclever-be` plan (separate plan, unrelated)
- New link-checking features beyond Go parity
- External link checking (intentionally excluded, same as Go version)
- Deployment pipeline changes

## Approach Summary

1. **Phase 0** — Create `libs/rust-commons/` with ported link-checking logic, full TDD, 90% coverage
2. **Phase 1** — Rewrite `apps/ose-cli/` in Rust using `libs/rust-commons/`, smoke tests, Nx targets
3. **Phase 2** — Archive Go source, remove Go artifacts, update tags
4. **Phase 3** — Local quality gates (typecheck, lint, test:quick, spec-coverage)
5. **Phase 4** — Post-push CI verification
6. **Phase 5** — Plan archival

## Documents

- [Business Requirements (BRD)](./brd.md)
- [Product Requirements (PRD)](./prd.md)
- [Technical Documentation](./tech-docs.md)
- [Delivery Checklist](./delivery.md)

## Status

In Progress
