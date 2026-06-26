# Delivery Checklist — Instruction-File Size-Budget Gate

**Legend**: every step is `[AI]` — this plan has **no `[HUMAN]` gates**; git-mechanical steps
(worktree, commit, push to `main`) are `[AI]` per the repo's git-mechanical-steps rule. Each
code step uses the RED → GREEN → REFACTOR template with a file path, a verbatim command, and
an acceptance criterion. **Phase gates** must pass before the next phase starts.

**Execution model** (per request):

- **Part A — `ose-public`** runs first, in a git worktree at
  `worktrees/instruction-file-size-budget-gate/` (Phases 0–6), then **commits and pushes to
  `ose-public` `origin/main`**.
- **Part B — `ose-primer` + `ose-infra`** run **in parallel** only after Part A has landed on
  `ose-public` `main` (Phases 7 and 8 — independent, no ordering between them).
- **Part C — cross-repo parity verification + archival** (Phase 9).

Each sibling repo carries its own granular sub-steps and **fixes its own existing
over-budget instruction files** — no repo ships a gate it currently fails.

---

## Part A — `ose-public`

### Phase 0 — Worktree + baseline `[AI]`

- [ ] **0.1** `[AI]` Create the worktree: `git worktree add worktrees/instruction-file-size-budget-gate -b instruction-file-size-budget-gate` (lands under `worktrees/` via the repo `WorktreeCreate` hook). Then **both** `npm install` **and** `npm run doctor -- --fix` inside it (worktree toolchain init).
- [ ] **0.2** `[AI]` Invoke `repo-setup-manager`: baseline-build rhino-cli (`nx build rhino-cli`), run `nx run rhino-cli:test:unit` + `nx run rhino-cli:lang:rust` to confirm a green start.
- [ ] **0.3** `[AI]` Capture current sizes: `wc -c AGENTS.md CLAUDE.md .amazonq/rules/00-agents-md.md` + resolved tree (`CLAUDE.md`+`AGENTS.md`). Record under "Baseline sizes (ose-public)" below.
- [ ] **0.4** `[AI]` Confirm where `agents-md-size` is currently invoked (grep hooks/CI). Note whether it is actually wired into a blocking gate today — Phase 2 fixes any gap.

**Baseline sizes (ose-public)** (fill in 0.3): _AGENTS.md = \_\_\_\_ B · CLAUDE.md = \_\_\_\_ B ·
resolved tree = \_\_\_\_ B · .amazonq/rules/00-agents-md.md = \_\_\_\_ B_

**Gate 0**: rhino-cli unit tests + rust coverage green; baseline sizes recorded.

### Phase 1 — Config + generalized validator + deterministic category (TDD) `[AI]`

- [ ] **1.1** `[AI]` Add the committed budget config.
  - RED: unit test in `instruction_size.rs` loading `instruction-size-budget.yaml`, asserting the `AGENTS.md` surface has `fail == 30000`. `cargo test … instruction_size::` → fails.
  - GREEN: create `instruction-size-budget.yaml` ([tech-docs §3](./tech-docs.md#3-config-file)) + `BudgetConfig`/`Surface`/`ResolvedTree` types + loader. → passes.
  - REFACTOR: dedupe loader against `env-contract.yaml` parsing patterns.
  - **Acceptance**: config parses; `AGENTS.md` surface `fail == 30000`.
- [ ] **1.2** `[AI]` Parameterize the tier classifier.
  - RED: tests `classify(24000,24000,27000,30000)==ok`, `classify(28000,…)==warn`, `classify(31000,…)==fail`. → fails.
  - GREEN: extract `classify(size,target,warn,fail)` into `instruction_size.rs`. → passes.
  - REFACTOR: re-point `agents_md_size.rs::classify` at the shared fn (alias preserved).
  - **Acceptance**: four tier boundaries assert; old `agents_md_size` tests still green.
- [ ] **1.3** `[AI]` Multi-file scan with no-op globs.
  - RED: `check_instruction_sizes` over a temp repo with over-ceiling `AGENTS.md` and no `.github/copilot-instructions.md`; assert one `fail` for `AGENTS.md`, no finding for the absent glob. → fails.
  - GREEN: implement glob + stat + classify; skip no-match globs. → passes.
  - REFACTOR: deterministic ordering.
  - **Acceptance**: absent globs are no-ops; present over-ceiling files are `fail`.
- [ ] **1.4** `[AI]` Claude resolved-tree check.
  - RED: fixture `CLAUDE.md` importing `@AGENTS.md` whose sum > 38000; assert a `resolved-tree` `fail`. → fails.
  - GREEN: implement `resolve_tree_size` (parse `@path`, recurse depth ≤ 4, sum bytes), classify against `ResolvedTree`. → passes.
  - REFACTOR: extract import-line parsing; depth + cycle guard.
  - **Acceptance**: resolved-tree finding emitted with correct severity.
- [ ] **1.5** `[AI]` CLI command + output modes + alias + **remediation pointer**.
  - RED: test `run` returns non-zero on any `fail`; `text`/`json`/`markdown` render; **every `fail` message contains the progressive-disclosure remediation pointer + path** (`repo-governance/principles/content/progressive-disclosure.md`); `convention agents-md-size` still measures only `AGENTS.md`. → fails.
  - GREEN: add `commands/convention_validate_instruction_size.rs` (`SCHEMA = rhino-cli/instruction-size/v1`); make `agents-md-size` a scoped alias; register `"instruction-size"` in `convention_audit::MEMBERS`; append the remediation pointer to every `fail` message ([tech-docs §6.1](./tech-docs.md#61-remediation-when-the-gate-fails)). → passes.
  - REFACTOR: share envelope/printing helpers with emoji/license commands.
  - **Acceptance**: non-zero on `fail`; three modes work; `fail` messages carry the remediation pointer; alias intact.
- [ ] **1.6** `[AI]` Emit `instruction-size` as a **deterministic preflight category**.
  - RED: test that `repo-governance audit -o json` envelope (`schema rhino-cli/repo-governance-audit/v1`) includes a category named `instruction-size` with the budget findings. → fails.
  - GREEN: register an `instruction_size` category module under `application/repo_governance/` and add it to the `repo-governance audit` orchestrator's category list (alongside `layer-coherence`, `traceability-audit`, `vendor-audit`) per [tech-docs §5.4](./tech-docs.md#54-deterministic-preflight-integration). → passes.
  - REFACTOR: reuse the standalone validator's finding shape; no duplicated logic.
  - **Acceptance**: the preflight JSON carries an `instruction-size` category that the checker can consume.

**Gate 1**: `nx run rhino-cli:test:unit` + `nx run rhino-cli:lang:rust` (≥90% lines) green; `cargo run … -- convention validate instruction-size -o text` runs (currently reports `AGENTS.md` `fail` — expected until Phase 3); `repo-governance audit -o json` carries the new category.

### Phase 2 — Wiring (pre-push + pre-commit + PR quality gate) `[AI]`

- [ ] **2.1** `[AI]` Add the `instruction-size:validation` Nx target to `apps/rhino-cli/project.json` ([tech-docs §5.1](./tech-docs.md#51-nx-target)). **Acceptance**: `nx run rhino-cli:instruction-size:validation` resolves and runs.
- [ ] **2.2** `[AI]` Extend the `.husky/pre-push` changed-path block with the instruction-file glob gate ([tech-docs §5.2](./tech-docs.md#52-pre-push-hook)). **Acceptance**: `shellcheck .husky/pre-push` warning-clean; the new `if` mirrors the existing ones.
- [ ] **2.3** `[AI]` Keep pre-commit coverage: `instruction-size` rides `convention audit` (member added in 1.5), so it runs at pre-commit. **Acceptance**: a staged over-budget instruction file is flagged at pre-commit.
- [ ] **2.4** `[AI]` **PR quality gate**: add a step running `npx nx run rhino-cli:instruction-size:validation` to `.github/workflows/commons-quality-gate.yml` (the `pull_request` + `push:main` gate — natural home: the "Markdown quality gate" job, or a dedicated "Instruction-size budget" step) ([tech-docs §5.3](./tech-docs.md#53-pr-quality-gate)). **Acceptance**: `actionlint .github/workflows/commons-quality-gate.yml` clean; the step runs on PRs.

**Gate 2**: target + pre-push + pre-commit + PR-gate wired; hooks/workflow lint-clean. (The gate now _fails_ a push/PR because `AGENTS.md` is over ceiling — the intended trigger for Phase 3.)

### Phase 3 — Fix the existing violation: trim `AGENTS.md` under budget `[AI]`

> The worked example of the sanctioned remediation
> ([tech-docs §6.1](./tech-docs.md#61-remediation-when-the-gate-fails)): apply **progressive
> disclosure**; never delete a rule, compress to dense prose, or split into another
> auto-loaded file.

- [ ] **3.1** `[AI]` List the inline-expanded `AGENTS.md` sections that duplicate content already linked into `repo-governance/` (candidates: "Current Apps" + "Web Sites" duplication, verbose Markdown-Quality / Cross-Language / Git-Hooks gate prose, inline AI-Agents roster). Record the list here before editing.
- [ ] **3.2** `[AI]` Trim **by progressive disclosure**: replace each duplicated block with a one-line summary + existing `See` link, lifting detail to its canonical `repo-governance/` home. **No rule deleted, no dense-prose compression, no move into another auto-loaded file.** Target ≤ 24,000 B; minimum ≤ 30,000 B.
- [ ] **3.3** `[AI]` Self-review the trimmed `AGENTS.md` diff against a rule-inventory checklist (every pre-trim rule still present via summary + link; links resolve; meaning preserved).
- [ ] **3.4** `[AI]` Re-run `nx run rhino-cli:instruction-size:validation` → **exits 0** (`AGENTS.md` ≤ 30,000 B; resolved tree ≤ 38,000 B).
- [ ] **3.5** `[AI]` `npm run lint:md` + `npx nx run rhino-cli:links:validation` + `npx nx run rhino-cli:cross-vendor:parity-validation`; re-sync bindings if a binding surface changed (`npm run generate:bindings`).

**Gate 3**: `instruction-size:validation` exits 0 on the live repo; markdown/link/parity gates green; rule-inventory self-review recorded.

### Phase 4 — Governance convention + propagation `[AI]`

- [ ] **4.1** `[AI]` Invoke `repo-rules-maker` to author `repo-governance/conventions/structure/instruction-file-size-budget.md` — monitored file class, budget table, enforcement points (pre-push hard gate; pre-commit + PR-gate backstop; deterministic preflight; `repo-rules-checker` Step 6), rationale + durable source citations, a **"When the gate fails" remediation section** mandating progressive disclosure and forbidding the three anti-fixes ([tech-docs §6.1](./tech-docs.md#61-remediation-when-the-gate-fails)), and `Principles Implemented/Respected` (linking [progressive-disclosure.md](../../../repo-governance/principles/content/progressive-disclosure.md)) + `Vision Supported` (traceability).
- [ ] **4.2** `[AI]` Propagation sweep (via `repo-rules-maker` / edits): `repo-governance/conventions/README.md` index entry; `AGENTS.md` one-line gate entry + `See` link (under Markdown Quality / Cross-Language Lint Gates — **summary only**); `repo-governance/development/infra/nx-targets.md` target entry.
- [ ] **4.3** `[AI]` Backlink the principle: add the new convention to `progressive-disclosure.md` "Related Conventions" + a "How It Applies → Instruction-File Size Budget" example (two-way traceability).
- [ ] **4.4** `[AI]` `npm run generate:bindings` (keep `.opencode/` / `.amazonq/` in parity); `npx nx run rhino-cli:governance:vendor-audit-validation` clean.

**Gate 4**: convention exists with traceability + remediation sections; principle backlinks the convention; reference surfaces updated; vendor-audit + bindings parity green.

### Phase 5 — Deterministic checker + workflow integration + specs `[AI]`

- [ ] **5.1** `[AI]` Make `repo-rules-checker` **consume the deterministic preflight** for size: edit Step 0.5 "Consume Deterministic Preflight" — add an `instruction-size` row to the category→skip table (the AI checker must NOT re-derive byte counts); edit Step 6 ("AGENTS.md Size Check" → "Instruction-File Size Budget") to defer to the preflight finding, judge only qualitative bloat across the whole instruction-file class, and recommend **progressive disclosure** as the remediation. Re-sync bindings.
- [ ] **5.2** `[AI]` Edit `repo-governance/workflows/repo/repo-rules-quality-gate.md`: list `instruction-size` as a **fourth preflight category** in the Step 0.5 paragraph (so the workflow tracks it deterministically via the JSON envelope), reference it in the Step 6 annotation, and add a "What changed" note.
- [ ] **5.3** `[AI]` Companion specs (two-path rule): add/extend `specs/apps/rhino/**` Gherkin for the `instruction-size` validator + the deterministic-category emission (mirror [prd.md](./prd.md)).
  - RED: `nx run rhino-cli:specs:coverage` flags the new validator as lacking a companion feature → fails.
  - GREEN: add feature file(s); wire consumption. → passes.
  - REFACTOR: align scenario naming with existing rhino specs.
  - **Acceptance**: `specs:coverage` green for rhino-cli.
- [ ] **5.4** `[AI]` `npx nx run rhino-cli:naming:harness-validation` + `validate:sync` checks clean after agent edits.

**Gate 5**: checker Step 0.5 + Step 6 consume the deterministic preflight; workflow lists the category; `specs:coverage` green; naming/sync validators clean.

### Phase 6 — `ose-public` verify + land on `main` `[AI]`

- [ ] **6.1** `[AI]` Full affected pre-push dry run in the worktree: `npx nx affected -t typecheck lint test:quick specs:coverage` + `nx run rhino-cli:instruction-size:validation` all green.
- [ ] **6.2** `[AI]` Behavioral proof of the gate (non-destructive): in a scratch copy, push a throwaway edit taking `AGENTS.md` over 30k, confirm the pre-push hook **blocks** it, then discard the scratch edit.
- [ ] **6.3** `[AI]` Confirm the original Claude Code 40k warning no longer fires (resolved tree ≤ 38,000 B).
- [ ] **6.4** `[AI]` Stage explicit paths (no `-A`), commit (conventional), **push the worktree branch and merge to `ose-public` `origin/main`** (git-mechanical = `[AI]`); remove the worktree (`git worktree remove`).

**Gate 6**: all gates green; changes on `ose-public` `main`; worktree removed.

---

## Part B — `ose-primer` + `ose-infra` (run **in parallel** after Part A lands)

> Mechanism: copy this plan folder into each sibling repo at the start of its phase so the
> same checklist drives execution there (the multi-repo parity method). `rhino-cli` is ported
> across all three repos, so the validator + config + target + category land in each repo's
> own `rhino-cli` copy. Each repo **fixes its own existing over-budget instruction files**.

### Phase 7 — `ose-primer` propagation `[AI]` (parallel with Phase 8)

- [ ] **7.1** `[AI]` Worktree + baseline in `ose-primer` (`npm install` + `npm run doctor -- --fix`); capture its instruction-file sizes (`AGENTS.md`, `CLAUDE.md`, resolved tree, `.amazonq/rules/*`). Record under "Baseline sizes (ose-primer)".
- [ ] **7.2** `[AI]` Port the validator + `instruction-size-budget.yaml` + deterministic category to primer's `rhino-cli` (mirror Phase 1). Primer is the polyglot-demo template — keep the same budget numbers; the config's globs are repo-relative.
- [ ] **7.3** `[AI]` Wire pre-push + pre-commit + PR quality gate (mirror Phase 2; use primer's own hook/workflow files).
- [ ] **7.4** `[AI]` **Fix primer's existing violations**: trim primer's `AGENTS.md` (and any other over-budget surface) under budget by progressive disclosure (mirror Phase 3); re-run the gate → exits 0.
- [ ] **7.5** `[AI]` Author/propagate the convention + principle backlink + reference sweep in primer's governance tree (mirror Phase 4); re-sync bindings.
- [ ] **7.6** `[AI]` Deterministic checker + workflow + specs integration in primer (mirror Phase 5); `specs:coverage` green.
- [ ] **7.7** `[AI]` Verify + commit + push to `ose-primer` `origin/main`; remove worktree.

**Baseline sizes (ose-primer)** (fill in 7.1): _AGENTS.md = \_\_\_\_ B · CLAUDE.md = \_\_\_\_ B
· resolved tree = \_\_\_\_ B_

**Gate 7**: primer gate green; convention + deterministic integration landed on primer `main`.

### Phase 8 — `ose-infra` propagation `[AI]` (parallel with Phase 7)

> `ose-infra` is a **bare repo with worktrees** — commit to `main` via a worktree; the top
> dir fails `git status`. Private repo; same governance machinery.

- [ ] **8.1** `[AI]` Create/enter an `ose-infra` worktree off `main`; `npm install` + `npm run doctor -- --fix`; capture instruction-file sizes. Record under "Baseline sizes (ose-infra)".
- [ ] **8.2** `[AI]` Port the validator + config + deterministic category to infra's `rhino-cli` (mirror Phase 1).
- [ ] **8.3** `[AI]` Wire pre-push + pre-commit + PR quality gate (mirror Phase 2; infra's own hook/workflow files).
- [ ] **8.4** `[AI]` **Fix infra's existing violations**: trim infra's `AGENTS.md` (and any other over-budget surface) under budget by progressive disclosure (mirror Phase 3); re-run the gate → exits 0.
- [ ] **8.5** `[AI]` Author/propagate the convention + principle backlink + reference sweep in infra's governance tree (mirror Phase 4); re-sync bindings.
- [ ] **8.6** `[AI]` Deterministic checker + workflow + specs integration in infra (mirror Phase 5); `specs:coverage` green.
- [ ] **8.7** `[AI]` Verify + commit + push to `ose-infra` `main` via the worktree; clean up.

**Baseline sizes (ose-infra)** (fill in 8.1): _AGENTS.md = \_\_\_\_ B · CLAUDE.md = \_\_\_\_ B ·
resolved tree = \_\_\_\_ B_

**Gate 8**: infra gate green; convention + deterministic integration landed on infra `main`.

---

## Part C — Cross-repo verification + archival

### Phase 9 — Parity verification + archival `[AI]`

- [ ] **9.1** `[AI]` Confirm gate **mechanics parity** across all three repos: same validator surface (`convention instruction-size` + alias), same `instruction-size-budget.yaml` numbers, same Nx target name, same pre-push glob gate, same PR-gate step, same deterministic preflight category, same checker Step 6 + workflow wiring. Record divergences (legitimately repo-specific: which instruction surfaces exist).
- [ ] **9.2** `[AI]` Confirm every repo's `instruction-size:validation` exits 0 and no resolved tree exceeds 38,000 B.
- [ ] **9.3** `[AI]` Archive: move the plan folder to `plans/done/2026-MM-DD__instruction-file-size-budget-gate/` in `ose-public` (and mirror archival in the sibling repos per their convention).

**Gate 9**: three-repo parity confirmed; all gates green; plan archived.

---

## Notes

- **Plan-only right now**: this checklist is authored and pushed to `ose-public` `main` as
  documentation. **No implementation is performed yet** — execution begins in a later session.
- **No self-failing gate**: in every repo, the trim phase (3 / 7.4 / 8.4) lands no later than
  the wiring phase so the repo is green when the work is considered done.
- **Parallelism**: Phases 7 and 8 are independent and run concurrently; both depend only on
  Part A having landed on `ose-public` `main`.
