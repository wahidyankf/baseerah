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
its first zero — more than triple the stated target. The full chain survives on disk in
`generated-reports/` (`plan__b0a12b*__2026-07-20--*`) as **16 audit reports before the confirming
zero, spanning 07-01 through 09-57, plus the terminal 17th audit at 10-06** (explicitly labelled
"Iteration 17", the report that satisfies the double-zero rule), and 2 fix reports [Repo-grounded].

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

## Why sequential rounds could never have converged

The 17-round chain is not evidence that the checker was careless. It is evidence that **one lens,
iterated, is structurally incapable of converging** — and the research base says so directly.

- **Capture-recapture defect estimation** (Petersson et al.; the IEEE TSE "Comprehensive Evaluation
  of Capture-Recapture Models" study) estimates residual defects from overlap between reviewers, but
  requires **4+ genuinely independent** reviewers; too few reviewers causes substantial
  underestimation. One checker iterating over its own prior output violates independence by
  construction, so the double-zero rule was never estimating residual defects — it was observing the
  same lens agree with itself twice.
  [Web-cited — via the 2026-07-20 research brief; see DECISION 12 for the re-verification step]
- **Perspective-Based Reading** (Basili et al., plus the Springer replication) shows that reviewers
  applying **genuinely different lenses** find largely **non-overlapping** defect sets — but the
  replication adds the decisive caveat: reviewers given merely differently-**labelled** perspectives
  converge on the same defects. Relabelling one procedure is not a second lens.
  [Web-cited — via the 2026-07-20 research brief]

That is the mechanistic explanation for both this plan's 17-round chain and the sibling plan's
14-round chain, and for the sibling's five-axis guard-hole sequence: each round found a defect class
the previous round's lens shape structurally excluded, so each round could only ever discover one new
class. Adding rounds adds observations of the same shape; adding **lenses** adds shapes.

The primary change is therefore **not** "more rounds" or "fewer checks". It is **parallel,
operationally-disjoint lenses per round** — different questions, different artifacts consulted — which
reduces rounds _and_ raises quality at the same time. See [XD-4](#xd-4--parallel-operationally-disjoint-lenses-replace-sequential-rounds).

## Approach

Every mechanism below carries an explicit **disposition** — whether it REDUCES ROUNDS, RAISES
QUALITY, or BOTH. A mechanism that does neither is cut, not kept for tidiness. This table is the
plan's guard against reintroducing overhead disguised as rigor, and `plan-checker` treats a
mechanism with no disposition as a HIGH finding.

| #   | Mechanism                                                                                                      | Disposition    | Why                                                                                                            |
| --- | -------------------------------------------------------------------------------------------------------------- | -------------- | -------------------------------------------------------------------------------------------------------------- |
| 1   | **Defect-Class Registry (DCR)** — catalogued traps with runnable proofs                                        | BOTH           | Each trap is paid for once repo-wide (rounds) and stops recurring at new sites (quality)                       |
| 2   | **Deterministic pre-flight validator** — `rhino-cli plan validate-acceptance`                                  | BOTH           | Mechanical classes leave the semantic budget entirely (rounds); detection is exhaustive not sampled (quality)  |
| 3   | **Symmetric empirical verification**, transcript-enforced                                                      | BOTH           | Removes the fix-site injection round (rounds); the largest single observed waste cause (quality)               |
| 4   | **Class-level remediation + mechanized class closure**                                                         | BOTH           | Collapses the 9/10/11 three-round recurrence into one pass (rounds); closure is verified not claimed (quality) |
| 5   | **Parallel disjoint lenses** ([XD-4](#xd-4--parallel-operationally-disjoint-lenses-replace-sequential-rounds)) | BOTH           | N lenses in one round instead of N rounds (rounds); non-overlapping discovery per PBR (quality)                |
| 6   | **Saturation-based termination** ([XD-3](#xd-3--one-termination-doctrine-saturation-not-round-counting))       | BOTH           | Stops at evidence of exhaustion rather than fatigue (rounds); a zero means the curve flattened (quality)       |
| 7   | **Scope partition** (in-surface / latent) — **no longer the terminator**                                       | REDUCES ROUNDS | Bounds the expensive lens; legal only because the latent region gets its own disjoint lens (see XD-3)          |

**Cut from the previous draft** — each failed the disposition test:

- **The "latent follow-up backlog plan must exist on disk" termination precondition.** It is neither
  a round-reducer (it adds artifact creation to every chain surfacing one latent finding) nor a
  quality-raiser (this README's own earlier draft conceded such follow-ups "reliably evaporate", so
  the artifact does not cause the fix). See [DECISION 3](#decision-3-was-q3--how-aggressive-should-the-latentin-surface-split-be)
  for what replaces it.
- **Calling the in-surface/latent split "the actual terminator."** A deferral is not a terminator.
  The terminator is saturation ([XD-3](#xd-3--one-termination-doctrine-saturation-not-round-counting)).

## Scope

**In scope**:

- `repo-governance/workflows/plan/plan-quality-gate.md` — step model, termination criteria,
  convergence target
- New `repo-governance/development/quality/plan-acceptance-defect-classes.md` — the DCR
- `repo-governance/conventions/structure/deterministic-vs-ai-validation-split.md` — **extended, not
  re-derived**: one new row in The Split table, and its existing implementation contract adopted as
  this plan's Phase 2 Gate bar ([XD-1](#xd-1--extend-the-existing-deterministic-vs-ai-split-convention-rather-than-re-derive-it))
- `repo-governance/development/pattern/maker-checker-fixer.md` — the single shared termination
  doctrine ([XD-3](#xd-3--one-termination-doctrine-saturation-not-round-counting)), landed
  idempotently with the sibling plan ([XD-2](#xd-2--one-shared-substrate-built-once-landed-idempotently))
- `.claude/agents/plan-checker.md`, `plan-fixer.md`, `plan-maker.md`, `plan-execution-checker.md`
- `.claude/skills/plan-creating-project-plans/SKILL.md` — authoring-time simulation rule
- `apps/rhino-cli` — the deterministic pre-flight validator plus its Gherkin behavior tree
- Regeneration of `.opencode/` and `.amazonq/` via `npm run generate:bindings`
- Tri-repo propagation: `ose-public` (source of truth) → `ose-primer` → `ose-infra`

**Out of scope**:

- The audited plan itself (`plans/in-progress/parallel-orchestration-shared-machine-governance/`) —
  it is evidence, not a target. This plan does not edit it.
- The PR-review quality gate (`repo-governance/workflows/pr/pr-review-quality-gate.md`) — **resolved,
  no longer an open question**: the sibling plan already lands the two termination-rule edits this
  plan would otherwise duplicate, and its third gap is that plan's filed follow-up. See
  [DECISION 5](#decision-5-was-q5--does-the-same-treatment-propagate-to-pr-review-quality-gatemd).
- Any relaxation of an existing check, threshold, or criticality level. Explicitly forbidden.

## Navigation

- [brd.md](./brd.md) — why this matters
- [prd.md](./prd.md) — what gets built, with Gherkin acceptance criteria
- [tech-docs.md](./tech-docs.md) — architecture, design decisions, surface inventory
- [delivery.md](./delivery.md) — phased, gated checklist
- [learnings.md](./learnings.md) — Knowledge Capture running log

## Cross-plan decisions (XD) — shared verbatim with the sibling plan

These seven decisions bind **both** convergence plans, referred to below by their folder slugs:

- **`plan-quality-gate-convergence`** — the acceptance-clause / plan-document gate.
- **`repo-rules-quality-gate-convergence`** — the governance-sweep gate.

The block is authored **verbatim-identically** in both plans' READMEs and contains no
"this plan" / "the sibling" phrasing, so the two copies are byte-comparable; a divergence between
them is a defect in whichever copy drifted. The decisions were taken during the 2026-07-20
goal-alignment rework, without a grill, each with stated reasoning and each reversible.

### XD-1 — Extend the existing deterministic-vs-AI split convention rather than re-derive it

[`repo-governance/conventions/structure/deterministic-vs-ai-validation-split.md`](../../../repo-governance/conventions/structure/deterministic-vs-ai-validation-split.md)
**already exists** [Repo-grounded — read in full during this rework] and already codifies: the
owner-selection decision tree ("can the rule be encoded as an exact predicate? → Deterministic"), the
deterministic implementation contract (≥90% line coverage, a Gherkin feature with both happy- and
failure-path scenarios, unit **and** integration tests, byte-determinism), and the process for adding
a category ("propose a new deterministic subcommand in a plan").

Both plans' DD-2 sections re-derived that decision tree from scratch, and neither plan's Surface
Inventory contained the convention. **DECIDED**: cite it as the authority, add each new category as a
row to its Split table, and adopt its implementation contract as the Phase 2 Gate bar — replacing the
weaker ad hoc "`nx run rhino-cli:test:unit` exits 0" criterion, which is below what the convention
already requires of every other deterministic category.

**This omission was itself an instance of the blind-spot classes
`repo-rules-quality-gate-convergence` catalogues.** The
convention's Split table is exactly a **BS-10 definition block** (sweep the usage sites, miss the
glossary block defining the term) reached only through **BS-2 generative-source-only scope** (fix the
doc that generates the rule, leave the convention sibling stale). Two plans that spent nine phases
each cataloguing those classes committed both of them, in the document whose entire purpose is being
the canonical answer to the question they were re-deriving. Recording that here is not
self-flagellation — it is the registry's own evidence that a catalogue nobody consults is inert, which
is why XD-2 makes consultation mechanical rather than advisory.

### XD-2 — One shared substrate, built once, landed idempotently

Both plans independently edit `apps/rhino-cli/src/cli.rs` and `commands.rs`, and both add a row to the
same two governance index READMEs. Because `apps/rhino-cli/**` must stay byte-identical across
`ose-public`, `ose-primer` and `ose-infra`, two independent copies of that plumbing cost **six**
propagation PRs for what is two small additions to the same two files.

**DECIDED**: a **Phase S (Shared Substrate)** is authored **identically and idempotently in both
plans**. Whichever plan executes first lands it; the second detects it already present and records
"already landed". Idempotency is what preserves the repo's parallel-by-default posture — neither plan
waits on the other, and neither does the work twice. Phase S contains exactly four items, all shared:

1. Both new categories added as rows to the XD-1 Split table.
2. Both new subcommands registered in `cli.rs` / `commands.rs` in one edit.
3. Both governance index README rows.
4. The **registry-replay harness** — a generic `rhino-cli governance registry-replay` that re-runs a
   registry entry's recorded detection command against a target and diffs before/after. Both
   registries already store runnable commands per entry, so this converts class-closure verification
   from a trust-based AI re-derivation into a deterministic count-diff, for both plans, from one
   implementation.

### XD-3 — One termination doctrine: saturation, not round-counting

The two plans previously installed **opposite** doctrines into structurally identical loops —
`plan-quality-gate-convergence` shrank the required-fix target, `repo-rules-quality-gate-convergence`
raised the confidence required for a zero — and only the latter promoted its half into the shared
[`maker-checker-fixer.md`](../../../repo-governance/development/pattern/maker-checker-fixer.md)
§Preventing Iteration Loops, which every maker-checker-fixer family reads. A third gate adopting "the
convention" later would have silently inherited one doctrine with no signpost that a competing one
existed.

**DECIDED**: reconcile into **one** doctrine, promoted once (idempotently, per XD-2):

> **Saturation-based termination.** A gate terminates when the cumulative **new-category discovery
> curve** has visibly flattened across rounds whose lenses are **operationally disjoint**. A round
> count is not a stopping rule. If round N+1 is structurally **narrower** than round N, two clean
> rounds prove the **lens** is exhausted, not the artifact — so a narrowing is legal only when it is
> recorded and the narrowed-out region is covered by a different lens.

This is where "two consecutive clean rounds" is formally documented — thematic saturation in
qualitative research (PLOS ONE 2020, PMC7200005) — but **only** as valid alongside a tracked
cumulative new-category discovery curve that has flattened. A bare round counter is not a stopping
rule. [Web-cited — via the 2026-07-20 research brief]

Supporting evidence for dropping round-count rules entirely: the Chromium OS security-defect
case-control study (ICSE 2021, arXiv 2102.06909, AUC 0.91) found that **broader review scope and
higher reviewer workload predict missed defects, while round count does not** — which supports scoped
re-validation over broad re-sweeps and removes the last justification for counting.
[Web-cited — via the 2026-07-20 research brief]

Under this single doctrine, both plans' halves become corollaries rather than competitors:

- `plan-quality-gate-convergence`'s **in-surface/latent split is a scope narrowing**, legal only
  because the narrowed-out latent region is covered by its own disjoint lens (the single non-looping
  latent sweep) and the narrowing is recorded.
- `repo-rules-quality-gate-convergence`'s **adversarial round adds a disjoint lens** so the curve's
  flattening is trustworthy rather than an artefact of one shape repeating.

Same rule, applied at two points in the same loop. Only this text lands in the shared convention.

### XD-4 — Parallel operationally-disjoint lenses replace sequential rounds

**DECIDED** — this is the primary speed lever in both plans, and it raises quality rather than
trading against it.

Per round, the gate runs its lenses **in parallel** (subject to the repo's concurrency cap of 2
background subagents, 3 total including the main thread), instead of discovering one class per
sequential round. A lens qualifies only if it is **operationally disjoint**: it asks a different
question **and** consults a different artifact set. Each lens declares the artifacts it reads, and
overlap in that declared set is the disjointness metric — a lens whose artifact set is a subset of
another's is a relabel, not a lens, and the PBR replication is explicit that relabelled perspectives
converge on the same defects rather than partitioning them.

**Disposition: BOTH.** N lenses in one round instead of N rounds (reduces rounds); non-overlapping
discovery per PBR, approaching the reviewer independence capture-recapture actually requires (raises
quality).

Triage precedes the multi-lens spend rather than applying uniform rigor to everything: Meta's RADAR
risk-triage-before-review study (arXiv 2605.30208, 535K+ diffs) reports median time-to-close cut by
more than 330%, with one third the revert rate and one fiftieth the incident rate. Uniform rigor on
everything is the wrong shape — multi-lens rigor is spent on the high-risk stratum.
[Web-cited — via the 2026-07-20 research brief]

Consistent with this, lightweight review beats heavyweight formal inspection: the Cisco/SmartBear
study (2,500 reviews, 3.2M LOC) found lightweight review substantially more defect-productive per hour
than formal inspection, with effectiveness degrading above roughly 400 LOC of scope per review. Cite
the **qualitative** finding only — that study is from 2006, used human reviewers, and examined code
rather than prose, so its per-hour rates do not transfer.
[Web-cited — via the 2026-07-20 research brief]

### XD-5 — Guards are invariants, not enumerations

`repo-rules-quality-gate-convergence`'s enumeration-fails-open rule (its DECISION 9) is correct and
now carries standards backing: the OWASP Developer Guide's security principles and NIST SP 800-207 / SP 800-167 both rest on
the same asymmetry — **denylists fail open and silently; allowlists and default-deny fail closed and
loudly**. A loud failure is catchable on the next round; a silent one is not, which is precisely why
the five-axis guard sequence failed four consecutive times without anyone noticing until the fifth.
[Web-cited — via the 2026-07-20 research brief]

**DECIDED**: every guard in both plans is expressed as an **invariant** — "assert I holds for every
file/clause/step in scope" — never as a list of forbidden patterns. The constructive form is
**metamorphic testing** (Chen et al. 1998; the MST-wi catalogue, arXiv 2208.09505, 22 system-agnostic
metamorphic relations): assert relations that must hold across transformed inputs, instead of
enumerating expected outputs one at a time. [Web-cited — via the 2026-07-20 research brief]

`plan-quality-gate-convergence` restates its defect-class detectors in invariant form;
`repo-rules-quality-gate-convergence` restates its blind-spot sweep forms the same way.

### XD-6 — Every proposed validator passes the Tricorder inclusion criterion

Because every `apps/rhino-cli/**` addition carries tri-repo byte-identity cost, no detector is added
on the grounds that it is merely computable.

**DECIDED**: adopt Google Tricorder's criterion (ICSE 2015) — mechanize only where **the problem is
obvious and the fix is clear**, ideally with an auto-generated fix. Tricorder explicitly **rejected**
computable-but-unactionable checks such as complexity warnings and fault-prediction scores, and notes
that analyses flagging **missing** content cannot be auto-fixed and consequently earn low developer
trust. [Web-cited — via the 2026-07-20 research brief]

The boundary this criterion enforces is real and both plans respect it: prose tooling
(Vale / textlint / markdownlint, per the published Datadog and GitLab docs-as-code writeups) reliably
catches **lexical and structural** violations and **cannot** detect that document A contradicts
document B. Contradiction detection stays with the AI checker; only lexical/structural predicates
become validators. [Web-cited — via the 2026-07-20 research brief]

### XD-7 — Control probes and seeded fixtures are standing practice; mutation is the escalation

**DECIDED**: every detector ships **paired valid/invalid fixtures**, scoring both false negatives and
false positives — the OWASP Benchmark pattern (2,740 test cases combining real vulnerabilities with
non-exploitable look-alikes), and the submission requirement already enforced by ESLint's `RuleTester`
and Semgrep's `--test`. Every zero-asserting acceptance clause in both plans carries a known-positive
control probe. [Web-cited — via the 2026-07-20 research brief]

**The honest limit is recorded with the practice**: fixtures authored by the rule's own author
validate the author's **intent**, not the author's **unimagined blind spots** — which is the same
failure the five-axis guard sequence demonstrates. When hand fixtures stop finding anything, the
escalation is **Mutation-based Soundness Evaluation** (μSE, arXiv 2102.06829; and arXiv 1806.09761):
systematically mutate inputs along axes the guard does **not** name and measure the catch rate. μSE
empirically falsified real static analyzers' own coverage claims, which makes it the direct answer to
the five-axis failure.

**Flagged honestly**: there is **no citable precedent for applying mutation testing to a prose or
markdown linter**. Applying μSE here is a reasonable extrapolation from code-domain work, **not**
off-the-shelf practice, and neither plan may present it as established. It is scoped as an escalation
path recorded in the registries, not as a phase either plan executes.

## Decisions (this plan)

`Q1`-`Q6` of the previous draft are now **resolved** and renumbered `DECISION 1`-`DECISION 6`
one-to-one, same subject in each case. `DECISION 7` onward were taken during the 2026-07-20
goal-alignment rework. None was grilled; each carries stated reasoning and each is reversible without
restructuring the plan.

### DECISION 1 (was Q1) — Where does the deterministic pre-flight pass live?

- **A. `rhino-cli` validator** — **DECIDED**. Deterministic, zero-token, uniformly
  available to maker/checker/fixer and wireable into pre-commit. Cost: `apps/rhino-cli` must stay
  byte-identical across all three repos, so it adds a Gherkin behavior tree and tri-repo propagation
  weight — which XD-2 halves by sharing the plumbing, and XD-6 bounds by admitting only detectors
  whose problem is obvious and whose fix is clear.
- **B. A new skill** (`plan-linting-acceptance-criteria`) — far cheaper to land, no byte-identity
  constraint. Cost: a skill cannot guarantee execution; the archived chain shows agents skip
  self-checks precisely when under budget pressure.
- **C. Inline self-check prose inside `plan-maker` / `plan-fixer`** — cheapest. Cost: same
  non-guarantee as B, plus it duplicates the rules across three agent files.
- **D. Markdownlint config only** — set `MD046: {style: fenced}` and stop there. Verified to catch
  the indented-fence class (see tech-docs DD-2), but catches none of the grep classes.

**Reasoning for A**: unchanged from the previous draft, and now additionally grounded in XD-1 — the
split convention's own decision tree puts an exact-predicate rule in the deterministic layer, and
these classes are exact predicates. Phase 2 remains **separable**: dropping it degrades the plan to
mechanisms 1 and 3-7 without restructuring any other phase.

### DECISION 2 (was Q2) — Should `MD046: {style: fenced}` be enabled repo-wide?

Empirically verified during authoring: the repo's `.markdownlint-cli2.jsonc` leaves `MD046` unset
(default `consistent`), and under that config the indented-fence trap produces **0 errors**; Prettier
also reports the broken form as correctly formatted. With `MD046: {style: fenced}` the same file
produces **1 error**.

- **A. Enable repo-wide** — cheapest possible fix for the highest-frequency CommonMark class. Cost:
  every legitimate indented code block in the repo newly fails; requires a repo-wide sweep first.
  Impact count is unmeasured — Phase 0 measures it.
- **B. Enable scoped to the plans tree only** via a nested config — **DECIDED**, pending the Phase 0
  impact measurement, which is retained as a Phase 0 step so the decision rests on a measured count
  rather than an estimate.
- **C. Leave unset; rely on the pre-flight validator** — no config churn, but loses the pre-commit
  hook's free coverage.

**Reasoning for B**: it buys the pre-commit hook's free coverage for the highest-frequency CommonMark
class without requiring a repo-wide sweep of legitimate indented blocks first. If Phase 0 measures the
repo-wide impact as trivially small, option A becomes strictly better and the decision flips on that
evidence.

### DECISION 3 (was Q3) — How aggressive should the latent/in-surface split be?

The previous draft's option A made `pass` conditional on a follow-up **backlog plan existing on
disk** while this same README conceded that such follow-ups "reliably evaporate". That is a deferral
dressed as a terminator: filing a ticket is not fixing the finding it names, and nothing in the repo
forces a filed backlog plan to execute.

- **A. Latent findings reported, auto-filed as a backlog plan, backlog-plan existence gating `pass`**
  — the previous draft's choice. **REJECTED** on the disposition test: it neither reduces rounds (it
  adds artifact creation to every chain surfacing one latent finding) nor raises quality (by this
  README's own admission the artifact does not cause the fix).
- **B. Latent findings reported only, no mandatory filing** — lighter, but leaves nothing at all
  catching the finding next time.
- **C. No split; keep chasing everything to zero** — status quo; maximal rigor, unbounded cost, and
  the observed 17-round chain is what it produces.
- **D. Split, with every latent finding bound to a mechanism that cannot evaporate** — **DECIDED**.

**Reasoning for D**: the split survives (it is a genuine round-reducer, and under XD-3 it is a legal
scope narrowing because the latent region gets its own disjoint lens). What is removed is the
artifact-existence gate; what replaces it is a binding that runs whether or not anyone remembers:

1. A latent finding **instantiating a registry class** needs no ticket. The deterministic pre-flight
   runs **unconditionally on every subsequent invocation** against that plan, so the finding is
   re-detected mechanically next time by a pass nobody can skip. This is the binding that cannot
   evaporate — it is not a promise, it is an unconditional step in the control flow.
2. A latent finding instantiating a **new** class is appended to the registry during Knowledge
   Capture (already a Phase 10 gate item), which converts it into case 1 permanently.
3. A latent finding that is neither, and is neither CRITICAL nor execution-reachable, is closed as
   **explicitly accepted risk with a recorded rationale in the audit report** — an auditable
   acceptance the maintainer can read, rather than a phantom ticket that reads as ownership.

The four anti-loophole guards of the previous draft (mechanical surface derivation, `git log -L`
provenance, CRITICAL never latent-exempt, execution-reachability promotion) all survive unchanged.
Under XD-5 they are restated as invariants rather than as a list of exempted cases.

### DECISION 4 (was Q4) — What happens to `max-iterations` (currently default 7)?

- **A. Keep 7** — unchanged; the phased model should fit inside it.
- **B. Lower to 5 for the in-surface phase, with a separate single-pass latent budget** — the
  previous draft's choice.
- **C. Raise to 10** — acknowledges observed reality rather than aspiring against it.
- **D. Retain a cap purely as a runaway circuit-breaker, and remove every claim that it is a
  convergence expectation** — **DECIDED**.

**Reasoning for D**: XD-3 makes round count not a stopping rule at all, and the Chromium OS study
finds round count does not predict missed defects. Any number in this field is therefore a
resource-exhaustion guard, not a quality signal, and the plan says so in the frontmatter rather than
implying a target. Tuning the constant would be bookkeeping that neither reduces rounds nor raises
quality — the disposition test rejects it.

### DECISION 5 (was Q5) — Does the same treatment propagate to `pr-review-quality-gate.md`?

- **A. Out of scope here; file a separate backlog plan** — the previous draft's choice, now known to
  be wrong: it would have filed a plan duplicating work the sibling has already scoped.
- **B. Fold into this plan** — doubles blast radius on a file the sibling already edits.
- **C. Do nothing here; the sibling plan already covers it** — **DECIDED**.

**Reasoning for C**: the sibling plan does not merely discuss this file, it lands two concrete edits
to it (evidence-based cycle termination, and the committed-fix merge precondition), and files the
third gap — `pr-review-maker` cannot post `REQUEST_CHANGES` — as its own Knowledge Capture follow-up.
Re-opening any of the three here would collide with work already scoped elsewhere. See the
[sequencing note](#sequencing-with-the-sibling-plan) for the ordering constraint this creates.

### DECISION 6 (was Q6) — Should the DCR be enforced retroactively against plans already in progress?

- **A. New and modified plans only** — **DECIDED**. No retroactive churn.
- **B. Sweep every in-progress plan in this plan's own delivery** — highest immediate quality, large
  unscoped diff, and an unscoped diff is precisely the "broader scope" the Chromium OS study
  identifies as a predictor of missed defects.
- **C. Sweep the in-progress tree as a filed follow-up backlog plan** — rejected for the same reason
  DECISION 3 rejects backlog-filing as a binding mechanism.

**Reasoning for A**: the deterministic pre-flight runs against any plan the gate is invoked on, so
existing plans are covered the moment they are next validated — without a retroactive sweep and
without a ticket. Coverage arrives through the unconditional mechanism rather than through a campaign.

### DECISION 7 — Mechanism 3 becomes enforceable, or its claim is dropped

Symmetric authoring/fix-time simulation targets the single largest observed cause of waste (fix-site
defect injection, roughly 5 of the archived chain's 17 iterations), yet the previous draft left it as
an **unenforced prose obligation** — contradicting its own DD-2 reasoning that a self-check "cannot
guarantee it actually runs, and the archived chain shows self-checks are exactly what gets skipped
under budget pressure". The plan mechanized the smaller half of the problem and trusted the larger.

- **A. Leave it as prose and keep the claim** — internally inconsistent; rejected.
- **B. Drop the mechanism and the claim** — loses the highest-leverage round-reducer in the plan.
- **C. Require the literal shell transcript pasted inline, and check for it mechanically** —
  **DECIDED**.

**Reasoning for C**: a fix report claiming `APPLIED (verified)` must carry the observed output as a
fenced transcript block, not a claim that verification happened. Presence of that block is an exact
predicate, so under XD-1's decision tree it is a deterministic check, and under XD-6 it qualifies —
the problem is obvious (no transcript), the fix is clear (paste the transcript), and the check is
cheap. This converts an unfalsifiable claim into a falsifiable one without inventing new tooling.
**Disposition: BOTH.**

### DECISION 8 — Class closure is mechanized, not re-derived by the checker

The previous draft left class-closure re-verification as an AI re-derivation of an enumeration that
is, by construction, a mechanical pattern-count operation — the exact operation that failed three
consecutive times (iterations 9, 10, 11) when left to an agent's instance-scoped attention.

**DECIDED**: use the XD-2 registry-replay harness. Every registry entry already stores a runnable
detection command; re-running it against the whole plan and diffing before/after turns "the checker
independently re-runs the class enumeration" into a deterministic count-diff. **Disposition: BOTH** —
it removes the recurrence rounds and it removes the trust assumption.

### DECISION 9 — DC-2 / DC-2b get detectors; the registry gains DC-8 from this plan's own defect

Two gaps closed:

- **DC-2 / DC-2b** (the absent-file trap and its masking corollary) are exact predicates — `test -f`
  plus a count comparison — no harder to detect than DC-5, and they had no detector. Under XD-6 they
  qualify (obvious problem, clear fix, auto-fixable by inserting the `test -f` companion). Detectors
  added.
- **DC-8** is new, and it was found in the sibling plan's own delivery checklist during this rework:
  inside a POSIX bracket expression a backslash is **not** an escape, so `[^\n]` means "not `\` and
  not `n`" rather than "not a newline". Reproduced live on this host against the real agent file —
  every match truncated at the first literal lowercase `n`
  (`Initialize`→`I`, `Validation`→`Validatio`, `Governance`→`Gover`). See
  [tech-docs DC-8](./tech-docs.md#dc-8--inside-a-bracket-expression-a-backslash-is-not-an-escape).

DC-8 is the strongest available evidence for this plan's whole thesis: the defect appeared in the
acceptance clauses that a sibling plan authored **to demonstrate its new search-tool discipline**,
and it survived because the clause's `sort -u | wc -l` still returned the right number by luck. It is
also a textbook XD-5 case — the clause enumerated what to exclude instead of asserting an invariant.

### DECISION 10 — The gate's own lens set is declared and disjointness-checked

XD-4 is only real if the lenses are genuinely disjoint, so the plan makes disjointness auditable
rather than assumed: each lens declares the artifact set it reads, and a lens whose declared set is a
**subset** of another lens's is rejected as a relabel. The PBR replication is the reason this check
exists at all — differently-labelled perspectives converge on the same defects, so a lens roster
without a disjointness check silently degenerates into the sequential loop it replaced.
**Disposition: BOTH.**

### DECISION 11 — Observability parity with the sibling gate

The sibling gate's workflow already states a deterministic-coverage ratio target in its Observability
Metrics section; this plan's workflow had no equivalent, leaving two sibling gates describing their
own health differently. **DECIDED**: port an equivalent target into `plan-quality-gate.md`'s
Observability Metrics. [Judgment call] — the specific ratio is a target, not a measurement, and is
labelled as such in the workflow text. **Disposition: RAISES QUALITY** (it makes drift between the two
sibling gates visible); it is admitted despite being cheap bookkeeping because consistency between two
gates that share a convention is the failure XD-1 documents.

### DECISION 12 — The research citations are re-verified before any policy depends on them

The R1-R11 findings underpinning XD-3, XD-4, XD-5, XD-6 and XD-7 entered these plans through a
research brief and were **not** re-fetched from primary sources during this rework. They are labelled
`[Web-cited — via the 2026-07-20 research brief]` rather than carrying inline excerpts, URLs and
access dates, which is below this repo's normal standard for an external claim.

**DECIDED**: Phase 0 carries an explicit re-verification step delegating to `web-researcher`, and no
gate text may cite a finding whose primary source has not been confirmed. One source is additionally
flagged: the LLM multi-agent audit paper (arXiv 2605.12280 / MDPI _Software_ 2026, Calboreanu)
reports a 9-round convergence with non-monotonic discovery (15, 8, 12, 2, 8, 1, 4, 1, 0) and a
cross-vendor panel against 5 seeded defects at Cohen's κ = 0.46 on severity. **Only the round data is
confirmed, from the abstract.** Its stopping criteria and recommendations are **paywalled and
unverified** — marked `[Needs Verification]` wherever referenced, and no policy in either plan may
depend on them until the full text is fetched.

The non-monotonic discovery sequence is worth noting even at abstract-only confidence, because it is
the shape XD-3 predicts: discovery does not decay smoothly, so a round counter that stops at the
first quiet round (the `2` at round four, or the `1` at round six) would have terminated with three
productive rounds still ahead. A flattening **curve** distinguishes those; a **count** cannot.

## Sequencing with the sibling plan

Both plans may execute concurrently — that is the point of XD-2's idempotent Phase S. Two ordering
constraints apply and are stated here so neither plan discovers them at merge time:

1. **Phase S is first-writer-wins.** Whichever plan reaches Phase S first lands the shared substrate;
   the other detects it and records "already landed". Both plans' Phase S acceptance clauses are
   falsifiable in both directions on exactly this basis.
2. **`pr-review-quality-gate.md` belongs to the sibling plan alone.** Per DECISION 5 this plan makes
   no edit to that file, so there is no race. If a future change to it is needed here, it is a new
   plan, not an amendment to this one.

The shared `maker-checker-fixer.md` termination text (XD-3) is likewise idempotent: both plans carry
the identical block, and the second writer verifies it is present and identical rather than appending
a second copy.
