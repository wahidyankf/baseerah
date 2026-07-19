# Delivery — Doc Command Existence Validation

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Initial worktree path (PR-1, Phases 0-2): `worktrees/doc-command-existence-validation/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree doc-command-existence-validation
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

Per the maintainer's standing rule — **1 PR ↔ 1 worktree** — this is only the _initial_ worktree.
Phase 3 (PR-2), Phase 4 (PR-3), and Phase 8 (PR-6) each explicitly provision their own additional
`worktrees/<name>/` directory, branched from the then-latest `origin/main`, at the start of the
phase — mirroring the same `fetch` + `worktree add` + branch-point-verification discipline Phase 5
and Phase 6 already use for the sibling-repo worktrees. The PR table below names every worktree.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Delivery Mode: worktree-to-pr

Work happens in `worktrees/doc-command-existence-validation/`; changes land via draft PR against
`main`; `[HUMAN]` merges. Per-phase PRs where the DAG allows, one PR per worktree.

**Per-phase PR grouping** (each group is one PR from one worktree; PR-4 and PR-5 — Phases 5 and 6 —
may run in parallel once PR-3, Phase 4, merges):

| PR   | Phases                       | Worktree                                                 |
| ---- | ---------------------------- | -------------------------------------------------------- |
| PR-1 | 0-2 (core + detectors)       | `worktrees/doc-command-existence-validation/`            |
| PR-2 | 3 (remediation)              | `worktrees/doc-command-existence-remediation/`           |
| PR-3 | 4 (wiring)                   | `worktrees/doc-command-existence-wiring/`                |
| PR-4 | 5 (ose-primer propagation)   | `ose-primer:worktrees/doc-command-existence-validation/` |
| PR-5 | 6 (ose-infra propagation)    | `ose-infra:worktrees/doc-command-existence-validation/`  |
| PR-6 | 7-8 (verification + capture) | `worktrees/doc-command-existence-verify/`                |

Each `*-to-pr` PR runs the **PR-Review Maker→Fixer Cycle** (3 sequential CI-gated
`pr-review-maker` → `pr-review-fixer` cycles) before the `[HUMAN]` merge. See the
[PR Review Quality Gate workflow](../../../repo-governance/workflows/pr/pr-review-quality-gate.md).

---

## Phase 0: Environment Setup and Baseline

> _Suggested executor: `repo-setup-manager`_

- [ ] [AI] Install dependencies in the root worktree: `npm install` — acceptance: exits 0,
      `node_modules/` synchronized
- [ ] [AI] Converge the polyglot toolchain in the root worktree: `npm run doctor -- --fix` —
      acceptance: exits 0 with no unresolved drift
- [ ] [AI] Verify the Rust toolchain builds rhino-cli:
      `cargo build --release --manifest-path apps/rhino-cli/Cargo.toml` — acceptance: exits 0
- [ ] [AI] Record the Nx target ground truth for later assertion:
      `npx nx show project rhino-cli --json` — acceptance: JSON captured in `learnings.md`;
      the resolved target list contains no `links:validation`, `mermaid:validation`, or
      `headings:hierarchy-validation`
- [ ] [AI] **Promote this plan from `backlog/` to `in-progress/` BEFORE any other Phase 0 step
      references an in-progress path.** The plan folder currently lives at
      `plans/backlog/doc-command-existence-validation/`, but later steps (the `learnings.md` scaffold
      below, Phase 3's grep acceptance, and Plan Archival's `git mv`) all address
      `plans/in-progress/doc-command-existence-validation/`. Nothing else moves it, and
      `plan-execution.md` performs no automatic move — so without this step Phase 0 creates an
      orphaned duplicate `learnings.md` at the wrong path, Phase 3's grep fails on a missing file,
      and archival dies with a fatal git error. Run
      `git mv plans/backlog/doc-command-existence-validation plans/in-progress/doc-command-existence-validation`,
      then update `plans/backlog/README.md` (remove the entry) and `plans/in-progress/README.md`
      (add it) — acceptance: `test -d plans/in-progress/doc-command-existence-validation && test ! -d plans/backlog/doc-command-existence-validation`
      exits 0, and `grep -c "doc-command-existence-validation" plans/in-progress/README.md` returns ≥1
      (returns **0** pre-edit — verified live). No date prefix is added: `in-progress/` uses bare
      slugs per the plans convention.
- [ ] [AI] Create the Knowledge Capture running log at
      `plans/in-progress/doc-command-existence-validation/learnings.md` if absent — acceptance:
      file exists with the scaffold header comments **and an H1** (`# Learnings: doc-command-existence-validation`);
      a scaffold of bare HTML comments fails markdownlint MD041 on the first commit
- [ ] [AI] Establish the test baseline: `npx nx run rhino-cli:test:quick` — acceptance: baseline
      pass/fail count recorded in `learnings.md`; all preexisting failures documented
- [ ] [AI] Resolve all preexisting failures before proceeding — acceptance: no preexisting
      failure remains unresolved

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [ ] [AI] `npx nx run rhino-cli:test:quick` baseline recorded and every preexisting failure
      resolved (zero unresolved). This single command satisfies the broader
      `typecheck lint test:quick specs:behavior:coverage` check: `test:quick`'s `project.json`
      chain already runs `typecheck` → `lint` → `test:unit` → `test:coverage` → `test:specs`, and
      `test:specs` in turn runs `specs:structure-validation` and `specs:behavior:coverage`
      — no separate `npx nx affected -t ...` invocation is needed

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no feature
> work exists yet. Safe to stop indefinitely. To resume: re-run the baseline command and confirm
> it is still clean.

---

## Phase 1: Capability Oracles (pure core + shell builders)

> _Suggested executor: `swe-rust-dev`_

### Gherkin scaffold

- [ ] [AI] Create `specs/apps/rhino/behavior/rhino-cli/gherkin/md/docs-validate-commands.feature`
      with the `@docs-validate-commands` tag and the Feature narrative, following the shape of
      sibling `docs-validate-links.feature` — acceptance: file exists; `head -3` shows the tag and
      `Feature:` line
- [ ] [AI] Copy every scenario from `prd.md §Acceptance Criteria` into the feature file verbatim
      — acceptance: `grep -c "^  Scenario:" specs/apps/rhino/behavior/rhino-cli/gherkin/md/docs-validate-commands.feature`
      equals the scenario count in `prd.md`
- [ ] [AI] Add the new feature file to
      `specs/apps/rhino/behavior/rhino-cli/gherkin/md/README.md` index — acceptance:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md readme-index validate`
      exits 0
- [ ] [AI] Verify cardinality compliance:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- specs gherkin-cardinality validate`
      — acceptance: exits 0

### rhino-cli clap oracle (TDD cycle)

**Gherkin (underpins) →** the rhino-cli subcommand scenarios in `prd.md §rhino-cli subcommand detection`

**Gherkin (binds) →** "The subcommand oracle is derived from the live clap tree rather than a hardcoded list"

```gherkin
Scenario: The subcommand oracle is derived from the live clap tree rather than a hardcoded list
  Given a new subcommand is added to the rhino-cli clap command tree
  And no list of valid subcommands is edited anywhere in the validator source
  When the developer runs "rhino-cli md commands validate" against a file citing the new subcommand
  Then the command exits with status zero
  And no finding is reported for the new subcommand
```

- [ ] [AI] **RED**: write a failing test `clap_oracle_enumerates_live_command_tree` in
      `apps/rhino-cli/src/domain/doc_commands.rs` asserting the oracle contains the chain
      `["md", "links", "validate"]` and does not contain `["md", "ghost", "validate"]`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails with "cannot find function `clap_command_chains`"
- [ ] [AI] **GREEN**: implement `clap_command_chains(root: &clap::Command) -> BTreeSet<Vec<String>>`
      in `apps/rhino-cli/src/domain/doc_commands.rs`, recursing via `Command::get_subcommands()`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes; no other rhino-cli test breaks
- [ ] [AI] **REFACTOR**: extract the recursion into a private helper and document the
      no-hardcoded-list invariant in a doc comment
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass; `cargo clippy --manifest-path apps/rhino-cli/Cargo.toml -- -D warnings` exits 0

### Nx graph oracle (TDD cycle)

**Gherkin (underpins) →** the Nx-target scenarios in `prd.md §Nx target detection`

- [ ] [AI] **RED**: write a failing test `nx_snapshot_parses_projects_and_targets` in
      `apps/rhino-cli/src/domain/doc_commands.rs` feeding a captured `nx show projects --json`
      fixture and asserting `rhino-cli` resolves with target `test:quick` present and
      `links:validation` absent
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails with "cannot find type `NxSnapshot`"
- [ ] [AI] **GREEN**: implement `NxSnapshot` plus `parse_nx_snapshot(json: &str) -> Result<NxSnapshot>`
      in `apps/rhino-cli/src/domain/doc_commands.rs`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: replace ad-hoc string keys with newtypes `ProjectName` / `TargetName`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass
- [ ] [AI] **RED**: write a failing test `nx_resolution_failure_is_hard_error` in
      `apps/rhino-cli/src/commands/md_validate_commands.rs` _New file_ asserting the shell-side
      snapshot builder returns `Result::Err` when the `npx nx show projects --json` subprocess
      exits nonzero
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails with "cannot find function" (the builder function does not exist yet)
- [ ] [AI] **GREEN**: implement the shell-side snapshot builder invoking
      `npx nx show projects --json` as a subprocess in
      `apps/rhino-cli/src/commands/md_validate_commands.rs`, returning a hard error (never a
      silent pass) when resolution fails
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: `nx_resolution_failure_is_hard_error` passes; no other test breaks
- [ ] [AI] **REFACTOR**: extract the subprocess-invocation boilerplate into a small helper shared
      with the other capability-oracle shell builders in this file
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

### npm script oracle (TDD cycle)

**Gherkin (underpins) →** the npm-script scenarios in `prd.md §npm script detection`

- [ ] [AI] **RED**: write a failing test `npm_scripts_parsed_from_package_json` asserting
      `lint:md:fix` resolves and `ghost:script` does not, against a fixture `package.json`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails with "cannot find function `parse_npm_scripts`"
- [ ] [AI] **GREEN**: implement `parse_npm_scripts(json: &str) -> BTreeSet<String>`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: fold the three oracles into a single `CapabilitySnapshot` struct
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `cargo test --manifest-path apps/rhino-cli/Cargo.toml` — expected: exits 0, all oracle
      tests pass
- [ ] [AI] `cargo clippy --manifest-path apps/rhino-cli/Cargo.toml -- -D warnings` — expected:
      exits 0
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- specs gherkin-cardinality validate`
      — expected: exits 0

> **Pause Safety**: the three capability oracles exist and are unit-tested, but nothing consumes
> them yet — the CLI surface is unchanged and no gate references the new code. Safe to stop. To
> resume: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`.

---

## Phase 2: Detectors, Exemptions, and CLI Surface

> _Suggested executor: `swe-rust-dev`_

### Citation extraction (TDD cycle)

**Gherkin (binds) →** "A markdown file citing a nonexistent Nx target fails validation"

```gherkin
Scenario: A markdown file citing a nonexistent Nx target fails validation
  Given a tracked markdown file containing the command "npx nx run rhino-cli:links:validation"
  And the resolved Nx project graph contains no target "links:validation" on project "rhino-cli"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the output names the file, the line number, and the cited target
  And the output states that the target does not exist on the project
```

- [ ] [AI] **RED**: write failing test `extracts_nx_run_citation_from_fenced_block` in
      `apps/rhino-cli/src/domain/doc_commands.rs`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails with "cannot find function `extract_citations`"
- [ ] [AI] **GREEN**: implement `extract_citations(markdown: &str) -> Vec<Citation>` recognizing
      `nx run`, `npx nx run`, and `nx run-many -t`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: extract the fenced-block scanner into a reusable iterator
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "A markdown file citing an existing Nx target passes validation"

```gherkin
Scenario: A markdown file citing an existing Nx target passes validation
  Given a tracked markdown file containing the command "npx nx run rhino-cli:test:quick"
  And the resolved Nx project graph contains target "test:quick" on project "rhino-cli"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for that file
```

- [ ] [AI] **RED**: write failing test `existing_nx_target_produces_no_finding` in
      `apps/rhino-cli/src/domain/doc_commands.rs`, asserting a citation of a real target on a real
      project yields an empty finding list
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (the detector reports every citation until the oracle is consulted)
- [ ] [AI] **GREEN**: consult the Nx target oracle before emitting a finding, so a resolvable
      project+target pair is passed over
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: `existing_nx_target_produces_no_finding` passes
- [ ] [AI] **REFACTOR**: share the resolve-then-report branch with the other three resolver classes
      so false-positive suppression is implemented once
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "An inferred Nx target that is absent from project.json still passes validation"

```gherkin
Scenario: An inferred Nx target that is absent from project.json still passes validation
  Given a tracked markdown file citing an Nx target present only via plugin inference
  And the target is absent from the project's literal "project.json" targets map
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for that target
```

- [ ] [AI] **RED**: write failing test `inferred_target_absent_from_project_json_produces_no_finding`
      in `apps/rhino-cli/src/domain/doc_commands.rs`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (a `project.json`-only oracle reports the inferred target as missing).
      **This is the load-bearing case for DD-2**: the oracle MUST read the _resolved_ graph
      (`nx show project <p> --json`), not the literal `project.json`, or every plugin-inferred target
      in the repo becomes a false positive.
- [ ] [AI] **GREEN**: source the oracle from the resolved project graph so inferred targets resolve
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: `inferred_target_absent_from_project_json_produces_no_finding` passes
- [ ] [AI] **REFACTOR**: cache the resolved graph across citations so the oracle is built once per run
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "A citation naming a nonexistent Nx project is reported distinctly"

```gherkin
Scenario: A citation naming a nonexistent Nx project is reported distinctly
  Given a tracked markdown file containing the command "npx nx run ghost-app:build"
  And the resolved Nx project graph contains no project named "ghost-app"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding states that the project does not exist, distinct from a missing-target finding
```

- [ ] [AI] **RED**: write failing test `unknown_project_reported_distinctly_from_unknown_target`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails; no `FindingKind::UnknownProject` variant exists
- [ ] [AI] **GREEN**: add `FindingKind::{UnknownProject, UnknownTarget}` and resolve accordingly
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: unify finding construction behind a single constructor
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "A run-many target citation is validated against the union of all project targets"

```gherkin
Scenario: A run-many target citation is validated against the union of all project targets
  Given a tracked markdown file containing the command "npx nx run-many -t phantom-target"
  And no project in the resolved graph defines a target named "phantom-target"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding identifies the cited run-many target
```

- [ ] [AI] **RED**: write failing test `run_many_target_checked_against_union`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (run-many resolution unimplemented)
- [ ] [AI] **GREEN**: resolve run-many targets against the union of all project target sets
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: memoize the union set on `CapabilitySnapshot`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "A cargo-run citation of a nonexistent rhino-cli subcommand chain fails validation"

```gherkin
Scenario: A cargo-run citation of a nonexistent rhino-cli subcommand chain fails validation
  Given a tracked markdown file citing "cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md ghost validate"
  And the rhino-cli clap command tree contains no subcommand "ghost" under "md"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding names the unresolved segment of the subcommand chain
```

- [ ] [AI] **RED**: write failing test `cargo_run_chain_resolved_against_clap_tree`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (the `-- <chain>` split is unimplemented)
- [ ] [AI] **GREEN**: implement the `--` separator split and chain resolution, also matching bare
      `rhino-cli <chain>` invocations
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: report the first unresolved segment rather than the whole chain
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "A cargo-run citation of an existing rhino-cli subcommand chain passes validation"

```gherkin
Scenario: A cargo-run citation of an existing rhino-cli subcommand chain passes validation
  Given a tracked markdown file citing "cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate"
  And the rhino-cli clap command tree resolves the chain "md links validate"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for that citation
```

- [ ] [AI] **RED**: write failing test `existing_cargo_run_subcommand_chain_produces_no_finding` in
      `apps/rhino-cli/src/domain/doc_commands.rs`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (the chain resolver reports every cargo-run citation until the clap oracle
      is consulted). Note the chain must be resolved **after** the `--` separator and walked level by
      level (`md` → `links` → `validate`) against the built clap tree.
- [ ] [AI] **GREEN**: resolve the post-`--` chain against the clap subcommand oracle before reporting
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: `existing_cargo_run_subcommand_chain_produces_no_finding` passes
- [ ] [AI] **REFACTOR**: reuse the same chain walker for the nonexistent-chain case so both paths
      share one traversal
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "A citation of a nonexistent npm script fails validation"

```gherkin
Scenario: A citation of a nonexistent npm script fails validation
  Given a tracked markdown file containing the command "npm run ghost:script"
  And the repository root "package.json" declares no script named "ghost:script"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding names the cited script and the package.json consulted
```

- [ ] [AI] **RED**: write failing test `npm_run_citation_resolved_against_package_json`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (npm citation kind unimplemented)
- [ ] [AI] **GREEN**: implement `npm run <script>` extraction and resolution
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: include the consulted `package.json` path in the finding message
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

### False-positive suppression (TDD cycles)

**Gherkin (binds) →** "A citation of an existing npm script passes validation"

```gherkin
Scenario: A citation of an existing npm script passes validation
  Given a tracked markdown file containing the command "npm run lint:md:fix"
  And the repository root "package.json" declares a script named "lint:md:fix"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for that citation
```

- [ ] [AI] **RED**: write failing test `existing_npm_script_produces_no_finding` in
      `apps/rhino-cli/src/domain/doc_commands.rs`, using `lint:md:fix` — a script the root
      `package.json` really declares (verified live)
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (every `npm run` citation is reported until the script oracle is consulted)
- [ ] [AI] **GREEN**: consult the root `package.json` scripts map before reporting
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: `existing_npm_script_produces_no_finding` passes
- [ ] [AI] **REFACTOR**: load the scripts map once per run alongside the other oracles
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "A templated command containing an angle-bracket placeholder is ignored by default"

```gherkin
Scenario: A templated command containing an angle-bracket placeholder is ignored by default
  Given a tracked markdown file containing the command "nx run <project>:test:quick"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the templated citation
```

- [ ] [AI] **RED**: write failing test `angle_bracket_placeholder_suppressed`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (currently reports a finding)
- [ ] [AI] **GREEN**: add placeholder detection to the classifier
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: consolidate suppression predicates into one `is_templated` function
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "A command containing a shell variable is ignored by default"

```gherkin
Scenario: A command containing a shell variable is ignored by default
  Given a tracked markdown file containing the command "npx nx run $PROJECT:build"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the variable-bearing citation
```

- [ ] [AI] **RED**: write failing test `shell_variable_suppressed` covering `$VAR` and `${VAR}`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails
- [ ] [AI] **GREEN**: extend `is_templated` to shell variables
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: table-drive the suppression tests
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "Prose mentions of a command outside a fenced block are ignored by default"

```gherkin
Scenario: Prose mentions of a command outside a fenced block are ignored by default
  Given a tracked markdown file mentioning "nx run some-app:ghost-target" in a prose sentence
  And the mention is not inside a fenced code block
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the prose mention
```

- [ ] [AI] **RED**: write failing test `prose_mention_suppressed_in_default_mode`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails
- [ ] [AI] **GREEN**: gate non-fenced citations behind the strict flag
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: pass mode as an explicit `Mode` enum rather than a bare bool
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "Strict mode reports prose mentions that the default mode suppresses"

```gherkin
Scenario: Strict mode reports prose mentions that the default mode suppresses
  Given a tracked markdown file mentioning "nx run some-app:ghost-target" in a prose sentence
  And the target does not exist in the resolved Nx project graph
  When the developer runs "rhino-cli md commands validate --strict"
  Then the command exits with a nonzero status
  And the finding identifies the prose mention
```

- [ ] [AI] **RED**: write failing test `strict_mode_reports_prose_mention`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (no `--strict` plumbing)
- [ ] [AI] **GREEN**: add the `--strict` flag to the args struct and thread it to the core
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: document the audit-tool-not-a-gate intent in the flag help text
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "A multi-line continuation command is reassembled before validation"

```gherkin
Scenario: A multi-line continuation command is reassembled before validation
  Given a tracked markdown file containing a fenced command split across lines with trailing backslashes
  And the reassembled command cites an existing Nx target
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the continued command
```

- [ ] [AI] **RED**: write failing test `backslash_continuation_reassembled`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (continuation split produces a spurious finding)
- [ ] [AI] **GREEN**: join trailing-backslash lines before extraction
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: preserve the original first-line number for finding reporting
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

### Exemption mechanism (TDD cycles)

**Gherkin (binds) →** "An inline exemption annotation with a reason suppresses a finding"

```gherkin
Scenario: An inline exemption annotation with a reason suppresses a finding
  Given a tracked markdown file citing a nonexistent Nx target
  And the citation is preceded by the annotation "<!-- doc-command-exempt: planned in ROADMAP -->"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the exempted citation
```

- [ ] [AI] **RED**: write failing test `annotated_exemption_with_reason_suppresses`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (annotation unparsed)
- [ ] [AI] **GREEN**: parse `<!-- doc-command-exempt: <reason> -->` and suppress the next citation
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: model the annotation as a typed `Exemption { reason }` value
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "An inline exemption annotation without a reason is itself a finding"

```gherkin
Scenario: An inline exemption annotation without a reason is itself a finding
  Given a tracked markdown file citing a nonexistent Nx target
  And the citation is preceded by the bare annotation "<!-- doc-command-exempt -->"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding states that an exemption annotation requires a written reason
```

- [ ] [AI] **RED**: write failing test `bare_exemption_annotation_is_a_finding`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (bare annotation currently suppresses silently)
- [ ] [AI] **GREEN**: emit `FindingKind::ExemptionMissingReason` for a reasonless annotation
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: reject a whitespace-only reason as equivalent to absent
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "An exemption annotation applies only to the citation that immediately follows it"

```gherkin
Scenario: An exemption annotation applies only to the citation that immediately follows it
  Given a tracked markdown file with an annotated exempt citation followed by a second unannotated nonexistent citation
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And exactly one finding is reported, naming the second citation
```

- [ ] [AI] **RED**: write failing test `exemption_scoped_to_next_citation_only`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (exemption currently leaks to subsequent citations)
- [ ] [AI] **GREEN**: consume the exemption on first use
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: make the consume-on-use semantics explicit via `Option::take()`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

**Gherkin (binds) →** "A path in the configured exclusion allowlist is not scanned"

```gherkin
Scenario: A path in the configured exclusion allowlist is not scanned
  Given a markdown file under "plans/done/" citing a nonexistent Nx target
  And "plans/done" is listed in the validator's configured exclusions
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the excluded path
```

- [ ] [AI] **RED**: write failing test `excluded_path_not_scanned`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (no `--exclude` support)
- [ ] [AI] **GREEN**: add repeatable `--exclude <path>` matching the idiom of
      `md links validate` in `.husky/pre-push`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: share the exclusion-matching helper with the existing walker
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass

### CLI surface and aggregation

- [ ] [AI] Add the `Commands(MdCommandsCommands)` variant to the `MdCommands` enum at
      `apps/rhino-cli/src/cli.rs` (enum begins line ~237) with `#[command(name = "commands", subcommand)]`,
      plus the `MdCommandsCommands::Validate` inner enum following the `MdLinksCommands` pattern
      — acceptance: `cargo build --manifest-path apps/rhino-cli/Cargo.toml` exits 0
- [ ] [AI] Add the dispatch arm `MdCommands::Commands(cc) => …` to the router at
      `apps/rhino-cli/src/cli.rs` (match begins line ~707) — acceptance:
      `cargo run --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done`
      runs the validator (exit 0 or 1 — a real result, not clap's exit-2 "unrecognized subcommand").
      **Do NOT use `--help` as the probe**: `--help` in this CLI is a custom global bool intercepted in
      `run()` and routed to `print_help_and_exit()`, which always prints **root**-level help regardless
      of subcommand depth — verified live, `md links validate --help` never prints its own real
      `--exclude` flag
- [ ] [AI] Register the module in `apps/rhino-cli/src/commands.rs` — acceptance: build exits 0

**Gherkin (binds) →** "The validator participates in the aggregate md audit"

```gherkin
Scenario: The validator participates in the aggregate md audit
  Given a tracked markdown file citing a nonexistent Nx target
  When the developer runs "rhino-cli md audit"
  Then the command exits with a nonzero status
  And the aggregated output includes the command-existence finding
```

- [ ] [AI] **RED**: write a failing test `md_audit_includes_command_existence` in
      `apps/rhino-cli/src/commands/md_audit.rs` asserting the aggregate `md audit` run invokes the
      new command-existence validator
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml md_audit`
      — acceptance: fails (the validator is not yet wired into the aggregate runner)
- [ ] [AI] **GREEN**: add the new validator to the aggregate runner in
      `apps/rhino-cli/src/commands/md_audit.rs`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml md_audit`
      — acceptance: `md_audit_includes_command_existence` passes
- [ ] [AI] **REFACTOR**: align the new validator's aggregation call with the existing sibling
      validators' call order and formatting in `md_audit.rs`
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml md_audit`
      — acceptance: all tests still pass
- [ ] [AI] Add the `commands:validation` target to `apps/rhino-cli/project.json` following the
      `{domain}:{work}` rule — acceptance:
      `npx nx show project rhino-cli --json` lists `commands:validation`

**Gherkin (binds) →** "Findings are emitted as machine-readable JSON on request"

```gherkin
Scenario: Findings are emitted as machine-readable JSON on request
  Given a tracked markdown file citing a nonexistent Nx target
  When the developer runs "rhino-cli md commands validate -o json"
  Then the output parses as valid JSON
  And each finding object carries a file path, a line number, the cited command, and a reason
```

- [ ] [AI] **RED**: write a failing test `output_json_flag_produces_valid_json` in
      `apps/rhino-cli/src/domain/doc_commands.rs` asserting `-o json` output parses as valid
      JSON with the expected finding-list shape
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: fails (the `-o json` branch is unimplemented)
- [ ] [AI] **GREEN**: implement `-o json` output consistent with sibling validators
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: `output_json_flag_produces_valid_json` passes
- [ ] [AI] **REFACTOR**: share the JSON-serialization shape with the sibling validators'
      `-o json` implementation to avoid divergence
      — command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`
      — acceptance: all tests still pass; end-to-end check
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate -o json | jq .`
      exits 0

### Local Quality Gates (Before Push)

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root cause orientation principle — proactively fix preexisting
> errors encountered during work. Do not defer or skip existing issues. Commit preexisting
> fixes separately with appropriate conventional commit messages.

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
- [ ] [AI] Run affected spec coverage: `npx nx affected -t specs:behavior:coverage`
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by your changes
- [ ] [AI] Re-run failing checks to confirm resolution — acceptance: zero failures

### Commit Guidelines

- [ ] [AI] Commit changes thematically — group related changes into logically cohesive commits
- [ ] [AI] Follow Conventional Commits format: `<type>(<scope>): <description>`
- [ ] [AI] Split different domains/concerns into separate commits (oracles / detectors /
      exemptions / CLI surface)
- [ ] [AI] Preexisting fixes get their own commits, separate from plan work
- [ ] [AI] Commit and push to origin `doc-command-existence-validation` (the PR-1 branch)

### Post-Push CI Verification

- [ ] [AI] Open the draft PR for PR-1 against `main`
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 2 minutes with
      one `gh run view --json status,conclusion` per wakeup; never `gh run watch`)
- [ ] [AI] Verify ALL CI checks pass — no exceptions
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit
- [ ] [AI] Repeat until ALL GitHub Actions pass with zero failures

### PR-Review Maker→Fixer Cycle (PR-1)

- [ ] [AI] Cycle 1: run `pr-review-maker` on PR-1, then `pr-review-fixer` — acceptance: CI green
      after the fixer's push
- [ ] [AI] Cycle 2: run `pr-review-maker`, then `pr-review-fixer` — acceptance: CI green
- [ ] [AI] Cycle 3: run `pr-review-maker`, then `pr-review-fixer` — acceptance: CI green and the
      final maker pass reports no unresolved findings
- [ ] [HUMAN] Merge PR-1 to `main` — the human merges on their own schedule; plan completion is
      not blocked on the merge. Observable resume signal: `gh pr view <n> --json state` reports
      `MERGED`

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] Verify the flag surface via **clap introspection**, not `--help`: add a unit test
      `md_commands_validate_flag_surface` in `apps/rhino-cli/src/cli.rs` that walks
      `<Cli as clap::CommandFactory>::command()` down to the `md commands validate` leaf and asserts
      its argument ids are exactly `strict` and `exclude` (plus the inherited globals) and that **no
      `format` arg exists**.
      **You MUST call `.build()` on the root `Command` before walking to the leaf.** clap propagates
      `global = true` args into a subcommand's own `get_arguments()` only inside `Command::build()`
      (`clap_builder-4.6.0/src/builder/command.rs`, `_propagate_global_args`) — never automatically.
      Empirically confirmed against the pinned `clap_builder 4.6.0` from `Cargo.lock`: without
      `.build()` the leaf reports `["strict", "exclude"]`; with `.build()` it reports
      `["strict", "exclude", "output"]`. A test written without `.build()` would fail its
      inherited-globals assertion even though the CLI is correct — command:
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml md_commands_validate_flag_surface`
      — expected: exits 0.
      **`--help` MUST NOT be used to check the flag list**: this CLI declares `--help` as a custom
      global bool intercepted in `run()` (cli.rs lines ~602-604) and handled by `print_help_and_exit()`
      (~882-887), which always prints **root**-level `Cli::command()` help regardless of subcommand
      depth — live-verified, `md links validate --help` prints zero occurrences of its own real
      `--exclude`. JSON output likewise comes from the single global `-o`/`--output` arg declared
      `global = true` in `cli.rs` and propagated by clap to every subcommand; adding a per-subcommand
      `--format` flag would violate DD-1
- [ ] [AI] `cargo test --manifest-path apps/rhino-cli/Cargo.toml` — expected: exits 0
- [ ] [AI] `npx nx run rhino-cli:specs:behavior:coverage` — expected: exits 0, every scenario in
      `docs-validate-commands.feature` is bound
- [ ] [AI] `npx nx show project rhino-cli --json` — expected: `commands:validation` present

> **Pause Safety**: the validator exists and is fully tested, but is not wired into any hook or
> CI job — the repository's gate behavior is unchanged and the known drift is still present.
> Safe to stop. To resume: `cargo test --manifest-path apps/rhino-cli/Cargo.toml doc_commands`.

---

## Phase 3: Remediation (land green, not red)

> _Suggested executor: `repo-rules-fixer`_

### Provision the PR-2 worktree

- [ ] [AI] Fetch and provision this phase's worktree at the repo-local `worktrees/<name>/` path:
      `git fetch origin` then
      `git worktree add worktrees/doc-command-existence-remediation -b doc-command-existence-remediation origin/main`
      — acceptance: `git worktree list` shows the new worktree at
      `worktrees/doc-command-existence-remediation`, and
      `git -C worktrees/doc-command-existence-remediation rev-parse HEAD` equals
      `git rev-parse origin/main` (proves it is branched from the LATEST `origin/main`, which by
      this point includes PR-1's merged changes — not a stale ref)
- [ ] [AI] Initialize the toolchain inside the new worktree and treat it as this phase's working
      directory for every subsequent step (`cd worktrees/doc-command-existence-remediation` — do
      not rely on the shell's inherited working directory):
      `npm install && npm run doctor -- --fix` — acceptance: exits 0;
      `git -C worktrees/doc-command-existence-remediation status --porcelain` is empty

- [ ] [AI] Produce the full violation inventory:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done --exclude apps/rhino-cli/tests/fixtures -o json > local-temp/doc-command-findings.json`
      — acceptance: file written; finding count recorded in `learnings.md`
- [ ] [AI] Record the six to-be-deleted target names in `learnings.md` BEFORE touching the table,
      so the roadmap intent survives the deletion (per DD-6; the entry already exists — confirm it
      is present and accurate) — acceptance:
      `grep -c "specs:domain:coverage" plans/in-progress/doc-command-existence-validation/learnings.md`
      is at least 1
- [ ] [AI] Delete the six nonexistent rows from the "Canonical governance and validation targets"
      table in `repo-governance/development/infra/nx-targets.md` (~L146):
      `specs:domain:coverage`, `links:validation`, `mermaid:validation`,
      `headings:hierarchy-validation`, `cross-vendor:parity-validation`,
      `harness:bindings-validation`. Do NOT create a replacement "Planned targets" table
      — acceptance: scoped to the table only (so legitimate mentions elsewhere in the file — e.g.
      `specs:domain:coverage`, which IS a real target on `organiclever-be`/`ose-be` — cannot mask a
      leftover row), all six names return 0:
      `sed -n '/Canonical governance and validation targets/,/\*\*Rule\*\*:/p' repo-governance/development/infra/nx-targets.md | grep -cE "specs:domain:coverage|links:validation|mermaid:validation|headings:hierarchy-validation|cross-vendor:parity-validation|harness:bindings-validation"`
      returns 0 (the same command returns 6 against the current unfixed file, so it is a real
      before/after check rather than a vacuous one)
- [ ] [AI] Verify every remaining row in that table resolves against the live graph by
      cross-checking each against `npx nx show project rhino-cli --json` — acceptance: every listed
      target appears in the resolved target list; no row remains that does not
- [ ] [AI] Check prose elsewhere in `nx-targets.md` for references to the three deleted targets
      most likely to appear outside the table (the surrounding paragraphs and the `{domain}:{work}`
      examples may cite them). `specs:domain:coverage` is deliberately excluded from this prose
      check — it has legitimate, unrelated occurrences elsewhere in the file (it IS a real target on
      other projects such as `organiclever-be`/`ose-be`); `cross-vendor:parity-validation` and
      `harness:bindings-validation` are also excluded — both have zero occurrences anywhere in the
      file outside the table itself, so checking them here would be a no-op — acceptance:
      `grep -nE "links:validation|mermaid:validation|headings:hierarchy-validation" repo-governance/development/infra/nx-targets.md`
      returns no line that asserts the target exists
- [ ] [AI] Fix every remaining finding in `local-temp/doc-command-findings.json` — for each,
      decide whether the doc is wrong (correct the citation) or the citation is legitimately
      unresolvable (apply a Tier-1 `<!-- doc-command-exempt: <reason> -->` annotation per DD-5)
      — acceptance: each finding has a recorded disposition; no finding is left unaddressed
- [ ] [AI] Re-run the validator against the full corpus:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done --exclude apps/rhino-cli/tests/fixtures`
      — acceptance: exits 0 with zero findings
- [ ] [AI] Run the local quality gates (see the Phase 2 template) — acceptance: zero failures
- [ ] [AI] Commit and push to origin `doc-command-existence-remediation` (the PR-2 branch)
- [ ] [AI] Open the draft PR, monitor CI to green, run the 3-cycle PR-Review Maker→Fixer Cycle
      — acceptance: CI green, no unresolved review findings
- [ ] [HUMAN] Merge PR-2 to `main` — resume signal: `gh pr view <n> --json state` reports `MERGED`

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done --exclude apps/rhino-cli/tests/fixtures`
      — expected: exits 0, zero findings
      **Gherkin (binds) →** "The repository corpus is clean after remediation"

```gherkin
Scenario: The repository corpus is clean after remediation
  Given the remediation phase has corrected every known citation of a nonexistent command
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no findings are reported
```

- [ ] [AI] `npx nx show project rhino-cli --json` cross-checked against every row of the canonical
      targets table in `nx-targets.md` — expected: every listed target resolves
- [ ] [AI] `sed -n '/Canonical governance and validation targets/,/^\*\*Rule\*\*/p' repo-governance/development/infra/nx-targets.md | grep -cE "specs:domain:coverage|links:validation|mermaid:validation|headings:hierarchy-validation|cross-vendor:parity-validation|harness:bindings-validation"`
      — expected: `0` (all six deleted rows are gone from the canonical table; the check is scoped
      to that table section so it does not false-positive on `specs:domain:coverage`'s legitimate,
      unrelated occurrences elsewhere in the file — e.g. the general Target Naming Standards table,
      where it is a real target on other projects such as `organiclever-be`/`ose-be`)
- [ ] [AI] The six deleted names are recorded in `learnings.md` — expected: present, so the
      deletion did not silently destroy the roadmap information

> **Pause Safety**: the repository corpus is clean and `nx-targets.md` now asserts only targets that
> actually exist — an improvement that stands on its own even if the validator is never wired up.
> Safe to stop. To resume: re-run the validator command above.

---

## Phase 4: Hook and CI Wiring

> _Suggested executor: `ci-fixer`_

### Provision the PR-3 worktree

- [ ] [AI] Fetch and provision this phase's worktree at the repo-local `worktrees/<name>/` path:
      `git fetch origin` then
      `git worktree add worktrees/doc-command-existence-wiring -b doc-command-existence-wiring origin/main`
      — acceptance: `git worktree list` shows the new worktree at
      `worktrees/doc-command-existence-wiring`, and
      `git -C worktrees/doc-command-existence-wiring rev-parse HEAD` equals
      `git rev-parse origin/main` (proves it is branched from the LATEST `origin/main`, which by
      this point includes PR-1 and PR-2's merged changes — not a stale ref)
- [ ] [AI] Initialize the toolchain inside the new worktree and treat it as this phase's working
      directory for every subsequent step (`cd worktrees/doc-command-existence-wiring` — do not
      rely on the shell's inherited working directory):
      `npm install && npm run doctor -- --fix` — acceptance: exits 0;
      `git -C worktrees/doc-command-existence-wiring status --porcelain` is empty

- [ ] [AI] Add the validator invocation to `.husky/pre-push` after the existing
      `md readme-index validate` line (currently line 12):
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done --exclude apps/rhino-cli/tests/fixtures`
      — acceptance: `sh -n .husky/pre-push` exits 0 and the line is present
- [ ] [AI] Add a step to the `markdown-per-file` job in `.github/workflows/main-ci.yml`
      (job begins line ~103), after the heading-hierarchy step, named
      `Doc command existence validation (all .md)` with the same invocation — acceptance:
      `actionlint .github/workflows/main-ci.yml` exits 0
- [ ] [AI] Verify the hook fires end-to-end by temporarily adding a citation of
      `npx nx run rhino-cli:headings:hierarchy-validation` to a scratch tracked markdown file and
      attempting a push — acceptance: the push is rejected and the finding names the target
      **Gherkin (binds) →** "Reintroducing an originally-cited nonexistent target is rejected"

```gherkin
Scenario: Reintroducing an originally-cited nonexistent target is rejected
  Given a tracked markdown file containing the command "npx nx run rhino-cli:headings:hierarchy-validation"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding names the target "headings:hierarchy-validation"
```

- [ ] [AI] Remove the scratch citation — acceptance: `git status` shows no scratch file
- [ ] [AI] Measure the added pre-push cost:
      `time cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done`
      — acceptance: elapsed time recorded in `learnings.md`; if it exceeds the runtime of the
      existing `md links validate` step by more than a factor of three, record the finding and
      raise the hook-placement question with the user before proceeding
- [ ] [AI] Run the local quality gates (see the Phase 2 template) — acceptance: zero failures
- [ ] [AI] Commit and push to origin `doc-command-existence-wiring` (the PR-3 branch)
- [ ] [AI] Open the draft PR, monitor CI to green, run the 3-cycle PR-Review Maker→Fixer Cycle
      — acceptance: CI green, no unresolved review findings
- [ ] [HUMAN] Merge PR-3 to `main` — resume signal: `gh pr view <n> --json state` reports `MERGED`

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `sh -n .husky/pre-push` — expected: exits 0
- [ ] [AI] `actionlint .github/workflows/main-ci.yml` — expected: exits 0
- [ ] [AI] `gh run list --workflow main-ci.yml --limit 1 --json conclusion` — expected:
      `success`

> **Pause Safety**: the gate is armed in `ose-public` and green. The sibling repos do not yet have
> the validator, which is a divergence but not a breakage — their gates are unchanged. Safe to
> stop. To resume: `gh run list --workflow main-ci.yml --limit 1 --json conclusion`.

---

## Phase 5: Propagate to ose-primer

> _Byte-identity boundary: `apps/rhino-cli/**` and `specs/apps/rhino/behavior/rhino-cli/gherkin/**`
> must be byte-identical across all three repos, zero carve-outs, per the
> [SDLC Gate Standard](../../../docs/reference/sdlc-gate-standard.md#rhino-cli-byte-identity-boundary)._

- [ ] [AI] **Confirm the sibling's repo topology BEFORE anything else** —
      `git -C /Users/wkf/ose-projects/ose-primer rev-parse --is-bare-repository`
      — acceptance: prints `true`. **`ose-primer` is a BARE repo** (verified 2026-07-19): it has no
      top-level working tree, so `git -C /Users/wkf/ose-projects/ose-primer status` fails with
      `fatal: this operation must be run in a work tree`. All file work happens inside a worktree.
      If this prints `false`, the topology changed — STOP and re-derive the commands below rather
      than assuming (this repo's topology HAS changed before — `ose-infra` was non-bare on
      2026-07-02). See [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md)
      §Sibling-Repo Relative Paths From Inside a Worktree, which records a real prior incident of
      silent stale-content propagation in a structurally identical tri-repo plan.
- [ ] [AI] Fetch and provision the worktree at the repo-local `worktrees/<name>/` path:
      `git -C /Users/wkf/ose-projects/ose-primer fetch origin main` then
      `git -C /Users/wkf/ose-projects/ose-primer worktree add worktrees/doc-command-existence-validation -b doc-command-existence-validation origin/main`
      — acceptance: `git -C /Users/wkf/ose-projects/ose-primer worktree list` shows the new worktree
      at `/Users/wkf/ose-projects/ose-primer/worktrees/doc-command-existence-validation`, and
      `git -C <primer-worktree> rev-parse HEAD` equals
      `git -C /Users/wkf/ose-projects/ose-primer rev-parse origin/main` (proves it is branched from
      the LATEST `origin/main`, not a stale local ref)
- [ ] [AI] Bind `<public>` = the **PR-3 worktree** at
      `/Users/wkf/ose-projects/ose-public/worktrees/doc-command-existence-wiring` for every
      `<public>`-referencing step in this phase and Phase 7 (Phase 6 restates this same binding
      locally, under its own declared "may run in parallel with Phase 5" path). This is the freshest
      fully-synced `ose-public` checkout by this point (PR-3 has merged), and it is what the
      byte-identity diffs must compare against — **not** the primary checkout at
      `/Users/wkf/ose-projects/ose-public`, which may lag behind the merged rhino-cli source this
      plan just landed. Fetch the primary checkout's own `origin/main` ref before comparing (it may
      not have been fetched since PR-1/PR-2/PR-3 merged):
      `git -C /Users/wkf/ose-projects/ose-public fetch origin main` — acceptance:
      `git -C <public> rev-parse HEAD:apps/rhino-cli` equals
      `git -C /Users/wkf/ose-projects/ose-public rev-parse origin/main:apps/rhino-cli` (proves the
      bound checkout carries the merged rhino-cli subtree)
- [ ] [AI] Set `<primer-worktree>` = `/Users/wkf/ose-projects/ose-primer/worktrees/doc-command-existence-validation`
      for every subsequent step in this phase; run `npm install && npm run doctor -- --fix`
      **inside that worktree** (`cd` into it — do not rely on the shell's inherited working
      directory) — acceptance: `git -C <primer-worktree> status --porcelain` is empty; toolchain
      converged
- [ ] [AI] Copy `apps/rhino-cli/` from `ose-public` to `<primer-worktree>` verbatim — acceptance:
      `diff -rq --exclude=target --exclude=dist <public>/apps/rhino-cli <primer-worktree>/apps/rhino-cli`
      produces no output. Both `target/` and `dist/` are gitignored and untracked, and since
      `plans/done/2026-07-19__rust-cargo-target-dir-sharing/` merged the same day this plan was
      authored, `apps/rhino-cli/target` becomes a per-repo symlink into
      `$HOME/.cache/ose-cargo-target/<repo-name>/rhino-cli` once `npm run doctor -- --fix` runs (as
      this phase's own preceding toolchain-init step, above, already does) — post-`doctor`, it
      legitimately differs between `ose-public` and `ose-primer`, so a plain `diff -r` would
      false-fail. Excluding both matches the idiom established in that sibling plan's `delivery.md`
- [ ] [AI] Copy `specs/apps/rhino/behavior/rhino-cli/gherkin/` verbatim into `<primer-worktree>` —
      acceptance:
      `diff -r <public>/specs/apps/rhino/behavior/rhino-cli/gherkin <primer-worktree>/specs/apps/rhino/behavior/rhino-cli/gherkin`
      produces no output
- [ ] [AI] Apply the equivalent `.husky/pre-push` and `.github/workflows/main-ci.yml` wiring
      **inside `<primer-worktree>`**, adapted to `ose-primer`'s hook and workflow structure —
      acceptance: `sh -n .husky/pre-push` and `actionlint .github/workflows/main-ci.yml` (both run
      from inside `<primer-worktree>`) exit 0
- [ ] [AI] Run the remediation sweep **from inside `<primer-worktree>`**:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done --exclude apps/rhino-cli/tests/fixtures`
      — acceptance: exits 0 after fixing findings
- [ ] [AI] Apply the DD-6 deletion to `<primer-worktree>`'s own
      `repo-governance/development/infra/nx-targets.md`, checking its table **independently**
      against `ose-primer`'s live graph (`npx nx show project rhino-cli --json` run from inside
      `<primer-worktree>`) — do NOT assume the `ose-public` row set applies, since per-repo drift
      may differ — acceptance: every remaining row resolves in `ose-primer`'s graph; any deleted
      name not already listed in `learnings.md` is appended there
- [ ] [AI] Provision polyglot dependencies before pushing (the `crud-*` demo apps depend on
      rhino-cli and a fresh worktree fails pre-push until F#/Elixir deps are fetched) —
      acceptance: `npm run doctor -- --fix` (run from inside `<primer-worktree>`) exits 0
- [ ] [AI] Run the local quality gates (see the Phase 2 template), **from inside `<primer-worktree>`**
      — acceptance: zero failures
- [ ] [AI] Commit with explicit paths (never `git add -A` — the sibling repo carries unrelated WIP)
      and push to the `ose-primer` PR branch:
      `git -C <primer-worktree> add <explicit paths> && git -C <primer-worktree> commit && git -C <primer-worktree> push origin doc-command-existence-validation`
      — acceptance: `git -C <primer-worktree> status --porcelain` shows no unintended files staged;
      push succeeds; pre-push gates exit 0
- [ ] [AI] Open the draft PR, monitor CI to green, run the 3-cycle PR-Review Maker→Fixer Cycle
      — acceptance: CI green, no unresolved review findings
- [ ] [HUMAN] Merge the `ose-primer` PR — resume signal: `gh pr view <n> --json state` reports
      `MERGED`

### Phase 5 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `diff -rq --exclude=target --exclude=dist <public>/apps/rhino-cli <primer-worktree>/apps/rhino-cli`
      — expected: no output (excludes the per-repo `target/` symlink and untracked `dist/`; see the
      copy step above)
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done`
      run from inside `<primer-worktree>` — expected: exits 0

> **Pause Safety**: `ose-primer` matches `ose-public` and its gate is green. `ose-infra` is still
> behind, which is a known divergence, not a breakage. Safe to stop. To resume: re-run the `diff -r`
> above.

---

## Phase 6: Propagate to ose-infra

> _May run in parallel with Phase 5 — both depend only on Phase 4._

- [ ] [AI] **Confirm the sibling's repo topology BEFORE anything else** —
      `git -C /Users/wkf/ose-projects/ose-infra rev-parse --is-bare-repository`
      — acceptance: prints `true`. **`ose-infra` is a BARE repo** (verified 2026-07-19): it has no
      top-level working tree, so `git -C /Users/wkf/ose-projects/ose-infra status` fails with
      `fatal: this operation must be run in a work tree`. All file work happens inside a worktree.
      Note this repo's topology has CHANGED before (it was non-bare on 2026-07-02), so treat the
      check as live state — if it prints `false`, STOP and re-derive the commands below rather than
      assuming. See [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md)
      §Sibling-Repo Relative Paths From Inside a Worktree, which records a real prior incident of
      silent stale-content propagation in a structurally identical tri-repo plan.
- [ ] [AI] Fetch and provision the worktree at the repo-local `worktrees/<name>/` path:
      `git -C /Users/wkf/ose-projects/ose-infra fetch origin main` then
      `git -C /Users/wkf/ose-projects/ose-infra worktree add worktrees/doc-command-existence-validation -b doc-command-existence-validation origin/main`
      — acceptance: `git -C /Users/wkf/ose-projects/ose-infra worktree list` shows the new worktree
      at `/Users/wkf/ose-projects/ose-infra/worktrees/doc-command-existence-validation`, and
      `git -C <infra-worktree> rev-parse HEAD` equals
      `git -C /Users/wkf/ose-projects/ose-infra rev-parse origin/main` (proves it is branched from
      the LATEST `origin/main`, not a stale local ref)
- [ ] [AI] Bind `<public>` = the **PR-3 worktree** at
      `/Users/wkf/ose-projects/ose-public/worktrees/doc-command-existence-wiring` for every
      `<public>`-referencing step in this phase (same binding as Phase 5 — restated here so this
      phase is self-contained under its own declared "may run in parallel with Phase 5" path, and
      does not silently depend on a sibling phase's step list). Fetch the primary checkout's own
      `origin/main` ref before comparing:
      `git -C /Users/wkf/ose-projects/ose-public fetch origin main` — acceptance:
      `git -C <public> rev-parse HEAD:apps/rhino-cli` equals
      `git -C /Users/wkf/ose-projects/ose-public rev-parse origin/main:apps/rhino-cli` (proves the
      bound checkout carries the merged rhino-cli subtree)
- [ ] [AI] Set `<infra-worktree>` = `/Users/wkf/ose-projects/ose-infra/worktrees/doc-command-existence-validation`
      for every subsequent step in this phase; run `npm install && npm run doctor -- --fix`
      **inside that worktree** (`cd` into it — do not rely on the shell's inherited working
      directory) — acceptance: `git -C <infra-worktree> status --porcelain` is empty; toolchain
      converged
- [ ] [AI] Copy `apps/rhino-cli/` verbatim from `ose-public` into `<infra-worktree>` — acceptance:
      `diff -rq --exclude=target --exclude=dist <public>/apps/rhino-cli <infra-worktree>/apps/rhino-cli`
      produces no output. Both `target/` and `dist/` are gitignored and untracked, and since
      `plans/done/2026-07-19__rust-cargo-target-dir-sharing/` merged the same day this plan was
      authored, `apps/rhino-cli/target` becomes a per-repo symlink into
      `$HOME/.cache/ose-cargo-target/<repo-name>/rhino-cli` once `npm run doctor -- --fix` runs (as
      this phase's own preceding toolchain-init step, above, already does) — post-`doctor`, it
      legitimately differs between `ose-public` and `ose-infra`, so a plain `diff -r` would
      false-fail. Excluding both matches the idiom established in that sibling plan's `delivery.md`
- [ ] [AI] Copy `specs/apps/rhino/behavior/rhino-cli/gherkin/` verbatim into `<infra-worktree>` —
      acceptance:
      `diff -r <public>/specs/apps/rhino/behavior/rhino-cli/gherkin <infra-worktree>/specs/apps/rhino/behavior/rhino-cli/gherkin`
      produces no output
- [ ] [AI] Apply the equivalent hook and CI wiring **inside `<infra-worktree>`**, adapted to
      `ose-infra`'s structure — acceptance: `sh -n .husky/pre-push` and
      `actionlint .github/workflows/main-ci.yml` (both run from inside `<infra-worktree>`) exit 0
- [ ] [AI] Run the remediation sweep **from inside `<infra-worktree>`**:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done --exclude apps/rhino-cli/tests/fixtures`
      — acceptance: exits 0 after fixing findings
- [ ] [AI] Apply the DD-6 deletion to `<infra-worktree>`'s own
      `repo-governance/development/infra/nx-targets.md`, checking its table **independently**
      against `ose-infra`'s live graph (`npx nx show project rhino-cli --json` run from inside
      `<infra-worktree>`) — do NOT assume the `ose-public` row set applies, since per-repo drift may
      differ — acceptance: every remaining row resolves in `ose-infra`'s graph
- [ ] [AI] Verify no infra-private content (real hostnames, inventories, Terraform state) leaked
      into any file copied back toward the public repos — acceptance: the propagation is
      strictly one-way, `ose-public` → `ose-infra`; no reverse copy occurred
- [ ] [AI] Run the local quality gates (see the Phase 2 template), **from inside `<infra-worktree>`**
      — acceptance: zero failures
- [ ] [AI] Commit with explicit paths (never `git add -A` — the sibling repo carries unrelated WIP)
      and push to the `ose-infra` PR branch:
      `git -C <infra-worktree> add <explicit paths> && git -C <infra-worktree> commit && git -C <infra-worktree> push origin doc-command-existence-validation`
      — acceptance: `git -C <infra-worktree> status --porcelain` shows no unintended files staged;
      push succeeds; pre-push gates exit 0
- [ ] [AI] Open the draft PR, monitor CI to green, run the 3-cycle PR-Review Maker→Fixer Cycle
      — acceptance: CI green, no unresolved review findings
- [ ] [HUMAN] Merge the `ose-infra` PR — resume signal: `gh pr view <n> --json state` reports
      `MERGED`

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `diff -rq --exclude=target --exclude=dist <public>/apps/rhino-cli <infra-worktree>/apps/rhino-cli`
      — expected: no output (excludes the per-repo `target/` symlink and untracked `dist/`; see the
      copy step above)
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done`
      run from inside `<infra-worktree>` — expected: exits 0

> **Pause Safety**: all three repos carry the validator and all three gates are green. Safe to
> stop. To resume: re-run the `diff -r` above.

---

## Phase 7: Three-Way Byte-Identity Verification

- [ ] [AI] Bind `<repo>` for each of the three legs before running any comparison below:
      **ose-public** = `/Users/wkf/ose-projects/ose-public/worktrees/doc-command-existence-wiring`
      (the PR-3 worktree carrying the merged rhino-cli source — the primary checkout may lag);
      **ose-primer** = `/Users/wkf/ose-projects/ose-primer`; **ose-infra** =
      `/Users/wkf/ose-projects/ose-infra`. The two siblings are **bare** repos whose own `HEAD`
      resolves to the bare root's **local** `main` branch, not `origin/main` — Phase 5/6's `fetch`
      calls only ever advance a _worktree's_ ref namespace, never the bare root's own
      `refs/heads/main`, so a bare-root `HEAD:<path>` read can silently return stale content (the
      same failure class documented in
      [Worktree Toolchain Initialization §Absolute Source Paths in Delivery-Checklist Commands](../../../repo-governance/development/workflow/worktree-setup.md#absolute-source-paths-in-delivery-checklist-commands-same-repo-worktree-vs-primary-checkout)).
      Fetch each bare sibling root's `origin/main` explicitly, immediately before comparing, and
      read `origin/main:<path>` on the two siblings — never bare `HEAD:<path>`:
      `git -C /Users/wkf/ose-projects/ose-primer fetch origin main` and
      `git -C /Users/wkf/ose-projects/ose-infra fetch origin main` — acceptance: both fetches exit
      0, and `git -C /Users/wkf/ose-projects/ose-public/worktrees/doc-command-existence-wiring rev-parse HEAD`,
      `git -C /Users/wkf/ose-projects/ose-primer rev-parse origin/main`, and
      `git -C /Users/wkf/ose-projects/ose-infra rev-parse origin/main` each succeed and print a
      commit SHA
- [ ] [AI] Capture the `apps/rhino-cli` tree SHA in each repo, reading `origin/main` — never bare
      `HEAD` — on the two bare sibling roots:
      `git -C /Users/wkf/ose-projects/ose-public/worktrees/doc-command-existence-wiring rev-parse HEAD:apps/rhino-cli`,
      `git -C /Users/wkf/ose-projects/ose-primer rev-parse origin/main:apps/rhino-cli`,
      `git -C /Users/wkf/ose-projects/ose-infra rev-parse origin/main:apps/rhino-cli`
      — acceptance: all three SHAs recorded in `learnings.md`
- [ ] [AI] Assert the three SHAs are identical — acceptance: identical content yields an identical
      tree SHA; any mismatch is a hard stop requiring reconciliation before proceeding
- [ ] [AI] Capture and compare the Gherkin tree SHA in each repo, again reading `origin/main` — never
      bare `HEAD` — on the two bare sibling roots:
      `git -C /Users/wkf/ose-projects/ose-public/worktrees/doc-command-existence-wiring rev-parse HEAD:specs/apps/rhino/behavior/rhino-cli/gherkin`,
      `git -C /Users/wkf/ose-projects/ose-primer rev-parse origin/main:specs/apps/rhino/behavior/rhino-cli/gherkin`,
      `git -C /Users/wkf/ose-projects/ose-infra rev-parse origin/main:specs/apps/rhino/behavior/rhino-cli/gherkin`
      — acceptance: all three SHAs identical
- [ ] [AI] If any SHA differs, identify the diverging file via
      `diff -r` and reconcile toward the `ose-public` version — acceptance: SHAs converge
- [ ] [AI] Verify the validator gate is green in all three repos' latest CI runs:
      `gh run list --workflow main-ci.yml --limit 1 --json conclusion` per repo — acceptance:
      `success` in all three

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [ ] [AI] Three-way `apps/rhino-cli` tree SHA equality confirmed — expected: one distinct value
- [ ] [AI] Three-way Gherkin tree SHA equality confirmed — expected: one distinct value
- [ ] [AI] All three repos report CI `success`

> **Pause Safety**: the byte-identity boundary is verified and all three gates are green. The
> feature is fully delivered; only knowledge triage and archival remain. Safe to stop. To resume:
> re-run the three-way tree-SHA comparison.

---

## Phase 8: Knowledge Capture

> _Triage every surviving `learnings.md` entry before archival. See the
> [Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md)._

### Provision the PR-6 worktree

- [ ] [AI] Fetch and provision this phase's worktree at the repo-local `worktrees/<name>/` path:
      `git fetch origin` then
      `git worktree add worktrees/doc-command-existence-verify -b doc-command-existence-verify origin/main`
      — acceptance: `git worktree list` shows the new worktree at
      `worktrees/doc-command-existence-verify`, and
      `git -C worktrees/doc-command-existence-verify rev-parse HEAD` equals
      `git rev-parse origin/main` (proves it is branched from the LATEST `origin/main`, which by
      this point includes PR-1 through PR-3's ose-public-side merges — not a stale ref)
- [ ] [AI] Initialize the toolchain inside the new worktree and treat it as the working directory
      for the rest of Phase 8 and Plan Archival (`cd worktrees/doc-command-existence-verify` — do
      not rely on the shell's inherited working directory):
      `npm install && npm run doctor -- --fix` — acceptance: exits 0;
      `git -C worktrees/doc-command-existence-verify status --porcelain` is empty

- [ ] [AI] Apply the litmus test to every `learnings.md` entry — keep only if a durable surface
      would catch this automatically next time; discard the rest with a one-line reason
      — acceptance: every entry has either a route or a discard reason
- [ ] [AI] Apply the **secret/sensitivity gate** to every surviving entry — sanitize any secret,
      credential, token, or private hostname to a `<placeholder>` token, or discard if
      unsanitizable — acceptance: `learnings.md` contains no raw secret
- [ ] [AI] Apply the **repo-relevance gate** to every surviving entry — infra-private content
      (Terraform, k3s, Proxmox, real hostnames/inventories) stays in `ose-infra` only and is NEVER
      cross-routed into `ose-public`/`ose-primer` — acceptance: no infra-private content appears
      in this repo's routed output
- [ ] [AI] Route each surviving learning to exactly one durable home per the open-ended routing
      matrix — non-code homes may land inline (small edit) or as a `plans/backlog/` follow-up
      (large); code homes (`apps/`, `libs/`, tests) are ALWAYS filed as a separate
      `plans/backlog/<slug>/` plan and NEVER landed inline
      — acceptance: every `learnings.md` entry records its terminal routing state
- [ ] [AI] Specifically consider filing the deferred shell/`make` detector (DD-7) as a
      `plans/backlog/` follow-up if Phase 3's finding inventory showed material shell-citation
      drift — acceptance: either a backlog plan exists or a one-line reason not to file it is
      recorded
- [ ] [AI] If no generalizable learning surfaced, record the explicit escape in `learnings.md`:
      `No generalizable learnings — <one-line reason>` — acceptance: `learnings.md` is never
      silently empty

### Phase 8 Gate

> All checks below must pass before Plan Archival.

- [ ] [AI] Every `learnings.md` entry is in a terminal state (routed inline, filed as backlog, or
      discarded with reason), or the file records the explicit "none" escape
- [ ] [AI] No code-homed learning landed inline in this plan's own commits/PRs

> **Pause Safety**: `learnings.md` is fully triaged (or explicitly recorded as empty); no future
> process depends on querying it later. Safe to stop. To resume: re-read `learnings.md` and confirm
> every entry is terminal.

---

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify the Knowledge Capture phase is complete — every `learnings.md` entry reached a
      terminal state or the file records the explicit `No generalizable learnings — <reason>`
      escape; both the secret/sensitivity gate and the repo-relevance gate were applied
- [ ] [AI] Verify ALL quality gates pass (local + CI) in all three repositories
- [ ] [AI] Confirm the rule-15 (web three-tester) and rule-16 (API exploratory) retests do NOT
      apply — this plan is CLI/text-output only, per the exemption stated in `tech-docs.md`
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/doc-command-existence-validation/ plans/done/YYYY-MM-DD__doc-command-existence-validation/`
      using today's date as the completion date (NOT the creation date)
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date
- [ ] [AI] Update any other READMEs that reference this plan
- [ ] [AI] Commit the archival: `chore(plans): move doc-command-existence-validation to done`
- [ ] [AI] Push to origin `doc-command-existence-verify` (the PR-6 branch)
- [ ] [AI] Open the draft PR for PR-6 against `main`
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 2 minutes with
      one `gh run view --json status,conclusion` per wakeup; never `gh run watch`) — acceptance:
      all CI checks report success
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit; repeat until ALL
      GitHub Actions pass with zero failures
- [ ] [AI] Run the 3-cycle PR-Review Maker→Fixer Cycle on PR-6:
  - [ ] [AI] Cycle 1: run `pr-review-maker` on PR-6, then `pr-review-fixer` — acceptance: CI green
        after the fixer's push
  - [ ] [AI] Cycle 2: run `pr-review-maker`, then `pr-review-fixer` — acceptance: CI green
  - [ ] [AI] Cycle 3: run `pr-review-maker`, then `pr-review-fixer` — acceptance: CI green and the
        final maker pass reports no unresolved findings
- [ ] [HUMAN] Merge PR-6 to `main` — the human merges on their own schedule; plan completion is not
      blocked on the merge. Observable resume signal: `gh pr view <n> --json state` reports
      `MERGED`
