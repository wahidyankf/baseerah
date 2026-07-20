# Plan Quality Gate — Faster Convergence Without Quality Loss

**Status**: Not Started
**Created**: 2026-07-20
**Delivery Mode**: `worktree-to-pr`

## Context

The [plan-quality-gate workflow](../../../repo-governance/workflows/plan/plan-quality-gate.md) is the
maker-checker-fixer loop that validates a plan document before execution starts. Its stated
convergence target is **3-5 iterations** ([Repo-grounded] — `plan-quality-gate.md:237`,
`plan-checker.md` §Convergence Target).

On 2026-07-20 a real chain against
`plans/in-progress/parallel-orchestration-shared-machine-governance/` took **17 iterations** to reach
its first zero — more than triple the stated target. The full chain survives on disk as 16 audit
reports and 2 fix reports in `generated-reports/` (`plan__b0a12b*__2026-07-20--*`), spanning
07-01 through 09-57 [Repo-grounded].

That chain is this plan's primary requirements input. It was not a low-quality chain — the checker
found real defects every single iteration, and the resulting plan is genuinely better for it. The
problem is **cost and terminability**, not rigor: the loop spent its expensive semantic lens
re-finding mechanical defects, re-injected defects at its own fix sites, re-discovered the same
defect class at new sites across three consecutive iterations, and finally drifted into an unbounded
pre-existing-defect surface with no principled stopping point.

This plan makes the gate converge faster **without relaxing a single quality standard**. Every
mechanism below either moves detection earlier (cheaper) or makes an existing implicit obligation
explicit — none removes a check.

## Diagnosis — the four causes, verified against the reports

Each cause below was verified against the archived chain rather than assumed.

### Cause 1 — Fix-site defect injection

Nearly every fix introduced a new defect at the very site it repaired. Verified instances:

| Iteration (report)       | Fix applied                                   | Defect the fix introduced                                                                     |
| ------------------------ | --------------------------------------------- | --------------------------------------------------------------------------------------------- |
| 07-30 (after 07-22 zero) | Added a pre-edit claim to a cross-link clause | `returns 0 today (file absent)` — grep on an absent file prints nothing, not `0`              |
| 07-42                    | Corrected the above                           | "corrects the letter of the flagged defect but not its substance"                             |
| 08-21                    | Added a pre-edit grep claim                   | Falsified by the immediately-preceding checkbox in the same phase                             |
| 09-14                    | Added a verification script                   | Single backtick instead of triple — fence renders as a garbled inline span                    |
| 09-26                    | Fixed the backticks                           | Fence indented 6 spaces → parses as an **indented** code block, swallowing all trailing prose |

Two domains account for almost all of it: **shell/grep semantics** and **CommonMark structure**.

### Cause 2 — Unbounded discovery surface

Once change-surface defects were exhausted, the checker switched from syntax to prose semantics and
began finding **pre-existing latent defects in areas the session never edited** — iteration 15's
stale merge-actor framing, iteration 16's `tech-docs.md:633` clause-lettering drift. Real defects,
but drawn from a 1000+ line, 178-checkbox document. The double-zero termination rule assumes a
stationary finding distribution; the distribution was not stationary, so the loop could not
terminate on its own merits.

### Cause 3 — No shared memory between iterations

The `grep -c` line-counting class recurred at new sites across iterations 9, 10 and 11 because each
fix addressed **instances**, not the **class**. Iteration 10's audit records it precisely: "the
iteration-9 conversion was scoped only to the six explicitly-named sites; the identical undercounting
defect recurs, unconverted, at six additional acceptance-threshold sites." It closed only when the
08-53 fix enumerated and classified every site in one pass (categories (a)/(b)/(c), 46+ sites).

### Cause 4 — Verification asymmetry

The checker empirically simulated acceptance clauses: it built fixture files, ran the literal
commands, and rendered markdown through multiple CommonMark implementations. The fixer and the
authoring path did not. `plan-fixer.md` §7 Self-Verification asks only to "re-read the modified file
section" and `grep -q "expected-pattern"` [Repo-grounded]. Defects were therefore found one full
expensive iteration after they were introduced.

## Approach

Six mechanisms, ordered cheapest-first:

1. **Defect-Class Registry (DCR)** — a governance catalogue of known traps, each with an empirical
   proof command, a safe rewrite form, and a detection method. Seeded with eight entries, all
   empirically verified during this plan's authoring. Open for append.
2. **Deterministic pre-flight validator** — `rhino-cli plan validate-acceptance`, a zero-token
   mechanical scan for statically-detectable DCR classes, run **before** the semantic checker spends
   budget on them.
3. **Symmetric empirical verification** — clause simulation becomes mandatory at **authoring**
   (`plan-maker`) and **fix** (`plan-fixer`) time, not only at check time.
4. **Class-level remediation contract** — a finding that instantiates a pattern obliges the fixer to
   enumerate and fix the whole class in one pass; the checker verifies **class closure**, not
   instance closure.
5. **Scope discipline** — findings partition into **in-surface** (this chain authored or edited it)
   and **latent** (pre-existing, untouched). Termination is evaluated on in-surface findings only;
   latent findings are fully reported and mandatorily filed as a follow-up backlog plan. Four
   anti-loophole guards prevent this from becoming a way to ship known-broken content.
6. **Iteration-budget shaping** — the loop runs a deterministic lens first, a semantic in-surface
   lens second, and a single non-looping latent sweep last.

Mechanism 5 is the actual terminator. Mechanisms 1-4 reduce how much work reaches it.

## Scope

**In scope**:

- `repo-governance/workflows/plan/plan-quality-gate.md` — step model, termination criteria,
  convergence target
- New `repo-governance/development/quality/plan-acceptance-defect-classes.md` — the DCR
- `.claude/agents/plan-checker.md`, `plan-fixer.md`, `plan-maker.md`, `plan-execution-checker.md`
- `.claude/skills/plan-creating-project-plans/SKILL.md` — authoring-time simulation rule
- `apps/rhino-cli` — the deterministic pre-flight validator plus its Gherkin behavior tree
- Regeneration of `.opencode/` and `.amazonq/` via `npm run generate:bindings`
- Tri-repo propagation: `ose-public` (source of truth) → `ose-primer` → `ose-infra`

**Out of scope**:

- The audited plan itself (`plans/in-progress/parallel-orchestration-shared-machine-governance/`) —
  it is evidence, not a target. This plan does not edit it.
- The PR-review quality gate (`repo-governance/workflows/pr/pr-review-quality-gate.md`) — a sibling
  loop with the same shape, but changing it here would double the blast radius. Flagged as an open
  question.
- Any relaxation of an existing check, threshold, or criticality level. Explicitly forbidden.

## Navigation

- [brd.md](./brd.md) — why this matters
- [prd.md](./prd.md) — what gets built, with Gherkin acceptance criteria
- [tech-docs.md](./tech-docs.md) — architecture, design decisions, surface inventory
- [delivery.md](./delivery.md) — phased, gated checklist
- [learnings.md](./learnings.md) — Knowledge Capture running log

## Open questions for the user (grill deferred)

This plan was authored without an interactive grill (the user was mid-task elsewhere). The following
decisions were made with stated reasoning but remain genuinely open. Each should be grilled when the
plan is picked up, before Phase 1 begins.

### Q1 — Where does the deterministic pre-flight pass live?

- **A. `rhino-cli` validator** (chosen provisionally) — deterministic, zero-token, uniformly
  available to maker/checker/fixer and wireable into pre-commit. Cost: `apps/rhino-cli` must stay
  byte-identical across all three repos, so it adds a Gherkin behavior tree and tri-repo propagation
  weight.
- **B. A new skill** (`plan-linting-acceptance-criteria`) — far cheaper to land, no byte-identity
  constraint. Cost: a skill cannot guarantee execution; the archived chain shows agents skip
  self-checks precisely when under budget pressure.
- **C. Inline self-check prose inside `plan-maker` / `plan-fixer`** — cheapest. Cost: same
  non-guarantee as B, plus it duplicates the rules across three agent files.
- **D. Markdownlint config only** — set `MD046: {style: fenced}` and stop there. Verified to catch
  the indented-fence class (see tech-docs DD-2), but catches none of the grep classes.
- **Type your own** / **Chat about this**.

Provisional choice: **A**, with the validator phase authored as separable so a grill can downgrade it
to B without restructuring the plan.

### Q2 — Should `MD046: {style: fenced}` be enabled repo-wide?

Empirically verified during authoring: the repo's `.markdownlint-cli2.jsonc` leaves `MD046` unset
(default `consistent`), and under that config the indented-fence trap produces **0 errors**; Prettier
also reports the broken form as correctly formatted. With `MD046: {style: fenced}` the same file
produces **1 error**.

- **A. Enable repo-wide** — cheapest possible fix for the highest-frequency CommonMark class. Cost:
  every legitimate indented code block in the repo newly fails; requires a repo-wide sweep first.
  Impact count is unmeasured — Phase 0 measures it.
- **B. Enable scoped to `plans/**` only\*\* via a nested config — no repo-wide sweep needed.
- **C. Leave unset; rely on the pre-flight validator** — no config churn, but loses the pre-commit
  hook's free coverage.
- **Type your own** / **Chat about this**.

Provisional choice: **B**, pending the Phase 0 impact measurement.

### Q3 — How aggressive should the latent/in-surface split be?

- **A. Latent findings reported + auto-filed as a backlog plan; CRITICAL never latent-exempt**
  (chosen provisionally) — four guards, described in tech-docs DD-5.
- **B. Latent findings reported only, no mandatory filing** — lighter, but the follow-up reliably
  evaporates.
- **C. No split; keep chasing everything to zero** — status quo; maximal rigor, unbounded cost.
- **D. Split, but require explicit user confirmation before any latent deferral** — safest against
  the loophole, but reintroduces a human stop into a "fully automated" workflow.
- **Type your own** / **Chat about this**.

### Q4 — What happens to `max-iterations` (currently default 7)?

The archived chain ran 17, so 7 was overridden in practice. Under the new phased model the
in-surface budget should be much smaller, but the number is a judgment call.

- **A. Keep 7** — unchanged; the phased model should fit inside it.
- **B. Lower to 5 for the in-surface phase, with a separate single-pass latent budget** (chosen
  provisionally) — makes the budget shape explicit.
- **C. Raise to 10** — acknowledges observed reality rather than aspiring against it.
- **Type your own** / **Chat about this**.

### Q5 — Does the same treatment propagate to `pr-review-quality-gate.md`?

The PR-review maker→fixer cycle has the same shape and plausibly the same pathologies, but no
evidence chain was mined for it.

- **A. Out of scope here; file a separate backlog plan** (chosen provisionally).
- **B. Fold into this plan** — one propagation round instead of two, but doubles blast radius.
- **C. Do nothing until an equivalent evidence chain exists.**
- **Type your own** / **Chat about this**.

### Q6 — Should the DCR be enforced retroactively against plans already in `plans/in-progress/`?

- **A. New and modified plans only** (chosen provisionally) — no retroactive churn.
- **B. Sweep all `in-progress/` plans in this plan's own delivery** — highest immediate quality,
  large unscoped diff.
- **C. Sweep `in-progress/` as a filed follow-up backlog plan.**
- **Type your own** / **Chat about this**.
