# Product Requirements — Dependency Bump June 2026

## Product overview

A maintenance deliverable that applies a focused, pre-approved set of dependency bumps across four
tiers, leaving every affected project building green with a clean post-bump security audit and
byte-stable platform bindings.

## Personas

Solo-maintainer repository — personas are the hats the maintainer wears plus the agents that
consume the deliverable:

- **Maintainer-as-Rust-dev** — wants the `serde_yml` advisory gone with no behavior regression.
- **Maintainer-as-FSharp-dev** — wants `crane-cli` on the supported xunit v3 / coverlet 8 stack
  with tests green.
- **Maintainer-as-infra** — wants Node, Debian base, and Actions tags current and pinned exactly.
- **Maintainer-as-security-reviewer** — wants `cargo deny` and `npm audit` clean afterward.
- **`repo-setup-manager` agent** — consumes Phase 0 to establish the baseline.

## User stories

- As a Rust developer, I want `rhino-cli` to stop depending on the unmaintained `serde_yml` crate,
  so that RUSTSEC-2025-0068 no longer applies and no waiver is needed.
- As an F#/.NET developer, I want the `crane-cli` test stack on `xunit.v3` and `coverlet` 8, so
  that the project is off the deprecated xunit v2 line and tests still pass.
- As an infra maintainer, I want Node pinned to the latest 24 LTS patch and the backend runtime
  images on a supported Debian base, so that the toolchain stays current and within support.
- As a security reviewer, I want a clean post-bump audit and an accurate waivers register, so that
  the security posture is verifiable.
- As an infra maintainer, I want GitHub Actions on confirmed-current major tags, so that CI runs on
  maintained actions.

## Acceptance criteria (Gherkin)

### Scenario 1 — serde_yml removed and advisory cleared

```gherkin
Given rhino-cli currently depends on serde_yml = "0.0.12" in apps/rhino-cli/Cargo.toml
  And RUSTSEC-2025-0068 applies to serde_yml
When rhino-cli's YAML handling is migrated to a maintained crate (serde_norway preferred)
  And the serde_yml dependency is removed from apps/rhino-cli/Cargo.toml
Then `grep -r serde_yml apps/rhino-cli/src` returns no matches
  And `cargo deny check advisories` no longer reports RUSTSEC-2025-0068
  And `nx run rhino-cli:test:quick` passes
  And no new waiver row is added to docs/reference/security-waivers.md for serde_yml
```

### Scenario 2 — tokio lockfile floor satisfied

```gherkin
Given organiclever-be and ose-app-be pin tokio = { version = "1", ... }
  And RUSTSEC-2025-0023 is patched at tokio >= 1.44.2
When the lockfiles are verified (or updated via `cargo update -p tokio --precise 1.51.0`)
Then Cargo.lock resolves tokio >= 1.51.0 for both backends
  And `cargo deny check advisories` reports no tokio broadcast-channel advisory
```

### Scenario 3 — Node pinned to 24.16.0

```gherkin
Given the root package.json volta block pins node "24.15.0"
When the volta node pin is updated to "24.16.0"
Then package.json volta.node equals "24.16.0" exactly (no caret or tilde)
  And npm stays pinned at "11.11.0"
  And `npm run doctor` reports the Node version satisfied
```

### Scenario 4 — Debian runtime on trixie-slim

```gherkin
Given apps/organiclever-be/Dockerfile.integration and apps/ose-app-be/Dockerfile.integration
      use `FROM debian:bookworm-slim` for the runtime stage
When the runtime FROM line is changed to `debian:trixie-slim`
Then both integration Dockerfiles reference debian:trixie-slim for the runtime stage
  And the `rust:1.95-slim` builder stage is left unchanged
  And the backend integration tests pass against the rebuilt images
```

### Scenario 5 — crane-cli test stack on xunit.v3 + coverlet 8 with tests green

```gherkin
Given both crane-cli test fsproj files pin xunit 2.9.2, Microsoft.NET.Test.Sdk 17.11.1,
      and xunit.runner.visualstudio 2.8.2
  And only the unit test fsproj pins coverlet.collector 6.0.2 and coverlet.msbuild 6.0.2
When the test stack is migrated to xunit.v3 3.2.2, Microsoft.NET.Test.Sdk 18.3.0,
     xunit.runner.visualstudio 3.1.5, coverlet.collector 8.0.1, coverlet.msbuild 8.0.1
Then both crane-cli test fsproj files reference xunit.v3 (not xunit v2)
  And `nx run crane-cli:test:quick` passes
  And the crane-cli integration tests pass
```

### Scenario 6 — @hey-api/client-fetch removed and codegen still works

```gherkin
Given root package.json devDependencies include @hey-api/client-fetch 0.13.1
  And @hey-api/openapi-ts 0.94.2 bundles the fetch client
When @hey-api/client-fetch is removed from devDependencies
  And the contract codegen config is adjusted if it referenced the standalone client
Then `npm install` resolves without @hey-api/client-fetch
  And `nx run organiclever-contracts:lint` succeeds
  And the organiclever codegen target regenerates types using openapi-ts's built-in fetch client
```

### Scenario 7 — GitHub Actions on confirmed-current major tags

```gherkin
Given .github/workflows/*.yml and .github/actions/*/action.yml reference action major tags
When the latest stable major of each in-scope action is re-verified before editing
  And confirmed-current majors are applied (e.g. volta-cli/action v4 -> v5)
Then every edited workflow references the confirmed latest stable major
  And actions-rust-lang/setup-rust-toolchain v1 and Swatinem/rust-cache v2 are left unchanged
  And all GitHub Actions workflows pass after the push
```

### Scenario 8 — no manifest left with caret/tilde for bumped items

```gherkin
Given every in-scope manifest has been edited
When the bumped pins are inspected
Then no bumped item uses a caret (^) or tilde (~) range
  And every bumped item is an exact version pin
```

### Scenario 9 — post-bump npm audit + cargo deny clean

```gherkin
Given all in-scope bumps are applied and lockfiles regenerated
When `npm audit --audit-level=moderate` runs at the repo root
  And `cargo deny check advisories` runs for the Rust workspace
Then npm audit reports no moderate-or-higher advisories introduced by this plan
  And cargo deny reports no advisories for serde_yml or tokio
```

### Scenario 10 — agents sync byte-stable after rhino-cli change

```gherkin
Given rhino-cli source was modified for the serde_yml migration
When `npm run generate:bindings` runs
Then `.opencode/` and `.amazonq/` show no diff
```

## Product scope

**In scope**: the four tiers and ten work items enumerated in [tech-docs.md](./tech-docs.md), plus
the post-bump re-audit, waivers-register check, full affected quality gate, and agents-sync check.

**Out of scope**: Rust toolchain `1.96.0`, PostgreSQL `18`, the held mass of already-current
npm/cargo deps, and vestigial CI language pins. The `golang:1.26-alpine` dev-Dockerfile cleanup is
an optional noted side-item, not a scoped bump.

## Product risks

- A breaking xunit v2 → v3 API change not covered by existing tests could pass the gate while
  hiding a runtime issue — mitigated by running both unit and integration suites.
- coverlet 8 dropped Newtonsoft.Json; if any coverlet config relied on it, coverage collection
  could break — verification step checks coverage output explicitly.
- Removing `@hey-api/client-fetch` could break codegen if the config referenced the standalone
  client — the codegen target is re-run as the acceptance check.
