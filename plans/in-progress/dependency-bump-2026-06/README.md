# Dependency Bump — June 2026 Sweep

> **Stage**: in-progress (promoted `2026-06-04`; created `2026-06-04`) · **Plan identifier**: `dependency-bump-2026-06`
> **Push target**: `origin main` (Trunk Based Development — direct push, no PR)

## Context

This plan operationalizes the
[Dependency Bump Stability & Safety Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md)
for the June 2026 maintenance window. The macro-decisions (which dependencies to bump, to which
versions, and which to hold) were already resolved by the upstream
`repo-dependency-bump-planning` workflow and confirmed at a human checkpoint. The backing
clearance report is
[`generated-reports/repo-dependency-bump-planning__906c25d8-cfd7-42a3-be2b-7207804c2cac__2026-06-04--07-51__report.md`](../../../generated-reports/repo-dependency-bump-planning__906c25d8-cfd7-42a3-be2b-7207804c2cac__2026-06-04--07-51__report.md).

The repo was swept recently by two completed plans —
[`2026-05-15__stack-update`](../../done/2026-05-15__stack-update/) and
[`2026-04-22__dependency-update`](../../done/2026-04-22__dependency-update/). Most pins already
sit at or above the Path B soak-eligible version. Under strict policy, the correct action for the
large majority of dependencies is **NO CHANGE** — never downgrade a pin that is newer than the
soak-eligible target. This plan therefore covers a focused, pre-approved set across four tiers.

This plan is a **snapshot as of 2026-06-04** (Path B soak cutoff `2026-04-05`). If promotion to
`in-progress/` is delayed, the eligibility check (versions + CVEs) MUST be re-run before execution.
See [tech-docs.md §Snapshot Validity](./tech-docs.md#snapshot-validity-and-re-verification).

## Scope

**In scope** (four approved tiers):

- **Tier 1 — Security**: migrate `rhino-cli` off the unmaintained `serde_yml` crate; ensure
  `tokio` lockfile floor ≥ 1.51.0 for `organiclever-be` and `ose-app-be`.
- **Tier 2 — LTS / stable refresh**: Node `24.15.0 → 24.16.0` (root volta); Debian runtime base
  `bookworm-slim → trixie-slim` in the two backend integration Dockerfiles.
- **Tier 3 — Soak-eligible breaking upgrades**: `crane-cli` .NET test stack
  (`Microsoft.NET.Test.Sdk`, `xunit` v2 → `xunit.v3`, `xunit.runner.visualstudio`,
  `coverlet.collector`, `coverlet.msbuild`); remove deprecated `@hey-api/client-fetch`.
- **Tier 4 — GitHub Actions major tags**: confirm-then-bump first-party and selected third-party
  actions, with mandatory re-verification of the latest stable major as the first step.

**Out of scope** (considered and held — see
[tech-docs.md §Considered and Held](./tech-docs.md#considered-and-held-out-of-scope)):

- Rust toolchain `1.96.0` (current `1.95.0` already past cutoff; `1.96.0` not soak-eligible).
- PostgreSQL `18` (a `pg_upgrade` data migration, not a tag bump).
- The broad mass of npm/cargo deps already at or above soak-eligible and CVE-clean.
- Vestigial CI language pins (Go, Java, Python, golangci-lint, cargo-llvm-cov, cargo-deny,
  cargo-hack) with no active consuming project.

**Noted side-item (optional, not a dependency bump)**: the
`infra/dev/{rhino-cli,ose-cli,ayokoding-cli}/Dockerfile.cli.dev` files still use
`golang:1.26-alpine` though those CLIs are now Rust — a correctness cleanup the plan may mention
but must not conflate with version bumps.

## Approach summary

Work proceeds tier by tier, each as a phase that ends in a coherent, green tree. Phase 0
(`repo-setup-manager`) establishes a clean baseline. Code-touching changes (serde_yml migration,
xunit v2 → v3 migration) are TDD-shaped — existing unit/integration tests are the regression
guard. Manifest-only changes (Node pin, Debian base, action tags, hey-api removal) are verified by
build + the affected quality gates. A final phase re-audits security (`npm audit`,
`cargo deny check advisories`), updates the security-waivers register if needed (no new waiver is
expected — the serde_yml migration removes the advisory rather than waiving it), runs the full
affected quality gate, and confirms agents-sync byte-stability.

## Navigation

- [brd.md](./brd.md) — business rationale (WHY)
- [prd.md](./prd.md) — product requirements and Gherkin acceptance criteria (WHAT)
- [tech-docs.md](./tech-docs.md) — architecture, per-item design decisions, held items (HOW)
- [delivery.md](./delivery.md) — phased, TDD-shaped delivery checklist (DO)
