# Delivery Checklist — Instruction-File Size-Budget Gate

**Legend**: `[AI]` = AI-executable · `[HUMAN]` = requires a human (approvals, judgement calls).
Git-mechanical steps (branch/commit/push) are `[AI]` per repo convention. Each code step uses
the RED → GREEN → REFACTOR template with a file path, a verbatim command, and an acceptance
criterion. **Phase gates** must pass before the next phase starts.

---

## Phase 0 — Baseline `[AI]`

- [ ] **0.1** `[AI]` Invoke `repo-setup-manager`: `npm install`, `npm run doctor -- --fix`,
      then baseline-build rhino-cli (`nx build rhino-cli`) and run
      `nx run rhino-cli:test:unit` + `nx run rhino-cli:lang:rust` to confirm a green start.
- [ ] **0.2** `[AI]` Capture current sizes for the record:
      `wc -c AGENTS.md CLAUDE.md .amazonq/rules/00-agents-md.md` and the resolved tree
      (`CLAUDE.md` + `AGENTS.md`). Paste into this file under "Baseline sizes" below.
- [ ] **0.3** `[AI]` Confirm where `agents-md-size` is currently invoked (grep hooks/CI). If
      it is _not_ actually wired into any blocking hook today, note it — Phase 2 fixes that.

**Baseline sizes** (fill in 0.2): _AGENTS.md = \_**\_ B · CLAUDE.md = \_\_** B · resolved tree =
\_**\_ B · .amazonq/rules/00-agents-md.md = \_\_** B_

**Gate 0**: rhino-cli unit tests + rust coverage green; baseline sizes recorded.

---

## Phase 1 — Config + generalized validator (TDD) `[AI]`

- [ ] **1.1** `[AI]` Add the committed budget config.
  - RED: write a unit test in `instruction_size.rs` that loads
    `instruction-size-budget.yaml` and asserts the parsed `AGENTS.md` surface has
    `fail == 30000`. Run `cargo test --manifest-path apps/rhino-cli/Cargo.toml
instruction_size::` → fails (no config, no parser).
  - GREEN: create `instruction-size-budget.yaml` (per [tech-docs §3](./tech-docs.md#3-config-file))
    and the `BudgetConfig`/`Surface`/`ResolvedTree` types + loader. Re-run → passes.
  - REFACTOR: dedupe the loader against existing `env-contract.yaml` parsing patterns.
  - **Acceptance**: config parses; `AGENTS.md` surface `fail == 30000`.
- [ ] **1.2** `[AI]` Parameterize the tier classifier.
  - RED: tests asserting `classify(24000, 24000, 27000, 30000) == ok`,
    `classify(28000, …) == warn`, `classify(31000, …) == fail`. Run → fails.
  - GREEN: extract `classify(size, target, warn, fail)` from the hardcoded version in
    `agents_md_size.rs`; move to `instruction_size.rs`. Re-run → passes.
  - REFACTOR: re-point `agents_md_size.rs::classify` at the shared fn (alias preserved).
  - **Acceptance**: all four tier boundaries assert correctly; old `agents_md_size` tests
    still green.
- [ ] **1.3** `[AI]` Multi-file scan with no-op globs.
  - RED: test `check_instruction_sizes` over a temp repo with `AGENTS.md` (over ceiling) and
    no `.github/copilot-instructions.md`; assert one `fail` finding for `AGENTS.md` and **no**
    finding for the absent glob. Run → fails.
  - GREEN: implement glob + stat + classify; skip no-match globs. Re-run → passes.
  - REFACTOR: tidy glob iteration; ensure deterministic ordering.
  - **Acceptance**: absent globs are no-ops; present over-ceiling files are `fail`.
- [ ] **1.4** `[AI]` Claude resolved-tree check.
  - RED: fixture `CLAUDE.md` importing `@AGENTS.md` where the sum exceeds 38000; assert a
    `resolved-tree` finding with severity `fail`. Run → fails.
  - GREEN: implement `resolve_tree_size` (parse `@path` directives, recurse depth ≤ 4, sum
    bytes), classify against `ResolvedTree`. Re-run → passes.
  - REFACTOR: extract import-line parsing; cap + cycle-guard.
  - **Acceptance**: resolved-tree finding emitted with correct severity.
- [ ] **1.5** `[AI]` CLI command + output modes + alias.
  - RED: test that `convention_validate_instruction_size::run` returns non-zero when any
    finding is `fail`, and that `text`/`json`/`markdown` render; test that
    `convention agents-md-size` still measures only `AGENTS.md`. Run → fails.
  - GREEN: add `commands/convention_validate_instruction_size.rs`
    (`SCHEMA = rhino-cli/instruction-size/v1`); make `agents-md-size` delegate as a scoped
    alias; register `"instruction-size"` in `convention_audit::MEMBERS`. Re-run → passes.
  - REFACTOR: share envelope/printing helpers with the emoji/license commands.
  - **Acceptance**: command exits non-zero on `fail`; three output modes work; alias intact.

**Gate 1**: `nx run rhino-cli:test:unit` + `nx run rhino-cli:lang:rust` (≥90% lines) green;
`cargo run … -- convention validate instruction-size -o text` runs (will currently report
`AGENTS.md` as `fail` — expected until Phase 3).

---

## Phase 2 — Wiring `[AI]`

- [ ] **2.1** `[AI]` Add the `instruction-size:validation` Nx target to
      `apps/rhino-cli/project.json` (per [tech-docs §5.1](./tech-docs.md#51-nx-target)).
      **Acceptance**: `nx run rhino-cli:instruction-size:validation` resolves and runs.
- [ ] **2.2** `[AI]` Extend the `.husky/pre-push` changed-path block with the instruction-file
      glob gate (per [tech-docs §5.2](./tech-docs.md#52-pre-push-hook)).
      **Acceptance**: `shellcheck .husky/pre-push` is warning-clean; the new `if` mirrors the
      existing ones.
- [ ] **2.3** `[AI]` Confirm pre-commit/CI coverage: `instruction-size` is a
      `convention audit` member so it rides the existing markdown/convention CI gate. If a
      direct CI step is clearer, add it to the markdown/convention workflow.
      **Acceptance**: the validator runs in CI on instruction-file changes.

**Gate 2**: target + hook wired; hooks shellcheck-clean. (The gate will _fail_ a push right
now because `AGENTS.md` is over ceiling — that is the intended trigger to do Phase 3.)

---

## Phase 3 — Bring `AGENTS.md` under budget `[AI]` + `[HUMAN]` review

- [ ] **3.1** `[AI]` Identify inline-expanded sections in `AGENTS.md` that duplicate content
      already linked into `repo-governance/` (candidates from the size audit: the full
      "Current Apps" + "Web Sites" duplication, the verbose Markdown-Quality / Cross-Language
      / Git-Hooks gate prose, the inline AI-Agents roster). List them here before editing.
- [ ] **3.2** `[AI]` Trim: replace each duplicated block with a one-line summary + existing
      `See` link. **Do not delete any rule** — only collapse content that already lives behind
      a link. Target ≤ 24,000 B; minimum ≤ 30,000 B.
- [ ] **3.3** `[HUMAN]` Review the trimmed `AGENTS.md` diff — confirm no rule lost, links
      resolve, meaning preserved. (Judgement call; not AI-final.)
- [ ] **3.4** `[AI]` Re-run `nx run rhino-cli:instruction-size:validation` →
      **must exit 0** (`AGENTS.md` ≤ 30,000 B, resolved tree ≤ 38,000 B).
- [ ] **3.5** `[AI]` `npm run lint:md` + `npx nx run rhino-cli:links:validation` +
      `npx nx run rhino-cli:cross-vendor:parity-validation` to confirm the trim broke no link
      or parity invariant. Re-sync bindings if any binding surface changed
      (`npm run generate:bindings`).

**Gate 3**: `instruction-size:validation` exits 0 on the live repo; markdown/link/parity
gates green; human sign-off on the `AGENTS.md` diff.

---

## Phase 4 — Governance convention + propagation `[AI]`

- [ ] **4.1** `[AI]` Invoke `repo-rules-maker` to author
      `repo-governance/conventions/structure/instruction-file-size-budget.md` — monitored
      file class, the budget table, enforcement points (pre-push hard gate; pre-commit/CI
      backstop; `repo-rules-checker` Step 6), rationale + durable source citations,
      `Principles Implemented/Respected` + `Vision Supported` sections (traceability).
- [ ] **4.2** `[AI]` Propagation sweep (via `repo-rules-maker` / edits):
      `repo-governance/conventions/README.md` index entry; `AGENTS.md` one-line gate entry +
      `See` link (under Markdown Quality / Cross-Language Lint Gates — **summary only**, no
      inline expansion); `repo-governance/development/infra/nx-targets.md` target entry.
- [ ] **4.3** `[AI]` `npm run generate:bindings` to keep `.opencode/` / `.amazonq/` in parity;
      `npx nx run rhino-cli:governance:vendor-audit-validation` clean.

**Gate 4**: convention exists with traceability sections; all reference surfaces updated;
vendor-audit + bindings parity green.

---

## Phase 5 — Checker + workflow integration + specs `[AI]`

- [ ] **5.1** `[AI]` Edit `.claude/agents/repo-rules-checker.md` Step 6: rename "AGENTS.md
      Size Check" → "Instruction-File Size Budget"; update the deterministic-gate annotation
      to point at the `instruction-size` gate; broaden the qualitative check to the whole
      instruction-file class (bloat the mechanical gate can't judge). Re-sync bindings.
- [ ] **5.2** `[AI]` Edit `repo-governance/workflows/repo/repo-rules-quality-gate.md`: name
      the `instruction-size` deterministic validator among the convention-tier gates in the
      Step 0.5 paragraph and the Step 6 reference; add a "What changed" note.
- [ ] **5.3** `[AI]` Companion specs (two-path rule): add/extend `specs/apps/rhino/**` Gherkin
      for the `instruction-size` validator (mirror the Gherkin in [prd.md](./prd.md)).
      Validate with the consuming rhino-cli test.
  - RED: `nx run rhino-cli:specs:coverage` flags the new validator as lacking a companion
    feature → fails.
  - GREEN: add the feature file(s); wire consumption. Re-run → passes.
  - REFACTOR: align scenario naming with existing rhino specs.
  - **Acceptance**: `specs:coverage` green for rhino-cli.
- [ ] **5.4** `[AI]` `npx nx run rhino-cli:naming:harness-validation` +
      `validate:sync` checks clean after agent edits.

**Gate 5**: checker Step 6 + workflow updated and binding-synced; `specs:coverage` green;
naming/sync validators clean.

---

## Phase 6 — Parity hand-off (note only, not executed) `[AI]`

- [ ] **6.1** `[AI]` Record a parity hand-off note (here + in
      `plans/in-progress/standardize-rhino-cli-sdlc-parity/` if relevant): the convention +
      `instruction-size` validator + config + checker Step 6 + workflow reference must
      propagate to `ose-primer` and `ose-infra` via the
      [multi-repo parity planning workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md).
      **No code changes downstream in this plan.**

**Gate 6**: hand-off note recorded.

---

## Final verification (V) `[AI]` + `[HUMAN]`

- [ ] **V.1** `[AI]` Full affected pre-push dry run: `npx nx affected -t typecheck lint
test:quick specs:coverage` + `nx run rhino-cli:instruction-size:validation` all green.
- [ ] **V.2** `[HUMAN]` Manual pre-push proof: stage a throwaway edit that pushes `AGENTS.md`
      over 30k, confirm the hook **blocks** the push; revert. (Behavioral proof of FR7.)
- [ ] **V.3** `[AI]` Confirm the original Claude Code 40k warning no longer fires (resolved
      tree ≤ 38,000 B).
- [ ] **V.4** `[HUMAN]` Archive: move plan to `plans/done/2026-MM-DD__instruction-file-size-budget-gate/`.

---

## Notes

- **Plan-only commit**: this delivery doc is authored and pushed to `main` first
  (documentation), with **no implementation** yet, per the current request. Execution begins
  in a later session.
- **No self-failing gate**: Phase 2 wires the gate, but Phase 3 (trim) is gated to land so the
  repo is green before the work is considered done. If splitting across PRs, Phase 3 must
  merge no later than Phase 2.
