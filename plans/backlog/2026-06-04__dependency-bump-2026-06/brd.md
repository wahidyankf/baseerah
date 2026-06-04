# Business Requirements — Dependency Bump June 2026

## Business goal

Keep the monorepo's dependency surface current, secure, and within the
[Dependency Bump Stability & Safety Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md),
closing the one outstanding security advisory and refreshing a focused set of soak-eligible
dependencies without destabilizing any active project.

## Business rationale (WHY)

- **Security debt is the highest-cost debt.** The `serde_yml` crate carries
  RUSTSEC-2025-0068 (unsound + unmaintained, no patched version available). Leaving an unmaintained
  crate with an open advisory in a repository-hygiene CLI (`rhino-cli`) undermines the credibility
  of the tooling that polices the rest of the repo. Migrating to a maintained crate removes the
  advisory entirely rather than waiving it. _[Repo-grounded: `apps/rhino-cli/Cargo.toml:22`
  pins `serde_yml = "0.0.12"`.]_
- **Routine soak-eligible refreshes prevent large, risky jumps later.** Bumping the `crane-cli`
  test stack and Node LTS patch now — while each step is individually soak-eligible and reversible —
  avoids accumulating a multi-major backlog that would force a high-risk batch upgrade later.
- **Base-image currency avoids end-of-support cliffs.** Debian 12 "bookworm" full support ends
  around `2026-06-10`; moving the runtime base to Debian 13 "trixie" (stable since `2025-08-09`,
  well soaked) keeps the backend integration images on a supported, patched base.
  _[Judgment call: end-of-support timing taken from the backing clearance report; re-verify on
  promotion.]_

## Business impact

- **Pain removed**: one open RUSTSEC advisory cleared; a deprecated npm dev dependency
  (`@hey-api/client-fetch`) removed; a deprecated xunit v2 test stack modernized to the supported
  v3 line.
- **Expected benefit**: a green `cargo deny check advisories` and `npm audit --audit-level=moderate`
  for the affected projects, with no new security waiver added.
- **Cost of inaction**: the advisory remains; the bookworm base drifts past full support; the
  xunit v2 stack continues to accrete migration distance from the supported v3 line.

## Affected roles

This is a solo-maintainer repository — no sign-off ceremonies. The maintainer wears:

- **Rust developer** hat — performs the `serde_yml` migration and tokio-floor verification.
- **F#/.NET developer** hat — performs the `crane-cli` xunit v3 + coverlet 8 migration.
- **Release/infra** hat — bumps the Node pin, Debian base, and GitHub Actions tags.
- **Security reviewer** hat — runs the post-bump re-audit and maintains the waivers register.

Consuming agents: `repo-setup-manager` (Phase 0 baseline), `swe-rust-dev`, `swe-fsharp-dev`,
`ci-checker` (CI verification), and the maker-checker-fixer planning agents.

## Business-level success metrics

- RUSTSEC-2025-0068 no longer reported by `cargo deny check advisories` after the migration
  (observable check — see [prd.md](./prd.md) Scenario 1). _[Repo-grounded: cargo-deny is the
  repo's advisory tool per the policy doc.]_
- Zero new rows added to
  [`docs/reference/security-waivers.md`](../../../docs/reference/security-waivers.md) for the
  serde*yml item (the migration removes, not waives, the advisory).*[Repo-grounded: waivers
  register exists.]\_
- Every bumped manifest pinned exactly — no `^`/`~` for any in-scope item (observable check).

## Business-scope non-goals

- Not a comprehensive "bump everything" sweep — the policy forbids downgrading pins already newer
  than soak-eligible; the held set stays untouched.
- Not a Rust toolchain bump, not a PostgreSQL major migration, not a CI vestigial-pin cleanup.
- Not a refactor of `rhino-cli` YAML semantics beyond swapping the crate behind the same behavior.

## Business risks and mitigations

| Risk                                                            | Mitigation                                                                                                                     |
| --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| Snapshot staleness if promotion is delayed                      | tech-docs mandates re-running the eligibility check (versions + CVEs) before execution; cutoff dates recorded in writing.      |
| serde_yml migration changes YAML behavior subtly                | TDD-shaped: existing `rhino-cli` unit tests that exercise YAML parsing are the regression guard and must stay green.           |
| xunit v2 → v3 breaking migration destabilizes `crane-cli` tests | Grouped into one coordinated phase with coverlet 6 → 8; unit + integration tests must stay green before the phase gate passes. |
| GitHub Actions research had low confidence on release dates     | Tier 4 mandates re-verifying the latest stable major of each action as its first step before any edit.                         |
| rhino-cli code change drifts the generated platform bindings    | Final phase asserts `npm run generate:bindings` produces no diff in `.opencode/`/`.amazonq/`.                                  |
