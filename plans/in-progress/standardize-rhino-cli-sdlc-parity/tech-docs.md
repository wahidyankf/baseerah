# Tech Docs — Standardize rhino-cli Checks & SDLC Commands

All facts below are grounded in the current commit of each repo (`apps/rhino-cli/src/cli.rs`,
`apps/rhino-cli/project.json`, `.husky/*`, `.github/workflows/*`) unless labelled otherwise.
The gate-check **standard** (§1) comes first; the rhino-cli **command triage** (§2) follows; per-repo
deltas are called out in [§4 Drift Catalog](#4-drift-catalog-per-surface).

## 1. Target Standard (Best-of-Three Synthesis)

> **Identical-result invariant (north star).** The end-state of this plan is **identical across all
> three repos for the entire standardization layer** — the rhino-cli command set + verb-last naming,
> the `:`-separated Nx target conventions, `repo-config.yml`'s section schema, the hook/gate
> mechanics + step order, the lint-staged formatter map, and the canonical GitHub CI workflow names
> (`pr-quality-gate.yml`, `validate-markdown.yml`, `validate-env.yml`, `main-ci.yml`). Working across
> `ose-public`, `ose-primer`, and `ose-infra` must feel **identical, logical, and intuitive**: the
> same command does the same thing, the same target name resolves the same way, the same file holds
> the same kind of config. The **only** legitimate divergence is the **project/app set itself** (and
> therefore the per-app deploy/CRON workflows + language-specific gate jobs) — see
> [§3 Divergence Policy](#3-divergence-policy-allowed-vs-drift). Everything else is byte-identical
> where the files are not data-bearing, and structurally identical where they are (e.g. `repo-config.yml`
> lists each repo's own surfaces under the same schema).

The gate-check standard is synthesized by picking the strongest wiring per surface, even where that
means changing `ose-public`. The named winner per surface:

| Surface                                | Standard (winner)                                                                                                                       | Rationale                                                                                                                                                                                |
| -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **commit-msg**                         | `npx --no -- commitlint --edit "$1"` + `@commitlint/config-conventional`                                                                | already identical in all three — lock it                                                                                                                                                 |
| **Lint gates as Nx targets**           | `rhino-cli:shell:check`, `rhino-cli:dockerfiles:check`, `rhino-cli:actions:check` (all 3 repos)                                         | wrapping shellcheck/hadolint/actionlint as cacheable Nx targets beats inline shell; `{tool}:check` matches existing `msrv:check`/`deny:check` verb (primer renames its `:lint` variants) |
| **PR quality-gate filename**           | `pr-quality-gate.yml`                                                                                                                   | 2-of-3 already use it; "pr" is clearer than "commons" for the gate's role                                                                                                                |
| **Markdown workflow filename**         | `validate-markdown.yml`                                                                                                                 | 2-of-3 already use it; verb-first matches `validate-env.yml`                                                                                                                             |
| **Env workflow filename**              | `validate-env.yml` (standalone)                                                                                                         | infra style; verb-first parity with `validate-markdown.yml`; primer must extract its folded-in env job into a standalone file                                                            |
| **Markdown validator set**             | mermaid + links + heading-hierarchy + **gherkin-cardinality** (4)                                                                       | primer/infra superset; public must add gherkin-cardinality                                                                                                                               |
| **specs-gate validator set (PR gate)** | adoption + tree + counts + links + behavior:coverage (+ domain:coverage on `*-be`) + gherkin-cardinality (full)                         | public's fuller set wins; primer must promote its deferred structural set                                                                                                                |
| **pre-push scoped validator set**      | union incl. `governance:vendor-audit-validation`                                                                                        | public/infra include it; primer must add it                                                                                                                                              |
| **pre-commit step order**              | identity → no-env → (sh/docker/actions check, tool-gated) → `git pre-commit` → `nx affected test:quick`                                 | already near-identical; lock the order                                                                                                                                                   |
| **pre-push step order**                | `nx affected -t test:quick` (= typecheck→lint→test:unit) → `lint:md` → `env:validation` → `specs:behavior:coverage` → scoped validators | **PR quality gate runs the identical set**; neither gate runs `test:integration`/`test:e2e` (CRON only) — see [§1.2](#12-testing-architecture--target-contents-standard)                 |
| **CRON pipeline shape**                | `*-test-local-deploy-{stag,prod}.yml` + paired `*-test-{stag}.yml` calling shared `_reusable-*` workflows                               | public's reusable-workflow factoring is cleanest; primer/infra keep their own app set but adopt the naming + reusable-call shape                                                         |

The standard is published as `docs/reference/sdlc-gate-standard.md` (new) in Phase 1, and the triage
as `docs/reference/rhino-cli-command-triage.md` (new).

### 1.1 Nx Target-Name Standard (Targets Invoked by Hooks/CI)

The canonical naming scheme already lives in
[repo-governance/development/infra/nx-targets.md](../../../repo-governance/development/infra/nx-targets.md)
and [nx-target-naming.md](../../../repo-governance/development/infra/nx-target-naming.md): a
**lifecycle scheme** (`build`, `typecheck`, `lint`, `test:unit`, `test:integration`, `test:e2e`,
`test:quick`, `format`, `format:check`, …) and the **`{domain}:{work}` scheme** for
governance/validation targets (`specs:tree-validation`, `links:validation`, `env:validation`, …).
[Repo-grounded]

Every Nx target a hook or CI workflow invokes MUST use a name from that scheme, identical across all
three repos. This plan extends the canonical scheme to close two gaps and converges the rhino-cli
target set. Decisions (locked):

| Decision                                    | Standard                                                                                                                                                                                                                                                                                                               | Action                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Formatting (file-type, not per-project)** | file-type-based **lint-staged** entries (glob → formatter: `prettier --write` for `*.{md,json,yml,yaml,css,scss,ts,tsx,js}`, `rustfmt` for `*.rs`, `fantomas` for `*.fs`) run in `.husky/pre-commit` — one identical glob→formatter map across all 3 repos; **no** per-project `format`/`format:check`/`fmt` Nx target | **remove** the per-project `format`/`format:check` Nx targets **and** the rhino-cli `fmt` target from all 3 repos; ensure the shared lint-staged config covers every shipped file type (the `*.rs`/`*.fs` entries replace the removed Rust/F# `fmt`/`format:check`); **drop** `format`/`format:check` from the canonical lifecycle list in `nx-targets.md` and document the lint-staged formatter map there instead                                                                                            |
| **Tool-lint wrappers**                      | `shell:check`, `dockerfiles:check`, `actions:check` as Nx targets                                                                                                                                                                                                                                                      | add to public + infra; **rename** primer's existing `shell:lint`/`dockerfiles:lint`/`actions:lint` → `:check`; add the trio to `nx-targets.md` `{domain}:{work}` list (domain = tool, work = `check`)                                                                                                                                                                                                                                                                                                          |
| **Binding-parity validation**               | `harness:bindings-validation` as an Nx target everywhere                                                                                                                                                                                                                                                               | primer already has it; public + infra invoke it via `npm run harness:bindings-validation` — add the Nx target so the name + mechanism match (canonical name already listed in `nx-targets.md`)                                                                                                                                                                                                                                                                                                                 |
| **Structural specs targets**                | `specs:adoption-validation`, `specs:counts-validation`, `specs:links-validation`, `specs:tree-validation`, `test:e2e` present on rhino-cli                                                                                                                                                                             | primer is **missing** all five as standalone targets — add them so the target set matches public/infra                                                                                                                                                                                                                                                                                                                                                                                                         |
| **Coverage enforcement**                    | each project enforces ≥ 90% line coverage at `test:unit` via its **native** test runner (`vitest --coverage` thresholds, `cargo llvm-cov`/`tarpaulin`, `dotnet test` coverage gate) — **no** central rhino-cli coverage parser                                                                                         | **remove** the rhino-cli `test-coverage validate` command **and** the `test-coverage` Nx target from all 3 repos; wire each project's native coverage threshold into a dedicated `test:coverage` target (≥ 90% line); add a `test:coverage` column (90% target) to the §1.3 matrices                                                                                                                                                                                                                           |
| **External coverage service**               | **no** third-party coverage service (Codecov) in any repo — coverage is a **local, native** gate at `test:unit`, never uploaded                                                                                                                                                                                        | **remove all Codecov residue from all 3 repos**: delete `ose-infra/codecov.yml`; scrub the stale `codecov-upload.yml` CRON + `Codecov`/`Codecov-algorithm` references from ose-infra governance docs (`three-level-testing-standard.md`, `ci-conventions.md`, `nx-targets.md`, `apps/rhino-cli/README.md`). public + primer already cleaned (only `ExcludeFromCodeCoverage` attrs remain — not Codecov). Acceptance: `grep -ri codecov` in each repo returns **only** `ExcludeFromCodeCoverage` attribute hits |

After convergence, `jq -r '.targets | keys[]' apps/rhino-cli/project.json` MUST return the **same
sorted key set** in all three repos. [Repo-grounded — current diff in §4.1]

### 1.1a Unified Repo Configuration (`repo-config.yml`)

Today each repo carries several separate root-level config files consumed by rhino-cli — `instruction-size-budget.yaml`, `env-contract.yaml`, `env-injection.yaml` (all `[Repo-grounded]` at repo root). This plan **merges them into a single `repo-config.yml`** at the repo root, with one top-level namespaced section per former file:

Each former file becomes a top-level **section key** (resolving the `surfaces:` root-key collision
between `instruction-size-budget.yaml` and `env-contract.yaml` by nesting each under its section).
The concrete shape — faithful to the current root files [Repo-grounded]:

```yaml
# repo-config.yml — schema: rhino-cli/repo-config/v1
# (root, all 3 repos; structure identical, values reflect each repo's actual surfaces)

instruction-size: # ← was instruction-size-budget.yaml (per-surface byte budgets)
  surfaces:
    - { glob: "AGENTS.md", target: 24000, warn: 27000, fail: 30000 }
    - { glob: "**/AGENTS.md", target: 24000, warn: 27000, fail: 30000 }
    - { glob: "CLAUDE.md", target: 6000, warn: 8000, fail: 10000 }
    - { glob: ".amazonq/rules/*.md", target: 4000, warn: 8000, fail: 12000 }
    # … one entry per auto-loaded instruction surface

env-contract: # ← was env-contract.yaml (code↔config drift surfaces)
  surfaces:
    - root: apps/ose-be
      kind: app # app | terraform | ansible (IaC kinds are forward-scaffold)
      lang: fsharp # rust | typescript | fsharp (app kind only)
      allowlist: # keys exempt from drift detection
        - OSE_BE_PORT
        - OSE_BE_CORS_ORIGINS
    # … one entry per app/lib env surface

env-injection: # ← was env-injection.yaml (value-less injection homes, NAMES ONLY)
  apps:
    - app: ose-www
      keys-from: apps/ose-www/.env.example # source of truth for the key set
      runtime: { local: env-local, local-ci: compose, production: vercel-production }
    # … one entry per deployable app
```

**Rules:**

- The rhino-cli config loaders (`convention instruction-size validate`, `env validate`/`init`/`backup`/`restore`, the env-injection checker) read their `repo-config.<section>.{surfaces,apps}` subtree instead of the standalone file; a missing section is a hard error (no silent default).
- The old standalone files are **deleted**; all Nx-target `inputs` globs and any docs/references repoint to `repo-config.yml`.
- Test fixtures under `apps/rhino-cli/tests/fixtures/**` keep their own standalone fixture files (they exercise the parser directly) — only the **repo-root** configs merge.
- **The section schema is byte-identical across all 3 repos**; only the per-repo **values** differ (each repo lists its own instruction surfaces, env surfaces, and deployable apps). [Judgment call — consolidation; section schemas are Repo-grounded]

### 1.2 Testing-Architecture & Target-Contents Standard

This subsection extends the standard from target _names_ to target _contents_ and the testing
architecture every project follows. It **revises** parts of
[nx-targets.md](../../../repo-governance/development/infra/nx-targets.md) (which today says "expose
only the targets you need" and treats no-op targets as anti-patterns); the revision is deliberate, to
make `nx affected -t <target>` cover every project uniformly with no special-casing. [Judgment call]

**Definition — project**: a **direct child folder of `apps/` or `libs/`** registered with Nx (i.e.
it has a `project.json`). `apps-labs/` is excluded (not in Nx).

**Mandatory six targets on every project** (declared even when the body is a no-op `echo`
placeholder, so the affected-graph always resolves them):

`test:unit`, `test:integration`, `test:e2e`, `test:quick`, `lint`, `typecheck`

Required where applicable (not part of the mandatory-six):

- `build` — per existing nx-targets.md rules, unchanged.
- **`test:coverage`** — wherever `test:unit` is real (native ≥ 90% gate, replacing the removed rhino-cli `test-coverage`).
- **`specs:behavior:coverage`** — on every project (formerly `specs:coverage`: Gherkin step-def + feature-consumption check).
- **`specs:domain:coverage`** — **only on `*-be` backend projects** (DDD domain-model coverage: every bounded-context/ubiquitous-language entity in `specs/apps/<domain>/domain/**` is exercised by a domain unit test).
- **Formatting is not a per-project target** — it runs as file-type **lint-staged** entries in `.husky/pre-commit` (glob → formatter), identical across all 3 repos (see [§1.1](#11-nx-target-name-standard-targets-invoked-by-hooksci)).

**Target contents**:

| Target             | Content rule                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `typecheck`        | real per language (`tsc --noEmit`, `dotnet build`, `cargo check`); `echo` for languages where compilation already covers it and no separate pass exists                                                                                                                                                                                                                                                                                                |
| `lint`             | real for every project (language linter; UI projects add `oxlint --jsx-a11y-plugin`)                                                                                                                                                                                                                                                                                                                                                                   |
| `test:unit`        | **BDD step tests + non-BDD unit tests**; mocks all I/O; consumes the project's Gherkin scenarios eligible at the unit level **and** may add non-Gherkin tests for behaviour not expressed as scenarios (the only level that may). Coverage is gated by the sibling **`test:coverage`** target (below), not here                                                                                                                                        |
| `test:coverage`    | the project's **native** test runner in coverage mode, enforcing **≥ 90% line** (`vitest --coverage` thresholds, `cargo llvm-cov`/`tarpaulin`, `dotnet test` coverage gate); `echo` where `test:unit` is `echo`. **No** third-party upload (Codecov removed) and **no** central rhino-cli `test-coverage` parser — the gate is local + native per project                                                                                              |
| `test:integration` | **BE**: real, **service-level** — calls service/repository functions directly, **never** through the HTTP API (real PostgreSQL via docker-compose). **FE**: `echo` placeholder **unless** the FE has DB-like integration (e.g. `organiclever-app-web`'s PGlite — `vitest --project integration` + `gen-migrations`), in which case it is real and in-process. **libs/CLI**: real where integration tests exist, else `echo`. Consumes the same Gherkin |
| `test:e2e`         | **real (non-`echo`) ONLY on `*-e2e` projects** — Playwright driving the running app over **HTTP/UI** (this is where the **API** surface is exercised). `echo` on every non-e2e project. Consumes the same Gherkin                                                                                                                                                                                                                                      |
| `test:quick`       | **sequential** `nx:run-commands` with `"parallel": false` running, in this exact order: `nx run <project>:typecheck` → `nx run <project>:lint` → `nx run <project>:test:unit`. Reuses each sibling target's definition + Nx cache; the order is guaranteed by `parallel: false`                                                                                                                                                                        |

**Three test levels consume the same Gherkin** — `test:unit`, `test:integration`, and `test:e2e`
all consume the **same** feature files (driven by the same `@tag`, per
[bdd-spec-test-mapping](../../../repo-governance/development/infra/bdd-spec-test-mapping.md)) from:

- **apps**: `specs/apps/<domain>/behavior/<container>/gherkin/**/*.feature` [Repo-grounded]
- **libs**: `specs/libs/<lib>/gherkin/**/*.feature` (no `behavior/<container>` layer) [Repo-grounded]

**Full-coverage rule**: across the three levels, **every `.feature` file and every scenario** in the
project's binding `behavior/` (or lib `gherkin/`) spec folder MUST be exercised by at least one
**eligible** level (unit, integration, and/or e2e — eligibility set by the scenario's `@tag` and the
level boundary below). No feature or scenario may be left unbound. `test:unit` MAY **additionally**
carry **non-Gherkin** unit tests — to cover behaviour (pure functions, edge cases, error paths) that
is intentionally not expressed as a Gherkin scenario and therefore not exercised at the integration or
e2e levels. So: Gherkin is fully consumed by the three levels together; non-Gherkin lives only in
`test:unit`.

The level boundary is the I/O depth, not the feature file: unit mocks I/O; BE integration uses real
infra at the service layer (no HTTP); e2e drives HTTP/UI in the `*-e2e` project.

**Feature-consumption enforcement** (rhino-cli) — the `specs behavior-coverage validate` command
(renamed from `specs validate coverage`; Nx `specs:behavior:coverage`) enforces the full-coverage rule:
it asserts (a) every Gherkin **step** has a matching step definition, **and** (b) every **feature file**
**and** every **scenario** in the binding spec folder is consumed by ≥ 1 test at an **eligible** level
(unit / integration / e2e), per the `@tag` mapping — not merely "some test somewhere"
(`--require-consumption`, default on). An unbound feature **or scenario** fails the gate; the message
names it: `uncovered scenario: <feature>:<scenario> not exercised by any eligible unit/integration/e2e test`
(and `orphan feature: <path> not consumed by any test` for a whole unconsumed file). See
[§2 triage row 34](#2-rhino-cli-command-triage-wired-vs-not-wired) and the delivery steps. [Repo-grounded — current command checks step-defs only; feature+scenario consumption is new behaviour to add]

**Domain-coverage enforcement** (`*-be` only) — the new `specs domain-coverage validate` command (Nx
`specs:domain:coverage`, **wired only on `*-be` backend projects**) asserts every domain entity in the
project's bounded-context/ubiquitous-language registry (`specs/apps/<domain>/domain/**`) is exercised
by ≥ 1 domain unit test. It runs in the same pre-push ≡ PR gate set as `specs:behavior:coverage`, but
the affected-graph resolves it only for `*-be` projects (non-`*-be` projects do not declare it).
[Judgment call — new command + target]

**Gate rule (pre-push ≡ PR quality gate)** — both gates run the **exact same** per-project command
for affected projects:

```
nx affected -t test:quick      # = typecheck → lint → test:unit, in order
```

plus the identical governance/spec validator set (`specs:behavior:coverage` incl. feature-consumption,
markdown, naming, env). **Neither pre-merge gate runs `test:integration` or `test:e2e`** — those run
**post-merge** (per affected project) plus the nightly CRON fallback (see
[§1.4](#14-post-merge-main-ci--per-project-staging-deploy)). This keeps both pre-merge gates fast and
identical.

```mermaid
flowchart TD
    Q["test:quick (parallel: false)"] --> Q1[nx run P:typecheck]
    Q1 --> Q2[nx run P:lint]
    Q2 --> Q3["nx run P:test:unit (BDD+cov)"]
    PP[pre-push] --> G["nx affected -t test:quick"]
    PR[PR quality gate] --> G
    G --> Q
    POST["post-merge/nightly CRON"] --> INT["test:integration (BE)"]
    POST --> E2E["test:e2e (*-e2e only)"]
    F["specs/.../gherkin/**.feature"] -.same feature files.-> Q3
    F -.same feature files.-> INT
    F -.same feature files.-> E2E

    classDef gate fill:#0072B2,color:#ffffff,stroke:#001f3f
    classDef cron fill:#E69F00,color:#000000,stroke:#5f4200
    classDef spec fill:#009E73,color:#ffffff,stroke:#003f2f
    class PP,PR,G,Q,Q1,Q2,Q3 gate
    class POST,INT,E2E cron
    class F spec
```

### 1.3 Per-Project Target Matrix (post-implementation, ose-public)

The symmetry goal: after this plan, **every** project exposes the **mandatory six** —
`typecheck`, `lint`, `test:unit`, `test:integration`, `test:e2e`, `test:quick` — with a
real command or an `echo` placeholder. The matrix below is the post-implementation target state for
ose-public. (`test:coverage`, `specs:behavior:coverage`, and `build` are shown too — required where
applicable, but not part of the symmetric six; `specs:domain:coverage` is shown for `*-be` only.
**`format` is not a per-project target** — formatting is file-type **lint-staged** (§1.1), so it has
no matrix column. Type-specific extras like `dev`/`start`/`run`/`install`/`codegen`/
`storybook`/`test:e2e:ui`/`test:e2e:report` and rhino-cli's governance targets are intentionally
**not** symmetric and are omitted here.)

**Legend**: ✅ real command · `echo` echo placeholder · — not declared (allowed only for the
non-symmetric `build`/`specs:behavior:coverage`; the seven are NEVER absent). Target presence is
[Repo-grounded] from each `project.json`; the real-vs-echo classification is a [Judgment call]
derived from the [§1.2 rules](#12-testing-architecture--target-contents-standard) and confirmed per
project during execution.

| Project                    | Type             | typecheck | lint | test:unit | test:coverage |  test:integration  | test:e2e | test:quick | specs:behavior:coverage | specs:domain:coverage | build  |
| -------------------------- | ---------------- | :-------: | :--: | :-------: | :-----------: | :----------------: | :------: | :--------: | :---------------------: | :-------------------: | :----: |
| `ayokoding-cli`            | CLI (Rust)       |    ✅     |  ✅  |    ✅     |     ≥90%      |         ✅         |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `ayokoding-www`            | FE (content)     |    ✅     |  ✅  |    ✅     |     ≥90%      |       `echo`       |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `ayokoding-www-be-e2e`     | E2E runner       |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —    |
| `ayokoding-www-fe-e2e`     | E2E runner       |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —    |
| `crane-cli`                | CLI (F#)         |    ✅     |  ✅  |    ✅     |     ≥90%      |         ✅         |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `fsharp-crane-core`        | Lib (F#)         |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅³         |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `organiclever-app-web`     | FE + DB (PGlite) |    ✅     |  ✅  |    ✅     |     ≥90%      |         ✅         |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `organiclever-app-web-e2e` | E2E runner       |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —    |
| `organiclever-be`          | BE (F#)          |    ✅     |  ✅  |    ✅     |     ≥90%      | ✅ (service-level) |  `echo`  |     ✅     |           ✅            |          ✅           |   ✅   |
| `organiclever-be-e2e`      | E2E runner       |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —    |
| `organiclever-www`         | FE (content)     |    ✅     |  ✅  |    ✅     |     ≥90%      |       `echo`       |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `organiclever-www-be-e2e`  | E2E runner       |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       | `echo`²  |     ✅     |           ✅            |           —           |   —    |
| `organiclever-www-fe-e2e`  | E2E runner       |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —    |
| `ose-app-web`              | FE + DB?         |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅¹         |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `ose-app-web-e2e`          | E2E runner       |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —    |
| `ose-be`                   | BE (F#)          |    ✅     |  ✅  |    ✅     |     ≥90%      | ✅ (service-level) |  `echo`  |     ✅     |           ✅            |          ✅           |   ✅   |
| `ose-be-e2e`               | E2E runner       |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —    |
| `ose-cli`                  | CLI (Rust)       |    ✅     |  ✅  |    ✅     |     ≥90%      |         ✅         |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `ose-www`                  | FE (content)     |    ✅     |  ✅  |    ✅     |     ≥90%      |       `echo`       |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `ose-www-be-e2e`           | E2E runner       |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —    |
| `ose-www-fe-e2e`           | E2E runner       |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —    |
| `rhino-cli`                | CLI (Rust)       |    ✅     |  ✅  |    ✅     |     ≥90%      |         ✅         |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `rust-commons`             | Lib (Rust)       |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅³         |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `wahidyankf-www`           | FE (content)     |    ✅     |  ✅  |    ✅     |     ≥90%      |       `echo`       |  `echo`  |     ✅     |           ✅            |           —           |   ✅   |
| `wahidyankf-www-fe-e2e`    | E2E runner       |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —    |
| `web-ui`                   | Lib (UI)         |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅³         |  `echo`  |     ✅     |           ✅            |           —           |  ✅⁴   |
| `web-ui-token`             | Lib (tokens)     |    ✅     |  ✅  |    ✅     |     ≥90%      |       `echo`       |  `echo`  |    ✅⁵     |           ✅            |           —           | `echo` |

Footnotes:

1. `ose-app-web` `test:integration` is real **only if** it is DB-backed/local-first (like `organiclever-app-web`'s PGlite); otherwise `echo`. Confirm its storage during execution.
2. `organiclever-www-be-e2e` is a placeholder slot (no backend API yet — [AGENTS.md](../../../AGENTS.md)); `test:e2e` stays `echo` until a backend exists.
3. Lib `test:integration` is real where the lib actually has integration tests today; otherwise `echo`. Confirm per lib.
4. `web-ui` builds via `build-storybook` (no plain `build`); treated as its build artifact.
5. `web-ui-token` currently **lacks `test:quick`** — this plan adds it (the most visible mandatory-six gap).

> **Formatting is file-type, not per-project**: this plan **removes** any per-project `format`/`format:check`
> Nx target and the rhino-cli `fmt` target. Formatting runs as **lint-staged** entries keyed by file type
> (`prettier --write` for `*.{md,json,yml,yaml,css,scss,ts,tsx,js}`, `rustfmt` for `*.rs`, `fantomas` for
> `*.fs`) in `.husky/pre-commit`, identical across all 3 repos — so there is **no** `format` matrix column.
> [Repo-grounded — TS already uses lint-staged; this extends the map to `*.rs`/`*.fs` and drops the targets]

### 1.3b Per-Project Target Matrix (post-implementation, ose-primer)

Same legend (✅ real · `echo` placeholder · — not declared). Rows are [Repo-grounded] from each
`project.json`; real/echo is a [Judgment call] per §1.2, confirmed during execution.

| Project                     | Type          | typecheck | lint | test:unit | test:coverage | test:integration | test:e2e | test:quick | specs:behavior:coverage | specs:domain:coverage | build |
| --------------------------- | ------------- | :-------: | :--: | :-------: | :-----------: | :--------------: | :------: | :--------: | :---------------------: | :-------------------: | :---: |
| `clojure-openapi-codegen`   | Lib (codegen) |  `echo`¹  |  ✅  |    ✅     |     ≥90%      |      `echo`      |  `echo`  |     ✅     |           ✅²           |           —           |  ✅   |
| `crud-be-clojure-pedestal`  | BE (Clojure)  |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `crud-be-csharp-aspnetcore` | BE (C#)       |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `crud-be-e2e`               | E2E runner    |    ✅     |  ✅  |  `echo`   |       —       |      `echo`      |    ✅    |     ✅     |           ✅            |           —           |   —   |
| `crud-be-elixir-phoenix`    | BE (Elixir)   |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `crud-be-fsharp-giraffe`    | BE (F#)       |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `crud-be-golang-gin`        | BE (Go)       |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `crud-be-java-springboot`   | BE (Java)     |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `crud-be-java-vertx`        | BE (Java)     |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `crud-be-kotlin-ktor`       | BE (Kotlin)   |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `crud-be-python-fastapi`    | BE (Python)   |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `crud-be-rust-axum`         | BE (Rust)     |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `crud-be-ts-effect`         | BE (TS)       |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `crud-fe-dart-flutterweb`   | FE (Dart)     |    ✅     |  ✅  |    ✅     |     ≥90%      |      `echo`      |  `echo`  |     ✅     |           ✅            |           —           |  ✅   |
| `crud-fe-e2e`               | E2E runner    |    ✅     |  ✅  |  `echo`   |       —       |      `echo`      |    ✅    |     ✅     |           ✅            |           —           |   —   |
| `crud-fe-ts-nextjs`         | FE            |    ✅     |  ✅  |    ✅     |     ≥90%      |      `echo`      |  `echo`  |     ✅     |           ✅            |           —           |  ✅   |
| `crud-fe-ts-tanstack-start` | FE            |    ✅     |  ✅  |    ✅     |     ≥90%      |      `echo`      |  `echo`  |     ✅     |           ✅            |           —           |  ✅   |
| `crud-fs-ts-nextjs`         | Fullstack     |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |           —           |  ✅   |
| `elixir-cabbage`            | Lib (Elixir)  |    ✅     |  ✅  |    ✅     |     ≥90%      |      `echo`      |  `echo`  |     ✅     |           ✅²           |           —           |   —   |
| `elixir-gherkin`            | Lib (Elixir)  |    ✅     |  ✅  |    ✅     |     ≥90%      |      `echo`      |  `echo`  |     ✅     |           ✅²           |           —           |   —   |
| `elixir-openapi-codegen`    | Lib (Elixir)  |    ✅     |  ✅  |    ✅     |     ≥90%      |      `echo`      |  `echo`  |     ✅     |           ✅²           |           —           |   —   |
| `golang-commons`            | Lib (Go)      |  `echo`¹  |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅²           |           —           |   —   |
| `rhino-cli`                 | CLI (Rust)    |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅        |  `echo`  |     ✅     |           ✅            |           —           |  ✅   |
| `ts-ui`                     | Lib (UI)      |    ✅     |  ✅  |    ✅     |     ≥90%      |      `echo`      |  `echo`  |     ✅     |           ✅²           |           —           |  ✅⁴  |
| `ts-ui-tokens`              | Lib (tokens)  |    ✅     |  ✅  |  `echo`   |       —       |      `echo`      |  `echo`  |    ✅³     |           ✅²           |           —           |   —   |

Footnotes: ¹ dynamic/compile-typed language (Clojure/Go) — `typecheck` is an `echo` placeholder.
² `specs:behavior:coverage` is **added** (not present today) per the all-apps-and-libs rule. ³ `ts-ui-tokens`
today has **only** `lint`+`typecheck` — four of the six are added (`test:unit`/`test:integration`/
`test:e2e` as `echo`, `test:quick`); formatting is lint-staged, not a target. ⁴ `ts-ui` builds via `build-storybook`.

> **Primer gaps (today → after)**: the 11 `crud-be-*`
> backends + `crud-fs-ts-nextjs` lack a `test:e2e` target (add `echo`); the `crud-fe-*` frontends lack
> `test:integration` + `test:e2e` (add `echo`); the support libs (`clojure-openapi-codegen`,
> `elixir-*`, `golang-commons`, `ts-ui*`) lack 2–4 of the six. Formatting is handled by the shared
> lint-staged map (no per-project `format` target). The 11 `crud-be-*` backends each gain
> `specs:domain:coverage`. [Repo-grounded]

### 1.3c Per-Project Target Matrix (post-implementation, ose-infra)

| Project             | Type         | typecheck | lint | test:unit | test:coverage |  test:integration  | test:e2e | test:quick | specs:behavior:coverage | specs:domain:coverage | build |
| ------------------- | ------------ | :-------: | :--: | :-------: | :-----------: | :----------------: | :------: | :--------: | :---------------------: | :-------------------: | :---: |
| `coralpolyp-be`     | BE           |    ✅     |  ✅  |    ✅     |     ≥90%      | ✅ (service-level) |  `echo`  |     ✅     |           ✅            |          ✅           |  ✅   |
| `coralpolyp-be-e2e` | E2E runner   |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —   |
| `coralpolyp-fe`     | FE + DB?     |    ✅     |  ✅  |    ✅     |     ≥90%      |        ✅¹         |  `echo`  |     ✅     |           ✅            |           —           |  ✅   |
| `coralpolyp-fe-e2e` | E2E runner   |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |    ✅    |     ✅     |           ✅            |           —           |   —   |
| `rhino-cli`         | CLI (Rust)   |    ✅     |  ✅  |    ✅     |     ≥90%      |         ✅         |  `echo`  |     ✅     |           ✅            |           —           |  ✅   |
| `ts-ui`             | Lib (UI)     |    ✅     |  ✅  |    ✅     |     ≥90%      |       `echo`       |  `echo`  |     ✅     |           ✅²           |           —           |  ✅⁴  |
| `ts-ui-tokens`      | Lib (tokens) |    ✅     |  ✅  |  `echo`   |       —       |       `echo`       |  `echo`  |    ✅³     |           ✅²           |           —           |   —   |

Footnotes as §1.3b. ¹ `coralpolyp-fe` `test:integration` is real only if DB-backed; else `echo` —
confirm. **Infra gaps**: formatting moves to the shared lint-staged map (no per-project `format`
target); `ts-ui-tokens` has only `lint`+`typecheck`; `coralpolyp-be` gains `specs:domain:coverage`;
`rhino-cli` lacks the `{tool}:check` + `harness:bindings-validation` Nx targets (§4.1).

### 1.4 Post-Merge (main) CI & Per-Project Staging Deploy

The pre-merge gates (pre-push, PR) run **only** `test:quick` (§1.2). The **heavy** levels and the
deploy happen **post-merge**, on push to `main`:

**Trigger**: push to `main` (a merged PR).

**Per affected project, in isolation** (a matrix job keyed on `nx show projects --affected`, so one
project's failure never blocks another's):

1. `test:quick` (typecheck → lint → test:unit)
2. `test:integration`
3. `test:e2e` (via the project's `*-e2e` runner against a locally stood-up stack)

**Then, if all three pass AND the project is deployable, deploy it to staging — individually**:

| Tier                                 | Deploy-on-merge target                                       | Notes                                                                              |
| ------------------------------------ | ------------------------------------------------------------ | ---------------------------------------------------------------------------------- |
| app-tier (`*-app-web`, `*-be`)       | existing `stag-*` branches → Vercel/k8s staging              | reuses `_reusable-*-test-local-deploy-stag`                                        |
| marketing (`*-www`)                  | **new** `stag-*-www` branch + **new staging Vercel project** | net-new infra (www sites have no staging env today) — provisioning is a setup step |
| non-deployable (libs, CLIs, `*-e2e`) | none                                                         | run tests only                                                                     |

**CRON relationship**: the merge-triggered pipeline is the **primary** staging path. The existing
scheduled `*-test-local-deploy-stag.yml` CRON is **retained at reduced cadence as a nightly
fallback** (re-tests + redeploys to catch dependency drift that lands without a merge).

**Prod**: unchanged — the scheduled **`*-test-stag.yml` → deploy-prod** pipeline promotes staging to
production. Pre-merge never runs integration/e2e; those run post-merge + the CRON fallback only.

**Per-repo deployable sets** (the post-merge **test** matrix runs in all three repos; only the
**deploy** leg differs by what each repo actually ships):

- **ose-public** — app-tier (`*-app-web`, `*-be`) → existing `stag-*`; marketing (`*-www`) → new `stag-*-www` staging. [Repo-grounded]
- **ose-infra** — `coralpolyp-be` + `coralpolyp-fe` → existing coralpolyp staging (`test-and-deploy-coralpolyp-development` reusable logic, now merge-triggered + nightly fallback); on the self-hosted runner. [Repo-grounded]
- **ose-primer** — **template repo**: the `crud-*` demo apps have **no real staging environment** (their `test-and-deploy-*-development` workflows are local-stack test harnesses). Post-merge runs the full per-project **test** matrix; the **deploy leg is a documented no-op** (the demo apps are reference scaffolding, not deployed services). [Judgment call — confirm primer has no live staging target]

```mermaid
flowchart TD
    M[push to main] --> AF[nx affected projects]
    AF --> J["quick→int→e2e (each)"]
    J --> D{pass & deployable?}
    D -- yes --> S[deploy → staging]
    D -- no --> X[stop]
    CR[nightly CRON] -.redeploy.-> S
    S --> PR2[stag → prod]

    classDef merge fill:#0072B2,color:#ffffff,stroke:#001f3f
    classDef deploy fill:#009E73,color:#ffffff,stroke:#003f2f
    classDef cron fill:#E69F00,color:#000000,stroke:#5f4200
    class M,AF,J merge
    class S,PR2 deploy
    class CR cron
```

> **Infra note**: giving every `*-www` site a staging environment (`stag-*-www` branch + Vercel
> staging project) is net-new. Branch + workflow wiring is `[AI]`; creating the Vercel staging
> projects may need dashboard access (`[HUMAN]` or via the Vercel MCP). Tracked in delivery.

## 2. rhino-cli Command Triage (Wired vs. Not-Wired)

A command is **wired** when some lifecycle automation (a `.husky` hook step, a `.github/workflows`
job, or an Nx target reachable from a hook/CI gate) invokes it. It is **not wired** when it exists
in the CLI but is only runnable by hand or solely via an aggregate `audit` subcommand that no
automation calls. All leaf subcommands are enumerated from `apps/rhino-cli/src/cli.rs`. [Repo-grounded]

### 2.0 Two Naming Conventions (locked)

This plan standardizes **two distinct surfaces** with **two distinct conventions** — the
**Command (leaf) — target** column applies the first; every Nx target referenced applies the second:

1. **rhino-cli CLI commands** — **noun-hierarchy then verb-last**:
   `{domain} {sub-domain…} {sub-sub-domain…} … {verb}`. The verb (`validate`, `audit`, `sync`,
   `emit`, `generate`, `clean`, `scaffold`, `init`, `backup`, `restore`) is **always the final
   token**; everything before it is the noun path. So `convention validate emoji` →
   `convention emoji validate`; `harness sync opencode` → `harness opencode sync`; the two new
   coverage validators are authored directly verb-last (`specs behavior-coverage validate`,
   `specs domain-coverage validate`).
   Family-level aggregates keep their verb last already (`md audit`, `convention audit`). Pre-existing
   `(alias)` shortcuts are removed in favour of the canonical verb-last form. [Judgment call —
   naming standard]

2. **Nx / `project.json` targets** — **`:`-separated** `{domain}:{work}` (validation/governance) or
   lifecycle names: `lint`, `test:unit`, `test:coverage`, `test:integration`, `test:e2e`,
   `test:quick`, `specs:behavior:coverage`, `specs:domain:coverage`, `build`,
   `instruction-size:validation`, `governance:vendor-audit-validation`, … — colon-segmented, never
   space-segmented. The Nx target name and the CLI command it invokes are **independent** (e.g. Nx
   `specs:behavior:coverage` → CLI `specs behavior-coverage validate`). [Repo-grounded — existing `:`
   scheme in `nx-targets.md`]

The **Command (leaf) — target** column below is the verb-last name each command converges to; **Decided
= ✅** marks that the row's target name is settled by these conventions (wiring keep/remove decisions
remain in the Status column + Open Questions).

| #   | Command (leaf) — current                     | Command (leaf) — target                      | What it does                                                                                                                                                                       | Status                      | Decided | Invocation site (if wired)                                                                                                   |
| --- | -------------------------------------------- | -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------- | :-----: | ---------------------------------------------------------------------------------------------------------------------------- |
| 1   | `test-coverage validate`                     | **— (removed)**                              | Check coverage output against a line-based threshold                                                                                                                               | **wired** → **drop**        |   ✅    | replaced by each project's **native** coverage gate at `test:unit` (≥ 90%); Nx `test-coverage` target deleted in all 3 repos |
| 2   | `repo-governance validate vendor`            | `repo-governance vendor validate`            | Scan governance markdown for forbidden vendor-specific terms                                                                                                                       | **wired**                   |   ✅    | Nx `governance:vendor-audit-validation` → pre-push (scoped); **drift: not in primer pre-push**                               |
| 3   | `repo-governance validate layer-coherence`   | `repo-governance layer-coherence validate`   | Audit governance docs for layer numbering/naming coherence                                                                                                                         | not wired                   |   ✅    | only via `repo-governance audit` (manual)                                                                                    |
| 4   | `repo-governance validate traceability`      | `repo-governance traceability validate`      | Audit governance docs for required traceability sections                                                                                                                           | not wired                   |   ✅    | only via `repo-governance audit` (manual)                                                                                    |
| 5   | `repo-governance audit`                      | `repo-governance audit`                      | Run all deterministic governance audits; emit JSON envelope                                                                                                                        | not wired                   |   ✅    | manual aggregate                                                                                                             |
| 6   | `md validate naming`                         | `md naming validate`                         | Validate markdown filenames are lowercase-kebab-case                                                                                                                               | not wired                   |   ✅    | only via `md audit` (manual)                                                                                                 |
| 7   | `md validate frontmatter`                    | `md frontmatter validate`                    | Validate doc YAML frontmatter against area-specific schemas                                                                                                                        | not wired                   |   ✅    | only via `md audit` (manual)                                                                                                 |
| 8   | `md validate heading-hierarchy`              | `md heading-hierarchy validate`              | Validate heading hierarchy (one H1, no skipped levels)                                                                                                                             | **wired**                   |   ✅    | Nx `headings:hierarchy-validation` → pre-commit + markdown workflow                                                          |
| 9   | `md validate links`                          | `md links validate`                          | Validate markdown links (relative paths + `#fragment` anchors resolve)                                                                                                             | **wired**                   |   ✅    | Nx `links:validation` → pre-commit + markdown workflow                                                                       |
| 10  | `md validate mermaid`                        | `md mermaid validate`                        | Validate Mermaid diagrams (label length, width/span, single-diagram)                                                                                                               | **wired**                   |   ✅    | Nx `mermaid:validation` → pre-commit + markdown workflow                                                                     |
| 11  | `md validate frontmatter-dates`              | `md frontmatter-dates validate`              | Audit markdown for forbidden manual date metadata                                                                                                                                  | not wired                   |   ✅    | only via `md audit` (manual)                                                                                                 |
| 12  | `md validate readme-index`                   | `md readme-index validate`                   | Audit directory README indexes against sibling markdown                                                                                                                            | not wired                   |   ✅    | only via `md audit` (manual)                                                                                                 |
| 13  | `md frontmatter-dates` (alias)               | — (removed)                                  | Alias of `md frontmatter-dates validate`; removed in favour of canonical verb-last                                                                                                 | not wired → remove          |   ✅    | —                                                                                                                            |
| 14  | `md readme-index` (alias)                    | — (removed)                                  | Alias of `md readme-index validate`; removed in favour of canonical verb-last                                                                                                      | not wired → remove          |   ✅    | —                                                                                                                            |
| 15  | `md audit`                                   | `md audit`                                   | Run all md validators in sequence; aggregate findings                                                                                                                              | not wired                   |   ✅    | manual aggregate                                                                                                             |
| 16  | `convention validate emoji`                  | `convention emoji validate`                  | Audit forbidden file types for emoji codepoints                                                                                                                                    | not wired                   |   ✅    | only via `convention audit` (manual)                                                                                         |
| 17  | `convention validate license`                | `convention license validate`                | Verify per-directory LICENSE files match the licensing convention                                                                                                                  | not wired                   |   ✅    | only via `convention audit` (manual)                                                                                         |
| 18  | `convention validate instruction-size`       | `convention instruction-size validate`       | Audit all auto-loaded instruction surfaces against per-surface byte budgets (`instruction-size-budget.yaml`); legacy alias: `agents-md-size`                                       | **wired**                   |   ✅    | Nx `rhino-cli:instruction-size:validation` → pre-push (changed-path gate) + PR gate                                          |
| 19  | `convention audit`                           | `convention audit`                           | Run all convention validators; aggregate findings                                                                                                                                  | not wired                   |   ✅    | manual aggregate                                                                                                             |
| 20  | `harness validate naming`                    | `harness naming validate`                    | Validate agent filename suffixes + `.claude`↔`.opencode` mirror parity                                                                                                             | **wired**                   |   ✅    | Nx `naming:harness-validation` → pre-push (scoped) + PR gate                                                                 |
| 21  | `harness validate duplication`               | `harness duplication validate`               | Detect verbatim duplication across agent + skill files                                                                                                                             | not wired                   |   ✅    | only via `harness audit` (manual)                                                                                            |
| 22  | `harness validate claude`                    | `harness claude validate`                    | Validate Claude Code agent/skill format in `.claude/`                                                                                                                              | not wired                   |   ✅    | npm `validate:claude` (manual script)                                                                                        |
| 23  | `harness validate sync`                      | `harness sync validate`                      | Validate `.claude/` and `.opencode/` are in sync                                                                                                                                   | not wired                   |   ✅    | npm `validate:sync` (manual script)                                                                                          |
| 24  | `harness validate bindings`                  | `harness bindings validate`                  | Validate Amazon Q binding bridge files + catalog coverage                                                                                                                          | **wired**                   |   ✅    | npm `harness:bindings-validation` → pre-push (scoped)                                                                        |
| 25  | `harness sync opencode`                      | `harness opencode sync`                      | Sync Claude Code agents → OpenCode format                                                                                                                                          | **wired** `[Unverified]`    |   ✅    | pre-commit `git pre-commit` auto-sync (CLAUDE.md claims auto-sync; confirm in Phase 1)                                       |
| 26  | `harness emit amazonq`                       | `harness amazonq emit`                       | Emit Amazon Q Developer binding bridge files (idempotent)                                                                                                                          | **wired** `[Unverified]`    |   ✅    | pre-commit `git pre-commit` auto-sync (confirm in Phase 1)                                                                   |
| 27  | `harness generate bindings`                  | `harness bindings generate`                  | Generate all platform bindings (sync OpenCode + emit Amazon Q)                                                                                                                     | **wired** `[Unverified]`    |   ✅    | pre-commit auto-sync + npm `generate:bindings` (confirm in Phase 1)                                                          |
| 28  | `harness audit`                              | `harness audit`                              | Run all harness validators; aggregate findings                                                                                                                                     | not wired                   |   ✅    | manual aggregate                                                                                                             |
| 29  | `workflows validate naming`                  | `workflows naming validate`                  | Validate workflow filename suffixes + frontmatter name consistency                                                                                                                 | **wired**                   |   ✅    | Nx `naming:workflows-validation` → pre-push (scoped) + PR gate                                                               |
| 30  | `specs validate adoption`                    | `specs adoption validate`                    | Verify an app has adopted BDD + DDD practices (no orphan app)                                                                                                                      | **wired**                   |   ✅    | Nx `specs:adoption-validation` → pre-push + PR gate                                                                          |
| 31  | `specs validate counts`                      | `specs counts validate`                      | Validate each required spec subfolder has ≥1 spec file                                                                                                                             | **wired**                   |   ✅    | Nx `specs:counts-validation` → pre-push + PR gate                                                                            |
| 32  | `specs validate links`                       | `specs links validate`                       | Check markdown links in spec files resolve                                                                                                                                         | **wired**                   |   ✅    | Nx `specs:links-validation` → pre-push + PR gate                                                                             |
| 33  | `specs validate tree`                        | `specs tree validate`                        | Validate canonical C4-aware five-folder spec tree                                                                                                                                  | **wired**                   |   ✅    | Nx `specs:tree-validation` → pre-push + PR gate                                                                              |
| 34  | `specs validate coverage`                    | `specs behavior-coverage validate`           | Validate every Gherkin step has a step definition **and** every feature **+ scenario** is exercised by ≥1 eligible unit/integration/e2e test (`--require-consumption`, new — §1.2) | **wired**                   |   ✅    | Nx `specs:behavior:coverage` (renamed from `specs:coverage`) → pre-push + PR gate                                            |
| 34b | **— (new)**                                  | `specs domain-coverage validate`             | Validate every domain entity in `specs/apps/<domain>/domain/**` (bounded-context/ubiquitous-language registry) is exercised by ≥1 domain unit test                                 | **new** → **wire (`*-be`)** |   ✅    | Nx `specs:domain:coverage` → pre-push + PR gate, **only on `*-be` backend projects**                                         |
| 35  | `specs validate bc`                          | `specs bc validate`                          | Validate bounded-context structural parity against the registry                                                                                                                    | not wired                   |   ✅    | no Nx target; manual                                                                                                         |
| 36  | `specs validate ul`                          | `specs ul validate`                          | Validate ubiquitous-language glossary parity against the registry                                                                                                                  | not wired                   |   ✅    | no Nx target; manual                                                                                                         |
| 37  | `specs validate gherkin-cardinality`         | `specs gherkin-cardinality validate`         | Audit `.feature` scenarios for repeated primary Given/When/Then keywords                                                                                                           | **wired**                   |   ✅    | Nx `specs:gherkin-cardinality-validation` → PR gate (+ markdown workflow in primer/infra)                                    |
| 38  | `specs clean java-imports`                   | `specs java-imports clean`                   | Strip unused/same-package imports from generated Java contract files                                                                                                               | not wired                   |   ✅    | dormant (primer-oriented)                                                                                                    |
| 39  | `specs scaffold dart`                        | `specs dart scaffold`                        | Generate Dart package scaffolding around generated contract types                                                                                                                  | not wired                   |   ✅    | dormant (primer-oriented)                                                                                                    |
| 40  | `specs audit`                                | `specs audit`                                | Run all specs validators; aggregate findings                                                                                                                                       | not wired                   |   ✅    | manual aggregate                                                                                                             |
| 41  | `lang java validate null-safety-annotations` | `lang java null-safety-annotations validate` | Check Java packages carry required null-safety annotations                                                                                                                         | not wired                   |   ✅    | dormant (primer-oriented)                                                                                                    |
| 42  | `git pre-commit`                             | `git pre-commit`                             | Run the full pre-commit pipeline (config sync, format, doc validation)                                                                                                             | **wired**                   |   ✅    | `.husky/pre-commit`                                                                                                          |
| 43  | `env init`                                   | `env init`                                   | Create `.env` files from `.env.example` templates                                                                                                                                  | not wired                   |   ✅    | developer convenience                                                                                                        |
| 44  | `env backup`                                 | `env backup`                                 | Back up `.env` files from the repository                                                                                                                                           | not wired                   |   ✅    | developer convenience                                                                                                        |
| 45  | `env restore`                                | `env restore`                                | Restore `.env` files from a backup                                                                                                                                                 | not wired                   |   ✅    | developer convenience                                                                                                        |
| 46  | `env validate`                               | `env validate`                               | Check code↔config drift for all `env-contract.yaml` surfaces                                                                                                                       | **wired**                   |   ✅    | Nx `env:validation` → pre-push + env workflow                                                                                |
| 47  | `doctor`                                     | `doctor`                                     | Check required tool versions are installed and correct                                                                                                                             | **wired**                   |   ✅    | `npm install` postinstall + `npm run doctor`                                                                                 |

**Summary**: ~18 wired, ~29 not-wired. The not-wired set is dominated by (a) per-family `audit`
aggregates and their leaf validators that only `audit` calls, (b) developer-convenience `env`
commands, and (c) dormant primer-oriented `specs clean` / `specs scaffold` / `lang java` commands.
[Repo-grounded — counts approximate pending Phase 1 confirmation of rows 25–27]

> **Phase 1 triage decisions to confirm with maintainer** (not resolved by this plan, recorded as Open Questions):
> whether the `*-audit` aggregates and their leaf validators (emoji, license, agents-md-size, frontmatter, naming, readme-index, layer-coherence, traceability) _should_ be wired into a periodic gate, and whether dormant commands should be removed. Triage only — no wiring change here.

## 3. Divergence Policy (Allowed vs. Drift)

Per the [identical-result invariant](#1-target-standard-best-of-three-synthesis), the standardization
layer is **identical across all 3 repos**; the **only** sanctioned variation is what each repo
actually _ships_ (its project/app set) and the data that follows from it. Everything in "Drift" below
MUST converge to one form.

**Allowed divergence** (NOT flagged, recorded in the standard doc):

- **App set & per-app deploy CRONs** — public ships content/web apps (`ose-www`, `ayokoding-www`, `organiclever-www`, `wahidyankf-www`, `*-app-web`, `*-be`); primer ships polyglot demo backends/frontends; infra ships `coralpolyp`. Each repo keeps only the deploy CRON workflows for apps it actually ships. [Repo-grounded]
- **Language gate jobs** — the PR gate's per-language jobs (golang, jvm, dotnet, python, rust, elixir, clojure, dart, typescript) exist only for languages present in that repo. [Repo-grounded]
- **Infra-only IaC gates** — `terraform fmt`/`validate`/`tflint`, `ansible-lint`, `yamllint` exist only in `ose-infra`, in both hooks and the PR gate. [Repo-grounded]
- **Self-hosted runner labels** — infra runs on `[self-hosted, linux, ose-infra-runner]`. [Repo-grounded]
- **lint-staged formatter entries** — only for languages present (e.g. `*.go`, `*.{ex,exs}` exist where that language ships). The common entries (`*.md`, `*.json`, `*.{yml,yaml}`, `*.{css,scss}`, `*.rs`, `*.fs`) MUST match. [Repo-grounded]

**Drift** (MUST converge — this is the work):

- Workflow **filenames** for the shared gates (PR gate, markdown, env).
- The **validator set** inside the markdown workflow and the specs-gate.
- The **invocation mechanism** for shell/docker/actions lint (inline vs. Nx target).
- The **pre-push scoped validator set** (governance vendor audit presence).
- The **job skeleton / names** in the PR gate (detect, markdown, naming, env, specs-gate, quality-gate sentinel; formatting is lint-staged at commit, not a gate job).
- The **placement** of env validation (standalone workflow vs. folded into the PR gate).
- The **Nx target names** invoked by hooks/CI, and the **rhino-cli target set** itself (see [§4.1](#41-nx-target-name-drift-rhino-cli)): `fmt`/`format:check` targets (removed — formatting via lint-staged), missing `{tool}:check` wrappers, `harness:bindings-validation` as Nx target vs npm script, primer's missing structural specs targets.

## 4. Drift Catalog (Per Surface)

| Surface                    | ose-public                  | ose-primer                                  | ose-infra                                     | Action                                                                          |
| -------------------------- | --------------------------- | ------------------------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------- |
| PR gate file               | `commons-quality-gate.yml`  | `pr-quality-gate.yml`                       | `pr-quality-gate.yml`                         | rename public → `pr-quality-gate.yml`                                           |
| Markdown file              | `markdown-validate.yml`     | `validate-markdown.yml`                     | `validate-markdown.yml`                       | rename public → `validate-markdown.yml`                                         |
| Env file                   | `commons-env-validate.yml`  | folded into PR gate                         | `validate-env.yml`                            | rename public → `validate-env.yml`; primer extract standalone                   |
| Markdown validators        | 3 (no gherkin-cardinality)  | 4                                           | 4                                             | public add gherkin-cardinality                                                  |
| specs-gate set             | full                        | coverage + gherkin only                     | run-many tree/counts/links/adoption + gherkin | primer promote structural set                                                   |
| sh/docker/actions lint     | inline shell in hook + jobs | Nx targets `shell/dockerfiles/actions:lint` | inline shell in hook + jobs                   | public + infra add `{tool}:check` Nx targets; primer renames `:lint` → `:check` |
| pre-push governance vendor | yes (scoped)                | no                                          | yes (scoped)                                  | primer add                                                                      |
| env validation placement   | standalone wf               | PR-gate job                                 | standalone wf                                 | primer extract to standalone                                                    |

All cells above are [Repo-grounded] from the Phase-mapping exploration and the workflow-directory
listings.

### 4.1 Nx Target-Name Drift (rhino-cli)

From `jq -r '.targets | keys[]' apps/rhino-cli/project.json` in each repo (current commit). ✅ = target
present, ❌ = absent. [Repo-grounded]

| rhino-cli target                                                                                             | public | primer  | infra | Action                                                                               |
| ------------------------------------------------------------------------------------------------------------ | :----: | :-----: | :---: | ------------------------------------------------------------------------------------ |
| `fmt` (write)                                                                                                |   ✅   |   ✅    |  ✅   | **remove** in all 3 (formatting → file-type lint-staged)                             |
| `format:check`                                                                                               |   ✅   |   ✅    |  ✅   | **remove** in all 3 (no per-project format target)                                   |
| `shell:check` / `dockerfiles:check` / `actions:check`                                                        |   ❌   | `:lint` |  ❌   | add to public + infra; primer rename `:lint`→`:check`                                |
| `harness:bindings-validation` (Nx target)                                                                    |   ❌   |   ✅    |  ❌   | add Nx target to public + infra (replace npm script)                                 |
| `specs:adoption-validation` / `specs:counts-validation` / `specs:links-validation` / `specs:tree-validation` |   ✅   |   ❌    |  ✅   | add all four to primer                                                               |
| `test-coverage`                                                                                              |   ✅   |   ❌    |  ✅   | **remove** from public + infra (native coverage at `test:unit`); not added to primer |
| `test:e2e`                                                                                                   |   ✅   |   ❌    |  ✅   | add to primer (no-op echo where no e2e)                                              |

Target convergence acceptance: the sorted `.targets` key set of `apps/rhino-cli/project.json` is
identical across all three repos after Phases 2–4.

### 4.2 GitHub CI Workflow Inventory (current → target, per repo)

**Plan scope boundary**: this plan is complete when **every project in all three repos is covered by a
standardized GitHub CI**, with workflow **filenames + job structure following the canonical
ose-public convention** (verb-first validators, `pr-quality-gate.yml`, `*-test-*` / `_reusable-*`
deploy shape). The canonical names are the §1 surface-table winners; ose-public adopts them via the
renames below, so post-plan ose-public **is** the convention. [Repo-grounded — workflow lists from
each repo's `.github/workflows/`]

**Canonical (post-plan ose-public) workflow set** — the standardizable, non-app-specific workflows
every repo MUST have, identically named:

| Role                      | Canonical filename      | Trigger                       |
| ------------------------- | ----------------------- | ----------------------------- |
| PR quality gate           | `pr-quality-gate.yml`   | `pull_request`                |
| Markdown validators       | `validate-markdown.yml` | `pull_request` + `push:main`  |
| Env contract validation   | `validate-env.yml`      | `pull_request` + `push:main`  |
| Post-merge per-project CI | `main-ci.yml` (**new**) | `push:main` (affected matrix) |

App/demo **deploy + CRON** workflows are **allowed divergence** (each repo keeps only the ones for
what it ships) but MUST adopt the canonical naming shape (`*-test-local-deploy-{stag,prod}.yml`,
`*-test-{stag}.yml`, `_reusable-*.yml`).

**ose-public** (current → target):

| Current workflow                                                                                                                                                   | Target                          | Action                                                          |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------- | --------------------------------------------------------------- |
| `commons-quality-gate.yml`                                                                                                                                         | `pr-quality-gate.yml`           | **rename**                                                      |
| `markdown-validate.yml`                                                                                                                                            | `validate-markdown.yml`         | **rename** + add gherkin-card                                   |
| `commons-env-validate.yml`                                                                                                                                         | `validate-env.yml`              | **rename**                                                      |
| _(none)_                                                                                                                                                           | `main-ci.yml`                   | **add** (post-merge matrix)                                     |
| `*-www-test-local-deploy-prod.yml`, `*-app-test-*-stag.yml`, `*-be-build-deploy-stag.yml`, `_reusable-*.yml`, `web-ui-build-deploy-prod.yml`, `publish-images.yml` | same (naming already canonical) | keep (allowed divergence); add `stag-*-www` staging legs (§1.4) |

**ose-primer** (current → target):

| Current workflow                                                                                                                           | Target                       | Action                                          |
| ------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------- | ----------------------------------------------- |
| `pr-quality-gate.yml`                                                                                                                      | `pr-quality-gate.yml`        | keep; promote full specs-gate + extract env     |
| `validate-markdown.yml`                                                                                                                    | `validate-markdown.yml`      | keep (already canonical)                        |
| _(env folded in PR gate)_                                                                                                                  | `validate-env.yml`           | **extract** standalone                          |
| _(none)_                                                                                                                                   | `main-ci.yml`                | **add** (post-merge matrix; deploy leg = no-op) |
| `test-and-deploy-{backend,frontend,fullstack}-development.yml`, `test-crud-*.yml`, `_reusable-backend-*.yml`, `_reusable-frontend-e2e.yml` | adopt canonical naming shape | keep (allowed divergence — demo apps)           |

**ose-infra** (current → target):

| Current workflow                                                                                   | Target                       | Action                                                                  |
| -------------------------------------------------------------------------------------------------- | ---------------------------- | ----------------------------------------------------------------------- |
| `pr-quality-gate.yml`                                                                              | `pr-quality-gate.yml`        | keep (already canonical)                                                |
| `validate-markdown.yml`                                                                            | `validate-markdown.yml`      | keep                                                                    |
| `validate-env.yml`                                                                                 | `validate-env.yml`           | keep                                                                    |
| _(none)_                                                                                           | `main-ci.yml`                | **add** (self-hosted; per-project matrix)                               |
| `test-and-deploy-coralpolyp-development.yml`, `test-coralpolyp-staging.yml`, `test-coralpolyp.yml` | adopt canonical naming shape | keep (allowed divergence — coralpolyp); retain `[self-hosted, …]` label |

Acceptance: in each repo, the four canonical workflows exist with the canonical names; every project
resolves into `main-ci.yml`'s affected matrix; deploy/CRON workflows follow the naming shape.

## 5. Diagrams

### 5.1 SDLC gate flow (target standard, shared mechanics)

```mermaid
flowchart TD
    A[git commit] --> B[commitlint]
    A --> C[pre-commit hook]
    C --> C1[identity + env check]
    C1 --> C2["sh/docker/actions"]
    C2 --> C3[rhino-cli pre-commit]
    C3 --> C4[nx affected test:quick]

    classDef hook fill:#0072B2,color:#ffffff,stroke:#001f3f
    class B,C,C1,C2,C3,C4 hook
```

```mermaid
flowchart TD
    D[git push] --> E[pre-push hook]
    E --> E1["behavior:coverage + lint:md"]
    E1 --> E2["env + naming/vendor/bind"]
    F[PR / push to main] --> G[pr-quality-gate.yml]
    F --> H[validate-markdown.yml]
    F --> I[validate-env.yml]
    G --> G1["detect→gates→sentinel"]

    classDef hook fill:#0072B2,color:#ffffff,stroke:#001f3f
    classDef ci fill:#009E73,color:#ffffff,stroke:#003f2f
    class D,E,E1,E2 hook
    class G,H,I,G1 ci
```

### 5.2 CRON deploy pipeline shape (allowed-divergent app set, identical shape)

```mermaid
flowchart LR
    T1["cron: test-local-deploy-stag"]
    T1 --> R1[_reusable-app-stag]
    R1 --> B1[be-build-deploy-stag]
    T2["cron: test-stag"]
    T2 --> R2[_reusable-test-stag]
    T3["cron: www-deploy-prod"]
    T3 --> R3[_reusable-www-deploy]

    classDef cron fill:#E69F00,color:#000000,stroke:#5f4200
    classDef reuse fill:#56B4E9,color:#000000,stroke:#1f4f6f
    class T1,T2,T3,B1 cron
    class R1,R2,R3 reuse
```

### 5.3 Convergence phase flow

```mermaid
flowchart TD
    P0[Phase 0: baseline] --> P1[Phase 1: author standard]
    P1 --> P2[Phase 2: converge public]
    P2 --> P3[Phase 3: converge primer]
    P3 --> P4[Phase 4: converge infra]
    P4 --> P5[Phase 5: parity verify]

    classDef phase fill:#009E73,color:#ffffff,stroke:#003f2f
    class P0,P1,P2,P3,P4,P5 phase
```

## 6. File Impact

| Path (per repo)                                                                           | Change                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ----------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `docs/reference/sdlc-gate-standard.md`                                                    | **new** — the §1 standard + §3 divergence policy                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `docs/reference/rhino-cli-command-triage.md`                                              | **new** — the §2 triage table                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `.github/workflows/commons-quality-gate.yml` → `pr-quality-gate.yml`                      | rename (public only)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `.github/workflows/markdown-validate.yml` → `validate-markdown.yml`                       | rename (public only)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `.github/workflows/commons-env-validate.yml` → `validate-env.yml`                         | rename (public only)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `.github/workflows/validate-markdown.yml`                                                 | add gherkin-cardinality validator (public)                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `.github/workflows/pr-quality-gate.yml`                                                   | promote structural specs-gate set (primer); align job skeleton                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `.github/workflows/validate-env.yml`                                                      | extract standalone env workflow (primer)                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `.husky/pre-commit`                                                                       | invoke Nx `shell:check`/`dockerfiles:check`/`actions:check` (public); lock step order                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `.husky/pre-push`                                                                         | add `governance:vendor-audit-validation` scoped step (primer); invoke `harness:bindings-validation` Nx target (public+infra); lock step order                                                                                                                                                                                                                                                                                                                                                                     |
| `apps/rhino-cli/project.json`                                                             | **remove `fmt` + `format:check` targets** (formatting → lint-staged, all 3); add `shell:check`/`dockerfiles:check`/`actions:check` (public+infra); add `harness:bindings-validation` Nx target (public+infra); add structural specs targets + `test:e2e` (primer); **remove `test-coverage` target (all 3)**; rename `specs:coverage`→`specs:behavior:coverage`; primer rename `:lint`→`:check`                                                                                                                   |
| `repo-governance/development/infra/nx-targets.md`                                         | **drop `format`/`format:check` from the lifecycle list** + document the file-type lint-staged formatter map instead; add `test:coverage` + `specs:behavior:coverage` (renamed) + `specs:domain:coverage` (`*-be` only); add `shell:check`/`dockerfiles:check`/`actions:check` to `{domain}:{work}` list; encode the §1.2 mandatory-six + echo-placeholder rule, the `test:quick` composition, and the FE/BE `test:integration` rules (all 3); **remove the `test-coverage`/Codecov-algorithm references (infra)** |
| `repo-governance/development/infra/nx-target-naming.md`                                   | document the `{tool}:check` derivation + that formatting is file-type lint-staged (no `format` target) (all 3)                                                                                                                                                                                                                                                                                                                                                                                                    |
| lint-staged config (`package.json` `lint-staged` block / `.lintstagedrc`)                 | ensure one identical glob→formatter map covering every shipped file type, incl. `*.rs`→`rustfmt` and `*.fs`→`fantomas` (replacing the removed Rust/F# `fmt` targets) (all 3)                                                                                                                                                                                                                                                                                                                                      |
| **every** `apps/*/project.json` and `libs/*/project.json` (all 3 repos)                   | ensure the §1.2 mandatory-six targets are present (add `echo` placeholders where missing); **no `format` target**; set `test:quick` to the sequential typecheck→lint→test:unit composition; apply the FE/BE/`*-e2e` content rules; add a native `test:coverage` target (≥ 90% line; `echo` where `test:unit` is `echo`); rename `specs:coverage`→`specs:behavior:coverage`; add `specs:domain:coverage` on `*-be` projects                                                                                        |
| `apps/rhino-cli/src/` (+ `specs/apps/rhino/`)                                             | extend + rename `specs validate coverage`→`specs behavior-coverage validate` with the `--require-consumption` orphan-feature check; **add `specs domain-coverage validate`** (`*-be` domain-model check); **remove the `test-coverage validate` command + its specs/tests** (all 3)                                                                                                                                                                                                                               |
| `ose-infra/codecov.yml`                                                                   | **delete** (infra only — last live Codecov config; public + primer already removed)                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ose-infra governance docs + `apps/rhino-cli/README.md`                                    | scrub stale Codecov references (`codecov-upload.yml` CRON, "Codecov algorithm") from `three-level-testing-standard.md`, `ci-conventions.md`, `nx-targets.md`, `apps/rhino-cli/README.md` (infra)                                                                                                                                                                                                                                                                                                                  |
| `repo-config.yml` (root, all 3)                                                           | **new** — merged config with `instruction-size`/`env-contract`/`env-injection` sections (§1.1a)                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `instruction-size-budget.yaml` + `env-contract.yaml` + `env-injection.yaml` (root, all 3) | **delete** — folded into `repo-config.yml`                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `apps/rhino-cli/src/` config loaders + `project.json` inputs                              | repoint `convention validate instruction-size`, `env validate`/`init`/`backup`/`restore`, env-injection checker to read `repo-config.yml` sections; update Nx-target `inputs` globs from the 3 old files → `repo-config.yml` (all 3)                                                                                                                                                                                                                                                                              |

Exact per-repo cross-references to the workflow filenames and the `fmt`/`format:check` target removal (READMEs,
`repo-governance/`, CI docs, npm scripts in `package.json`) are updated alongside each change —
enumerated in [delivery.md](./delivery.md).

## 7. Rollback

Each phase is a separate set of commits on `main`. Rollback = `git revert` the phase's commits in
the affected repo. The two new reference docs are additive (safe to keep). Workflow renames are the
only potentially-disruptive change; they are verified by a no-op-change CI run inside the phase gate
before the phase is marked done.

## Open Questions

- Rows 25–27 (`harness` auto-sync): is `generate bindings` / `sync opencode` / `emit amazonq` actually invoked by `rhino-cli git pre-commit`, or only by the manual `npm run generate:bindings`? Confirm by reading the `git pre-commit` Rust source in Phase 1. `[Unverified]`
- Should any not-wired `*-audit` aggregate be wired into a periodic (e.g. weekly CRON) governance gate? Out of scope here; recorded for a follow-up.
