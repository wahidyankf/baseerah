# Business Requirements — Doc Command Existence Validation

## Business goal

Eliminate a recurring, self-inflicted defect class: **documentation that instructs a reader (human
or agent) to run a command that does not exist**. Convert it from a manually-caught, reviewer-
dependent problem into a mechanically-gated one, consistent with how this repository already
treats links, headings, naming, and README indexes.

## Business rationale

This repository's operating model is unusual and makes the defect class unusually expensive: AI
agents read `AGENTS.md`, `repo-governance/`, and `plans/**/delivery.md` as **executable
instruction**, not as prose to be interpreted charitably. A human reading a stale command tries
the next-nearest thing; an agent runs it, fails, and either stalls or improvises. [Judgment call]

Three distinct failure surfaces manifested in one session: [Repo-grounded]

1. **`AGENTS.md`** — the canonical instruction file every agent and contributor auto-loads. A
   false command here is read by every agent on every task.
2. **A live plan's `delivery.md`** — cited as verbatim executable gate acceptance criteria. Two
   independent `plan-checker` runs flagged it CRITICAL. An executor reaching that gate would have
   stalled on a command that cannot succeed.
3. **`repo-governance/development/infra/nx-targets.md`** — the document that is _supposed to be_
   the authority on target names, itself listing six nonexistent targets.

Surface 3 is the most instructive: the canonical source of truth for command names had drifted
from the commands. No amount of "check the canonical doc" discipline helps when the canonical doc
is the thing that drifted. Only a mechanical check against the running system closes this.

## Business impact

### Pain points addressed

- **Agent execution stalls.** An agent that hits a nonexistent command mid-gate either halts
  (costing a human round-trip) or improvises a substitute (costing correctness). Both are
  expensive; the second is worse because it is silent.
- **Erosion of instruction-file trust.** Once agents encounter false commands in `AGENTS.md`,
  the value of every other claim in that file degrades — there is no way to tell which claims
  are still true.
- **Reviewer burden.** Catching these currently depends on a reviewer happening to know the
  target list. That knowledge does not scale and did not hold here.
- **Silent decay.** Commands are renamed and removed routinely; every rename silently invalidates
  an unknown number of citations across three repositories.

### Expected benefits

- Drift is caught at authoring time (pre-push) rather than at execution time (mid-gate), which is
  where the cost is lowest. [Judgment call]
- The `nx-targets.md` remediation forces an explicit, durable separation between _targets that
  exist_ and _targets that are planned_ — a distinction currently collapsed and misleading.
- Renaming a command becomes a safe, mechanically-verified operation across all three repos
  rather than a hopeful one.

### Success signals (observable, not fabricated)

- `md commands validate` exits 0 across all three repositories after the remediation phase.
- Reintroducing any one of the three originally-cited nonexistent targets into a tracked markdown
  file causes the pre-push hook to fail. (Verified directly by the regression scenarios in
  [prd.md](./prd.md).)
- No numeric adoption or defect-reduction target is claimed here; none has been measured.

## Affected roles

The maintainer wears these hats; there is no sign-off ceremony.

| Hat                        | Interest                                                      |
| -------------------------- | ------------------------------------------------------------- |
| Instruction-file author    | Wants `AGENTS.md` claims to stay true without manual auditing |
| Plan author (`plan-maker`) | Wants gate acceptance criteria to be runnable as written      |
| Plan executor              | Wants to never stall on a fabricated command                  |
| Governance maintainer      | Wants `nx-targets.md` to be trustworthy, or honestly labelled |
| Toolchain maintainer       | Wants command renames to be safe across three repositories    |

Consuming agents: `plan-checker` (which already flags these findings, manually and inconsistently),
`plan-maker` (which can cite the validator as a verification recipe), `swe-code-checker`,
`repo-rules-checker`.

## Business-scope non-goals

- **Not a documentation completeness tool.** It does not check that every command _is_ documented,
  only that documented commands exist.
- **Not a correctness oracle for command semantics.** A cited command that exists but does the
  wrong thing is out of scope.
- **Not a replacement for review.** It closes one mechanically-closable gap.

## Business risks and mitigations

| Risk                                                                 | Severity | Mitigation                                                                                                                                                                                                   |
| -------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Validator is noisy, gets disabled**                                | High     | Precision-first default; opt-in `--strict`; deliberate two-tier exemption mechanism designed before the first detector is written, not bolted on after complaints                                            |
| **Exemption becomes a blanket bypass**                               | High     | Inline annotation is per-occurrence and requires a written reason; config allowlist is restricted to structurally out-of-scope trees (`plans/done/`, fixtures, vendored) and reviewed as a diff              |
| **Runtime cost degrades the push loop**                              | Medium   | Placed on `pre-push` (already the home for repo-wide graph checks) rather than `pre-commit`; Nx graph snapshot resolved once per run and cached in-process                                                   |
| **Three-repo byte-identity divergence**                              | Medium   | Propagation phases plus a three-way `git rev-parse HEAD:apps/rhino-cli` tree-SHA equality check, per the [SDLC Gate Standard](../../../docs/reference/sdlc-gate-standard.md)                                 |
| **Validator lands red on existing drift**                            | Medium   | Dedicated remediation phase precedes wiring; the gate is only armed once the corpus is clean                                                                                                                 |
| **Deleting six `nx-targets.md` rows discards roadmap intent**        | Low      | The six names, and the fact that none was ever implemented, are recorded in `learnings.md` and enter Knowledge Capture triage — the canonical doc stops asserting them as real, but the information survives |
| **Per-repo drift in `nx-targets.md` differs across the three repos** | Medium   | Each repo's table is checked independently against its own live Nx graph rather than assuming the `ose-public` row set applies; the validator is the arbiter in each repo                                    |
