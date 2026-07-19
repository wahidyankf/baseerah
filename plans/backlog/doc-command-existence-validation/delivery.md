# Delivery — Doc Command Existence Validation

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/doc-command-existence-validation/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree doc-command-existence-validation
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Delivery Mode: worktree-to-pr

Work happens in `worktrees/doc-command-existence-validation/`; changes land via draft PR against
`main`; `[HUMAN]` merges. Per-phase PRs where the DAG allows, one PR per worktree.

**Per-phase PR grouping** (each group is one PR from one worktree; groups 5 and 6 may run in
parallel after group 4 merges):

| PR   | Phases                       | Worktree                                       |
| ---- | ---------------------------- | ---------------------------------------------- |
| PR-1 | 0-2 (core + detectors)       | `worktrees/doc-command-existence-validation/`  |
| PR-2 | 3 (remediation)              | `worktrees/doc-command-existence-remediation/` |
| PR-3 | 4 (wiring)                   | `worktrees/doc-command-existence-wiring/`      |
| PR-4 | 5 (ose-primer propagation)   | sibling repo worktree                          |
| PR-5 | 6 (ose-infra propagation)    | sibling repo worktree                          |
| PR-6 | 7-8 (verification + capture) | `worktrees/doc-command-existence-verify/`      |

Each `*-to-pr` PR runs the **PR-Review Maker→Fixer Cycle** (3 sequential CI-gated
`pr-review-maker` → `pr-review-fixer` cycles) before the `[HUMAN]` merge. See the
[PR Review Quality Gate workflow](../../../repo-governance/workflows/pr/pr-review-quality-gate.md).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

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
- [ ] [AI] Create the Knowledge Capture running log at
      `plans/in-progress/doc-command-existence-validation/learnings.md` if absent — acceptance:
      file exists with the scaffold header comments
- [ ] [AI] Establish the test baseline: `npx nx run rhino-cli:test:quick` — acceptance: baseline
      pass/fail count recorded in `learnings.md`; all preexisting failures documented
- [ ] [AI] Resolve all preexisting failures before proceeding — acceptance: no preexisting
      failure remains unresolved

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` baseline
      recorded and every preexisting failure resolved (zero unresolved)

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
- [ ] [AI] Add the shell-side snapshot builder invoking `npx nx show projects --json` as a
      subprocess in `apps/rhino-cli/src/commands/md_validate_commands.rs` _New file_, returning a
      hard error (never a silent pass) when resolution fails — acceptance: a unit test
      `nx_resolution_failure_is_hard_error` asserts a `Result::Err` when the subprocess exits
      nonzero

### npm script oracle (TDD cycle)

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
      `cargo run --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --help`
      exits 0 and prints the flag list
- [ ] [AI] Register the module in `apps/rhino-cli/src/commands.rs` — acceptance: build exits 0
- [ ] [AI] Add the validator to the aggregate runner in
      `apps/rhino-cli/src/commands/md_audit.rs` — acceptance: unit test
      `md_audit_includes_command_existence` passes
- [ ] [AI] Add the `commands:validation` target to `apps/rhino-cli/project.json` following the
      `{domain}:{work}` rule — acceptance:
      `npx nx show project rhino-cli --json` lists `commands:validation`
- [ ] [AI] Implement `--format json` output consistent with sibling validators — acceptance:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --format json | jq .`
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

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --help`
      — expected: exits 0, lists `--strict`, `--exclude`, `--format`
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

- [ ] [AI] Produce the full violation inventory:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done --exclude apps/rhino-cli/tests/fixtures --format json > local-temp/doc-command-findings.json`
      — acceptance: file written; finding count recorded in `learnings.md`
- [ ] [AI] Split the "Canonical governance and validation targets" table in
      `repo-governance/development/infra/nx-targets.md` (~L146) into two tables: **Targets that
      exist** (verified against `npx nx show project rhino-cli --json`) and **Planned targets**
      (explicitly labelled aspirational) — acceptance: every row in the first table resolves in
      the Nx graph
- [ ] [AI] Add a `<!-- doc-command-exempt: planned target, not yet implemented -->` annotation to
      each row of the Planned-targets table covering `specs:domain:coverage`,
      `links:validation`, `mermaid:validation`, `headings:hierarchy-validation`,
      `cross-vendor:parity-validation`, `harness:bindings-validation` — acceptance:
      `grep -c "doc-command-exempt" repo-governance/development/infra/nx-targets.md` equals the
      Planned-targets row count
- [ ] [AI] Fix every remaining finding in `local-temp/doc-command-findings.json` — for each,
      decide whether the doc is wrong (correct the citation) or the tooling is missing (annotate
      as planned with a reason) — acceptance: each finding has a recorded disposition
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
- [ ] [AI] `npx nx show project rhino-cli --json` cross-checked against the "Targets that exist"
      table in `nx-targets.md` — expected: every listed target resolves

> **Pause Safety**: the repository corpus is clean and `nx-targets.md` now honestly separates real
> from planned targets — an improvement that stands on its own even if the validator is never
> wired up. Safe to stop. To resume: re-run the validator command above.

---

## Phase 4: Hook and CI Wiring

> _Suggested executor: `ci-fixer`_

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

- [ ] [AI] Provision a worktree in the `ose-primer` checkout for this propagation —
      acceptance: `git -C <primer-worktree> status` reports a clean tree on the new branch
- [ ] [AI] Copy `apps/rhino-cli/` from `ose-public` to `ose-primer` verbatim — acceptance:
      `diff -r <public>/apps/rhino-cli <primer>/apps/rhino-cli` produces no output
- [ ] [AI] Copy `specs/apps/rhino/behavior/rhino-cli/gherkin/` verbatim — acceptance:
      `diff -r` on that tree produces no output
- [ ] [AI] Apply the equivalent `.husky/pre-push` and `.github/workflows/main-ci.yml` wiring,
      adapted to `ose-primer`'s hook and workflow structure — acceptance: `sh -n .husky/pre-push`
      and `actionlint` both exit 0
- [ ] [AI] Run the remediation sweep in `ose-primer`:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done --exclude apps/rhino-cli/tests/fixtures`
      — acceptance: exits 0 after fixing findings (expect `nx-targets.md` drift mirrored there)
- [ ] [AI] Provision polyglot dependencies before pushing (the `crud-*` demo apps depend on
      rhino-cli and a fresh worktree fails pre-push until F#/Elixir deps are fetched) —
      acceptance: `npm run doctor -- --fix` exits 0
- [ ] [AI] Run the local quality gates — acceptance: zero failures
- [ ] [AI] Commit and push to the `ose-primer` PR branch
- [ ] [AI] Open the draft PR, monitor CI to green, run the 3-cycle PR-Review Maker→Fixer Cycle
      — acceptance: CI green, no unresolved review findings
- [ ] [HUMAN] Merge the `ose-primer` PR — resume signal: `gh pr view <n> --json state` reports
      `MERGED`

### Phase 5 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `diff -r <public>/apps/rhino-cli <primer>/apps/rhino-cli` — expected: no output
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done`
      in `ose-primer` — expected: exits 0

> **Pause Safety**: `ose-primer` matches `ose-public` and its gate is green. `ose-infra` is still
> behind, which is a known divergence, not a breakage. Safe to stop. To resume: re-run the `diff -r`
> above.

---

## Phase 6: Propagate to ose-infra

> _May run in parallel with Phase 5 — both depend only on Phase 4._

- [ ] [AI] Provision a worktree in the `ose-infra` checkout — acceptance: clean tree on the new
      branch
- [ ] [AI] Copy `apps/rhino-cli/` verbatim from `ose-public` — acceptance: `diff -r` produces no
      output
- [ ] [AI] Copy `specs/apps/rhino/behavior/rhino-cli/gherkin/` verbatim — acceptance: `diff -r`
      produces no output
- [ ] [AI] Apply the equivalent hook and CI wiring adapted to `ose-infra`'s structure —
      acceptance: `sh -n .husky/pre-push` and `actionlint` both exit 0
- [ ] [AI] Run the remediation sweep in `ose-infra` — acceptance: exits 0 after fixing findings
- [ ] [AI] Verify no infra-private content (real hostnames, inventories, Terraform state) leaked
      into any file copied back toward the public repos — acceptance: the propagation is
      strictly one-way, `ose-public` → `ose-infra`; no reverse copy occurred
- [ ] [AI] Run the local quality gates — acceptance: zero failures
- [ ] [AI] Commit and push to the `ose-infra` PR branch
- [ ] [AI] Open the draft PR, monitor CI to green, run the 3-cycle PR-Review Maker→Fixer Cycle
      — acceptance: CI green, no unresolved review findings
- [ ] [HUMAN] Merge the `ose-infra` PR — resume signal: `gh pr view <n> --json state` reports
      `MERGED`

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `diff -r <public>/apps/rhino-cli <infra>/apps/rhino-cli` — expected: no output
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md commands validate --exclude plans/done`
      in `ose-infra` — expected: exits 0

> **Pause Safety**: all three repos carry the validator and all three gates are green. Safe to
> stop. To resume: re-run the `diff -r` above.

---

## Phase 7: Three-Way Byte-Identity Verification

- [ ] [AI] Capture the `apps/rhino-cli` tree SHA in each repo:
      `git -C <repo> rev-parse HEAD:apps/rhino-cli` for `ose-public`, `ose-primer`, `ose-infra`
      — acceptance: all three SHAs recorded in `learnings.md`
- [ ] [AI] Assert the three SHAs are identical — acceptance: identical content yields an identical
      tree SHA; any mismatch is a hard stop requiring reconciliation before proceeding
- [ ] [AI] Capture and compare the Gherkin tree SHA in each repo:
      `git -C <repo> rev-parse HEAD:specs/apps/rhino/behavior/rhino-cli/gherkin`
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
