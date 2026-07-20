# Delivery — Plan Quality Gate Convergence

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.
>
> **Acceptance-clause forms used in this plan** — this plan is subject to the standards it
> installs. Every clause below uses only forms verified during authoring: occurrence-unique counts
> (`grep -ohE '…' file | sort -u | wc -l`) rather than `grep -c` alternation thresholds; `test -f`
> for existence rather than a count claim; no `grep -L`; no multi-file `grep -c`; and every fenced
> block indented to its list item's content column. See
> [tech-docs.md §Defect-Class Registry](./tech-docs.md#defect-class-registry--seed-content).

## Worktree

Worktree path: `worktrees/plan-quality-gate-convergence/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree plan-quality-gate-convergence
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before deleting
the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Delivery Mode: worktree-to-pr

Work happens in `worktrees/plan-quality-gate-convergence/`; each phase group lands as a draft PR
against `main`; `[AI]` commits and pushes to the PR branch; the PR-Review Maker→Fixer Cycle (3
sequential CI-gated cycles) runs before the `[HUMAN]` merge. See the
[PR Review Quality Gate workflow](../../../repo-governance/workflows/pr/pr-review-quality-gate.md).

## Parallelization

Phases 2 and 3 are independent and may run concurrently; Phases 8 and 9 likewise. Respect the repo's
concurrency cap (2 background subagents, 3 total including the main thread) per
[Subagent Orchestration](../../../repo-governance/development/agents/subagent-orchestration.md).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Provision the worktree: `git worktree add worktrees/plan-quality-gate-convergence origin/main`
      — acceptance: `test -d worktrees/plan-quality-gate-convergence/.git || test -f worktrees/plan-quality-gate-convergence/.git`
      succeeds
- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized
- [ ] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift
- [ ] [AI] Record the baseline commit SHA into `learnings.md` in exactly this literal format —
      `Baseline SHA: <full-40-char-sha>` — obtained via `git rev-parse HEAD`
      — acceptance: `grep -ohE '^Baseline SHA: [0-9a-f]{40}$' plans/backlog/2026-07-20__plan-quality-gate-convergence/learnings.md | sort -u | wc -l`
      returns exactly 1 (returns 0 before this step)
- [ ] [AI] Record the **pre-change `plan-checker` validation-step inventory** into `learnings.md` as a
      list of every `### <n>. … (Step 5x …)` heading, obtained via
      `grep -ohE '\(Step 5[a-z] —[^)]*\)' .claude/agents/plan-checker.md | sort -u`
      — acceptance: the recorded inventory is non-empty; this is the AC-16 baseline
- [ ] [AI] Measure the `MD046: {style: fenced}` repo-wide impact for open question Q2: run
      `markdownlint-cli2` with `MD046` set to `fenced` over `plans/**` and over the repo, and record
      both error counts in `learnings.md`
      — acceptance: two integer counts recorded; Q2 becomes decidable on evidence
- [ ] [AI] Establish the test baseline: `npx nx affected -t typecheck lint test:quick specs:coverage`
      — acceptance: baseline pass/fail counts recorded in `learnings.md`; every preexisting failure
      documented
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: re-running the baseline command reports zero failures

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` reports zero failures
- [ ] [AI] `learnings.md` contains the Baseline SHA line, the step inventory, the Q2 impact counts,
      and the recorded baseline — verified by reading the file

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no plan work
> exists yet. Safe to stop indefinitely. To resume: re-run
> `npx nx affected -t typecheck lint test:quick specs:coverage` and confirm it is still clean.

---

## Phase 1: Defect-Class Registry

> _Suggested executor: `repo-rules-maker`_

- [ ] [AI] Create `repo-governance/development/quality/plan-acceptance-defect-classes.md` containing
      the eight seed entries DC-1, DC-2, DC-2b, DC-3, DC-4, DC-5, DC-6, DC-7 verbatim from
      [tech-docs.md §Defect-Class Registry](./tech-docs.md#defect-class-registry--seed-content), each
      with its symptom, runnable proof, safe form, and detection method, plus an "Appending a new
      class" section stating that any plan chain surfacing a new class appends it during Knowledge
      Capture
      — acceptance: `test -f repo-governance/development/quality/plan-acceptance-defect-classes.md`
      succeeds (fails today — file verified absent during authoring) **and**
      `grep -ohE 'DC-1|DC-2b|DC-2|DC-3|DC-4|DC-5|DC-6|DC-7' repo-governance/development/quality/plan-acceptance-defect-classes.md | sort -u | wc -l`
      returns 8
- [ ] [AI] Re-run every proof command in the new file in a scratch directory and confirm each
      reproduces its documented result (DC-1 packed=1/spread=3; DC-2 absent=empty/exit 2 versus
      present-no-match=0/exit 1; DC-2b both=0; DC-3 per-file `filename:count`; DC-5 six-space form
      lacks `language-sh` and two-space form has it)
      — acceptance: every documented result observed; any divergence corrects the registry text
      before the phase closes
- [ ] [AI] Register the new convention in
      `repo-governance/development/quality/README.md` and in
      `repo-governance/development/README.md` index tables
      — acceptance: **per file** (a union count across both files would return 1 whether one or both
      were updated — a DC-6 non-discriminating clause), for each of
      `repo-governance/development/quality/README.md` and `repo-governance/development/README.md`:
      `test -f <file>` succeeds and `grep -ohE 'plan-acceptance-defect-classes' <file> | sort -u | wc -l`
      returns 1 — each returns 0 today, verified during authoring
- [ ] [AI] Cross-link the registry from
      `repo-governance/development/quality/plan-anti-hallucination.md` and from
      `repo-governance/development/infra/acceptance-criteria.md`
      — acceptance: for each file, `test -f <file>` succeeds and
      `grep -ohE 'plan-acceptance-defect-classes' <file> | sort -u | wc -l` returns 1 (returns 0
      today for both)
- [ ] [AI] Run the markdown validators over the new file:
      `npx markdownlint-cli2 "repo-governance/development/quality/plan-acceptance-defect-classes.md"`
      and `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate`
      — acceptance: 0 markdownlint errors; all links valid

### Phase 1 Gate

> All checks below must pass before starting Phase 2 or Phase 3.

- [ ] [AI] `test -f repo-governance/development/quality/plan-acceptance-defect-classes.md` succeeds
- [ ] [AI] All eight class IDs present — the `grep -ohE … | sort -u | wc -l` clause above returns 8
- [ ] [AI] Every proof command re-run and observed to reproduce its documented result
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` reports zero failures

> **Pause Safety**: the registry is a standalone, inert governance document with no consumer yet —
> nothing references it that would break. Safe to stop. To resume: re-run the eight-class count
> clause and confirm it returns 8.

---

## Phase 2: Deterministic Pre-Flight Validator (`rhino-cli`)

> _Suggested executor: `swe-rust-dev`_
>
> **Separability**: this phase implements README open question Q1 option A. If the grill selects
> option B or C, this phase is dropped wholesale and its detection rules are relocated into the
> Phase 3 agent contracts; no other phase changes.

- [ ] [AI] Author the Gherkin behavior tree at
      `specs/apps/rhino/behavior/rhino-cli/gherkin/plan-acceptance/validate-acceptance.feature`
      covering the statically-detectable classes DC-1, DC-3, DC-4, DC-5
      — acceptance: `test -f specs/apps/rhino/behavior/rhino-cli/gherkin/plan-acceptance/validate-acceptance.feature`
      succeeds (file is new) and
      `npx nx run rhino-cli:specs:gherkin-cardinality-validation` exits 0

### TDD cycle 1 — DC-4 (`grep -L` prohibition)

- [ ] [AI] **RED**: add a failing unit test in
      `apps/rhino-cli/src/commands/plan_validate_acceptance.rs` asserting that a fixture delivery
      document containing `grep -L 'x' a.md b.md` in an acceptance clause yields one finding naming
      the environment-dependence of `-L`
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the test fails because the module does not exist yet
      **Gherkin (binds) →** "The -L flag is rejected in an acceptance clause"

  ```gherkin
  Scenario: The -L flag is rejected in an acceptance clause
    Given an acceptance clause using "grep -L" to assert files-without-match
    When the deterministic pre-flight pass evaluates the clause
    Then the pass reports a finding naming the environment-dependence of -L
    And the finding cites the safe per-file substitute using "grep -q" in a loop
  ```

- [ ] [AI] **GREEN**: implement the `-L` detector in
      `apps/rhino-cli/src/commands/plan_validate_acceptance.rs`
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the DC-4 test passes; no other rhino-cli test breaks
- [ ] [AI] **REFACTOR**: extract the clause-scanning scaffold shared by later detectors
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: all tests still pass; the detector body contains no duplicated line-scanning logic

### TDD cycle 2 — DC-1 (alternation threshold undercount)

- [ ] [AI] **RED**: add a failing test asserting that `grep -Ec 'a|b|c' f` compared against a
      threshold above 1 yields one finding recommending the occurrence-unique form
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the test fails (detector absent)
      **Gherkin (binds) →** "Multi-term alternation threshold undercounts when terms share a line"

  ```gherkin
  Scenario: Multi-term alternation threshold undercounts when terms share a line
    Given a fixture file containing three search terms packed onto a single line
    When the clause "grep -Ec 'alpha|beta|gamma' fixture" is evaluated against a threshold of 3
    Then the command returns 1 and the threshold is not met despite every term being present
    And the registry's safe form "grep -ohE 'alpha|beta|gamma' fixture | sort -u | wc -l" returns 3
    And the safe form returns 3 for a fixture with the same terms on separate lines
  ```

- [ ] [AI] **GREEN**: implement the alternation-threshold detector
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the DC-1 test passes; no other test breaks
- [ ] [AI] **REFACTOR**: unify the DC-1 and DC-4 finding-emission shape
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: all tests still pass

### TDD cycle 3 — DC-3 (multi-file `grep -c`)

- [ ] [AI] **RED**: add a failing test asserting that `grep -c pattern f1 f2` compared against a
      single numeric threshold yields one finding stating the per-file `filename:count` output shape
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the test fails
      **Gherkin (binds) →** "A multi-file count clause is flagged as non-comparable"

  ```gherkin
  Scenario: A multi-file count clause is flagged as non-comparable
    Given an acceptance clause of the form "grep -c pattern file1 file2" compared against a single numeric threshold
    When the deterministic pre-flight pass evaluates the clause
    Then the pass reports a finding stating that multi-file grep -c emits per-file "filename:count" output
    And the finding notes the output ordering is not guaranteed stable
  ```

- [ ] [AI] **GREEN**: implement the multi-file detector
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the DC-3 test passes
- [ ] [AI] **REFACTOR**: fold DC-1 and DC-3 into one grep-clause parser
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: all tests still pass

### TDD cycle 4 — DC-5 (indented fence inside a list item)

- [ ] [AI] **RED**: add a failing test asserting that a fenced block indented past its list item's
      CommonMark content column yields one finding naming the indented-code-block misparse
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the test fails
      **Gherkin (binds) →** "A fence indented past its list item content column is caught mechanically"

  ```gherkin
  Scenario: A fence indented past its list item content column is caught mechanically
    Given a delivery checkbox whose fenced code block is indented six spaces inside a top-level list item
    When the deterministic pre-flight pass evaluates the document
    Then the pass reports a finding stating the block parses as an indented code block rather than a fenced one
    And the finding notes that Prettier reports the broken form as correctly formatted
    And the finding names the correct content-column indentation as the fix
  ```

- [ ] [AI] **GREEN**: implement the content-column indentation detector
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the DC-5 test passes
- [ ] [AI] **REFACTOR**: extract the list-item content-column computation into a named helper
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: all tests still pass

### Wiring

- [ ] [AI] Register the subcommand in `apps/rhino-cli/src/cli.rs` and
      `apps/rhino-cli/src/commands.rs` as `plan validate-acceptance <path>`
      — acceptance: `grep -ohE 'validate-acceptance|validate_acceptance' -r apps/rhino-cli/src | sort -u | wc -l`
      returns at least 2 (returns 0 today, verified during authoring)
- [ ] [AI] Add the Nx target `plan:acceptance-validation` to `apps/rhino-cli/project.json`
      — acceptance: `npx nx run rhino-cli:plan:acceptance-validation --help` exits 0
- [ ] [AI] Run the validator against this plan's own folder — the plan must pass its own validator
      — acceptance: the validator reports 0 findings against
      `plans/backlog/2026-07-20__plan-quality-gate-convergence/`
- [ ] [AI] Run the validator against fixture files reproducing each seed class
      — acceptance: each fixture yields at least 1 finding of its own class, and the corrected form
      of each fixture yields 0 — falsifiable in both directions

### Phase 2 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `npx nx run rhino-cli:test:unit` exits 0
- [ ] [AI] `npx nx run rhino-cli:specs:behavior:coverage` exits 0
- [ ] [AI] Validator reports 0 findings against this plan's folder and ≥1 against each trap fixture
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` reports zero failures

> **Pause Safety**: the validator is a new, additive subcommand that no other surface invokes yet —
> the repo builds and tests clean with it present and unused. Safe to stop. To resume:
> `npx nx run rhino-cli:test:unit`.

---

## Phase 3: Symmetric Empirical Verification

> _Suggested executor: `agent-maker`_

- [ ] [AI] Add a **Pre-Write Clause Simulation** section to `.claude/agents/plan-maker.md` requiring
      that every acceptance clause containing a shell command be executed against the real repo or a
      fixture in both the pre-edit and post-edit directions before the clause is written, that the
      observed output be recorded, and that an unsimulatable clause be rewritten or omitted — never
      written on faith; link the registry
      — acceptance: `grep -ohEi 'empirically simulate|empirical simulation|simulate the acceptance|both directions' .claude/agents/plan-maker.md | sort -u | wc -l`
      returns at least 2 (returns 0 today, verified during authoring)
- [ ] [AI] Replace `.claude/agents/plan-fixer.md` §7 Self-Verification with an empirical form: re-run
      the rewritten clause's own command and record the observed output; render any touched markdown
      fence through a CommonMark parser and confirm it parses as fenced; record APPLIED (verified) or
      FAILED (not applied)
      — acceptance: `grep -ohEi 'empirically simulate|empirical simulation|simulate the acceptance|both directions' .claude/agents/plan-fixer.md | sort -u | wc -l`
      returns at least 2 (returns 0 today) **and**
      `grep -ohEi 'CommonMark' .claude/agents/plan-fixer.md | sort -u | wc -l` returns 1 (returns 0
      today)
- [ ] [AI] Add the same authoring-time rule to
      `.claude/skills/plan-creating-project-plans/SKILL.md`, in the Pre-Write Verification section,
      linking the registry
      — acceptance: `grep -ohEi 'empirically simulate|empirical simulation|simulate the acceptance|both directions' .claude/skills/plan-creating-project-plans/SKILL.md | sort -u | wc -l`
      returns at least 2 (returns 0 today)
- [ ] [AI] Add a registry-vocabulary reference to `.claude/agents/plan-execution-checker.md` so
      execution-time findings use the same class IDs
      — acceptance: `grep -ohE 'plan-acceptance-defect-classes' .claude/agents/plan-execution-checker.md | sort -u | wc -l`
      returns 1 (returns 0 today)
- [ ] [AI] Verify no vendor-specific content entered any `repo-governance/` file edited in this phase
      per the [Governance Vendor-Independence Convention](../../../repo-governance/conventions/structure/governance-vendor-independence.md)
      — acceptance: `npx nx run rhino-cli:governance:vendor-audit-validation` exits 0

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] All four surface-edit acceptance clauses above return their stated post-edit values
- [ ] [AI] `npx nx run rhino-cli:governance:vendor-audit-validation` exits 0
- [ ] [AI] `npx nx run rhino-cli:instruction-size:validation` exits 0 — no agent file exceeded its
      size budget
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` reports zero failures

> **Pause Safety**: agent contracts now require simulation, which is strictly additive — an agent
> that ignores it behaves exactly as before. No surface is left half-migrated. Safe to stop. To
> resume: re-run the four acceptance clauses above.

---

## Phase 4: Class-Level Remediation Contract

> _Suggested executor: `agent-maker`_

- [ ] [AI] Add a **Class-Level Remediation** section to `.claude/agents/plan-fixer.md`: a finding
      that instantiates a registry class obliges the fixer to enumerate every instance of that class
      across all plan documents in the same pass, fix the whole class, and list each enumerated site
      with its disposition in the fix report
      — acceptance: `grep -ohEi 'class-level|class-wide|enumerate every instance|class closure|class-closure' .claude/agents/plan-fixer.md | sort -u | wc -l`
      returns at least 3 (returns 0 today, verified during authoring)
- [ ] [AI] Add a `## Class Sweep Enumeration` required section to the fix-report template in
      `.claude/agents/plan-fixer.md`, listing each site and disposition
      — acceptance: `grep -ohE 'Class Sweep Enumeration' .claude/agents/plan-fixer.md | sort -u | wc -l`
      returns 1 (returns 0 today)
- [ ] [AI] Add a **Class-Closure Verification** step to `.claude/agents/plan-checker.md` requiring
      the checker to independently re-run the class enumeration rather than re-checking only the
      originally flagged site, and to report a residual instance as a class-closure failure
      — acceptance: `grep -ohEi 'class closure|class-closure' .claude/agents/plan-checker.md | sort -u | wc -l`
      returns at least 1 (returns 0 today)
- [ ] [AI] Add the class-sweep obligation to
      `repo-governance/workflows/plan/plan-quality-gate.md` Step 3 notes
      — acceptance: `grep -ohEi 'class-level|class closure|class-closure' repo-governance/workflows/plan/plan-quality-gate.md | sort -u | wc -l`
      returns at least 1 (returns 0 today)

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] All four acceptance clauses above return their stated post-edit values
- [ ] [AI] `npx nx run rhino-cli:instruction-size:validation` exits 0
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` reports zero failures

> **Pause Safety**: the class contract is stated in both the fixer and checker, so neither side is
> left expecting a counterpart that does not exist. Safe to stop. To resume: re-run the four clauses.

---

## Phase 5: Scope Discipline — In-Surface vs Latent

> _Suggested executor: `agent-maker`_
>
> This is the plan's highest-risk phase. Every guard in
> [tech-docs.md DD-5](./tech-docs.md#dd-5--the-in-surface--latent-split-and-its-four-anti-loophole-guards)
> must land; a partial implementation is a loophole, not a partial benefit.

- [ ] [AI] Add a **Finding Surface Partition** step to `.claude/agents/plan-checker.md` requiring
      every finding to carry an explicit in-surface or latent label, with the in-surface ledger
      derived mechanically from `git diff` plus the fix report's Changed Files list — never from the
      checker's impression of what feels pre-existing
      — acceptance: `grep -ohEi 'in-surface|change surface|change-surface|latent' .claude/agents/plan-checker.md | sort -u | wc -l`
      returns at least 3 (returns 0 today, verified during authoring)
- [ ] [AI] Encode **guard 2 (provenance)** in the same section: a latent classification must cite
      `git log -L` evidence that the offending line range predates this chain; an uncitable
      classification defaults to in-surface
      — acceptance: `grep -ohE 'git log -L' .claude/agents/plan-checker.md | sort -u | wc -l` returns
      1 (returns 0 today)
- [ ] [AI] Encode **guard 3 (severity)**: any CRITICAL finding is in-surface regardless of provenance
      — acceptance: the checker section states this unconditionally; verified by reading the added
      section
- [ ] [AI] Encode **guard 4 (execution reachability)**: a latent finding located inside a delivery
      checkbox that execution will act on is promoted to in-surface, with the promotion rationale
      recorded in the audit report
      — acceptance: `grep -ohEi 'promot' .claude/agents/plan-checker.md | sort -u | wc -l` returns at
      least 1 (returns 0 today)
- [ ] [AI] Add a **Latent Finding Filing** step to `.claude/agents/plan-fixer.md`: at chain end the
      fixer creates `plans/backlog/YYYY-MM-DD__<slug>/` capturing every latent finding, and records
      the path in the fix report
      — acceptance: `grep -ohEi 'latent' .claude/agents/plan-fixer.md | sort -u | wc -l` returns at
      least 1 (returns 0 today)
- [ ] [AI] Add the partition to the audit-report structure in
      `repo-governance/workflows/plan/plan-quality-gate.md` §Final Audit Report Structure, so findings
      are grouped by surface as well as by criticality
      — acceptance: `grep -ohEi 'in-surface|latent' repo-governance/workflows/plan/plan-quality-gate.md | sort -u | wc -l`
      returns at least 2 (returns 0 today, verified during authoring)

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] All four guards are present in `plan-checker.md` — verified by reading the section, not
      by grep alone, since guard 3 is a semantic statement
- [ ] [AI] The latent-filing obligation is present in `plan-fixer.md`
- [ ] [AI] The audit-report structure groups by surface
- [ ] [AI] `npx nx run rhino-cli:instruction-size:validation` exits 0
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` reports zero failures

> **Pause Safety**: the partition is defined but termination still requires double-zero on all
> findings until Phase 6 rewires it — so stopping here leaves the gate strictly no weaker than
> today. This is the important safety property of splitting Phases 5 and 6. Safe to stop. To resume:
> re-read the Finding Surface Partition section and confirm all four guards.

---

## Phase 6: Iteration-Budget Shaping and Termination

> _Suggested executor: `repo-workflow-maker`_

- [ ] [AI] Insert a new **Step 0: Deterministic Pre-Flight** into
      `repo-governance/workflows/plan/plan-quality-gate.md`, running the Phase 2 validator before
      Step 1's semantic pass, with a non-zero result routed to the fixer before the semantic lens is
      spent
      — acceptance: `grep -ohEi 'Deterministic Pre-Flight|validate-acceptance' repo-governance/workflows/plan/plan-quality-gate.md | sort -u | wc -l`
      returns at least 1 (returns 0 today)
- [ ] [AI] Rewrite §Termination Criteria so `pass` requires: two consecutive zero **in-surface**
      threshold-level validations, verified class closure for every swept class, and the latent
      follow-up backlog plan existing on disk — the absence of that plan blocks `pass`
      — acceptance: `grep -ohEi 'in-surface|latent' repo-governance/workflows/plan/plan-quality-gate.md | sort -u | wc -l`
      returns at least 2, and the Termination Criteria section names the backlog-plan precondition —
      verified by reading the section
- [ ] [AI] Add the **single non-looping latent sweep** as an explicit workflow step between the
      in-surface double-zero and finalization
      — acceptance: the workflow contains a latent-sweep step stated as running exactly once —
      verified by reading the step
- [ ] [AI] Correct the falsified convergence target at
      `repo-governance/workflows/plan/plan-quality-gate.md` and in `.claude/agents/plan-checker.md`
      §Convergence Target: replace the bare "3-5 iterations" claim with the phased model and its
      separate budgets, citing the archived 17-iteration chain as the evidence
      — acceptance: `grep -ohE '3-5 iterations' repo-governance/workflows/plan/plan-quality-gate.md .claude/agents/plan-checker.md | sort -u | wc -l`
      returns 0 after the edit (returns 1 today — the phrase is present in both files, verified
      during authoring), and each file states the phased budgets instead
- [ ] [AI] Update the `termination` frontmatter field in
      `repo-governance/workflows/plan/plan-quality-gate.md` to describe the in-surface rule
      — acceptance: the frontmatter `termination:` value names the in-surface partition — verified by
      reading the frontmatter

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] Step 0 deterministic pre-flight present and ordered before Step 1
- [ ] [AI] Termination criteria name the in-surface partition, class closure, and the backlog-plan
      precondition
- [ ] [AI] `grep -ohE '3-5 iterations' repo-governance/workflows/plan/plan-quality-gate.md .claude/agents/plan-checker.md | sort -u | wc -l`
      returns 0
- [ ] [AI] `npx nx run rhino-cli:naming:workflows-validation` exits 0
- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` reports zero failures

> **Pause Safety**: the workflow is internally consistent — the partition defined in Phase 5 is now
> consumed by termination, and no step references a mechanism that does not exist. Safe to stop. To
> resume: re-read §Termination Criteria.

---

## Phase 7: Historical Replay, Bindings, and the ose-public PR

- [ ] [AI] Build replay fixtures reproducing the archived chain's five real fix-site defects
      (07-30 absent-file claim, 08-21 same-phase falsification, 08-31 alternation undercount, 09-14
      single-backtick fence, 09-26 six-space indented fence) under a scratch directory
      — acceptance: five fixture files exist
- [ ] [AI] Run the Phase 2 validator against the replay fixtures
      — acceptance: the three statically-detectable fixtures (08-31, 09-14, 09-26) each yield at
      least 1 finding, and their corrected forms each yield 0 — falsifiable both ways; the two
      semantic fixtures (07-30, 08-21) are recorded as checker-scope, not validator-scope
- [ ] [AI] Verify the AC-16 no-check-removed invariant: re-derive the `plan-checker` step inventory
      via `grep -ohE '\(Step 5[a-z] —[^)]*\)' .claude/agents/plan-checker.md | sort -u` and compare
      against the Phase 0 baseline recorded in `learnings.md`
      — acceptance: every Phase 0 baseline step is still present and the post-change count is greater
      than or equal to the baseline count
- [ ] [AI] Regenerate the secondary harness bindings: `npm run generate:bindings`
      — acceptance: exits 0; `.opencode/` and `.amazonq/` reflect the `.claude/` changes; no
      `.opencode/` or `.amazonq/` file was hand-edited at any point in this plan
- [ ] [AI] Validate binding sync: `npx nx run rhino-cli:naming:harness-validation` and the repo's
      harness sync validation
      — acceptance: both exit 0 with no drift reported

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck` — exits 0
- [ ] [AI] `npx nx affected -t lint` — exits 0
- [ ] [AI] `npx nx affected -t test:quick` — exits 0
- [ ] [AI] `npx nx affected -t specs:coverage` — exits 0
- [ ] [AI] Fix ALL failures, including preexisting issues not caused by these changes
- [ ] [AI] Re-run every failing check to confirm resolution — zero failures before pushing

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work. Commit preexisting fixes separately with appropriate conventional commit messages.

### Commit Guidelines

- [ ] [AI] Commit thematically — the registry, the validator, the agent contracts, and the workflow
      are four distinct concerns and get separate commits
- [ ] [AI] Follow Conventional Commits: `<type>(<scope>): <description>`
- [ ] [AI] Preexisting fixes get their own commits, separate from plan work
- [ ] [AI] Do NOT bundle unrelated changes into a single commit

### Push and CI

- [ ] [AI] Commit and push to `origin <pr-branch>`
- [ ] [AI] Open a draft PR against `main` with the plan folder linked in the description
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push — poll every 2 minutes with a
      single `gh run view --json status,conclusion` per wakeup; never tight-loop; never use
      `gh run watch`
- [ ] [AI] Fix any CI failure immediately and push a follow-up commit; repeat until ALL checks pass

### PR-Review Maker→Fixer Cycle

- [ ] [AI] Cycle 1: `pr-review-maker` review, then `pr-review-fixer` remediation
      — acceptance: every inline comment answered; CI green before Cycle 2 starts
- [ ] [AI] Cycle 2: `pr-review-maker` review, then `pr-review-fixer` remediation
      — acceptance: every inline comment answered; CI green before Cycle 3 starts
- [ ] [AI] Cycle 3: `pr-review-maker` review, then `pr-review-fixer` remediation
      — acceptance: every inline comment answered; CI green
- [ ] [HUMAN] Merge the PR to `main`
      — the human merges on their own schedule once all three cycles are complete and CI is green;
      observable resume signal for the agent: `gh pr view <n> --json state` reports `MERGED`

### Phase 7 Gate

> All checks below must pass before starting Phase 8 or Phase 9.

- [ ] [AI] Replay fixtures each yield ≥1 finding, corrected forms each yield 0
- [ ] [AI] AC-16 inventory comparison passes — no validation step was removed
- [ ] [AI] `npm run generate:bindings` exits 0 and sync validation reports no drift
- [ ] [AI] All three PR-review cycles complete with CI green
- [ ] [HUMAN] PR merged — `gh pr view <n> --json state` reports `MERGED`

> **Pause Safety**: `ose-public` carries the complete, self-consistent change set and CI is green on
> `main`. The two sibling repos are simply not yet updated, which is a normal steady state for this
> repo. Safe to stop indefinitely. To resume: `git -C <repo> log --oneline -1 origin/main`.

---

## Phase 8: Propagate to `ose-primer`

- [ ] [AI] Provision a worktree in `ose-primer` and sync it with `origin/main`
      — acceptance: the worktree exists and is at `origin/main`
- [ ] [AI] Port surfaces 1-7 and 11 from the Surface Inventory
      ([tech-docs.md §Surface Inventory](./tech-docs.md#surface-inventory)) as byte-identical text
      where the repos do not legitimately diverge
      — acceptance: `test -f repo-governance/development/quality/plan-acceptance-defect-classes.md`
      succeeds in `ose-primer` and its eight-class count clause returns 8
- [ ] [AI] Port `apps/rhino-cli` changes **byte-identically** per the
      [SDLC Gate Standard](../../../docs/reference/sdlc-gate-standard.md)
      — acceptance: `diff -r` between the two repos' `apps/rhino-cli` reports no differences
- [ ] [AI] Regenerate bindings: `npm run generate:bindings`
      — acceptance: exits 0 with no drift
- [ ] [AI] Run local quality gates, then commit and push to `origin <pr-branch>`; open a draft PR
- [ ] [AI] Run the three PR-review cycles with CI green between each
- [ ] [HUMAN] Merge the `ose-primer` PR — resume signal: `gh pr view <n> --json state` reports `MERGED`

### Phase 8 Gate

- [ ] [AI] `diff -r` on `apps/rhino-cli` between `ose-public` and `ose-primer` reports no differences
- [ ] [AI] `ose-primer` CI green on `main`
- [ ] [HUMAN] PR merged

> **Pause Safety**: `ose-primer` matches `ose-public`; `ose-infra` remains on the prior state, which
> is independently coherent. Safe to stop. To resume: re-run the `diff -r` byte-identity check.

---

## Phase 9: Propagate to `ose-infra`

- [ ] [AI] Provision a worktree in `ose-infra` and sync it with `origin/main`
      — acceptance: the worktree exists and is at `origin/main`
- [ ] [AI] Port surfaces 1-7 and 11, applying the repo-relevance gate — no infra-private content
      flows outward, and no `ose-public` content that is meaningless in `ose-infra` is force-fitted
      — acceptance: `test -f repo-governance/development/quality/plan-acceptance-defect-classes.md`
      succeeds in `ose-infra` and its eight-class count clause returns 8
- [ ] [AI] Port `apps/rhino-cli` byte-identically
      — acceptance: `diff -r` between `ose-public` and `ose-infra` `apps/rhino-cli` reports no
      differences
- [ ] [AI] Regenerate bindings: `npm run generate:bindings` — acceptance: exits 0 with no drift
- [ ] [AI] Run local quality gates, then commit and push to `origin <pr-branch>`; open a draft PR
- [ ] [AI] Run the three PR-review cycles with CI green between each
- [ ] [HUMAN] Merge the `ose-infra` PR — resume signal: `gh pr view <n> --json state` reports `MERGED`

### Phase 9 Gate

- [ ] [AI] `diff -r` on `apps/rhino-cli` across all three repos reports no differences
- [ ] [AI] `ose-infra` CI green on `main`
- [ ] [HUMAN] PR merged

> **Pause Safety**: all three repos are converged. Safe to stop. To resume: re-run the tri-repo
> byte-identity check.

---

## Phase 10: Knowledge Capture

> _Triage every surviving `learnings.md` entry before archival. See the
> [Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md)._

- [ ] [AI] Apply the litmus test to every `learnings.md` entry — keep only if a durable surface would
      catch this automatically next time; discard the rest with a one-line reason
      — acceptance: every entry has either a route or a discard reason
- [ ] [AI] **Append any newly discovered defect class to the registry** — this plan's own execution
      is expected to surface at least one, exactly as its authoring surfaced DC-2b
      — acceptance: either a new DC-N entry exists in
      `repo-governance/development/quality/plan-acceptance-defect-classes.md`, or `learnings.md`
      records the explicit reason none was found
- [ ] [AI] Apply the **secret/sensitivity gate** to every surviving entry — sanitize any secret,
      credential, token, or private hostname to a `<placeholder>` token, or discard if unsanitizable
      — acceptance: `learnings.md` contains no raw secret
- [ ] [AI] Apply the **repo-relevance gate** — infra-private content stays in `ose-infra` only and is
      never cross-routed into `ose-public`/`ose-primer`
      — acceptance: no infra-private content appears in this repo's routed output
- [ ] [AI] Route each surviving learning to exactly one durable home; code-homed learnings
      (`apps/`, `libs/`, tests) are ALWAYS filed as a separate `plans/backlog/<slug>/` plan and NEVER
      landed inline
      — acceptance: every entry records its terminal routing state
- [ ] [AI] If no generalizable learning surfaced, record the explicit escape in `learnings.md`:
      `No generalizable learnings — <one-line reason>`
      — acceptance: `learnings.md` is never silently empty

### Phase 10 Gate

> All checks below must pass before Plan Archival.

- [ ] [AI] Every `learnings.md` entry is in a terminal state, or the explicit "none" escape is present
- [ ] [AI] No code-homed learning landed inline in this plan's own commits or PRs
- [ ] [AI] Any newly discovered defect class is appended to the registry in all three repos

> **Pause Safety**: `learnings.md` is fully triaged; no future process depends on querying it later.
> Safe to stop. To resume: re-read `learnings.md` and confirm every entry is terminal.

---

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify the Knowledge Capture phase is complete — every `learnings.md` entry reached a
      terminal state or the explicit "none" escape is recorded; both safety gates were applied
- [ ] [AI] Verify ALL quality gates pass (local + CI) in all three repos
- [ ] [AI] Verify the six open questions in [README.md](./README.md) were grilled and their outcomes
      recorded in the plan documents
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/plan-quality-gate-convergence/ plans/done/YYYY-MM-DD__plan-quality-gate-convergence/`
      using the completion date, not the creation date
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date
- [ ] [AI] Update any other READMEs referencing this plan
- [ ] [AI] Commit the archival: `chore(plans): move plan-quality-gate-convergence to done`

### Not Applicable

- **Manual UI verification (Playwright MCP)** — not applicable: this plan adds and changes no web UI.
- **Manual API verification (curl)** — not applicable: this plan adds and changes no HTTP endpoint.
- **Rule-15 three-tester retest** — not applicable: no web UI feature change.
- **Rule-16 API exploratory retest** — not applicable: no REST or GraphQL endpoint change.
- **UI-design-funnel** — not applicable; exemption recorded in
  [tech-docs.md §UI-Design-Funnel Exemption](./tech-docs.md#ui-design-funnel-exemption).
