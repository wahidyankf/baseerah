---
title: "Delivery: Harness/Vendor Neutrality Blueprint — Phase 1"
---

# Delivery Checklist: Harness/Vendor Neutrality Blueprint — Phase 1

## Worktree

Worktree path: `worktrees/harness-vendor-neutrality-blueprint/`

Provision before execution:

```bash
claude --worktree harness-vendor-neutrality-blueprint
```

**See**: [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment Setup

- [ ] Run `npm install` from repo root — must exit 0.
- [ ] Run `npm run doctor -- --fix` — verify all required tools are present.
- [ ] Run `npm run sync:claude-to-opencode` as a baseline check — must exit 0 (confirms
      rhino-cli is buildable and `agents sync` runs cleanly before the rename).
- [ ] Run `git diff --quiet .opencode/ .amazonq/` — must exit 0 (baseline is clean).

## Phase 1: package.json — Add generate:bindings and Remove Old Script

- [ ] Edit `package.json`: add `"generate:bindings"` where `"sync:claude-to-opencode"` currently
      sits with value `"cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- agents sync && cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- agents emit-bindings"`.
      Verify: `node -e "const p=require('./package.json'); console.log(p.scripts['generate:bindings'])"` — output must be the full cargo command chain.

- [ ] Edit `package.json`: **delete** `"sync:claude-to-opencode"` entirely (hard delete, no alias).
      Verify: `node -e "const p=require('./package.json'); console.log(p.scripts['sync:claude-to-opencode'])"` — output must be `undefined`.

- [ ] Edit `package.json`: change `"validate:config"` from
      `"npm run validate:claude && npm run sync:claude-to-opencode && npm run validate:opencode"` to
      `"npm run validate:claude && npm run generate:bindings && npm run validate:opencode"`.
      Verify: `node -e "const p=require('./package.json'); console.log(p.scripts['validate:config'])"` — must contain `generate:bindings`.

- [ ] Run `npm run generate:bindings` — must exit 0 with both `agents sync` and `agents emit-bindings` completing.

- [ ] Run `git diff --quiet .opencode/ .amazonq/` — must exit 0.

- [ ] Run `npm run validate:config` — must exit 0.

- [ ] **Do NOT commit yet** — all phases complete first; all commits land together in Phase 4.

## Phase 2: Documentation Sweep (governance + docs + scripts)

### Governance files

- [ ] Edit `repo-governance/development/agents/ai-agents.md`: replace all 5 occurrences of
      `sync:claude-to-opencode` with `generate:bindings`.
      Verify: `grep "sync:claude-to-opencode" repo-governance/development/agents/ai-agents.md` — zero matches.

- [ ] Edit `repo-governance/development/agents/model-selection.md`: replace all 2 occurrences.
      Verify: `grep "sync:claude-to-opencode" repo-governance/development/agents/model-selection.md` — zero matches.

- [ ] Edit `repo-governance/development/quality/code.md`: replace all occurrences.
      Verify: `grep "sync:claude-to-opencode" repo-governance/development/quality/code.md` — zero matches.

- [ ] Edit `repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md` in two steps:
  - Step A: replace all occurrences of `sync:claude-to-opencode` with `generate:bindings` (covers Invariant 3 tool string and any other references). Verify: `grep "sync:claude-to-opencode" repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md` — zero matches.
  - Step B: extend Invariant 3 diff check — replace `git diff --quiet .opencode/` with `git diff --quiet .opencode/ .amazonq/`. Verify: `grep "git diff --quiet .opencode/ .amazonq/" repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md` — must return at least one match.

- [ ] Edit `repo-governance/workflows/repo/repo-rules-quality-gate.md`: replace the 1 occurrence.
      Verify: `grep "sync:claude-to-opencode" repo-governance/workflows/repo/repo-rules-quality-gate.md` — zero matches.

- [ ] Edit `CLAUDE.md`: replace the 1 occurrence.
      Verify: `grep "sync:claude-to-opencode" CLAUDE.md` — zero matches.
      Note: `AGENTS.md` has zero occurrences — no edit needed.

### Docs reference files

- [ ] Edit `docs/reference/platform-bindings.md`: replace the 1 occurrence.
      Verify: `grep "sync:claude-to-opencode" docs/reference/platform-bindings.md` — zero matches.

- [ ] Edit `docs/reference/ai-model-benchmarks.md`: replace the 1 occurrence.
      Verify: `grep "sync:claude-to-opencode" docs/reference/ai-model-benchmarks.md` — zero matches.

### Shell scripts

- [ ] Edit `apps/rhino-cli/scripts/validate-cross-vendor-parity.sh`: replace all 2 occurrences.
      Verify: `grep "sync:claude-to-opencode" apps/rhino-cli/scripts/validate-cross-vendor-parity.sh` — zero matches.

## Phase 3: Agent Definition and Skill Files Sweep

- [ ] Edit `.claude/agents/repo-harness-compatibility-fixer.md`: replace all 8 occurrences (frontmatter + body).
      Verify: `grep "sync:claude-to-opencode" .claude/agents/repo-harness-compatibility-fixer.md` — zero matches.

- [ ] Edit `.claude/agents/repo-harness-compatibility-checker.md`: replace the 1 occurrence.
      Verify: `grep "sync:claude-to-opencode" .claude/agents/repo-harness-compatibility-checker.md` — zero matches.

- [ ] Edit `.claude/agents/repo-rules-fixer.md`: replace the 1 occurrence.
      Verify: `grep "sync:claude-to-opencode" .claude/agents/repo-rules-fixer.md` — zero matches.

- [ ] Edit `.claude/agents/README.md`: replace the 1 occurrence.
      Verify: `grep "sync:claude-to-opencode" .claude/agents/README.md` — zero matches.

- [ ] Edit `.claude/agents/agent-maker.md`: replace the 1 occurrence in description frontmatter.
      Verify: `grep "sync:claude-to-opencode" .claude/agents/agent-maker.md` — zero matches.

- [ ] Edit `.claude/agents/web-research-maker.md`: replace the 1 occurrence.
      Verify: `grep "sync:claude-to-opencode" .claude/agents/web-research-maker.md` — zero matches.

- [ ] Edit `.claude/skills/agent-developing-agents/SKILL.md`: replace the 1 occurrence.
      Verify: `grep "sync:claude-to-opencode" .claude/skills/agent-developing-agents/SKILL.md` — zero matches.

- [ ] Run `npm run generate:bindings` to sync all `.claude/agents/` edits to `.opencode/agents/`. Verify exits 0.

- [ ] Verify mirrors updated: `grep "sync:claude-to-opencode" .opencode/agents/*.md` — zero matches.

## Phase 4: Coordinated Commit and Push

All changes from Phases 1–3 are committed here in three domain commits then pushed together.
This ordering ensures no individual commit has `generate:bindings` in docs but absent from
`package.json`.

- [ ] Run comprehensive grep to confirm ZERO remaining occurrences:

```bash
grep -r "sync:claude-to-opencode" \
  --include="*.md" --include="*.json" --include="*.sh" --include="*.rs" \
  . | grep -v "node_modules\|\.git\|target/\|generated-reports/\|plans/\|worktrees/"
```

Expected: **zero matches**. Any match is a missed file — fix before committing.

### Commit Guidelines

Commit changes thematically using [Conventional Commits](https://www.conventionalcommits.org/)
format: `<type>(<scope>): <description>`. The three commits below are pre-split by domain
(package.json / governance+docs+scripts / agents+skills) — do not bundle them into a single
commit.

- [ ] Commit 1 (package.json first):
      `chore(package.json): add generate:bindings, remove sync:claude-to-opencode`

- [ ] Commit 2 (governance + docs + scripts):
      `docs(governance): replace sync:claude-to-opencode with generate:bindings`

- [ ] Commit 3 (agent definitions + skills):
      `chore(agents): replace sync:claude-to-opencode with generate:bindings`

- [ ] Run final quality gate. Fix ALL failures found — not only those caused by this plan's
      changes. Pre-existing failures must be fixed before pushing (root cause orientation principle).

```bash
npm run generate:bindings                               # exits 0
git diff --quiet .opencode/ .amazonq/                  # exits 0
npm run validate:config                                 # exits 0
npm run validate:harness-bindings                      # exits 0
npx nx affected -t typecheck lint test:quick spec-coverage  # all pass
npm run lint:md                                         # zero violations
```

- [ ] Push all three commits: `git push origin main`

- [ ] Verify GitHub Actions CI passes. Monitor with `gh run list --branch main --limit 5` at
      3-minute intervals; confirm all checks green before proceeding to Phase 5.

## Phase 5: Governance Propagation — repo-rules-maker + repo-rules-quality-gate

This phase ensures the harness-neutral npm script naming is fully reflected in `repo-governance/`
as a standing convention and that all governance docs are internally consistent after the Phase
2–3 sweep.

- [ ] Invoke `repo-rules-maker`: ask it to check whether a new or updated convention entry is
      needed in `repo-governance/` to document the harness-neutral npm script naming pattern
      (`generate:` namespace, vendor-neutral script names, one script per logical operation). If
      [Multi-Harness Binding Convention](../../../repo-governance/conventions/structure/multi-harness-binding.md)
      already covers this, record that determination — no new file needed. If a gap exists,
      create or update the appropriate convention file.
      Verify: `npm run lint:md` exits 0 on any new/modified governance files.

- [ ] Run [Repository Rules Quality Gate workflow](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md)
      in **strict mode** (default):

  ```
  Run repository rules quality gate workflow in strict mode
  ```

  Iterate until zero CRITICAL/HIGH/MEDIUM findings. Apply fixes with `repo-rules-fixer` and
  re-run until double-zero achieved.

- [ ] Confirm `repo-governance/` vendor-audit passes:

```bash
cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor-audit repo-governance/
```

Must exit 0. Any vendor-audit finding is blocking — fix prose to vendor-neutral terms first.

- [ ] Commit any governance files created or modified:
      `docs(governance): document harness-neutral npm script convention`
      (or `docs(governance): no new convention needed — coverage confirmed in multi-harness-binding.md`
      if the maker determined no new file was required).

## Phase 6: Plan Archival

- [ ] Verify all checklist items in Phases 0–5 are ticked.

- [ ] Rename and move the plan folder (replace `YYYY-MM-DD` with today's date):

```bash
git mv plans/in-progress/harness-vendor-neutrality-blueprint \
       plans/done/YYYY-MM-DD__harness-vendor-neutrality-blueprint
```

- [ ] Update `plans/in-progress/README.md`: remove this plan's entry.

- [ ] Update `plans/done/README.md`: add this plan's entry.

- [ ] Commit: `chore(plans): move harness-vendor-neutrality-blueprint to done`

## Quality Gates Summary

All of the following must pass before this plan is considered done:

```bash
npm run generate:bindings
git diff --quiet .opencode/ .amazonq/
npm run validate:config
npm run validate:harness-bindings
npx nx affected -t typecheck lint test:quick spec-coverage
npm run lint:md
grep -r "sync:claude-to-opencode" . \
  --include="*.md" --include="*.json" --include="*.sh" \
  | grep -v "node_modules\|\.git\|target/\|generated-reports/\|plans/\|worktrees/"
cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml \
  -- repo-governance vendor-audit repo-governance/
# repo-rules-quality-gate: zero CRITICAL/HIGH/MEDIUM findings on two consecutive checks
```

## Post-Push CI Verification

After pushing to `origin main`:

1. Run `gh run list --branch main --limit 3` to get the latest workflow run ID
2. Poll every 3 minutes with `gh run view <run-id> --json status,conclusion`
3. If any check fails, investigate root cause and fix — do not bypass hooks or skip checks
4. Confirm all checks green before declaring the plan complete
