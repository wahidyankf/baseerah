# Delivery — Standardize rhino-cli Checks & SDLC Commands

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.

<!-- -->

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes
> (root-cause orientation — fix preexisting errors encountered during work).

<!-- -->

> **Multi-repo note**: This plan is authored in `ose-public`. Phases 0–2 execute here. Phases 3–4
> execute in `ose-primer` and `ose-infra` respectively — each begins by propagating this plan folder
> and the two reference docs into the sibling repo (per the
> [multi-repo parity workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)),
> then converging that repo in its own working tree. ose-infra is a normal repo (not bare) — commit
> to `main` directly.

## Worktree

Worktree path: `worktrees/standardize-rhino-cli-sdlc-parity/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree standardize-rhino-cli-sdlc-parity
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline (ose-public)

- [ ] [AI] Provision worktree: `claude --worktree standardize-rhino-cli-sdlc-parity` — acceptance: `worktrees/standardize-rhino-cli-sdlc-parity/` exists.
- [ ] [AI] Initialize toolchain in the root worktree: `npm install && npm run doctor -- --fix` — acceptance: doctor reports all required tools present (rust, node, shellcheck, hadolint, actionlint).
- [ ] [AI] Build rhino-cli: `npx nx build rhino-cli` — acceptance: exits 0.
- [ ] [AI] Record baseline: run `npx nx affected -t typecheck lint test:quick specs:coverage` on a clean tree — acceptance: passes (or preexisting failures noted in implementation notes).

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npx nx build rhino-cli` — exits 0.
- [ ] [AI] `git status` — clean working tree (no stray edits).

> **Pause Safety**: clean baseline recorded, no edits applied. Safe to stop. To resume: `npx nx build rhino-cli`.

---

## Phase 1: Author Standard + Triage Reference Docs + Extend Canonical Nx Naming (ose-public)

- [ ] [AI] Confirm triage rows 25–27: `grep -rn 'pre_commit\|generate.bindings\|opencode.sync\|amazonq.emit' apps/rhino-cli/src/` — acceptance: each matched line either confirms binding sync is auto-run by a hook step (→ wired) or is absent (→ not-wired); update the triage status from `[Unverified]` to wired/not-wired with the cited source file and line number in `plans/in-progress/standardize-rhino-cli-sdlc-parity/tech-docs.md`.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Cross-check the triage against the CLI surface: `cargo run -p rhino-cli -- --help` recursively (or read `apps/rhino-cli/src/cli.rs`) — acceptance: every leaf subcommand in the CLI appears exactly once in the triage table; no command is missing.
- [ ] [AI] Create `docs/reference/rhino-cli-command-triage.md` containing the [tech-docs §3 triage table](./tech-docs.md#3-rhino-cli-command-triage-wired-vs-not-wired) (every command, its description, wired/not-wired status, and invocation site), with a short intro and a "wired = invoked by lifecycle automation" definition — acceptance: file exists; `npx nx run rhino-cli:links:validation` passes for it; `npm run lint:md` passes.
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Create `docs/reference/sdlc-gate-standard.md` containing [tech-docs §7 standard](./tech-docs.md#7-target-standard-best-of-three-synthesis) + [§7.1 divergence policy](./tech-docs.md#71-divergence-policy-allowed-vs-drift) — acceptance: file exists; lint:md passes; links:validation passes.
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Add both new docs to `docs/reference/README.md` index — acceptance: both linked; `npx nx run rhino-cli:headings:hierarchy-validation` and `links:validation` pass.
- [ ] [AI] Extend the canonical Nx naming scheme: in `repo-governance/development/infra/nx-targets.md` **drop `format`/`format:check` from the lifecycle target list** (formatting is file-type lint-staged, documented separately) and add `test:coverage`, `specs:behavior:coverage` (renamed from `specs:coverage`), `specs:domain:coverage` (`*-be` only), plus document shell/Dockerfile/workflow linting as **lint-staged file-type entries** (not Nx targets) — acceptance: all changes present; `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] In `repo-governance/development/infra/nx-target-naming.md` document the **lint-staged membership rule** ([tech-docs §5](./tech-docs.md#5-nx-target-name-standard-targets-invoked-by-hooksci)): a check belongs in `lint-staged` **iff** it is file-type-based **and** per-file isolated (no cross-file content dependency); therefore **both formatting and shell/Dockerfile/workflow linting are file-type lint-staged** (no per-project `format`/`format:check` target and no `shell:lint`/`dockerfiles:lint`/`actions:lint` Nx targets — `shellcheck`/`hadolint`/`actionlint` run as lint-staged entries), while project-scoped checks (`test:quick`) and whole-tree regen (`harness:bindings-generate`) stay Nx targets, and the **`env staged-guard` is the one deliberate carve-out** (it qualifies but stays a dedicated first-line secrets gate) — acceptance: the rule + the carve-out are documented; `npx nx run rhino-cli:links:validation` passes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Encode the [§4 testing-architecture standard](./tech-docs.md#4-testing-architecture--target-contents-standard) into `repo-governance/development/infra/nx-targets.md`: the mandatory targets + `echo`-placeholder rule, the `test:specs` aggregate target (all `specs:*` validators) and the `test:quick` = typecheck→lint→`test:unit`→`test:coverage`→`test:specs` (`parallel: false`) composition (all composed targets present on every project, `echo` where N/A), the native `test:coverage` ≥ 90% gate (replacing the removed rhino-cli `test-coverage`), BE service-level / FE-DB-only `test:integration`, `*-e2e`-only `test:e2e`, the file-type-based `format` via lint-staged (no per-project `format` target), and the all-four-gates rule (pre-commit/pre-push/PR/main-ci run only `test:quick`; integration/e2e are CRON-only) — acceptance: all rules present and self-consistent with existing sections (resolve the "expose only needed targets" / no-op-anti-pattern tension explicitly); `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Author the **Git Identity Guardrail** (replaces the removed `scripts/git-identity-check.sh`): add a guardrail line to `AGENTS.md` and a short subsection to `repo-governance/development/workflow/reproducible-environments.md` (or `conventions/security/secrets-and-env-standards.md`) stating — **no AI agent sets or modifies `user.name`/`user.email` at any scope**; forbids `git config --local user.*`, the **bare** `git config user.*` (writes local by default), and `--global`/`--system` identity, and editing `[user]` in `.git/config`; identity comes from the developer's global `~/.gitconfig` (optionally `includeIf` for per-tree identity); **CI service-account/bot identity configured in workflow YAML is exempt** (e.g. `github-actions[bot]` for the PR-gate format-commit-back). Then `npm run generate:bindings` to sync `.opencode/`/`.amazonq/` — acceptance: the guardrail appears in `AGENTS.md` + the convention; `npm run lint:md` passes; bindings re-synced (no parity drift).
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] **RED**: add a feature file under `specs/apps/rhino/behavior/rhino-cli/gherkin/` for the orphan-feature check, and a unit test asserting `specs behavior-coverage validate --require-consumption` fails on an unconsumed feature — command: `npx nx run rhino-cli:test:unit` — acceptance: new test fails (flag/behaviour not yet implemented).
  - **Gherkin (binds) →** "An orphan feature file fails the gate"

    ```gherkin
    Scenario: An orphan feature file fails the gate
      Given a feature file under specs that no test references
      When rhino-cli specs behavior-coverage validate runs with --require-consumption
      Then it fails and names the orphan feature file
    ```

  - _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **RED**: add a unit test asserting `specs behavior-coverage validate --require-consumption` fails on a feature file that is consumed (its file is referenced by a test) but one of its scenarios is not exercised by any eligible unit/integration/e2e test — command: `npx nx run rhino-cli:test:unit` — acceptance: new test fails (per-scenario consumption check not yet implemented).
  - **Gherkin (binds) →** "An uncovered scenario fails the gate"

    ```gherkin
    Scenario: An uncovered scenario fails the gate
      Given a scenario in a binding feature file that no eligible unit, integration, or e2e test exercises
      When rhino-cli specs behavior-coverage validate runs with --require-consumption
      Then it fails and names the uncovered feature and scenario
    ```

  - _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **GREEN**: implement the `--require-consumption` behaviour in `specs behavior-coverage validate` (rhino-cli `src/`) — every `.feature` file **and every scenario** under the scanned binding spec dir must be exercised by ≥1 **eligible** test (unit/integration/e2e, per `@tag`); emit `orphan feature: <path> not consumed by any test` for an unconsumed file and `uncovered scenario: <feature>:<scenario> not exercised by any eligible unit/integration/e2e test` for an unbound scenario, exit non-zero otherwise — command: `npx nx run rhino-cli:test:unit` — acceptance: new test passes; `npx nx run rhino-cli:specs:coverage` still exits 0 on the current tree.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: default `--require-consumption` on for the `specs:coverage` Nx target across projects; update `specs/apps/rhino/` Gherkin + `docs/reference/sdlc-gate-standard.md` to document the new check — command: `npx nx run rhino-cli:test:quick` — acceptance: all rhino-cli tests pass; `specs:coverage` gate (soon renamed to `specs:behavior:coverage` in §1b) documents both step-def and consumption checks.

### 1b. Rename `specs validate coverage`→`behavior-coverage` + add `specs domain-coverage validate`

- [ ] [AI] **RED**: Write tests in `apps/rhino-cli/tests/` (or the relevant test module) asserting that `cargo run -- specs behavior-coverage validate` succeeds and `cargo run -- specs validate coverage` fails with "unrecognized subcommand" — command: `npx nx run rhino-cli:test:unit` — acceptance: new tests fail (rename not yet applied). _Gherkin binding exempt: this is a pure CLI rename; the underlying behavior (`--require-consumption` orphan/uncovered checks) is already bound to the Gherkin scenarios in Phase 1. No new behavior, no new Gherkin scenario required._
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Rename the CLI dispatch entry from `coverage` to `behavior-coverage` in `apps/rhino-cli/src/cli.rs` (and the application module it resolves to); update `specs/apps/rhino/**` Gherkin and all unit tests to use the new command name — command: `npx nx run rhino-cli:test:unit` — acceptance: new tests pass; no other tests broken.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: Rename the Nx target from `specs:coverage` to `specs:behavior:coverage` in `apps/rhino-cli/project.json`; update all Nx target references (hooks, workflows, `nx-targets.md`) — command: `npx nx run rhino-cli:test:quick` — acceptance: `npx nx run rhino-cli:specs:behavior:coverage` exits 0; `npx nx run rhino-cli:specs:coverage` fails with "target not found"; all tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: add a feature file + unit test asserting `specs domain-coverage validate` fails when a domain entity in `specs/apps/<domain>/domain/**` has no domain unit test — command: `npx nx run rhino-cli:test:unit` — acceptance: test fails (command not yet implemented).
  - **Gherkin (binds) →** "An uncovered domain entity fails the gate"

    ```gherkin
    Scenario: An uncovered domain entity fails the gate
      Given a *-be project with a domain entity that no domain unit test exercises
      When rhino-cli specs domain-coverage validate runs
      Then it fails and names the uncovered domain entity
    ```

  - _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **GREEN**: implement `specs domain-coverage validate` (rhino-cli `src/`) — for `*-be` projects, every entity in the bounded-context/ubiquitous-language registry under `specs/apps/<domain>/domain/**` must be exercised by ≥1 domain unit test; emit `uncovered domain entity: <name>` and exit non-zero otherwise — command: `npx nx run rhino-cli:test:unit` — acceptance: new test passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: expose the new command as the Nx target `specs:domain:coverage`, wired **only on `*-be` projects**; document it in `docs/reference/sdlc-gate-standard.md` — command: `npx nx run rhino-cli:test:quick` — acceptance: tests pass; the target resolves for `*-be` projects and is absent on others.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: add a feature file + unit test asserting `env staged-guard validate` exits non-zero and names the file when a real `.env` is staged, and exits zero when only `.env.example` is staged — command: `npx nx run rhino-cli:test:unit` — acceptance: test fails (command not yet implemented).
  - **Gherkin (binds) →** "Committing a real .env file is rejected"

    ```gherkin
    Scenario: Committing a real .env file is rejected
      Given a real .env file is staged for commit
      When the pre-commit hook runs rhino-cli env staged-guard validate
      Then it exits non-zero and names the offending file
      And the commit is aborted
    ```

  - _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **GREEN**: implement `env staged-guard validate` (rhino-cli `src/`) — port `check-no-env-staged.sh`: list `git diff --cached --name-only --diff-filter=AM`, reject any path whose basename matches `.env*` except exactly `.env.example`, emit the offending paths + the "policy: guard-env-file-access" message, exit non-zero — command: `npx nx run rhino-cli:test:unit` — acceptance: new test passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: wire it as a **direct `cargo run -- env staged-guard validate`** call in pre-commit step 1 (no Nx target — staged-set-keyed, `cache: false`); document it in `docs/reference/sdlc-gate-standard.md` (pre-commit step 1) — command: `npx nx run rhino-cli:test:quick` — acceptance: `cargo run -- env staged-guard validate` exits 0 on a clean staged tree; staging a real `.env` makes it exit non-zero.

- [ ] [AI] Commit the rhino-cli source changes: `git commit -m "feat(rhino-cli): add specs behavior-coverage --require-consumption, specs domain-coverage, and env staged-guard validate commands"` — acceptance: `git log --oneline -1` shows this commit; `npx nx run rhino-cli:test:unit` exits 0.
- [ ] [AI] Commit: `docs(reference): add rhino-cli command triage and SDLC gate standard`.
- [ ] [AI] Commit the Git Identity Guardrail: `git commit -m "docs(governance): add Git Identity Guardrail (agents never set git identity); sync bindings"` — acceptance: `git log --oneline -1` shows this commit; the guardrail is in `AGENTS.md` + the convention + synced bindings.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `npx nx run rhino-cli:links:validation` — exits 0.
- [ ] [AI] `npx nx run rhino-cli:mermaid:validation` — exits 0 (validates the plan's mermaid diagrams).
- [ ] [AI] `npm run lint:md` — exits 0.

> **Pause Safety**: standard + triage + identity guardrail are published and self-consistent; no hooks/workflows changed yet. Safe to stop. To resume: `npm run lint:md`.

---

## Phase 2: Converge ose-public to the Standard

### 2a. Standardize rhino-cli target names (remove `fmt`/`format:check`, fold tool-lint into lint-staged; binding/env validators run direct via `cargo run`, no Nx targets)

- [ ] [AI] **Remove** the `fmt` and `format:check` targets from `apps/rhino-cli/project.json` (formatting moves to file-type lint-staged, §5) — acceptance: `npx nx run rhino-cli:fmt` and `:format:check` both fail with "target not found".
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Ensure the shared lint-staged config (`package.json` `lint-staged` block / `.lintstagedrc`) matches the [§5 SSOT formatter map](./tech-docs.md#5-nx-target-name-standard-targets-invoked-by-hooksci): `*.rs`→`rustfmt`, `*.fs`→`fantomas` (so the removed Rust/F# `fmt` is replaced by file-type formatting), and **replace the wrapper scripts with direct CLIs** — `*.cs`→`dotnet csharpier format`, `*.clj`→`cljfmt fix`, `*.dart`→`dart format` — then **delete `scripts/format-{csharp,clojure,dart}.sh`** (keep only `scripts/format-elixir.sh`, since `mix format` is project-root-bound) — acceptance: staging a `*.rs` file and committing reformats it via the hook; `test ! -f scripts/format-csharp.sh && test ! -f scripts/format-clojure.sh && test ! -f scripts/format-dart.sh && test -f scripts/format-elixir.sh`; `grep -rn 'rhino-cli:fmt\b\|rhino-cli:format:check' --include='*.json' --include='*.md' --include='*.sh' --include='*.yml' .` returns zero hits.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Add the new formatters to `npm run doctor`: **CSharpier** as a local dotnet tool (`dotnet tool install --local CSharpier`, pinned in `.config/dotnet-tools.json`; v1.0+ uses the `format` subcommand) and the **cljfmt native binary** (not the Clojure-tool form, which needs an incompatible `:paths` syntax) — acceptance: after `npm run doctor -- --fix`, `dotnet csharpier --version` and `cljfmt --version` both succeed.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Add the **tool-linters to the shared lint-staged config** (`package.json` `lint-staged` / `.lintstagedrc`): `*.sh`→`shellcheck --severity=warning`, `Dockerfile`/`*.Dockerfile`→`hadolint --failure-threshold warning`, `.github/workflows/*.{yml,yaml}`→`actionlint` — **do not** add `shell:lint`/`dockerfiles:lint`/`actions:lint` Nx targets (tool-lint is file-type dispatch, not project-scoped) — acceptance: staging a `*.sh` with a quoting bug then committing aborts via shellcheck; staging a clean `*.sh` commits.
- [ ] [AI] Add the **per-file markdown + gherkin validators to the shared lint-staged config** — `*.md`→`markdownlint-cli2` (this IS the real `lint:md`, now scoped to changed files) **and** `cargo run --release -- md mermaid validate` **and** `cargo run --release -- md heading-hierarchy validate`; `*.feature`→`cargo run --release -- specs gherkin-cardinality validate` (`.feature` files only — gherkin-cardinality is unchanged, no markdown scanning). Per the [§5 membership rule](./tech-docs.md#5-nx-target-name-standard-targets-invoked-by-hooksci): per-file isolated → lint-staged; lint-staged passes the changed file paths, which these commands already accept as positional args / `--staged-only`. **Do NOT add `md links validate` here** — it is cross-file (a deleted/renamed file breaks links elsewhere) and runs repo-wide at pre-push/PR/main — acceptance: staging a `*.md` with a malformed mermaid block then committing aborts via `md mermaid validate`; staging a `*.md` with a skipped heading level aborts; staging a `*.feature` with a duplicate primary keyword aborts; a clean `*.md` commits; `grep -c 'md links validate' .lintstagedrc package.json` returns 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **Drop the standalone Nx markdown targets that the markdown workflow used** — `mermaid:validation`, `links:validation`, `headings:hierarchy-validation` move to direct `cargo run` (lint-staged for the per-file three; the `md-links` gate job for links), so the Nx wrappers are no longer the gate mechanism — acceptance: the three per-file md validators run via lint-staged; `cargo run --release -- md links validate` runs in pre-push/PR/main; the old `npm run lint:md` aggregator usage in `.husky/pre-push` is replaced by the `md links validate` direct call (§2c).
- [ ] [AI] Confirm harness binding commands run directly via cargo (no Nx target wrappers): verify both `cargo run --release -- harness bindings validate` (pre-push, read-only) and `cargo run --release -- harness bindings generate` (pre-commit, regen + auto-stage) succeed — acceptance: both `cargo run` commands exit 0; `npx nx run rhino-cli:harness:bindings-validation` and `:harness:bindings-generate` both fail with "target not found".
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Verify hooks invoke harness and env validators via direct `cargo run` (not Nx targets): `grep -cE 'rhino-cli:(harness|env:staged-guard)' .husky/pre-commit .husky/pre-push` — acceptance: grep exits 1 (no matches; no Nx target wrappers for harness or env-guard).
- [ ] [AI] Replace the `npm run harness:bindings-validation` invocation in `.husky/pre-push` with a direct `cargo run --release -- harness bindings validate` (gate-invocation rule) — acceptance: the scoped pre-push step invokes `cargo run`; it exits 0; `grep -c 'nx run rhino-cli:harness' .husky/pre-push` returns 0.

#### 2a-cov. Remove the rhino-cli `test-coverage` command + Nx target (coverage goes native)

Implements the [§5 Coverage-enforcement decision](./tech-docs.md#5-nx-target-name-standard-targets-invoked-by-hooksci) — drop the central rhino-cli coverage parser in favour of each project's native ≥ 90% gate.

- [ ] [AI] **RED**: In `apps/rhino-cli/tests/` (or the relevant integration test module), add a test asserting `cargo run -- test-coverage validate` exits non-zero with "unrecognized subcommand `test-coverage`" — command: `npx nx run rhino-cli:test:unit` — acceptance: new test fails (command still present, so the assertion that it is absent fails).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: remove the `test-coverage validate` command from `apps/rhino-cli/src/` (CLI dispatch + application module + adapter), its `specs/apps/rhino/**` Gherkin, and its unit/integration tests; delete the `test-coverage` Nx target from `apps/rhino-cli/project.json` — command: `npx nx run rhino-cli:test:quick` — acceptance: `cargo run -- test-coverage validate` exits non-zero ("unrecognized subcommand"); `jq -e '.targets|has("test-coverage")|not' apps/rhino-cli/project.json` is true; all remaining rhino-cli tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: scrub every `test-coverage` / Codecov-algorithm reference from `apps/rhino-cli/README.md` and any `repo-governance/`/docs that describe the removed command (public) — acceptance: `grep -rin 'test-coverage\|codecov' apps/rhino-cli repo-governance docs` returns only `ExcludeFromCodeCoverage`-attribute hits; `npm run lint:md` passes.

#### 2a-cfg. Merge root config files into `repo-config.yml` (§5.1)

- [ ] [AI] **RED**: add a unit test asserting the rhino-cli config loader reads the `instruction-size`/`env-contract`/`env-injection` sections from `repo-config.yml`, and errors hard when a section is missing — command: `npx nx run rhino-cli:test:unit` — acceptance: test fails (loader still reads the standalone files).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: create `repo-config.yml` at the repo root with the three namespaced sections (migrate the contents of `instruction-size-budget.yaml`, `env-contract.yaml`, `env-injection.yaml` verbatim under `instruction-size:` / `env-contract:` / `env-injection:`); update the loaders in `apps/rhino-cli/src/` (`convention validate instruction-size` — pre-§2a-names current name; renamed to `harness instruction-size validate` in §2a-names later in this phase, `env validate`/`init`/`backup`/`restore`, env-injection checker) to read `repo-config.yml` sections — command: `npx nx run rhino-cli:test:unit` — acceptance: test passes; `npx nx run rhino-cli:instruction-size:validation` and `:env:validation` exit 0 against `repo-config.yml`.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: delete the three standalone root files; repoint every Nx-target `inputs` glob and any doc/reference from them to `repo-config.yml` (keep `apps/rhino-cli/tests/fixtures/**` standalone fixtures untouched) — command: `npx nx run rhino-cli:test:quick` — acceptance: `test ! -f instruction-size-budget.yaml && test ! -f env-contract.yaml && test ! -f env-injection.yaml`; `grep -rn 'instruction-size-budget.yaml\|env-contract.yaml\|env-injection.yaml' --include='*.json' --include='*.md' . | grep -v 'tests/fixtures'` returns nothing; gates exit 0.

#### 2a-names. Standardize rhino-cli command names to verb-last (§3.1)

- [ ] [AI] Document the two naming conventions in `repo-governance/development/infra/nx-target-naming.md` (and a short CLI-command-naming note): CLI commands are `{domain} {sub-domain…} {verb}` (verb last); Nx targets are `:`-separated `{domain}:{work}`/lifecycle — acceptance: both conventions documented with examples; `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] **RED**: add a test asserting the verb-last invocation works and the old form fails, for a representative sample covering both transforms — verb-reorder within domain (`convention emoji validate`, `harness opencode sync`, `repo-governance vendor validate`) **and** the two cross-domain relocations (`harness instruction-size validate`, was `convention validate instruction-size`; `repo-governance workflows naming validate`, was `workflows validate naming`) — command: `npx nx run rhino-cli:test:unit` — acceptance: test fails (commands still verb-middle / in their old domain).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: rename every rhino-cli leaf command in `apps/rhino-cli/src/cli.rs` (+ dispatch) to the verb-last **target** form in the [§3 triage table](./tech-docs.md#3-rhino-cli-command-triage-wired-vs-not-wired) (`{domain} {noun…} {verb}`); **two leaves also change top-level domain, not just verb position** — `convention validate instruction-size` → `harness instruction-size validate` (instruction surfaces are harness-loaded) and `workflows validate naming` → `repo-governance workflows naming validate` (workflow docs live in `repo-governance/workflows/`); drop the `(alias)` shortcuts (rows 13–14) in favour of canonical verb-last; keep Nx target names (`:`-separated) unchanged — command: `npx nx run rhino-cli:test:unit` — acceptance: every CLI command matches its triage target column; `cargo run -- --help` recursively shows verb-last leaves; tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: update every reference to a renamed command (Nx-target `command:` strings in `project.json`, `.husky/*`, `.github/workflows/*`, `package.json` scripts, `docs/`, `repo-governance/`, `specs/apps/rhino/**`) — command: `grep -rn -E '\b(validate|sync|emit|generate|clean|scaffold) [a-z][a-z-]*' --include='*.json' --include='*.yml' --include='*.sh' --include='*.md' . | grep -v 'rhino-cli-command-triage\.md\|standardize-rhino-cli-sdlc-parity/tech-docs\.md'` returns no verb-middle forms (the two triage docs deliberately preserve old forms in their "current" column and are excluded); `npx nx run rhino-cli:test:quick` and `npm run lint:md` pass.

### 2b. Rewire pre-commit (lint-staged tool-lint + `harness:bindings-generate`, drop `test:quick`); drop the git-identity guard

- [ ] [AI] Edit `.husky/pre-commit`: **delete the inline `shellcheck` / `hadolint` / `actionlint` blocks** (they are now lint-staged entries added in §2a) — acceptance: `grep -cE 'shellcheck|hadolint|actionlint' .husky/pre-commit` returns 0.
- [ ] [AI] Edit `.husky/pre-commit`: **replace the inline `./scripts/check-no-env-staged.sh` line with a direct `cargo run --release -- env staged-guard validate`** as pre-commit step 1 — acceptance: `grep -c 'env staged-guard validate' .husky/pre-commit` returns ≥ 1; `grep -c 'check-no-env-staged' .husky/pre-commit` returns 0.
- [ ] [AI] Edit `.husky/pre-commit`: **replace the opaque `rhino-cli git pre-commit` call with a direct `cargo run --release -- harness bindings generate`** as pre-commit step 3 — acceptance: `grep -c 'harness bindings generate' .husky/pre-commit` returns ≥ 1; `grep -c 'git pre-commit' .husky/pre-commit` returns 0.
- [ ] [AI] Edit `.husky/pre-commit`: **remove the `nx affected -t test:quick` line entirely** (it moves to pre-push only — pre-commit must stay fast) — acceptance: `grep -c 'test:quick' .husky/pre-commit` returns 0; `bash .husky/pre-commit` on a staged no-op runs without error; step order matches [tech-docs §1](./tech-docs.md#1-lifecycle-stage--exact-commands-post-implementation-identical-across-3-repos) (env-staged-guard → lint-staged [format + tool-lint] → harness:bindings-generate).
- [ ] [AI] **Delete the converted shell guard**: `git rm scripts/check-no-env-staged.sh` (its logic now lives in the rhino-cli `env staged-guard validate` command, added in Phase 1) — acceptance: `test ! -f scripts/check-no-env-staged.sh`; `grep -c check-no-env-staged .husky/pre-commit` returns 0.
- [ ] [AI] **Remove the git-identity guard**: delete the `./scripts/git-identity-check.sh` line from `.husky/pre-commit` and `git rm scripts/git-identity-check.sh` — acceptance: `test ! -f scripts/git-identity-check.sh`; `grep -c git-identity-check .husky/pre-commit` returns 0; the [Git Identity Guardrail](./tech-docs.md#1-lifecycle-stage--exact-commands-post-implementation-identical-across-3-repos) replaces it.

### 2c. Rename PR/env workflow files; delete the markdown workflow; fix all references

- [ ] [AI] `git mv .github/workflows/commons-quality-gate.yml .github/workflows/pr-quality-gate.yml` — acceptance: file moved; `git status` shows a rename.
- [ ] [AI] **Delete the markdown workflow**: `git rm .github/workflows/markdown-validate.yml` — its three validators (mermaid, links, heading-hierarchy) now run via lint-staged (the per-file two) + the `md-links` repo-wide gate job (§2a, §2d); nothing unique remains in a standalone workflow — acceptance: `test ! -f .github/workflows/markdown-validate.yml`; `grep -rn 'mermaid:validation\|links:validation\|headings:hierarchy-validation' .github/workflows/` returns 0 (the Nx-target steps are gone).
- [ ] [AI] `git mv .github/workflows/commons-env-validate.yml .github/workflows/validate-env.yml` — acceptance: rename shown.
- [ ] [AI] Update the `name:` field inside each renamed workflow to match its new role — acceptance: `actionlint` passes on all three.
- [ ] [AI] Grep for old filenames repo-wide and update every reference: `grep -rn 'commons-quality-gate\|markdown-validate\|commons-env-validate' --include='*.md' --include='*.yml' .` — acceptance: zero hits remain except in this plan's drift catalog; `.github/workflows/README.md`, `repo-governance/development/quality/*.md`, and root `AGENTS.md`/`CLAUDE.md` updated as needed.
  - _Suggested executor: `repo-rules-fixer`_

### 2d. Wire the `md-links` repo-wide gate job + pre-push link check

- [ ] [AI] Edit `.husky/pre-push`: replace the `npm run lint:md` line with a direct `cargo run --release -- md links validate --exclude plans/done --exclude apps/ayokoding-www/content --exclude apps/ose-www/content` (the cross-file validator; the per-file md validators already run at pre-commit via lint-staged) — acceptance: `grep -c 'md links validate' .husky/pre-push` returns ≥ 1; `grep -c 'lint:md' .husky/pre-push` returns 0.
- [ ] [AI] Add the `md-links` job to `.github/workflows/pr-quality-gate.yml` and `.github/workflows/main-ci.yml`: a job running `cargo run --release -- md links validate --exclude …` repo-wide (NOT `--diff` — a deleted/renamed file breaks links in untouched files) — acceptance: `actionlint` passes; both workflows contain an `md-links` job invoking `md links validate`; `grep -rn 'mermaid:validation\|links:validation\|headings:hierarchy-validation\|gherkin-cardinality-validation' .github/workflows/pr-quality-gate.yml .github/workflows/main-ci.yml` returns 0 (per-file md + gherkin run via lint-staged, not as Nx-target CI steps).
- [ ] [AI] Confirm the PR gate's `lint-staged --diff` job and the main gate's lint-staged-equiv job carry the per-file md validators + gherkin (from §2a) so `(pre-commit ∪ pre-push) == PR == main` holds for markdown — acceptance: the PR `lint-staged` job runs the same `.lintstagedrc` as the commit hook; `main-ci.yml` runs `markdownlint-cli2 "**/*.md"` + `md mermaid validate` + `md heading-hierarchy validate` (all files) + `specs gherkin-cardinality validate` (all `.feature`).

### 2e. Apply the testing-architecture target contents to every project (ose-public)

- [ ] [AI] Enumerate projects: `npx nx show projects` — acceptance: the list matches the rows of the [§2.1 per-project target matrix](./tech-docs.md#21-per-project-target-matrix-post-implementation-ose-public); reconcile any new/removed project against the matrix before converging.
- [ ] [AI] For EACH project's `project.json`, ensure the [§4 mandatory-six targets](./tech-docs.md#4-testing-architecture--target-contents-standard) exist — add `echo` placeholders for any missing among `test:unit`, `test:integration`, `test:e2e`, `test:quick`, `lint`, `typecheck` (**no `format` target** — formatting is lint-staged) — acceptance: `npx nx show project <p> --json | jq '.targets|keys'` includes all six for every project; `npx nx affected -t typecheck lint test:unit test:integration test:e2e test:quick` resolves a task (real or echo) for every affected project.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Add a `test:specs` target to every project: an aggregate (`nx:run-commands` or `dependsOn`) of the project's `specs:*` validators — `specs:adoption-validation`, `specs:tree-validation`, `specs:counts-validation`, `specs:links-validation`, `specs:behavior:coverage`, and `specs:domain:coverage` (`*-be` only; `echo`/skip elsewhere) — acceptance: `npx nx show project <p> --json | jq -e '.targets|has("test:specs")'` is true for every project; `npx nx run <be>:test:specs` runs all six specs validators; a non-`*-be` project's `test:specs` runs the four structural + behavior:coverage (no domain:coverage).
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Set every project's `test:quick` to the sequential composition (`nx:run-commands`, `"parallel": false`, commands `nx run <p>:typecheck` → `nx run <p>:lint` → `nx run <p>:test:unit` → `nx run <p>:test:coverage` → `nx run <p>:test:specs`) — acceptance: running `test:quick` executes the five in order and stops at the first failure (verify by temporarily breaking lint in one project); the former separate specs-structural gate step is removed from `.husky/pre-push` and the PR/main workflows (the specs gate now runs inside `test:quick`).
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Apply the content rules: `test:e2e` real only on `*-e2e` projects (echo elsewhere); BE `test:integration` is service-level (no HTTP); FE `test:integration` is echo unless DB-backed (keep `organiclever-app-web`'s PGlite integration real); `test:unit` includes BDD + non-BDD (coverage gated by the sibling `test:coverage` target, not here) — acceptance: `npx nx show project organiclever-www --json | jq -r '.targets["test:integration"].options.command // ""' | grep -q "echo"` (FE-without-DB → echo); `npx nx show project organiclever-app-web --json | jq -e '.targets["test:integration"]'` is a non-echo real test target (PGlite integration remains real); `npx nx show project organiclever-be-e2e --json | jq -r '.targets["test:e2e"].options.command // ""' | grep -vq "echo"` (e2e runner → real command); `npx nx show project organiclever-be --json | jq -r '.targets["test:e2e"].options.command // ""' | grep -q "echo"` (BE non-e2e project → echo).
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [HUMAN] Confirm `organiclever-be:test:integration` invokes only service/repository functions (no HTTP client in test code): `grep -rn 'axios\|node-fetch\|got\|supertest\|HttpClient' apps/organiclever-be/tests/` — acceptance: returns 0 hits (pure service-level, no HTTP imports). Observable resume signal: zero grep hits; verify before proceeding.
- [ ] [AI] For EACH project with a real `test:unit`, add a native `test:coverage` target (≥ 90% line via the project's own runner — `vitest --coverage` thresholds, `cargo llvm-cov`/`tarpaulin`, `dotnet test` coverage gate) per the [§2.1 matrix](./tech-docs.md#21-per-project-target-matrix-post-implementation-ose-public) `test:coverage` column; `echo` where `test:unit` is `echo` — acceptance: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:coverage")' >/dev/null || echo "NO-COV: $p"; done` prints no `NO-COV`; a project under 90% fails its `test:coverage`.
- [ ] [AI] Research the correct Nx mechanism to wire `specs/` folders into the project affected graph: query `nx_docs` with "how to mark a project affected by changes outside its root (inputs namedInputs implicitDependencies)" — acceptance: the doc link + chosen mechanism snippet (one of `implicitDependencies`, `inputs`/`namedInputs`, or a project-inference plugin) is recorded in `plans/in-progress/standardize-rhino-cli-sdlc-parity/tech-docs.md §4` before any per-project edit is made.
- [ ] [AI] **Wire `specs/` into Nx `affected`**: for each project, map its `specs/apps/<domain>/**` (or `specs/libs/<lib>/**`) folder to the project so a feature-only change marks it affected — apply the mechanism confirmed in the research step above (`implicitDependencies` / `inputs`/`namedInputs`) to the project's Nx config — acceptance (**behavioural**): editing only a project's `.feature` file then `npx nx affected -t test:quick --base=HEAD~1 --head=HEAD` includes that project (so `specs:behavior:coverage`/`specs:domain:coverage` actually run on specs-only changes).
- [ ] [AI] Wire `specs:domain:coverage` (→ `rhino-cli specs domain-coverage validate`) **only on `*-be` backend projects** (`ose-be`, `organiclever-be`) per the §2.1 matrix `specs:domain:coverage` column — acceptance: `npx nx show project ose-be --json | jq -e '.targets|has("specs:domain:coverage")'` is true; a non-`*-be` project (e.g. `ose-www`) does **not** declare it.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Make pre-push and the PR quality gate run the identical per-project code command — `nx affected -t test:quick` — and confirm **neither** runs `test:integration`/`test:e2e`; those stay only on the CRON pipelines. Edit `.husky/pre-push` and `.github/workflows/pr-quality-gate.yml` accordingly per [§4 gate rule](./tech-docs.md#4-testing-architecture--target-contents-standard) — acceptance: `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation; both run `test:quick`.
- [ ] [AI] Run the extended `specs:behavior:coverage` (`--require-consumption`) across affected projects and fix any orphan feature files (add the missing consuming test, or remove the dead feature with justification) — command: `npx nx affected -t specs:behavior:coverage` — acceptance: exits 0 with no orphan-feature errors.

### 2f. Post-merge main CI (fast); heavy tests + deploy stay CRON-only (ose-public)

Implements the [§6 standard](./tech-docs.md#6-post-merge-main-ci--per-project-staging-deploy).

- [ ] [AI] Add a new `.github/workflows/main-ci.yml` triggered on `push: branches: [main]` that runs the **same check set as the PR gate, but across _all_ projects** (not affected): `nx run-many --all -t test:quick` + the lint-staged-equiv pass over all files (tool-lint + per-file md validators `markdownlint-cli2`/`md mermaid validate`/`md heading-hierarchy validate` + `specs gherkin-cardinality validate` on `.feature`) + `md links validate` (repo-wide) + `env validate` + structural specs across all projects + the full governance validator set, all jobs **in parallel** — acceptance: `actionlint` passes; `grep -nc 'run-many --all' .github/workflows/main-ci.yml` returns ≥ 1 and `grep -c 'nx affected' .github/workflows/main-ci.yml` returns 0; a merge to main runs test:quick + validators across every project; `grep -n 'test:integration\|test:e2e' .github/workflows/main-ci.yml` returns nothing.
- [ ] [AI] Leave the heavy levels + deploy **CRON-only**: the scheduled `*-test-local-deploy-stag.yml` (full suite `test:quick`+`test:integration`+`test:e2e` per app → staging deploy on green) and `*-test-stag.yml` → deploy-prod remain the **sole** place integration/e2e run; no gate touches them — acceptance: `grep -rln 'test:integration\|test:e2e' .github/workflows/*-test-local-deploy-stag.yml` lists those CRON files; the four gate surfaces list none.
- [ ] [AI] Confirm the gate scope split per the [gate-composition rule](./tech-docs.md#1-lifecycle-stage--exact-commands-post-implementation-identical-across-3-repos): **pre-push + PR gate run `test:quick` for _affected_ projects** (`nx affected`); **main-ci runs the same set for _all_ projects** (`nx run-many --all`); **pre-commit runs the fast file-type set only — no `test:quick`**; and **no gate runs integration/e2e** — acceptance: `grep -n 'test:integration\|test:e2e' .husky/pre-commit .husky/pre-push .github/workflows/pr-quality-gate.yml .github/workflows/main-ci.yml` returns no gate invocation; `grep -c 'nx affected' .github/workflows/pr-quality-gate.yml` returns ≥ 1; `grep -c 'run-many --all' .github/workflows/main-ci.yml` returns ≥ 1; `grep -c 'test:quick' .husky/pre-commit` returns 0; `grep -c 'test:quick' .husky/pre-push` returns ≥ 1.

- [ ] [AI] Commit rhino-cli target-name standardization: `git commit -m "chore(rhino-cli): standardize Nx target names (remove fmt/format:check, fold tool-lint into lint-staged, add bindings targets, remove test-coverage, rename specs:coverage to specs:behavior:coverage, add specs:domain:coverage)"` — acceptance: `git log --oneline -1` shows this commit; `npx nx run rhino-cli:harness:bindings-validation` exits 0; `npx nx run rhino-cli:fmt` fails with "target not found".
- [ ] [AI] Commit lint-staged formatter map: `git commit -m "chore(config): finalize file-type lint-staged map (add *.rs/*.fs; replace format-{csharp,clojure,dart}.sh wrappers with dotnet csharpier/cljfmt/dart format; add CSharpier+cljfmt to doctor)"` — acceptance: `git log --oneline -1` shows this commit; staging a `*.rs` file and running pre-commit reformats it via rustfmt; the three deleted wrapper scripts are absent.
- [ ] [AI] Commit repo-config.yml merge: `git commit -m "chore(config): merge instruction-size-budget.yaml, env-contract.yaml, env-injection.yaml into repo-config.yml"` — acceptance: `git log --oneline -1` shows this commit; `test ! -f instruction-size-budget.yaml && test ! -f env-contract.yaml && test ! -f env-injection.yaml` passes.
- [ ] [AI] Commit hook rewire + identity-guard removal: `git commit -m "chore(hooks): rewire pre-commit (lint-staged tool-lint + bindings-generate, drop test:quick); remove git-identity-check guard (replaced by AGENTS.md Git Identity Guardrail)"` — acceptance: `git log --oneline -1` shows this commit; `bash .husky/pre-commit` on a staged no-op exits 0; `test ! -f scripts/git-identity-check.sh`.
- [ ] [AI] Commit workflow renames + ref updates: `git commit -m "chore(ci): rename workflow files to canonical names (pr-quality-gate, validate-env); delete markdown-validate (folds into gates)"` — acceptance: `git log --oneline -1` shows this commit; `test -f .github/workflows/pr-quality-gate.yml && test -f .github/workflows/validate-env.yml && test ! -f .github/workflows/markdown-validate.yml` passes.
- [ ] [AI] Commit markdown-into-gates: `git commit -m "chore(ci): fold markdown validation into the gates (per-file md validators + gherkin in lint-staged, md-links repo-wide job); drop npm run lint:md"` — acceptance: `git log --oneline -1` shows this commit; `grep -c 'md links validate' .husky/pre-push` returns ≥ 1; `actionlint .github/workflows/pr-quality-gate.yml .github/workflows/main-ci.yml` exits 0.
- [ ] [AI] Commit per-project target-contents: `git commit -m "chore(nx): add mandatory-six targets + test:quick sequential composition + native test:coverage to all projects"` — acceptance: `git log --oneline -1` shows this commit; the mandatory-six `jq` check (Phase 2 gate) prints no `MISSING` line.
- [ ] [AI] Commit gate rule (test:quick-only for pre-push + PR gate): `git commit -m "chore(ci): restrict pre-push and PR gate to test:quick; integration/e2e reserved for CRON"` — acceptance: `git log --oneline -1` shows this commit; `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation.
- [ ] [AI] Commit post-merge main-ci (fast gate): `git commit -m "chore(ci): add main-ci.yml running the fast gate (test:quick + validators) on push to main; heavy tests stay CRON-only"` — acceptance: `git log --oneline -1` shows this commit; `actionlint .github/workflows/main-ci.yml` exits 0.
- [ ] [AI] Push to `origin main`; monitor GitHub Actions; verify `pr-quality-gate.yml`, `validate-env.yml`, `main-ci.yml` all run green (and `markdown-validate.yml` is gone) — acceptance: all CI checks pass; the markdown checks run inside the PR gate (lint-staged job + `md-links` job).

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `npx nx affected -t test:quick` — exits 0 (fix any preexisting failures).
- [ ] [AI] `cargo run -- harness bindings validate` exits 0, `cargo run -- harness bindings generate` regenerates+stages bindings, and `cargo run -- env staged-guard validate` rejects a staged real `.env`; `npx nx run rhino-cli:fmt`, `:format:check`, `:shell:lint`, `:dockerfiles:lint`, `:actions:lint`, `:harness:bindings-generate`, `:harness:bindings-validation`, `:env:staged-guard-validation` all fail (not targets); staging a `*.sh` with a shellcheck warning aborts the commit (lint-staged tool-lint).
- [ ] [AI] Every project exposes the mandatory-six targets: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:unit") and has("test:integration") and has("test:e2e") and has("test:quick") and has("lint") and has("typecheck")' >/dev/null || echo "MISSING: $p"; done` — acceptance: prints no `MISSING` line.
- [ ] [AI] Coverage went native: `jq -e '.targets|has("test-coverage")|not' apps/rhino-cli/project.json` is true; every project with a real `test:unit` also exposes `test:coverage`; `grep -rin 'test-coverage\|codecov' apps repo-governance docs --include='*.md' --include='*.json' --include='*.rs' | grep -vi 'ExcludeFromCodeCoverage'` returns nothing — acceptance: no stale `test-coverage`/Codecov references remain in ose-public.
- [ ] [AI] `actionlint .github/workflows/main-ci.yml` — exits 0; it runs the check set across **all** projects (`nx run-many --all -t test:quick` + repo-wide validators), `grep -c 'run-many --all' .github/workflows/main-ci.yml` returns ≥ 1, and `grep -n 'test:integration\|test:e2e' .github/workflows/main-ci.yml` returns nothing.
- [ ] [AI] `npm run lint:md` — exits 0.

> **Pause Safety**: ose-public is fully converged and green on CI. Safe to stop. To resume: `npx nx affected -t lint`.

---

## Phase 3: Propagate + Converge ose-primer

> Executes in the `ose-primer` repo (`/Users/wkf/ose-projects/ose-primer`). Target state = the
> [§2.2 primer matrix](./tech-docs.md#22-per-project-target-matrix-post-implementation-ose-primer).
> Use primer's own worktree; commit to its `main`.

### 3a. Baseline + propagate

- [ ] [AI] Provision primer worktree + toolchain: `npm install && npm run doctor -- --fix` in ose-primer; `npx nx build rhino-cli` — acceptance: doctor green; rhino-cli builds.
- [ ] [AI] Record primer baseline: `npx nx run-many -t typecheck lint test:quick specs:coverage` — acceptance: pass, or preexisting failures noted.
- [ ] [AI] Propagate the artifacts: copy `plans/in-progress/standardize-rhino-cli-sdlc-parity/`, `docs/reference/rhino-cli-command-triage.md`, `docs/reference/sdlc-gate-standard.md`, and the `nx-targets.md`/`nx-target-naming.md` additions into ose-primer; replace the §2.1 matrix with the §2.2 primer matrix; adjust triage/standard for primer's app+language set per the divergence policy — acceptance: artifacts exist; `npm run lint:md` passes.
- [ ] [AI] Apply the same rhino-cli source changes to primer (propagated rhino-cli): merge root configs into `repo-config.yml` + delete the 3 standalone files (§2a-cfg); ensure the lint-staged map covers `*.rs`/`*.fs` (§2a) — acceptance: `test -f repo-config.yml`; the 3 old files absent; `npx nx run rhino-cli:instruction-size:validation`/`:env:validation` exit 0.

### 3b. Standardize rhino-cli target names

- [ ] [AI] In primer `apps/rhino-cli/project.json`: **remove `fmt` + `format:check` targets** (formatting → lint-staged); **drop primer's `shell:lint`/`dockerfiles:lint`/`actions:lint` targets** (tool-lint folds into the lint-staged config — add the three linter entries there if missing); ensure `harness:bindings-generate` + the env-guard run as **direct `cargo run` calls** (no Nx targets — parity with public); update every `fmt`/`format:check` reference (`grep -rn 'rhino-cli:fmt\b\|rhino-cli:format:check' --include='*.json' --include='*.yml' --include='*.sh' --include='*.md' .`) — acceptance: `cargo run -- harness bindings generate` regenerates bindings; `:fmt`/`:format:check`/`:shell:lint`/`:dockerfiles:lint`/`:actions:lint`/`:harness:bindings-generate` all fail (not targets); staging a bad `*.sh` aborts the commit; zero stale `fmt`/`format:check` references.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Add the structural targets primer's rhino-cli is **missing** so its key set matches public/infra: `specs:adoption-validation`, `specs:counts-validation`, `specs:links-validation`, `specs:tree-validation`, `test:e2e` (echo) — and mirror public's coverage decision: **do not** add `test-coverage` (it is removed everywhere); add a native `test:coverage` target instead — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json | sort` equals public's sorted key set (which contains `test:coverage`, not `test-coverage`).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Rename `specs:coverage`→`specs:behavior:coverage` in `ose-primer/apps/rhino-cli/project.json` (propagated from Phase 1b ose-public rename) — acceptance: `jq -e '.targets|has("specs:behavior:coverage") and (has("specs:coverage")|not)' apps/rhino-cli/project.json` is true; `npx nx run rhino-cli:specs:coverage` fails with "target not found"; `npx nx run rhino-cli:specs:behavior:coverage` exits 0.
  - _Suggested executor: `swe-rust-dev`_

### 3c. Hook + workflow parity

- [ ] [AI] Rewire primer's `.husky/pre-commit`: **fold the shell/Dockerfile/workflow linters into lint-staged** (delete any inline blocks), wire a **direct `cargo run -- env staged-guard validate`** (step 1, replacing the inline `check-no-env-staged.sh`), **replace any `git pre-commit` call with a direct `cargo run -- harness bindings generate`**, **remove the `test:quick` line** (moves to pre-push), **and remove the git-identity guard**: `git rm scripts/check-no-env-staged.sh scripts/git-identity-check.sh` and drop both lines from the hook — acceptance: `test ! -f scripts/check-no-env-staged.sh && test ! -f scripts/git-identity-check.sh`; `grep -cE 'git-identity-check|check-no-env-staged' .husky/pre-commit` returns 0; `grep -c 'test:quick' .husky/pre-commit` returns 0; step order matches [§1](./tech-docs.md#1-lifecycle-stage--exact-commands-post-implementation-identical-across-3-repos). (The `env staged-guard validate` command + the Git Identity Guardrail in `AGENTS.md`, authored in Phase 1, propagate with the plan folder.)
- [ ] [AI] Add a scoped `cargo run --release -- repo-governance vendor validate` step to primer's `.husky/pre-push` (gated on `^repo-governance/.*\.md$`) — acceptance: editing a `repo-governance/*.md` then running pre-push triggers it; exits 0.
- [ ] [AI] Promote primer's deferred structural specs-gate in `.github/workflows/pr-quality-gate.yml` to the full set (`specs:adoption-validation` + `specs:tree-validation` + `specs:counts-validation` + `specs:links-validation` + `specs:behavior:coverage` + `specs:gherkin-cardinality-validation`) — acceptance: `actionlint` passes; job lists all six.
- [ ] [AI] Extract a standalone `.github/workflows/validate-env.yml` from primer's folded-in PR-gate env job (`npx nx run rhino-cli:env:validation` on `pull_request` + `push:main`); remove the duplicated env logic from the PR gate — acceptance: `actionlint` passes; `validate-env.yml` matches the public/infra shape.
- [ ] [AI] **Delete primer's `validate-markdown.yml`** and fold markdown into the gates (parity with public): per-file md validators + `specs gherkin-cardinality validate` (`.feature`) in lint-staged; `md links validate` as the `md-links` job in `pr-quality-gate.yml`/`main-ci.yml` — acceptance: `test ! -f .github/workflows/validate-markdown.yml`; primer's `.lintstagedrc`/`package.json` carries the per-file md validators; the `md-links` job runs `md links validate`.
- [ ] [AI] Align primer's PR-gate job skeleton to the standard (detect, language gates, markdown, naming, env, specs-gate, quality-gate sentinel; formatting is enforced by lint-staged at commit, not a PR-gate job); **keep** primer's per-language jobs (golang/jvm/dotnet/python/rust/elixir/clojure/dart — allowed divergence) — acceptance: `actionlint` passes; skeleton matches, language jobs preserved.

### 3d. Mandatory-six sweep across all 26 primer projects

- [ ] [AI] For EACH primer project, bring its `project.json` to the [§2.2 matrix](./tech-docs.md#22-per-project-target-matrix-post-implementation-ose-primer) — biggest gaps: add `test:e2e` (echo) to the 11 `crud-be-*` + `crud-fs-ts-nextjs`; add `test:integration`+`test:e2e` (echo) to `crud-fe-*`; fill the support libs (`ts-ui-tokens` needs 4: `test:unit`/`test:integration`/`test:e2e` echo + `test:quick`; `golang-commons`/`clojure-openapi-codegen` need `typecheck` echo + more; `elixir-*` + `ts-ui` need `test:integration`/`test:e2e` echo); add `specs:behavior:coverage` to libs lacking it; add `specs:domain:coverage` to the 11 `crud-be-*` backends (**no `format` target anywhere** — lint-staged handles it) — acceptance: the mandatory-six `jq` check (Phase 2 gate) prints no `MISSING` for any primer project.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Add a `test:specs` target to every primer project (aggregate of its `specs:*` validators, `*-be` adds domain:coverage) and set every primer project's `test:quick` to the sequential typecheck→lint→`test:unit`→`test:coverage`→`test:specs` composition (`nx:run-commands`, `parallel:false`; `test:unit` + `test:coverage` + `test:specs` present everywhere, echo where N/A) — acceptance: `test:specs` present on every project; order verified by breaking lint in one project; no separate specs-structural step in primer's hooks/workflows.
- [ ] [AI] Add a native `test:coverage` target (≥ 90% line via each project's own runner; `echo` where `test:unit` is `echo`) to every primer project per the [§2.2 matrix](./tech-docs.md#22-per-project-target-matrix-post-implementation-ose-primer) `test:coverage` column — acceptance: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:coverage")' >/dev/null || echo "NO-COV: $p"; done` prints no `NO-COV`.
- [ ] [AI] Wire `specs:domain:coverage` on the 11 `crud-be-*` backend projects (per §2.2 matrix) — acceptance: `npx nx show project crud-be-rust-axum --json | jq -e '.targets|has("specs:domain:coverage")'` is true; `crud-fe-*`/libs do **not** declare it.
- [ ] [AI] Resolve orphan features: `npx nx run-many -t specs:behavior:coverage` with `--require-consumption` — acceptance: no orphan-feature errors.

### 3e. Post-merge main CI (fast); heavy tests CRON-only (primer is a template — deploy is a no-op)

- [ ] [AI] Add `.github/workflows/main-ci.yml` mirroring public's main gate — the PR check set across **all** projects: `nx run-many --all -t test:quick` + the repo-wide validators on push to main — acceptance: `actionlint` passes; `grep -c 'run-many --all' .github/workflows/main-ci.yml` returns ≥ 1 and `grep -c 'nx affected' .github/workflows/main-ci.yml` returns 0; `grep -n 'test:integration\|test:e2e' .github/workflows/main-ci.yml` returns nothing.
- [ ] [AI] Keep heavy tests CRON-only and document that primer's deploy leg is a **no-op** (the `crud-*` demo apps have no live staging env — they are reference scaffolding); the `test-and-deploy-*-development` local-stack workflows remain the scheduled full-suite (quick+int+e2e) harness — acceptance: `docs/reference/sdlc-gate-standard.md` in primer states the no-deploy rationale; integration/e2e run only in those scheduled workflows.
- [ ] [AI] Confirm the primer gate scope split: pre-push + PR run `test:quick` for **affected** (`nx affected`); main-ci runs it for **all** projects (`nx run-many --all`); pre-commit runs the fast file-type set only (no `test:quick`); no gate runs integration/e2e — acceptance: `grep -n 'test:integration\|test:e2e' .husky/pre-commit .husky/pre-push .github/workflows/pr-quality-gate.yml .github/workflows/main-ci.yml` returns no gate invocation; `grep -c 'nx affected' .github/workflows/pr-quality-gate.yml` returns ≥ 1; `grep -c 'run-many --all' .github/workflows/main-ci.yml` returns ≥ 1; `grep -c 'test:quick' .husky/pre-commit` returns 0.

- [ ] [AI] Commit propagated artifacts + config merge: `git commit -m "chore(config): propagate standardize-rhino-cli-sdlc-parity plan artifacts and merge repo-config.yml into ose-primer"` — acceptance: `git log --oneline -1` shows this commit; `test -f repo-config.yml` passes.
- [ ] [AI] Commit rhino-cli target-name standardization: `git commit -m "chore(rhino-cli): standardize Nx target names in ose-primer (remove fmt/format:check, add bindings targets, rename specs:coverage to specs:behavior:coverage)"` — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json | sort` equals public's sorted key set.
- [ ] [AI] Commit hook + workflow parity: `git commit -m "chore(ci): align primer hooks and workflows to canonical standard (validate-env.yml, full specs-gate, governance-vendor in pre-push)"` — acceptance: `actionlint` passes; `grep -n 'test:integration\|test:e2e' .husky/pre-push .github/workflows/pr-quality-gate.yml` returns no gate invocation.
- [ ] [AI] Commit mandatory-six sweep: `git commit -m "chore(nx): add mandatory-six targets + sequential test:quick + native test:coverage + specs:domain:coverage to all 26 primer projects"` — acceptance: mandatory-six `jq` check prints no `MISSING`; no `NO-COV` project.
- [ ] [AI] Commit post-merge CI: `git commit -m "chore(ci): add main-ci.yml fast gate (test:quick + validators) to ose-primer (template — heavy tests CRON-only, no deploy leg)"` — acceptance: `actionlint .github/workflows/main-ci.yml` exits 0.
- [ ] [AI] Push ose-primer to `origin main` and poll CI: `gh run view --json status,conclusion` every 2 min until complete — acceptance: all checks green (incl. new `validate-env.yml`, promoted specs-gate, `main-ci.yml`).

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] In ose-primer: `npx nx run-many -t test:quick` — exits 0.
- [ ] [AI] In ose-primer, every project exposes the mandatory-six: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:unit") and has("test:integration") and has("test:e2e") and has("test:quick") and has("lint") and has("typecheck")' >/dev/null || echo "MISSING: $p"; done` — acceptance: prints no `MISSING` line.
- [ ] [AI] In ose-primer: `npm run lint:md` and `actionlint` on changed workflows — exit 0.

> **Pause Safety**: ose-public + ose-primer converged and green. Safe to stop. To resume (primer): `npx nx affected -t lint`.

---

## Phase 4: Propagate + Converge ose-infra

> Executes in `ose-infra` (normal repo; commit to `main` directly). Target state =
> the [§2.3 infra matrix](./tech-docs.md#23-per-project-target-matrix-post-implementation-ose-infra).
> Infra already matches the workflow filenames (`pr-quality-gate.yml`, `validate-env.yml`; its
> `validate-markdown.yml` is **deleted** like the others — markdown folds into the gates) +
> governance-vendor pre-push, but (like public) lacks the
> any `harness:*` Nx targets (the gates run the binding/env validators as direct `cargo run` calls), folds shell/docker/actions
> lint into lint-staged (no tool-lint Nx targets), and still has the obsolete `fmt`/`format:check`
> targets (to be removed → lint-staged). CI runs on the self-hosted runner.

### 4a. Baseline + propagate

- [ ] [AI] In ose-infra: `npm install && npm run doctor -- --fix`; `npx nx build rhino-cli` — acceptance: doctor green; rhino-cli builds.
- [ ] [AI] Propagate the artifacts + the `nx-targets.md`/`nx-target-naming.md` additions into ose-infra; replace the matrix with the §2.3 infra matrix; document infra-only IaC gates (terraform/ansible/yamllint) and the self-hosted runner in the divergence section of `docs/reference/sdlc-gate-standard.md` — acceptance: artifacts exist; `npm run lint:md` passes.
- [ ] [AI] Apply the same rhino-cli source changes to infra (propagated rhino-cli): merge root configs into `repo-config.yml` + delete the 3 standalone files (§2a-cfg); ensure the lint-staged map covers `*.rs`/`*.fs` (§2a) — acceptance: `test -f repo-config.yml`; the 3 old files absent; `:instruction-size:validation`/`:env:validation` exit 0.

### 4b. Standardize rhino-cli target names

- [ ] [AI] In infra `apps/rhino-cli/project.json`: **remove `fmt` + `format:check` targets** (formatting → lint-staged); **add no `harness:*` Nx targets** (the gates run the binding/env validators as direct `cargo run` calls, same as public); **fold shell/Dockerfile/workflow lint into lint-staged** (no tool-lint Nx targets); **remove the `test-coverage` target** (the command is gone from the propagated rhino-cli source); update references; rewire `.husky/pre-commit` to a direct `cargo run -- env staged-guard validate` (step 1) + lint-staged (format + tool-lint, step 2) + a direct `cargo run -- harness bindings generate` (step 3) (replacing the inline `check-no-env-staged.sh` and `shellcheck`/`hadolint`/`actionlint` blocks; drop the `test:quick` line) **and remove the git-identity guard** (`git rm scripts/check-no-env-staged.sh scripts/git-identity-check.sh` + drop both hook lines), and `.husky/pre-push` to a direct `cargo run --release -- harness bindings validate` — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json | sort` equals public's sorted key set (no `fmt`/`format:check`/`test-coverage`); `test ! -f scripts/check-no-env-staged.sh && test ! -f scripts/git-identity-check.sh`; each new target exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Rename `specs:coverage`→`specs:behavior:coverage` in `ose-infra/apps/rhino-cli/project.json` (propagated from Phase 1b ose-public rename) — acceptance: `jq -e '.targets|has("specs:behavior:coverage") and (has("specs:coverage")|not)' apps/rhino-cli/project.json` is true; `npx nx run rhino-cli:specs:coverage` fails with "target not found"; `npx nx run rhino-cli:specs:behavior:coverage` exits 0.
  - _Suggested executor: `swe-rust-dev`_

### 4c-codecov. Remove Codecov residue (infra — last repo still carrying it)

- [ ] [AI] Delete `ose-infra/codecov.yml` (the last live Codecov config across the three repos; public + primer already removed it) — acceptance: `test ! -f codecov.yml`.
- [ ] [AI] Scrub stale Codecov references from infra governance docs + `apps/rhino-cli/README.md`: remove the `codecov-upload.yml` CRON row/bullets from `repo-governance/development/quality/three-level-testing-standard.md`, the `codecov-upload.yml` upload step from `repo-governance/development/infra/ci-conventions.md`, the "Codecov algorithm"/`test-coverage validate` text from `repo-governance/development/infra/nx-targets.md` and `apps/rhino-cli/README.md` — acceptance: `grep -rin codecov . | grep -vi 'ExcludeFromCodeCoverage'` returns nothing in ose-infra; `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-fixer`_

### 4c. Confirm workflow + hook parity (record IaC divergence)

- [ ] [AI] **Delete infra's `validate-markdown.yml`** and fold markdown into the gates (parity with public/primer); verify `pr-quality-gate.yml`, `validate-env.yml`, `main-ci.yml` match the standard filenames + validator sets (per-file md + gherkin `.feature` in lint-staged; `md-links` job; specs-gate full set) — acceptance: `test ! -f .github/workflows/validate-markdown.yml`; filenames identical; validator sets match; any gap recorded as a fix step.
- [ ] [AI] Confirm infra's pre-commit/pre-push step order matches the standard, with terraform/ansible/yamllint as **documented allowed additions** (not drift) and the `[self-hosted, linux, ose-infra-runner]` label retained — acceptance: order matches; IaC + runner appear only in the divergence section.
- [ ] [AI] Fix any gaps found above — acceptance: each fixed gate exits 0 locally.

### 4d. Mandatory-six sweep across all 7 infra projects

- [ ] [AI] Bring each infra project to the [§2.3 matrix](./tech-docs.md#23-per-project-target-matrix-post-implementation-ose-infra): `coralpolyp-be` keeps service-level `test:integration` **and gains `specs:domain:coverage`**; `coralpolyp-fe` integration real only if DB-backed else echo; `ts-ui-tokens` gains its 4 missing targets; `ts-ui` gains `test:integration`/`test:e2e` echo; `*-e2e` keep real `test:e2e`, echo `test:unit`/`test:integration` (**no `format` target anywhere** — lint-staged handles it) — acceptance: the mandatory-six `jq` check prints no `MISSING` for any infra project.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Add a `test:specs` target to every infra project (aggregate of its `specs:*` validators, `*-be` adds domain:coverage) and set every infra project's `test:quick` to the sequential typecheck→lint→`test:unit`→`test:coverage`→`test:specs` composition; resolve orphan features via `specs:behavior:coverage --require-consumption` — acceptance: `test:specs` present on every project; order verified; no orphan-feature errors; no separate specs-structural step.
- [ ] [AI] Add a native `test:coverage` target (≥ 90% line; `echo` where `test:unit` is `echo`) to every infra project per the [§2.3 matrix](./tech-docs.md#23-per-project-target-matrix-post-implementation-ose-infra) `test:coverage` column — acceptance: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:coverage")' >/dev/null || echo "NO-COV: $p"; done` prints no `NO-COV`.
- [ ] [AI] Wire `specs:domain:coverage` on `coralpolyp-be` (the only infra `*-be` backend) per §2.3 matrix — acceptance: `npx nx show project coralpolyp-be --json | jq -e '.targets|has("specs:domain:coverage")'` is true; `coralpolyp-fe`/libs do **not** declare it.

### 4e. Post-merge main CI (fast); coralpolyp heavy tests + deploy stay CRON-only

- [ ] [AI] Add `.github/workflows/main-ci.yml` (self-hosted) running the PR check set across **all** projects: `nx run-many --all -t test:quick` + the repo-wide validators + IaC on push to main — acceptance: `actionlint` passes; `grep -c 'run-many --all' .github/workflows/main-ci.yml` returns ≥ 1 and `grep -c 'nx affected' .github/workflows/main-ci.yml` returns 0; `grep -n 'test:integration\|test:e2e' .github/workflows/main-ci.yml` returns nothing.
- [ ] [AI] Leave coralpolyp heavy tests + deploy **CRON-only**: the scheduled `test-and-deploy-coralpolyp-development` runs the full suite (quick+int+e2e) and deploys to coralpolyp staging; `test-coralpolyp-staging.yml` → prod promotion stays scheduled — acceptance: `actionlint` passes; integration/e2e run only in those scheduled workflows; prod promotion unchanged.
- [ ] [AI] Confirm the infra gate scope split: pre-push + PR run `test:quick` (+ validators + IaC) for **affected** (`nx affected`); main-ci runs it for **all** projects (`nx run-many --all`); pre-commit runs the fast file-type set only (no `test:quick`); no gate runs integration/e2e — acceptance: `grep -n 'test:integration\|test:e2e' .husky/pre-commit .husky/pre-push .github/workflows/pr-quality-gate.yml .github/workflows/main-ci.yml` returns no gate invocation; `grep -c 'nx affected' .github/workflows/pr-quality-gate.yml` returns ≥ 1; `grep -c 'run-many --all' .github/workflows/main-ci.yml` returns ≥ 1; `grep -c 'test:quick' .husky/pre-commit` returns 0.

- [ ] [AI] Commit propagated artifacts + config merge: `git commit -m "chore(config): propagate standardize-rhino-cli-sdlc-parity plan artifacts and merge repo-config.yml into ose-infra"` — acceptance: `git log --oneline -1` shows this commit; `test -f repo-config.yml` passes; 3 standalone config files absent.
- [ ] [AI] Commit rhino-cli target-name standardization: `git commit -m "chore(rhino-cli): standardize Nx target names in ose-infra (remove fmt/format:check/test-coverage, fold tool-lint into lint-staged, add bindings, specs:behavior:coverage)"` — acceptance: `jq -r '.targets|keys[]' apps/rhino-cli/project.json | sort` equals public's sorted key set; `npx nx run rhino-cli:harness:bindings-validation` exits 0.
- [ ] [AI] Commit Codecov removal + workflow parity: `git commit -m "chore(ci): remove Codecov residue and confirm workflow+hook parity in ose-infra"` — acceptance: `test ! -f codecov.yml`; `grep -rin codecov . | grep -vi 'ExcludeFromCodeCoverage'` returns nothing; `actionlint` passes.
- [ ] [AI] Commit mandatory-six sweep: `git commit -m "chore(nx): add mandatory-six targets + sequential test:quick + native test:coverage + specs:domain:coverage to all 7 infra projects"` — acceptance: mandatory-six `jq` check prints no `MISSING`; no `NO-COV` project.
- [ ] [AI] Commit post-merge CI: `git commit -m "chore(ci): add main-ci.yml fast gate (test:quick + validators) to ose-infra; coralpolyp heavy tests + deploy stay CRON-only"` — acceptance: `actionlint .github/workflows/main-ci.yml` exits 0; `grep -n 'test:integration\|test:e2e' .github/workflows/main-ci.yml` returns nothing.
- [ ] [AI] Push ose-infra to `origin main` and poll CI: `gh run view --json status,conclusion` every 2 min until complete — acceptance: all checks green on the self-hosted runner (incl. `main-ci.yml`).

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] In ose-infra: `npx nx run-many -t test:quick` — exits 0.
- [ ] [AI] In ose-infra, every project exposes the mandatory-six: `for p in $(npx nx show projects); do npx nx show project "$p" --json | jq -e '.targets|has("test:unit") and has("test:integration") and has("test:e2e") and has("test:quick") and has("lint") and has("typecheck")' >/dev/null || echo "MISSING: $p"; done` — acceptance: prints no `MISSING` line.
- [ ] [AI] In ose-infra, coverage went native + Codecov gone: `jq -e '.targets|has("test-coverage")|not' apps/rhino-cli/project.json` is true; every real-`test:unit` project also exposes `test:coverage`; `test ! -f codecov.yml`; `grep -rin codecov . | grep -vi 'ExcludeFromCodeCoverage'` returns nothing — acceptance: no `test-coverage`/Codecov residue in ose-infra.
- [ ] [AI] In ose-infra: `npm run lint:md` and `actionlint` on changed workflows — exit 0.

> **Pause Safety**: all three repos converged and green. Safe to stop. To resume (infra): `npx nx affected -t lint`.

---

## Phase 5: Cross-Repo Parity Verification & Archival

- [ ] [AI] Build the parity table comparing all three repos across every mechanics row (PR-gate filename, markdown filename, env filename, markdown validator set, specs-gate set, lint invocation mechanism, pre-push governance-vendor presence, hook step order, rhino-cli target-key set, **rhino-cli command set verb-last + identical**, **`repo-config.yml` section schema identical**, mandatory targets on every project, `test:quick` = typecheck→lint→`test:unit`→`test:coverage`→`test:specs` composition (test:specs aggregates specs:\*), native `test:coverage` ≥ 90% gate on every real-`test:unit` project, **no** `test-coverage` target + **no** Codecov anywhere, `format` via file-type lint-staged (no per-project `format` target), pre-push ≡ PR runs only `test:quick`, `specs:behavior:coverage --require-consumption` (feature **+ scenario** eligible coverage) enabled, canonical CI workflow names present) — acceptance: a table with a ✅/❌ per repo per row is produced; every mechanics row is ✅ across all three (allowed-divergence rows excluded); the standardization layer is confirmed **identical** cross-repo.
- [ ] [AI] Record the parity table in each repo's `docs/reference/sdlc-gate-standard.md` under a "Parity Status" heading — acceptance: present in all three; lint:md passes.

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` (each repo).
- [ ] [AI] Run affected linting: `npx nx affected -t lint` (each repo).
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` (each repo).
- [ ] [AI] Run affected spec coverage: `npx nx affected -t specs:behavior:coverage` (each repo).
- [ ] [AI] Fix ALL failures found — including preexisting issues not caused by these changes.

### Post-Push Verification

- [ ] [AI] Push final changes to `main` in each repo.
- [ ] [AI] Monitor GitHub Actions for each push (poll every 2 minutes; one `gh run view --json status,conclusion` per wakeup).
- [ ] [AI] Verify all CI checks pass in all three repos.
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit; do NOT archive until all three are green.

### Commit Guidelines

- [ ] [AI] Commit changes thematically — group by surface (docs / hooks / workflows / Nx targets).
- [ ] [AI] Follow Conventional Commits: `<type>(<scope>): <description>`.
- [ ] [AI] Split per repo and per concern; do NOT bundle unrelated fixes.

### Phase 5 Gate

> All checks below must pass before archival.

- [ ] [AI] The parity table shows ✅ on every mechanics row across all three repos (allowed-divergence rows excluded) — acceptance: no ❌ in any mechanics row.
- [ ] [AI] **CI standardization complete** (plan scope boundary): in each repo the three canonical workflows exist with the exact ose-public names (`pr-quality-gate.yml`, `validate-env.yml`, `main-ci.yml`), no standalone `validate-markdown.yml` remains (markdown folds into the gates), and `main-ci.yml` covers every project via `nx run-many --all` — acceptance: `for w in pr-quality-gate validate-env main-ci; do test -f .github/workflows/$w.yml || echo "MISSING-CI: $w"; done` prints nothing in each repo; `test ! -f .github/workflows/validate-markdown.yml`; `grep -c 'run-many --all' .github/workflows/main-ci.yml` returns ≥ 1 (total project coverage by construction).
- [ ] [AI] **Config + coverage cleanup complete**: `repo-config.yml` exists and the 3 standalone config files are absent; no `format`/`format:check`/`test-coverage` targets; `grep -ri codecov` returns only `ExcludeFromCodeCoverage` — in all three repos.
- [ ] [AI] `docs/reference/sdlc-gate-standard.md` (with the Parity Status table) and `rhino-cli-command-triage.md` exist in all three repos — acceptance: `npm run lint:md` passes in each.
- [ ] [AI] All three repos green on local gates (`npx nx affected -t test:quick`) and on CI for the latest `main` push — acceptance: each repo's latest `gh run view --json conclusion` reports `success`.

> **Pause Safety**: all three repos converged, parity-verified, and green; nothing half-applied. Safe to stop. To resume: re-run the cross-repo parity verification table (Phase 5 step 1) and confirm all-green.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI) in all three repos.
- [ ] [AI] Move plan folder from `plans/in-progress/` to `plans/done/` via `git mv` in each repo: `git mv plans/in-progress/standardize-rhino-cli-sdlc-parity plans/done/2026-MM-DD__standardize-rhino-cli-sdlc-parity` (use the actual completion date).
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Commit: `chore(plans): move standardize-rhino-cli-sdlc-parity to done`.

## Validation Checklist

- [ ] [AI] All TDD cycles complete (the rhino-cli Nx-target additions in Phase 2a).
- [ ] [AI] All tests pass (`npx nx affected -t test:quick`) in all three repos.
- [ ] [AI] Command triage doc covers every leaf subcommand.
- [ ] [AI] SDLC standard doc + parity table present in all three repos.
- [ ] [AI] Divergence policy documents every retained difference.
- [ ] [AI] Acceptance criteria in [prd.md](./prd.md) verified.
