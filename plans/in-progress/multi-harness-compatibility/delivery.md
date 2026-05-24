# Delivery — Multi-Harness Compatibility

## Worktree

Worktree path: `worktrees/multi-harness-compatibility/`

Provision before execution (run from repo root):

```bash
claude --worktree multi-harness-compatibility
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Environment Setup

- [ ] Provision worktree: `claude --worktree multi-harness-compatibility` (creates
      `worktrees/multi-harness-compatibility/` in repo root).
- [ ] Initialize toolchain in the root worktree (not the new worktree): `npm install && npm run doctor -- --fix`.
      Verify by `npm run doctor` exits 0. See
      [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md).
- [ ] Build the Rust CLI once: `npx nx build rhino-cli` — `apps/rhino-cli/dist/rhino-cli` exists.
- [ ] Verify the existing vendor-audit baseline is green: run
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor-audit repo-governance/`
      — exits 0 before any changes.
- [ ] Verify existing rhino-cli tests pass before changes: `npx nx run rhino-cli:test:quick` — exits 0.

## Phase 1 — Governance neutrality (vendor-audit + convention)

- [ ] Edit `repo-governance/conventions/structure/governance-vendor-independence.md`: add the new
      coding-agent product names (`\bJunie\b`, `\bJetBrains\b`, `Amazon Q\b`, `\bAntigravity\b`,
      `Pi Coding Agent`, `pi\.dev`, `\bEarendil\b`) to the "Coding-agent / harness product names" table, add
      binding paths (`\.junie/`, `\.amazonq/`, `\.pi/`, `\.gemini/`, `\.agent/`, `\.agents/`) to the
      "Vendor-specific binding directory paths" table, update the combined audit regex, and add FP notes for
      `Amazon Q`/`pi`/`agy`. Per `tech-docs.md` §Vendor-Audit Extension.
  - _Suggested executor: `repo-rules-maker`_
  - Acceptance: the file's forbidden-terms tables and combined regex include every new term; FP notes added.
- [ ] TDD (Red): add a failing Gherkin scenario and Rust integration test asserting that a seeded string
      `Junie` (and `Amazon Q`, `Antigravity`) in a temp governance fixture is reported by the vendor-audit.
      Edit `specs/apps/rhino/behavior/cli/gherkin/repo-governance/repo-governance-vendor-audit.feature` and the
      paired rhino-cli test under `apps/rhino-cli/tests/`. Verify the new test **fails**:
      `npx nx run rhino-cli:test:unit`.
  - _Suggested executor: `swe-rust-dev`_
- [ ] TDD (Green): edit `apps/rhino-cli/src/internal/repo_governance/vendor_audit.rs` to add the new term and
      path patterns with FP guards (no bare `\bQ\b`/`\bpi\b`/`\bagy\b`). Verify
      `npx nx run rhino-cli:test:unit` — new tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] TDD (Refactor): run the full audit on the repo:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor-audit repo-governance/`
      — exits 0 (existing prose stays neutral; any newly-flagged leak is fixed at source or allowlisted).
  - _Suggested executor: `swe-rust-dev`_
- [ ] Add FP-safety scenarios (AC2) to the vendor-audit feature file: math constant `pi` in plain prose and a
      vendor name inside a "Platform Binding Examples" section are NOT reported. Verify
      `npx nx run rhino-cli:test:unit` passes.
  - _Suggested executor: `swe-rust-dev`_

## Phase 2 — Multi-harness binding convention + catalog

- [ ] Create `repo-governance/conventions/structure/multi-harness-binding.md` documenting: the two-tier
      strategy (AD2), AGENTS.md-canonical rule (AD1), no-shadowing rule (AD3 — `GEMINI.md`, `.junie/AGENTS.md`,
      `AGENTS.override.md` must never carry divergent content), and mechanical-generation rule (AD4). Include a
      Principles/Conventions-respected section per convention-writing standards.
  - _Suggested executor: `repo-rules-maker`_
  - Acceptance: file exists, kebab-case, single H1, links resolve, vendor names only inside allowlisted regions.
- [ ] Update `docs/reference/platform-bindings.md`: expand the Platform Binding Directories table to all nine
      named harnesses + OpenCode with columns from `tech-docs.md` §Harness Compatibility Matrix; document
      provenance of pre-existing `.github/{agents,prompts,skills}` and `.codex/` bindings; add the no-shadowing
      note. Verify links: `npm run lint:md` — exits 0.
  - _Suggested executor: `docs-maker`_
  - Acceptance: each of the nine + OpenCode has a row recording root instruction file + AGENTS.md-native status
    - binding status (AC3).
- [ ] Run `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor-audit repo-governance/`
      — exits 0 after the new convention is added.

## Phase 3 — Binding emitter (rhino-cli) + binding files

- [ ] TDD (Red): add a failing Gherkin feature + rhino-cli test for the Amazon Q bridge emitter — given
      `AGENTS.md`, the emitter writes `.amazonq/rules/00-agents-md.md` pointing to `AGENTS.md` and a default
      agent JSON whose `resources` glob `file://AGENTS.md`. New file under
      `specs/apps/rhino/behavior/cli/gherkin/agents/` + paired test under `apps/rhino-cli/tests/`. Verify it
      **fails**: `npx nx run rhino-cli:test:unit`.
  - _Suggested executor: `swe-rust-dev`_
- [ ] TDD (Green): implement the emitter in `apps/rhino-cli/src/internal/agents/` (new module or extend
      `sync.rs`) and wire any subcommand/flag in `apps/rhino-cli/src/main.rs`. Verify
      `npx nx run rhino-cli:test:unit` — passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Generate the Amazon Q bridge files by running the emitter; confirm `.amazonq/rules/00-agents-md.md` and
      the default agent JSON exist and reference `AGENTS.md` (AC4). Do NOT duplicate `AGENTS.md` content
      verbatim. Acceptance:
      `test -f .amazonq/rules/00-agents-md.md && grep 'AGENTS.md' .amazonq/rules/00-agents-md.md` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Decide on optional thin pointers (default = none per AD2); record the decision in a new
      `§Optional Thin Pointers` section of `docs/reference/platform-bindings.md`. Acceptance: a sentence
      recording the decision exists in `docs/reference/platform-bindings.md` and `npm run lint:md` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] If thin pointers were decided: emit each pointer (`.github/copilot-instructions.md`,
      `.cursor/rules/000-agents-md.mdc`, `.windsurf/rules/000-agents-md.md`) via `rhino-cli` and verify each is
      a pure pointer to `AGENTS.md`. Acceptance: `grep 'AGENTS.md' <pointer-file>` returns the pointer text and
      does not contain any body paragraph from `AGENTS.md`.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Verify `.gitignore` tracks new binding dirs: `git check-ignore .amazonq/rules/00-agents-md.md` returns
      nothing (not ignored). Fix `.gitignore` if needed.
- [ ] Confirm no shadowing file was created: `test ! -f GEMINI.md && test ! -f AGENTS.override.md && test ! -f .junie/AGENTS.md`
      (or, if any exists, it is a pure `AGENTS.md` pointer) (AC5).

## Phase 3.5 — Deterministic pre-push parity guard (no agent)

Implements AD7: a deterministic, agent-free `rhino-cli` check that fails when a committed binding file drifts
from `AGENTS.md` or when a binding directory lacks a catalog row. Distinct from the Phase 4 agent workflow
(which handles external convention drift).

- [ ] TDD (Red): add a failing Gherkin feature + rhino-cli test for `agents validate-bindings` — given a
      committed `.amazonq/rules/00-agents-md.md` deliberately mutated to differ from a regenerate, the command
      exits non-zero; given a binding dir with no catalog row, it exits non-zero. New feature under
      `specs/apps/rhino/behavior/cli/gherkin/agents/` + paired test under `apps/rhino-cli/tests/`. Verify it
      **fails**: `npx nx run rhino-cli:test:unit`.
  - _Suggested executor: `swe-rust-dev`_
- [ ] TDD (Green): implement the deterministic guard in `apps/rhino-cli/src/internal/agents/` (new
      `binding_validator.rs` or extend `sync_validator.rs`) and wire the `agents validate-bindings` subcommand
      in `apps/rhino-cli/src/main.rs`. The guard re-derives each generated binding file from `AGENTS.md` in
      memory, asserts byte-equality with the committed file, and asserts every binding dir on disk has a row in
      `docs/reference/platform-bindings.md`. It performs NO network calls and invokes NO agent. Verify
      `npx nx run rhino-cli:test:unit` — passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Add a `validate:harness-bindings` script to `package.json` wrapping
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- agents validate-bindings`.
      Acceptance: `npm run validate:harness-bindings` exits 0 on the clean tree.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Wire the guard into `.husky/pre-push`: append `npm run validate:harness-bindings` to the existing
      deterministic validation chain (alongside `validate:repo-governance-vendor-audit` and
      `validate:cross-vendor-parity`). Acceptance: `grep 'validate:harness-bindings' .husky/pre-push` returns a
      match; a manual `git push --dry-run`-style run of the hook chain exits 0 on the clean tree.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Prove the guard blocks drift: mutate `.amazonq/rules/00-agents-md.md`, run
      `npm run validate:harness-bindings` — exits non-zero; restore the file — exits 0 (AC9).
  - _Suggested executor: `swe-rust-dev`_

## Phase 4 — Compatibility-audit workflow + agents

- [ ] Create `.claude/agents/repo-harness-compatibility-checker.md` — checker that, for each supported harness,
      delegates to `web-research-maker` to fetch current config conventions, diffs against
      `docs/reference/platform-bindings.md` + committed binding files, and writes a dual-labelled drift audit to
      `generated-reports/`. Follow agent frontmatter + naming conventions.
  - _Suggested executor: `agent-maker`_
  - Acceptance: `test -f .claude/agents/repo-harness-compatibility-checker.md` succeeds and
    `npx nx run rhino-cli:validate:naming-agents` exits 0 (the new agent name conforms; this validator takes
    no path argument and checks all agents).
- [ ] Create `.claude/agents/repo-harness-compatibility-fixer.md` — fixer that applies validated catalog/binding
      updates from a drift audit and re-validates before applying.
  - _Suggested executor: `agent-maker`_
  - Acceptance: `test -f .claude/agents/repo-harness-compatibility-fixer.md` succeeds and
    `npx nx run rhino-cli:validate:naming-agents` exits 0 (the new agent name conforms; validates all agents,
    no path argument).
- [ ] Create `repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md` following the workflow
      pattern (frontmatter: name/title/goal/termination/inputs/outputs; phases; Gherkin success criteria),
      delegating to the two new agents and `web-research-maker` (AC6). Add it to
      `repo-governance/workflows/repo/README.md`.
  - _Suggested executor: `repo-workflow-maker`_
  - Acceptance: `npx nx run rhino-cli:validate:naming-workflows` exits 0 (the new workflow name conforms;
    this validator takes no path argument and checks all workflows).
- [ ] Sync agents to OpenCode: `npm run sync:claude-to-opencode` then `npm run validate:opencode` [Repo-grounded — package.json scripts] — both exit 0;
      `.opencode/agents/repo-harness-compatibility-checker.md` and `...-fixer.md` generated.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Add the two agents to the `AGENTS.md` agent catalog (Validation + Fixing lists) and `.claude/agents/README.md`.
  - _Suggested executor: `repo-rules-maker`_
  - Acceptance: `grep 'repo-harness-compatibility-checker' AGENTS.md` and
    `grep 'repo-harness-compatibility-fixer' AGENTS.md` both return matches; same grep passes against
    `.claude/agents/README.md`.
- [ ] Validate workflow naming: `npx nx run rhino-cli:validate:naming-workflows` — exits 0 (name matches
      `repo-harness-compatibility-quality-gate`).
- [ ] Invoke `repo-workflow-checker` on `repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md`;
      resolve all findings. Acceptance: `repo-workflow-checker` reports zero HIGH or CRITICAL findings.

## Phase 5 — Specs coverage

- [ ] Ensure every new/changed rhino-cli behavior has a paired Gherkin feature under `specs/apps/rhino/`
      (vendor-audit extension scenarios, binding-emitter feature). Run
      `npx nx run rhino-cli:spec-coverage` — exits 0 (AC7).
  - _Suggested executor: `specs-maker`_
- [ ] Run `npx nx run rhino-cli:test:quick` — exits 0.

## Phase 5.5 — Update all related Markdown files

Closing documentation sweep so no `.md` references a stale binding/vendor/agent/workflow set (AC10).

- [ ] Build the authoritative target list by grep: run
      `grep -rln --include='*.md' -e 'Platform Binding' -e 'platform-bindings' -e 'Gemini CLI' -e 'Future\**:.*\.cursor' -e 'repo-parity-checker' .`
      and review each hit for staleness. Acceptance: a reviewed list exists (paste into the commit body or an
      Open Questions note).
- [ ] Update `AGENTS.md`: refresh the "Platform Bindings Catalog" sub-list and the `**Future**:` bindings line
      under "Platform Binding Examples" to reflect the nine harnesses; add the two new agents to the agent
      roster lists. Acceptance: `grep 'repo-harness-compatibility' AGENTS.md` returns matches and the
      `**Future**:` line no longer lists now-supported bindings.
  - _Suggested executor: `repo-rules-maker`_
- [ ] Update `CLAUDE.md` only if a new binding affects its documented dual-mode format-differences section.
      Acceptance: `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor-audit repo-governance/`
      still exits 0 (CLAUDE.md prose stays vendor-neutral outside allowlisted regions).
  - _Suggested executor: `repo-rules-maker`_
- [ ] Update index docs: `.claude/agents/README.md`, `repo-governance/workflows/repo/README.md`,
      `repo-governance/workflows/README.md`, `repo-governance/conventions/README.md`, and (if present)
      `docs/reference/README.md`. Acceptance: each index references the new convention/workflow/agents and
      `npm run lint:md` exits 0.
  - _Suggested executor: `docs-maker`_
- [ ] Add a downstream-propagation note to `repo-governance/conventions/structure/ose-primer-sync.md` that the
      new bindings propagate to `ose-primer`. Acceptance: the file mentions the harness bindings and the
      vendor-audit exits 0.
  - _Suggested executor: `repo-rules-maker`_
- [ ] Re-grep for staleness: the Phase-5.5 grep returns no remaining stale references (AC10). Run
      `npm run lint:md` — exits 0.

## Local Quality Gates (Before Push)

- [ ] Run affected typecheck: `npx nx affected -t typecheck` — exits 0.
- [ ] Run affected linting: `npx nx affected -t lint` — exits 0.
- [ ] Run affected quick tests: `npx nx affected -t test:quick` — exits 0.
- [ ] Run affected spec coverage: `npx nx affected -t spec-coverage` — exits 0.
- [ ] Run markdown lint: `npm run lint:md` — exits 0 (run `npm run lint:md:fix` first if needed).
- [ ] Run the governance vendor-audit:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor-audit repo-governance/`
      — exits 0.
- [ ] Run the deterministic binding-parity guard: `npm run validate:harness-bindings` — exits 0 (also runs
      automatically in `.husky/pre-push`).
- [ ] Fix ALL failures found — including preexisting issues not caused by these changes (root cause orientation).

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes. This
> follows the root cause orientation principle — proactively fix preexisting errors encountered during work.

## Phase 6 — Governance rule propagation + validation

- [ ] Invoke `repo-rules-maker` to finalize/propagate the governance rules authored in Phases 1–2 and 4
      (vendor-independence update, multi-harness-binding convention, catalog/agent index entries), ensuring
      cross-links and indexes are consistent. Acceptance: `npm run lint:md` — exits 0; and
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor-audit repo-governance/`
      exits 0 after propagation.
- [ ] Run the `repo-rules-quality-gate` workflow in strict mode over the changed governance scope (invoke
      `repo-rules-checker` → `repo-rules-fixer` iteratively). Per
      [Repository Rules Quality Gate](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md).
      Acceptance: two consecutive `repo-rules-checker` runs over the changed governance files both return zero
      HIGH or CRITICAL findings (AC8).
- [ ] Re-run the vendor-audit and `npx nx affected -t test:quick lint typecheck spec-coverage` — all exit 0.

## Manual Behavioral Verification (CLI)

This plan touches a CLI and governance docs, not web UI or HTTP APIs — Playwright MCP and curl assertions are
**not applicable**. CLI behavior is verified by running the binary directly:

- [ ] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor-audit repo-governance/`
      — exits 0; seed a temp `Junie`/`Amazon Q` string and confirm it is reported, then remove it.
- [ ] Run the Amazon Q bridge emitter and inspect `.amazonq/rules/00-agents-md.md` — it references `AGENTS.md`
      and does not duplicate its body.
- [ ] (Optional, manual) Trigger the `repo-harness-compatibility-quality-gate` workflow and confirm it emits a
      drift report under `generated-reports/` citing web sources.

## Commit Guidelines

- [ ] Commit changes thematically — group related changes into logically cohesive commits.
- [ ] Follow Conventional Commits format: `<type>(<scope>): <description>`.
- [ ] Suggested split: (1) `feat(rhino-cli): extend vendor-audit for new harness vendors`,
      (2) `docs(governance): add multi-harness-binding convention`,
      (3) `docs(reference): expand platform-bindings catalog to nine harnesses`,
      (4) `feat(rhino-cli): emit Amazon Q binding bridge`,
      (5) `feat(rhino-cli): add deterministic binding-parity pre-push guard`,
      (6) `feat(agents): add harness-compatibility checker/fixer + workflow`,
      (7) `test(rhino-cli): spec coverage for harness bindings`,
      (8) `docs: update related markdown for multi-harness bindings`.
- [ ] Do NOT bundle unrelated fixes into a single commit.

## Post-Push Verification

- [ ] Push changes to `main`.
- [ ] Monitor GitHub Actions workflows for the push (poll every 3 minutes; do not use `gh run watch`).
- [ ] Verify all CI checks pass.
- [ ] If any CI check fails, fix immediately and push a follow-up commit.
- [ ] Do NOT proceed to archival until CI is green.

## Plan Archival

- [ ] Verify ALL delivery checklist items are ticked.
- [ ] Verify ALL quality gates pass (local + CI).
- [ ] Move plan folder from `plans/in-progress/` to `plans/done/` via `git mv` with completion date prefix:
      `git mv plans/in-progress/multi-harness-compatibility plans/done/YYYY-MM-DD__multi-harness-compatibility`.
- [ ] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] Update any other READMEs that reference this plan.
- [ ] Commit: `chore(plans): move multi-harness-compatibility to done`.
