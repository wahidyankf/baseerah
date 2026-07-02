# Tech Docs — Unify rhino-cli, SDLC & Repo Structure (Second Pass)

## 1. Relationship to the First Plan

This plan **inherits the entire target standard** of the first plan
([tech-docs](../../done/2026-07-01__standardize-rhino-cli-sdlc-parity/tech-docs.md)) — the SDLC gate
mechanics (§1 lifecycle), the Nx target-name standard (§5), the testing-architecture standard (§4),
the harness coverage standard (§3.2), the target-standard synthesis (§7), and the divergence policy
(§7.1). Those are **not re-derived here**; they remain authoritative.

What this plan adds is a **second, stricter target** the first plan did not achieve: the standardized
layer must be **byte-identical**, including `apps/rhino-cli`'s own source, and every `⚠️`
"functionally-equivalent mechanism divergence" must converge. This document records the **verified
current state** (§2), the delta to close (§3), the **rhino-cli source-identity standard** (§4), the
canonical decisions (§5), the phase design (§6), and the divergence policy (§7).

## 2. Current State (Verified 2026-07-02)

A fresh three-repo read-only sweep (superseding all stale delivery.md "done" notes). ✅ = at target,
⚠️ = functionally-equivalent but mechanism-divergent, ❌ = divergent/incomplete.

### 2.1 rhino-cli source

| Aspect                     | public                      | primer                            | infra                                               | Verdict |
| -------------------------- | --------------------------- | --------------------------------- | --------------------------------------------------- | :-----: |
| `src/*.rs` file count      | 155                         | 231                               | 235                                                 |   ❌    |
| `src` diff vs public       | —                           | 5 differ, 15 only-in-one          | **100 differ, 51 only-in-one** (diff module naming) |   ❌    |
| `cli.rs`                   | canonical (1193 ln)         | **byte-identical to public**      | differs (1325 ln, +132)                             |   ❌    |
| `Cargo.toml` cucumber      | `0.23.0` (dep, **unwired**) | `0.22.1` (**fully wired**)        | `0.23.0` (dep, **unwired**)                         |   ❌    |
| `Cargo.lock`               | distinct sha                | distinct sha                      | distinct sha                                        |   ❌    |
| `license` field            | MIT                         | MIT                               | **LicenseRef-Proprietary** → relicense to MIT (D3)  |   ❌    |
| lint policy                | strict (`deny` docs)        | deferred `allow`                  | deferred `allow`                                    |   ❌    |
| `project.json` target KEYS | 21                          | 21                                | 21                                                  |   ✅    |
| `project.json` COMMANDS    | canonical                   | minor differ                      | differ (deps:audit tool, env globs, coverage regex) |   ❌    |
| cucumber BDD harness       | **unwired**                 | **wired** (11 `[[test]]` + feats) | **unwired**                                         |   ❌    |

**Interpretation**: public↔primer `cli.rs` is already byte-identical and the src delta is small (5
files + primer's extra testcoverage/cucumber). infra is a different refactor generation. primer is
_ahead_ on cucumber + testcoverage. Canonical synthesis therefore pulls primer's advances into public,
then propagates public→primer(trim to canonical)→infra(regenerate).

### 2.2 SDLC wiring

| Surface                        | public                       | primer                       | infra                                                       | Verdict |
| ------------------------------ | ---------------------------- | ---------------------------- | ----------------------------------------------------------- | :-----: |
| `.husky/commit-msg`            | canonical                    | identical                    | identical                                                   |   ✅    |
| `.husky/pre-commit`            | canonical                    | **byte-identical to public** | no shebang/`set -e`/Step comments; **inline tool-lint**     |   ❌    |
| `.husky/pre-push`              | canonical                    | identical (modulo excludes)  | **`npx nx`/`npm run` wrappers** replace every `cargo run`   |   ❌    |
| lint-staged `*.cs/.clj/.dart`  | native tools                 | `scripts/format-*.sh`        | `scripts/format-*.sh`                                       |   ⚠️    |
| lint-staged sh/Docker/actions  | present                      | present                      | **absent** (handled inline in pre-commit)                   |   ❌    |
| canonical workflow filenames   | present                      | present                      | present                                                     |   ✅    |
| `validate-markdown.yml` absent | ✅                           | ✅                           | ✅                                                          |   ✅    |
| `pr-quality-gate.yml` jobs     | **missing gherkin-card**     | canonical                    | Title-Case `name:`, no `env:` NX_BASE/HEAD, extra md job    |   ❌    |
| `main-ci.yml` jobs             | canonical                    | canonical                    | no standalone `compat-min-version`/`env-validate`; extra md |   ❌    |
| Codecov removed                | ✅                           | ✅                           | ✅                                                          |   ✅    |
| naming trigger path            | **`.opencode/agent/` (bug)** | **`.opencode/agent/` (bug)** | `.opencode/agents/`                                         |   ❌    |

### 2.3 config / targets / specs

| Surface                       | public                            | primer                     | infra                                                | Verdict |
| ----------------------------- | --------------------------------- | -------------------------- | ---------------------------------------------------- | :-----: |
| `repo-config.yml` body/schema | canonical                         | identical                  | identical                                            |   ✅    |
| `repo-config.yml` header cmt  | canonical                         | drops `env-injection` line | reworded coverage/specs/size cmts                    |   ❌    |
| mandatory-six + extras        | 27/27 clean                       | 25/25 clean                | **5 projects missing** deps:audit/compat:min-version |   ❌    |
| `namedInputs.specs` rollout   | **16/27**                         | **20/25**                  | **6/7**                                              |   ❌    |
| specs C4 structure            | 1 stale orphan (`golang-commons`) | complete                   | complete                                             |   ❌    |
| `coverage.projects` registry  | **omits 4 real projects**         | complete                   | complete                                             |   ❌    |
| old 3 config files absent     | ✅                                | ✅                         | ✅                                                   |   ✅    |

Infra's 5 gap projects: `coralpolyp-be-e2e`, `coralpolyp-fe-e2e` (both miss `deps:audit` +
`compat:min-version`), `coralpolyp-fe` (miss `compat:min-version`), `libs/ts-ui`, `libs/ts-ui-tokens`
(both miss `deps:audit` + `compat:min-version`).

public's `namedInputs.specs` gaps: 11 projects (most `*-cli` except crane/rhino, most e2e — yet
`organiclever-be-e2e`/`ose-be-e2e` DO have it → internally inconsistent). primer gaps: 5
(`clojure-openapi-codegen`, `elixir-cabbage`, `elixir-gherkin`, `elixir-openapi-codegen`,
`ts-ui-tokens`). infra gap: 1 (`ts-ui-tokens`).

public `coverage.projects` omits: `fsharp-crane-core`, `web-ui-token`, `organiclever-contracts`,
`ose-contracts` (registry lists 26; `nx show projects` = 29).

## 3. The Delta to Close

Everything with an ❌ or ⚠️ above. Grouped by owning phase:

- **Canonical synthesis (Phase 1, public)**: pull primer's cucumber harness + testcoverage into
  public; unify lint policy; drive repo-specific behaviour from `repo-config.yml`; fix the
  `.opencode/agent/` bug; canonicalize `repo-config.yml` header comment.
- **public closeout (Phase 2)**: full `namedInputs.specs`; complete `coverage.projects`; delete the
  `golang-commons` orphan; add `gherkin-cardinality` to the PR gate.
- **primer propagation (Phase 3)**: align rhino-cli 5-file delta + cucumber `0.22.1`→canonical;
  full `namedInputs.specs`; fix `.opencode/agent/` bug; agree `*.cs/.clj/.dart` mechanism.
- **infra propagation (Phase 4)**: regenerate rhino-cli to canonical; `npx nx`/`npm run` → direct
  `cargo run`; inline tool-lint → lint-staged; pre-commit shebang/Step comments; add
  `compat-min-version`/`env-validate` jobs; add `gherkin-cardinality`; lower-kebab workflow `name:`;
  add missing targets to 5 projects; full `namedInputs.specs`; wire cucumber.

## 4. rhino-cli Source-Identity Standard

The end-state: `apps/rhino-cli` is **100% byte-identical** across all three repos — **zero
carve-outs** (per Decisions 3 + 5). Achieved by:

1. **One canonical generation.** Synthesize in ose-public the best-of-three: public's strict lint
   policy + primer's cucumber harness + primer's testcoverage module + the richest internal module
   tree. This becomes the canonical `src/`, `Cargo.toml`, `Cargo.lock`, `project.json`.
2. **Data-drive ALL repo-specific behaviour.** Everything that legitimately differs per repo —
   env-validation scan paths, domain-areas, ddd-areas — moves into `repo-config.yml` (the per-repo
   data file), so the Rust source **and** every `project.json` command string are identical.
   `application/repo_config/mod.rs` must read these rather than hard-code them; the `env:validation`
   target reads its scan paths from `repo-config.yml` (Decision 5) so infra's IaC globs are data,
   not a divergent command.
3. **No carve-outs.** infra's rhino-cli is relicensed to MIT (Decision 3), so the `Cargo.toml`
   `license` field matches too. The self-hosted runner label lives in CI-workflow YAML, **not** in
   `apps/rhino-cli`, so it does not affect CLI byte-identity.
4. **cucumber harness is canonical.** primer's `tests/*.rs` (11 `[[test]]` harness=false suites),
   `tests/fixtures`, `tests/golden-master`, and `specs/apps/rhino/behavior/rhino-cli/gherkin/**` are
   the canonical BDD surface, copied identically into public and infra. Canonical cucumber version =
   `0.23.0` (public/infra's current pin; primer's harness code is adapted from `0.22.1` if needed).

**Byte-identity acceptance** (Phase 5): `diff -rq apps/rhino-cli/src` empty pairwise; `diff` of
`Cargo.toml`/`Cargo.lock`/`project.json` **shows no differences** (zero carve-out lines); `cargo test`
cucumber suites pass in all three.

## 5. Canonical Decisions (user-ratified 2026-07-02)

| #   | Decision                | Ratified choice                                                                                                                             |
| --- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | Canonical rhino-cli     | **synthesize in ose-public**, then propagate public→primer→infra                                                                            |
| 2   | Infra rhino-cli scope   | **full port**, isolated as gated Phase 4 (descopable without unwinding Phases 1–3)                                                          |
| 3   | Infra rhino-cli license | **relicense to MIT** — no license carve-out                                                                                                 |
| 4   | `*.cs/.clj/.dart` fmt   | **native tools inline** (`dotnet csharpier format`/`cljfmt fix`/`dart format`); primer+infra converge to public, drop `scripts/format-*.sh` |
| 5   | env-validation paths    | **data-driven from `repo-config.yml`** → `project.json` byte-identical → **zero rhino-cli carve-outs**                                      |
| —   | cucumber direction      | **level up** — adopt primer's wired harness everywhere (not strip); canonical version `0.23.0`                                              |

Net effect of 3 + 5: `apps/rhino-cli` is 100% byte-identical across all three repos, no exceptions.

## 6. Phase Design

- **Phase 0 — Baseline & re-audit (public).** Install/doctor; run each repo's affected pre-push on a
  no-op to confirm a green starting point; commit the §2 matrices as evidence; resolve any preexisting
  failure before work begins.
- **Phase 1 — Canonical synthesis (public).** Build the canonical rhino-cli (cucumber + testcoverage +
  strict lints + data-driven repo-config), fix latent bugs, finalize canonical docs. RED/GREEN/REFACTOR
  for every rhino-cli source change with companion `.feature` specs.
- **Phase 2 — public closeout.** `namedInputs.specs` on all 29 projects; complete `coverage.projects`;
  delete orphan spec; add PR-gate `gherkin-cardinality`; canonicalize `repo-config.yml` header.
- **Phase 3 — primer propagation.** Copy canonical rhino-cli into primer (align its 5-file delta +
  bump cucumber `0.22.1`→`0.23.0`); full `namedInputs.specs`; fix `.opencode/agent/` bug; converge
  `*.cs/.clj/.dart` to native-tool formatters (drop `scripts/format-*.sh`).
- **Phase 4 — infra propagation (largest; gated).** Regenerate rhino-cli to canonical; **relicense to
  MIT**; **data-drive env-validation paths via `repo-config.yml`** (no project.json carve-out);
  converge `*.cs/.clj/.dart` to native-tool formatters; convert hooks to direct `cargo run`; move
  tool-lint to lint-staged; add pre-commit shebang/Step comments; add missing CI jobs +
  `gherkin-cardinality`; lower-kebab workflow `name:`; add missing targets to 5 projects; full
  `namedInputs.specs`; wire cucumber. Result: `apps/rhino-cli` byte-identical to public, zero carve-outs.
- **Phase 5 — cross-repo byte-identity verification + archival.** The `diff -rq` matrix, `jq` key +
  command comparison, hook diffs, cucumber pass in all three, parity table with zero `⚠️`.

Phases 3 and 4 copy this plan folder into the sibling repo at the start of the phase (per the
[multi-repo parity workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)),
so the same checklist drives execution there.

## 7. Divergence Policy (Allowed vs. Drift)

**Allowed divergence** (recorded, not flagged):

- App set & per-app deploy CRONs; language gate jobs; infra-only IaC gates
  (terraform/ansible/yamllint); self-hosted runner label; lint-staged formatter entries for languages
  present only in that repo. (All carried forward from the first plan.)
- **`apps/rhino-cli` has NO carve-outs** (Decisions 3 + 5) — `src/`, `Cargo.toml`, `Cargo.lock`, and
  `project.json` are 100% byte-identical across all three repos. The self-hosted runner label is a
  CI-workflow-YAML concern, not part of `apps/rhino-cli`.
- `repo-config.yml` per-repo **data values** (domain-areas, ddd-areas, env-validation scan paths)
  differ; the **schema, header comment, and harness list** are identical.

**Drift** (MUST converge — the work): everything with ❌/⚠️ in §2 — rhino-cli source/Cargo/commands,
cucumber wiring, hook/CI mechanism, workflow `name:` casing + jobs, `namedInputs.specs`, missing
targets, `coverage.projects`, orphan spec, header comment, the two latent bugs.

## 8. Evidence Sources

All §2 cells are Repo-grounded from the 2026-07-02 read-only sweep (three parallel per-surface
audits: rhino-cli identity, SDLC wiring, config/targets/specs). Phase 0 re-runs the sweep and commits
its output so the plan record is reproducible rather than resting on this document alone.
