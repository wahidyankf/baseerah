# Business Requirements Document

## Business Goal

Rewrite `apps/crane-cli/` from Rust to F# with strict hexagonal (ports-and-adapters) architecture,
restoring the original implementation language while elevating the structural design to a clean
ports-and-adapters model. The Rust implementation (plan `2026-05-26__crane-cli-rust-migration`) was
a fidelity port; this F# rewrite is a deliberate architectural improvement that positions
crane-cli for long-term maintainability alongside the F# ecosystem.

## Business Impact

**Pain points addressed:**

- The Rust implementation of crane-cli is the only Rust CLI without a natural downstream consumer
  in the F# ecosystem. The pdf-to-md pipeline agents that call crane-cli are authored in the same
  F#-friendly context where shared tooling (Fantomas formatter, fsharplint, TickSpec BDD) already
  exists from the original crane-cli delivery.
- The archived F# source (`archived/crane-cli/` [Repo-grounded]) represents approximately 124
  unit tests and a BDD suite (10 features, 34 scenarios, 131 steps) [Judgment call —
  approximate counts from the original delivery; the shared BDD spec has grown during the Rust
  port period] from the original `2026-05-15__crane-cli` plan.
  Maintaining Rust alongside that reference creates two diverging implementations of the same
  domain logic.
- The `remove-inactive-tech-stack-remnants` plan (currently in-progress, not yet executed
  [Repo-grounded]) plans to delete dotnet/F# toolchain artifacts. This rewrite makes F# active
  again in `ose-public`, invalidating Phase 1 of that plan — the scope conflict must be resolved
  as a prerequisite.

**Expected benefits:**

- Single language for the crane-cli domain and its test suite (F# + TickSpec throughout)
- Hexagonal architecture enforced at project structure level: `Core/Ports.fs` makes all I/O
  boundaries explicit as function type aliases
- Impureim Sandwich pattern: pure domain logic never touches I/O; composition root in `Program.fs`
  wires adapters to ports at the boundary
- `archived/crane-cli/` becomes a directly comparable reference for every module, reducing
  implementation risk
- `.github/actions/setup-dotnet/` [Repo-grounded] (which already exists) is repurposed rather
  than deleted

## Affected Roles

- **Maintainer (developer hat)** — primary consumer of the rewritten CLI; runs `crane check-all`
  in pdf-to-md pipelines
- **pdf-to-md-maker / pdf-to-md-checker / pdf-to-md-fixer agents** [Repo-grounded] — call the
  `crane` binary indirectly through the pdf pipeline; no agent code changes required, only
  the binary must remain ABI-compatible (same subcommands, same flags, same exit codes)
- **swe-fsharp-dev agent** [Repo-grounded] — primary suggested executor for implementation phases
- **CI system** — `.github/workflows/crane-cli-integration.yml` [Repo-grounded] must be updated
  to use `dotnet test` instead of `cargo test`

## Business Success Metrics

[Judgment call] After plan completion:

- `npx nx run crane-cli:build` exits 0 and produces a `crane` binary under
  `apps/crane-cli/dist/`
- `npx nx run crane-cli:test:quick` exits 0 with ≥95% line coverage [Judgment call — matches
  the target set in the original F# crane-cli plan and maintained by the Rust port]
- `npx nx run crane-cli:test:integration` exits 0 with all Gherkin integration scenarios passing
- `npx nx run crane-cli:spec-coverage` exits 0 — all feature scenarios are covered by step
  definitions
- `npx nx run crane-cli:lint` exits 0 (Fantomas format check + fsharplint)
- The `plans/in-progress/remove-inactive-tech-stack-remnants/` Phase 1 checklist is amended to
  exclude dotnet/F#/C# artifacts before any execution begins

## Business Non-Goals

- No change to crane-cli's user-facing CLI interface (subcommands, flags, output format, exit
  codes remain identical)
- No changes to `ose-app-be`, `organiclever-be`, or any other app
- No deletion of `archived/crane-cli/` (F# original) or `archived/crane-cli-rust/` (post-archive)
- No addition of new CLI features beyond those already implemented in Rust
- No changes to the pdf-to-md agent logic — only the binary it calls changes language

## Business Risks and Mitigations

| Risk                                                                                                | Mitigation                                                                                                                                                                                                  |
| --------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `remove-inactive-tech-stack-remnants` Phase 1 executes before this plan and deletes F# toolchain    | **Prerequisite step**: amend that plan's delivery.md to exclude dotnet/F# in Phase 1 before any other work begins                                                                                           |
| PdfPig 0.1.14 API differences from lopdf (Rust) create divergence in PDF extraction behavior        | Gherkin integration scenarios in `specs/apps/crane/` [Repo-grounded] act as contract tests; any behavior difference surfaces as a test failure                                                              |
| TesseractOCR 5.5.2 NuGet package native lib discovery differs by platform (Linux vs macOS)          | The archived crane-cli already includes `tessdata/eng.traineddata` [Repo-grounded] bundled with the project; CI uses Ubuntu and the `setup-dotnet` action [Repo-grounded] already handles tool installation |
| F# compile order sensitivity (all files must be listed explicitly in `.fsproj`) causes build errors | Delivery checklist orders files precisely; `dotnet build` failures surface immediately                                                                                                                      |
| TickSpec step binding at runtime (reflection-based) may fail silently for unmatched steps           | xunit.runner.json sets `maxParallelThreads: 1`; TickSpec throws on unbound steps; integration test phase verifies all scenarios bind correctly                                                              |
