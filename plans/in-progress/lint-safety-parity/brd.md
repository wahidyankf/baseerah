# Business Requirements — lint-safety-parity (ose-public)

## Business Goal

Bring `ose-public`'s linting strictness and unsafe-code posture into **equal** standing with its
two sibling repositories (`ose-primer`, `ose-infra`), closing the strictness drift that has
accumulated across the shared scaffolding layer. Concretely for `ose-public`: lift the F# stack
to the primer-grade strict standard, add the three cross-language lint gates (Dockerfile, shell,
GitHub Actions) that all three repos will share, and retire the dead Go lint config.

This is a **planning-only** deliverable: the goal of this plan is a validated, executable plan —
not the executed change. Execution is a downstream effort via plan-execution.

## Business Rationale (WHY)

The three siblings advertise a single, coherent "scaffolding quality bar" to downstream adopters
(`ose-primer` is a public template; `ose-public` is the upstream source of truth). When the lint
strictness drifts between them, the quality bar becomes a fiction: a contributor moving between
repos meets different gates, and the template ships a weaker standard than the upstream claims to
hold. Parity restores a single, honest, enforceable standard.

For `ose-public` specifically:

- The F# stack (`crane-be`, `crane-cli`, `fsharp-crane-core`) lints with `fantomas --check` +
  `dotnet fsharplint` today `[Repo-grounded]`, but does **not** treat warnings as errors and does
  **not** run the G-Research analyzers — so latent F# warnings accumulate silently. The primer F#
  stack already holds the stricter bar; `ose-public` is the laggard here.
- There is **no** Dockerfile, shell, or GitHub Actions linting at all today (no `.hadolint.yaml`,
  `.shellcheckrc`, or actionlint gate exist) `[Repo-grounded]` — a whole class of
  config/script/CI defects is currently uncaught.
- A `.golangci.yml` sits at the repo root but `ose-public` has **no active Go** (Go lives only in
  `archived/`) `[Repo-grounded]` — dead config that misleads readers into thinking Go is linted.

## Business Impact

### Pain points addressed

- **Silent F# warning drift**: warnings compile clean today, so quality erodes invisibly.
- **Uncaught Docker/shell/CI defects**: no gate catches a broken `Dockerfile`, an unquoted shell
  variable, or an invalid workflow expression before it lands on `main`.
- **Misleading dead config**: the root `.golangci.yml` implies Go coverage that does not exist.
- **Cross-repo inconsistency**: a contributor's mental model of "the gate" differs per repo.

### Expected benefits

- A single shared strictness standard across all three siblings (honest quality bar).
- Earlier defect capture for Docker, shell, and CI YAML (shift-left to pre-commit + CI).
- F# warnings become build-breaking, halting silent quality erosion.
- A clean repo with no dead lint config.

## Affected Roles

This is a solo-maintainer repository — the maintainer wears multiple hats; no sign-off ceremony
applies. Affected hats and consumers:

- **Maintainer-as-platform-owner**: owns the cross-repo parity standard and the governance docs.
- **Maintainer-as-F#-developer**: must clean latent F# warnings before the TWAE gate flips on.
- **Maintainer-as-CI-engineer**: wires the new gates into `pr-quality-gate.yml` and the husky hooks.
- **Consuming agents**: `plan-checker` / `plan-execution-checker` (validate this plan),
  `swe-fsharp-dev` (executes F# cleanup), `repo-setup-manager` (Phase 0 baseline), `ci-checker`
  (CI gate wiring), `repo-rules-maker` (governance/convention doc updates).

## Business-Level Success Metrics

- **Parity achieved** (observable on execution): all three sibling repos enforce the same
  cross-language strict standard (D2/D6/D7/D8 wired identically; D10 dead config removed where Go
  is absent). _For this planning plan, success = a validated five-document plan that, when
  executed, produces that state._ `[Judgment call]`
- **Zero latent F# warnings remain** when the TWAE gate flips on (clean-then-gate guarantees the
  first gated build is green). `[Repo-grounded]` (clean-then-gate is the locked rollout policy).
- **No dead lint config** remains in `ose-public` after D10 (root `.golangci.yml` removed).
  `[Repo-grounded]`
- **Plan passes plan-quality-gate** in strict mode (double-zero: zero CRITICAL, zero HIGH
  findings). `[Judgment call]` (gate defined in the resolved-decisions brief).

## Business-Scope Non-Goals

- **Not executing** the lint changes — this plan is the deliverable; execution is downstream.
- **Not adding** C#, Python, or IaC linting to `ose-public` (those languages are absent here).
- **Not changing** the existing Rust lint posture — `ose-public` Rust is already the reference
  standard.
- **Not implementing** the dropped D5 (TS DDD import-boundaries) — deferred to a future plan; only
  the deferral rationale is documented.

## Business Risks and Mitigations

| Risk                                                                    | Likelihood | Mitigation                                                                                      |
| ----------------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------------- |
| F# latent-warning backlog larger than estimated, stalling D2            | Medium     | Clean-then-gate sequences cleanup BEFORE the gate flip; each `.fsproj` is an independent cycle  |
| New gates produce noisy false positives on existing Dockerfiles/scripts | Medium     | Justified per-rule `ignore` lists in `.hadolint.yaml` / `.shellcheckrc`, documented inline      |
| Parity drifts again after execution                                     | Low        | Shared cross-language strictness convention doc + identical gate wiring across siblings         |
| Removing `.golangci.yml` breaks a workflow that referenced it           | Low        | Phase 0 greps for references; D10 removal step verifies no workflow/script references the file  |
| main-to-main push lands a half-applied gate on `main`                   | Low        | Every phase is a natural pause with a green gate; clean-then-gate means no gate flips while red |
