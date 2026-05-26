# Business Requirements Document — ose-app-be Rust Migration

## Business Goal

Migrate `apps/ose-app-be/` from F#/Giraffe to Rust/Axum so that the OSE Application backend
joins the same technology tier as `apps/organiclever-be/`. A single Rust/Axum stack across both
backends reduces cognitive overhead, eliminates the .NET 10 toolchain dependency from the repo,
and ensures future bounded-context implementations are written in the language the team already
uses for backend work in this monorepo.

## Business Rationale

The F# implementation exists because `ose-app-be` was bootstrapped before the Rust standard
emerged. Now that `organiclever-be` has proved the Rust/Axum pattern — strict clippy, llvm-cov
coverage, cucumber BDD, deny.toml, hexagonal DDD layout — keeping a parallel .NET toolchain adds
cost without adding value:

- .NET SDK, AltCover, Fantomas, and FSharpLint require separate installation and version
  management in the Volta-free .NET path.
- Developer context-switching between F# and Rust increases onboarding friction.
- CI time is longer because both toolchains must be installed and warmed.

The migration replaces a two-toolchain repository with a single Rust backend standard.
_Judgment call: qualitative reasoning — no measured build-time data available._

## Affected Roles

This is a solo-maintainer repository. The maintainer wears all relevant hats:

- **Platform engineer** — configures Nx targets, CI, and Docker
- **Backend engineer** — writes and reviews Rust code
- **Spec author** — maintains `bounded-contexts.yaml` and Gherkin feature files
- **Operations** — monitors the running service

No sign-off ceremonies, sponsors, or stakeholder approvals are required.

## Business-Level Success Metrics

All metrics are observable facts; no fabricated KPIs.

1. `cargo build --release --manifest-path apps/ose-app-be/Cargo.toml` exits 0 with zero
   warnings. _Observable fact._
2. `nx run ose-app-be:test:quick` exits 0 (DDD validation + ≥90% line coverage). _Observable
   fact._
3. `nx run ose-app-be:lint` exits 0 (clippy -D warnings). _Observable fact._
4. No `.fsproj`, `global.json`, `dotnet-tools.json`, or `fsharplint.json` files remain in
   `apps/ose-app-be/`. _Observable fact._
5. `specs/apps/ose-app/ddd/bounded-contexts.yaml` lists `code_lang: [rs]` for all four
   YAML-listed contexts (`regulatory-source`, `internal-policy`, `gap-analysis`,
   `ai-orchestration`). The `health` context exists in code but has no YAML entry and is
   not added in this plan. _Observable fact._

## Business-Scope Non-Goals

- Implementing any real feature logic in the four stub contexts. Those are deferred to
  future plans scoped to each bounded context individually.
- OpenRouter integration. The `ai-orchestration` stub documents the dependency; the actual
  HTTP client is out of scope.
- Kubernetes deployment changes. `ose-app-be` is not yet deployed; deployment configuration
  is a separate concern.
- Changing the `ose-app-web` frontend. No API contract changes are introduced.

## Business Risks and Mitigations

| Risk                                                                                   | Likelihood | Mitigation                                                                                                                                                       |
| -------------------------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Rust toolchain version drift between `ose-app-be` and `organiclever-be`                | Low        | Copy `rust-toolchain.toml` verbatim from `organiclever-be`; both pin channel 1.95.0                                                                              |
| Integration test Docker image fails to build                                           | Low        | Mirror `Dockerfile.integration` from `organiclever-be`; adjust binary name and port only                                                                         |
| `deny.toml` license check fails on a transitive dependency                             | Low        | Copy `deny.toml` verbatim; same dependency graph as `organiclever-be`                                                                                            |
| `bounded-contexts.yaml` `code` path mismatch triggers rhino-cli DDD validation failure | Medium     | Delivery checklist includes explicit `rhino-cli ddd bc ose-app` and `ddd ul ose-app` steps before push                                                           |
| F# generated-contracts directory left behind causes Nx cache invalidation noise        | Low        | Delivery checklist explicitly deletes `generated-contracts/OpenAPI/` and the Rust `codegen` target uses `git diff --exit-code` to catch accidental re-generation |
