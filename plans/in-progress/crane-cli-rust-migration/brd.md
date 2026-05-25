# Business Requirements Document — crane-cli Rust Migration

## Business Goal

Eliminate the F# / .NET 10 toolchain from the repository. `apps/crane-cli/` is the sole consumer
of .NET in the monorepo. After this migration the CI and developer environment need only a single
compiled-language toolchain (Rust) for all non-JavaScript/TypeScript apps.

## Business Impact

### Pain Points

- `apps/crane-cli/` is the sole consumer of .NET in the entire monorepo; it forces every
  contributor and every CI run to install a full .NET 10 / F# toolchain for a single app.
- The CI pipeline carries `dotnet`, `fantomas`, and `altcover` steps exclusively for crane-cli,
  adding one major compiled-language toolchain to an otherwise Rust-and-Go-and-JS setup.
- Developer environment setup requires documenting and verifying a second compiled-language
  toolchain (dotnet) beyond Rust, which already covers rhino-cli, organiclever-be, ose-cli,
  and rust-commons.

### Expected Benefits

- After migration the CI and developer environment require only Rust (a toolchain already
  present for rhino-cli, organiclever-be, and rust-commons) — zero additional installs for
  any contributor who can already build those apps (excluding system OCR libs, which are
  documented separately).
- Contributor onboarding simplifies: one compiled-language toolchain instead of two.
- CI dependency surface shrinks: `dotnet`, `fantomas`, and `altcover` steps are eliminated
  from crane-cli targets.

## Affected Roles

- **Platform maintainer** — removes one major toolchain from the dev environment setup checklist.
- **pdf-to-md-maker / pdf-to-md-checker / pdf-to-md-fixer agents** — these agents invoke `crane`
  as a subprocess; binary interface is unchanged.
- **CI pipeline** — drops all `dotnet`, `fantomas`, and `altcover` steps from crane-cli targets.

## Business-Level Success Metrics

- `_Judgment call:_` Any contributor who can already build rhino-cli can build crane-cli with
  zero additional toolchain installs (excluding system OCR libs which are documented).
- All agents that currently call `crane` continue to work without any change to their invocation
  commands or expected JSON output.

## Business-Scope Non-Goals

- No new crane features are added. This is a pure port.
- The PDF-to-Markdown workflow itself (pdf-to-md-maker agent, pdf fixtures, specs) is not changed.
- crane-cli is not published to crates.io.

## Business Risks

| Risk                                                          | Mitigation                                                                                           |
| ------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| PDF text extraction fidelity differs between PdfPig and lopdf | Integration tests with the existing real PDF fixtures in `tests/integration/fixtures/` gate the port |
| tesseract system lib unavailable in CI                        | Document the brew/apt install; CI doctor step verifies                                               |
| F# source archived before all callers migrated                | Callers invoke `crane` binary, not F# source — no breakage                                           |
