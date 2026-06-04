# Technical Documentation — Dependency Bump June 2026

## Source of truth and backing report

This plan operationalizes the
[Dependency Bump Stability & Safety Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md)
(Path A / Path B / Path C decision tree; Rule 5 recency + functional-stability; exact-pin
requirement). The clearance report backing the approved scope is
[`generated-reports/repo-dependency-bump-planning__906c25d8-cfd7-42a3-be2b-7207804c2cac__2026-06-04--07-51__report.md`](../../../generated-reports/repo-dependency-bump-planning__906c25d8-cfd7-42a3-be2b-7207804c2cac__2026-06-04--07-51__report.md).

**Key dates** (recorded in writing per policy):

- As-of-date: `2026-06-04`
- Path B soak cutoff: `2026-04-05` (a Path B candidate must have been released on or before this
  date to be soak-eligible)
- Path A items (LTS latest patch, stable base images) are exempt from the 60-day soak.

## Snapshot validity and re-verification

This plan is a **snapshot as of the cutoff above**. Version availability and CVE status drift over
time. **If promotion from `backlog/` to `in-progress/` is delayed, the eligibility check (current
latest versions + CVE clearance for every in-scope item) MUST be re-run before execution begins,**
and any item that has changed eligibility re-approved. Do not execute a stale snapshot.

## Per-item design decisions

### Tier 1 — Security

#### 1. serde_yml → migrate off (rhino-cli)

- **Current**: `serde_yml = "0.0.12"` _[Repo-grounded: `apps/rhino-cli/Cargo.toml:22`]_.
- **Advisory**: RUSTSEC-2025-0068 — unsound + unmaintained; advisory lists **no patched version**.
- **Approved resolution**: migrate `rhino-cli`'s YAML handling to a maintained crate —
  **`serde_norway` (preferred; serde_yaml-compatible API)** or `serde-saphyr`. This is a **code
  change**, not a version bump: replace the dependency and update all `use serde_yml` call sites.
- **Call sites to migrate** _[Repo-grounded: `grep -rln serde_yml apps/rhino-cli/src`]_:
  - `apps/rhino-cli/src/internal/repo_governance/frontmatter_audit.rs`
  - `apps/rhino-cli/src/internal/bcregistry.rs`
  - `apps/rhino-cli/src/internal/agents/converter.rs`
  - `apps/rhino-cli/src/internal/agents/skill_validator.rs`
  - `apps/rhino-cli/src/internal/agents/sync_validator.rs`
  - `apps/rhino-cli/src/internal/agents/frontmatter.rs`
  - `apps/rhino-cli/src/internal/agents/agent_validator.rs`
  - `apps/rhino-cli/src/internal/docs/frontmatter.rs`
- **TDD guard**: the existing `rhino-cli` unit tests that exercise YAML parsing are the regression
  guard — they must stay green through the migration.
- **Waiver impact**: after migration `serde_yml` is removed entirely, so **no security waiver is
  needed** — the advisory is removed, not waived.
- **Acceptance**: `cargo build` + `nx run rhino-cli:test:quick` green; `grep -r serde_yml
apps/rhino-cli/src` returns nothing; `npm run generate:bindings` still byte-stable.

#### 2. tokio lockfile floor (organiclever-be, ose-app-be)

- **Current**: `tokio = { version = "1", features = ["full"] }` _[Repo-grounded:
  `apps/organiclever-be/Cargo.toml:29,40`, `apps/ose-app-be/Cargo.toml:29,40`]_.
- **Advisory**: RUSTSEC-2025-0023 (broadcast channel); patched ≥ `1.44.2`. Tokio `1.51.0`
  (released `2026-04-03`) is soak-eligible.
- **Action**: ensure `Cargo.lock` resolves tokio ≥ `1.51.0`. Run
  `cargo update -p tokio --precise 1.51.0` (or confirm the lock is already ≥ `1.51.0`), then
  re-audit. The version-range string in `Cargo.toml` (`"1"`) does not change — this is a lockfile
  floor, not a manifest pin change. **Likely already satisfied — verify, do not assume.**

### Tier 2 — LTS / stable refresh

#### 3. Node 24.15.0 → 24.16.0 (root volta)

- **Current**: volta `node` `24.15.0`, `npm` `11.11.0` _[Repo-grounded: `package.json:49-50`]_.
- **Path A** (Node 24 Active LTS; `24.16.0` is the latest LTS patch, released `2026-05-21`,
  includes March-2026 security fixes; Path A is exempt from the 60-day soak).
- **Action**: edit only `package.json` volta `node` → `24.16.0`. Leave `npm` at `11.11.0` (npm
  `11.16.0` released 2026-05-27 is post-cutoff, so not Path B soak-eligible _[Judgment call: npm
  11.16.0 release date is after the 2026-04-05 Path B cutoff; re-verify on promotion]_). Docker
  `node:24-alpine` tags float —
  no edit needed.

#### 4. Debian base bookworm-slim → trixie-slim (backend integration Dockerfiles)

- **Current**: `FROM debian:bookworm-slim` runtime stage _[Repo-grounded:
  `apps/organiclever-be/Dockerfile.integration:10`, `apps/ose-app-be/Dockerfile.integration:10`]_.
- **Path A** (Debian 13 "trixie" stable since `2025-08-09`, well soaked; Debian 12 full-support
  ends ~`2026-06-10`).
- **Action**: change only the `debian` runtime `FROM` line to `debian:trixie-slim`. **Leave the
  `rust:1.95-slim` builder stage as-is** _[Repo-grounded: builder at line 2 of each Dockerfile]_ —
  the Rust toolchain bump is held (see below).

### Tier 3 — Soak-eligible breaking upgrades

All in `apps/crane-cli/` test fsproj files. The main `apps/crane-cli/crane-cli.fsproj` carries no
test dependencies _[Repo-grounded: grep returned no matches]_, so only the two test projects change:

- `apps/crane-cli/tests/unit/crane-cli-unit-tests.fsproj`
- `apps/crane-cli/tests/integration/crane-cli-integration-tests.fsproj`

| Item                         | Current | Target         | Notes                                                                                                                       |
| ---------------------------- | ------- | -------------- | --------------------------------------------------------------------------------------------------------------------------- |
| 5. Microsoft.NET.Test.Sdk    | 17.11.1 | 18.3.0         | soak-eligible 2026-02-24                                                                                                    |
| 6. xunit → xunit.v3          | 2.9.2   | xunit.v3 3.2.2 | **breaking** v2→v3 (package rename, API changes); v2 deprecated                                                             |
| 7. xunit.runner.visualstudio | 2.8.2   | 3.1.5          | pairs with xunit.v3 (2025-09-27)                                                                                            |
| 8. coverlet.collector        | 6.0.2   | 8.0.1          | **unit fsproj ONLY** (integration fsproj has no coverlet); 6→8 major; needs .NET SDK 8+ (net10 OK); dropped Newtonsoft.Json |
| 9. coverlet.msbuild          | 6.0.2   | 8.0.1          | **unit fsproj ONLY** (integration fsproj has no coverlet); 6→8 major (2026-03-17)                                           |

_[Repo-grounded: versions verified at
`apps/crane-cli/tests/unit/crane-cli-unit-tests.fsproj:36-47` and
`apps/crane-cli/tests/integration/crane-cli-integration-tests.fsproj:15-17`.]_

- **Coordination**: items 5–9 are **one coordinated migration** — they must land together because
  xunit v2↔v3 and coverlet 6↔8 are interdependent. Group them in a single delivery phase.
- **xunit v3 migration reference**: <https://xunit.net/docs/getting-started/v3/migration>
  _[Web-cited — access this during execution to confirm the current API delta; the migration is a
  package rename to `xunit.v3` plus API changes.]_
- **coverlet 8 caveat**: v8 dropped Newtonsoft.Json — verify no coverlet config depends on it
  (check `apps/crane-cli/tests/unit/xunit.runner.json` and any coverage settings).
- **TDD guard**: crane-cli unit + integration tests must stay green.

#### 10. Remove @hey-api/client-fetch (housekeeping)

- **Current**: `@hey-api/client-fetch: "0.13.1"` in root devDependencies; `@hey-api/openapi-ts:
"0.94.2"` already present _[Repo-grounded: `package.json:55-56`]_.
- **Rationale**: `@hey-api/client-fetch` is deprecated; the client is bundled into
  `@hey-api/openapi-ts` (≥ 0.73; repo has 0.94.2). Housekeeping removal, not a version bump.
- **Action**: remove the devDependency; verify the contract codegen (`nx run
organiclever-contracts:lint` and any `codegen` target) still works using openapi-ts's built-in
  fetch client; adjust codegen config if it referenced the standalone client.

### Tier 4 — GitHub Actions major tags (low research confidence)

The version research for GitHub Actions returned internally inconsistent release dates. **The FIRST
step of this phase MUST re-verify the actual latest stable major of each action** (via `gh api` or
the action's releases page) before any edit. No CVEs found against first-party `actions/*`.

Candidate bumps to **confirm-then-apply** across `.github/workflows/*.yml` and
`.github/actions/*/action.yml` _[Repo-grounded: current tags present per
`grep -rn 'uses:' .github/workflows .github/actions`]_:

- `actions/checkout` v4 → latest
- `actions/cache` v4 → latest
- `actions/upload-artifact` v4 → latest
- `actions/setup-node` v4 → latest
- `actions/setup-go` v5 → latest
- `actions/setup-java` v4 → latest
- `actions/setup-python` v5 → latest
- `actions/setup-dotnet` v4 → latest
- `docker/setup-buildx-action` v3 → latest
- `volta-cli/action` v4 → v5 (released `2026-04-01`; soak completes ~`2026-06-01`)

**Hold (already latest major)** _[Repo-grounded: both present in workflows]_:

- `actions-rust-lang/setup-rust-toolchain` v1
- `Swatinem/rust-cache` v2

**Optional defense-in-depth note**: SHA-pinning actions could be considered, but this plan does
**not** mandate it.

## Considered and held (out of scope)

| Held item                                                                                               | Reason                                                                                                                                   |
| ------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Rust toolchain `1.96.0` (all `rust-toolchain.toml` + `rust:1.95-slim` builder stages)                   | Latest `1.96.0` (2026-05-28) NOT soak-eligible until ~`2026-07-27`; current `1.95.0` (2026-04-16) already past cutoff. Hold; next sweep. |
| PostgreSQL `18-alpine`                                                                                  | PG 18 is a `pg_upgrade` data migration, not a tag bump — out of routine scope; flag for a dedicated future plan.                         |
| Broad mass of npm/cargo deps (nx 22.5.4, prettier 3.8.1, jsdom, clap, axum, cucumber, reqwest, sqlx, …) | Current pins ≥ soak-eligible and CVE-clean. Policy forbids downgrading a pin newer than soak-eligible. Hold.                             |
| CI Go 1.25.8 / golangci-lint / Java 25 / Python 3.13 / cargo-llvm-cov / cargo-deny / cargo-hack         | No active projects in those languages (vestigial CI pins). Low priority; hold.                                                           |

## Cleanup observation (noted, not a bump)

`infra/dev/{rhino-cli,ose-cli,ayokoding-cli}/Dockerfile.cli.dev` still use `golang:1.26-alpine`
though those CLIs are now Rust _[Repo-grounded: all three files line 1]_. A correctness cleanup the
plan may mention but must not conflate with version bumps. If addressed, do it in a separate commit
under a separate concern; otherwise leave for a dedicated cleanup plan.

## Testing strategy

- **Tier 1 serde_yml (code)**: TDD via existing `rhino-cli` YAML-parsing unit tests
  (`nx run rhino-cli:test:unit` / `:test:quick`) — RED proves the migration target compiles only
  after call-site updates; GREEN is the suite passing; REFACTOR tidies the new crate usage.
- **Tier 1 tokio (lockfile)**: verification-only via `cargo deny check advisories`.
- **Tier 2 (manifests)**: build + affected quality gates; Debian change verified by backend
  integration tests (`nx run organiclever-be-e2e:test:e2e` / `ose-app-be-e2e:test:e2e` as relevant
  and the integration Dockerfile build).
- **Tier 3 crane-cli (code/test-stack)**: TDD via `nx run crane-cli:test:quick` plus the crane-cli
  integration tests — the test stack migration is validated by its own green suite.
- **Tier 4 (CI config)**: validated by the post-push GitHub Actions run.

| Acceptance criterion (prd.md) | Test level                                                      |
| ----------------------------- | --------------------------------------------------------------- |
| Scenario 1 serde_yml          | unit (`rhino-cli:test:unit`) + advisory audit (`cargo deny`)    |
| Scenario 2 tokio              | advisory audit (`cargo deny`)                                   |
| Scenario 3 Node pin           | `npm run doctor`                                                |
| Scenario 4 Debian             | integration (backend integration Dockerfile build + e2e)        |
| Scenario 5 crane-cli xunit v3 | unit + integration (`crane-cli:test:quick` + integration suite) |
| Scenario 6 hey-api removal    | codegen (`organiclever-contracts:lint` + codegen target)        |
| Scenario 7 Actions tags       | CI (post-push GitHub Actions)                                   |
| Scenario 8 exact pins         | static inspection (`grep` for `^`/`~`)                          |
| Scenario 9 post-bump audit    | `npm audit` + `cargo deny`                                      |
| Scenario 10 agents sync       | `npm run generate:bindings` diff check                          |

## Definition of done (mirrors policy Application Workflow steps 8–12)

1. Every in-scope manifest pinned **exactly** (no `^`/`~`) to its approved target.
2. Lockfiles regenerated: `npm install` (root), `cargo update -p <crate>` (Rust), .NET restore
   (crane-cli).
3. Post-bump security re-audit clean: `npm audit --audit-level=moderate`; for Rust,
   `cargo deny check advisories` — confirm the serde_yml advisory is **gone** after migration and
   tokio ≥ `1.51.0`.
4. Any residual WAIVER / FUNCTIONAL-HOLD entries **appended** to
   [`docs/reference/security-waivers.md`](../../../docs/reference/security-waivers.md) (append model
   — do NOT redefine existing rows). The serde_yml migration means **no new waiver for it**; only
   add a FUNCTIONAL-HOLD row if some item ends up pinned below latest due to a defect.
5. Affected-project quality gates pass: typecheck, lint, test:quick, spec-coverage (rhino-cli,
   organiclever-be, ose-app-be, crane-cli, and the root for node/CI changes).
6. Agents sync byte-stable after any rhino-cli change: `npm run generate:bindings` produces no diff
   in `.opencode/`/`.amazonq/`.

## Git workflow

Trunk Based Development — direct push to `origin main`, no PR. The plan was not requested with a PR
and `delivery.md` contains no PR step. Commits are thematic and grouped by tier/concern per
Conventional Commits.

## Rollback

Each phase is independently revertable: dependency edits are git-reverted and lockfiles regenerated.
The serde_yml and xunit v3 migrations are the only code-touching changes — revert restores the
prior crate/test-stack and the prior green test baseline.
