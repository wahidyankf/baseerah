# Business Requirements Document — organiclever-be Rust Migration

## Business Goal

Replace the Java/Spring Boot 4 JVM runtime in `apps/organiclever-be/` with a Rust/Axum binary,
aligning the OrganicLever backend with the repository's Rust-first platform direction and
eliminating the JVM operational overhead from the organiclever service stack.

## Business Rationale

### Pain Points

The current Java/Spring Boot 4 backend introduces three operational friction points for a
solo-maintainer repo:

1. **JVM startup and memory overhead**: Spring Boot 4 applications start in 2-5 seconds and
   idle at ~200–400 MB heap. A Rust binary starts in under 50 ms and idles at 5–20 MB RSS.
   _Judgment call: magnitudes from publicly available JVM-vs-Rust benchmarks; exact numbers for
   this workload were not measured._

2. **Polyglot toolchain breadth**: The repo already maintains Go, F#, TypeScript, and Rust
   toolchains. Eliminating the Java/Maven toolchain shrinks developer environment setup and CI
   build-matrix complexity by one runtime. _Observable fact: `apps/organiclever-be/pom.xml`
   exists; removing it removes the Maven dependency._

3. **Platform-direction divergence**: `apps/rhino-cli/` completed its Go→Rust port in May 2026.
   Two parallel CLI migrations (`ose-cli-rust-migration`, `ayokoding-cli-rust-migration`) are in
   progress. Keeping `organiclever-be` on Java creates an island of JVM maintenance in an
   otherwise Rust/Go/TypeScript/F# ecosystem.

### Expected Benefits

- Rust binary replaces JVM: startup latency reduced from seconds to milliseconds
  _[Judgment call: consistent with published Rust/Axum vs Spring Boot comparisons]_
- JVM removed from local dev and CI toolchain
- `organiclever-be` linting and test patterns unify with `rhino-cli` (shared `deny.toml`,
  clippy pedantic, `cargo llvm-cov`)
- Developer onboarding for this service uses the same muscle memory as rhino-cli

## Affected Roles

The maintainer (Wahidyan Kresna Fridayoka) wears two hats relevant to this migration:

- **Backend developer**: maintains `organiclever-be` API code; gains Rust idiom consistency
- **DevOps / platform operator**: manages CI workflows, Docker images, and Nx workspace;
  gains by removing the JVM from the build matrix

Consuming agents: `swe-rust-dev` (implementation), `swe-e2e-dev` (integration tests),
`plan-checker` (plan validation), `plan-execution-checker` (post-execution validation).

## Business-Level Success Metrics

1. **Java/Maven removed**: `apps/organiclever-be/pom.xml`, `checkstyle.xml`, `pmd-ruleset.xml`,
   and `src/main/java/` are absent from `git ls-files`. _Observable fact — verifiable via
   `git ls-files apps/organiclever-be/`._

2. **Rust binary starts and responds**: `cargo run --manifest-path apps/organiclever-be/Cargo.toml`
   starts a server on port 8202; `curl -s http://localhost:8202/api/v1/health` returns HTTP 200
   with body `{"status":"ok"}`. _Observable fact — verifiable via curl._

3. **Test coverage ≥ 90%**: `cargo llvm-cov --fail-under-lines 90` passes without manual
   override. _Observable fact — exits 0._

4. **CI green**: All GitHub Actions workflows triggered by the migration commit pass.
   _Observable fact — verifiable via `gh run view`._

## Business-Scope Non-Goals

- **No new API endpoints**: this migration is scope-locked to porting what exists.
- **No new database entities**: no CRUD tables are introduced; SQLx dependency is included for
  future-readiness (the OpenAPI contract references DB-backed resources) but no migrations run.
- **No deployment pipeline changes**: Kubernetes manifests and Vercel-adjacent infrastructure are
  out of scope.
- **No organiclever-web or e2e test suite changes**: `organiclever-web-e2e` and
  `organiclever-be-e2e` are not modified by this plan.

## Business Risks and Mitigations

| Risk                                                                                                   | Likelihood | Impact | Mitigation                                                                                                                                                                                               |
| ------------------------------------------------------------------------------------------------------ | ---------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Rust codegen for OpenAPI spec is immature or unavailable as a drop-in for the Java generator           | Medium     | Medium | Plan notes this as a research item; codegen target update is scoped as a best-effort step with a fallback of marking the target as `[Unverified — requires research]` and filing it under Open Questions |
| cucumber-rs 0.23.0 BDD harness syntax differs enough from cucumber-jvm to require rewriting step files | Low        | Low    | Scope of existing Gherkin features is tiny (one feature, two scenarios); rewrite cost is minimal                                                                                                         |
| Docker integration test setup requires non-trivial Rust multi-stage build                              | Low        | Medium | Dockerfile.integration is adapted (not created from scratch); the ose-primer pattern provides a working model                                                                                            |
| SQLx 0.8 `migrate` feature requires PostgreSQL connection at compile time (`sqlx::query!` macros)      | Low        | Medium | Use `sqlx::query()` (runtime-checked) rather than `sqlx::query!()` (compile-time) until a migration file set exists; avoids DATABASE_URL requirement at build time                                       |
