# Knowledge Capture Log — Parallel-Orchestration & Shared-Machine Governance

<!-- Knowledge Capture running log — append entries during execution. -->
<!-- Triage every entry (or record the explicit "none" escape) before archival. -->

<!--
Entry shape:

## Learning: <one-line summary>

- **Context**: what was being done when this surfaced
- **Observation**: what was noticed (sanitized — see the secret/sensitivity gate)
- **Why it might generalize**: the litmus reasoning
-->

## Phase 0 baseline — old cap phrasing ("surfaces to update")

Captured 2026-07-20 in the plan worktree at `a207b66e7`, via the Phase 0 baseline command:

```bash
grep -rn "cap at 2\|3 total\|Cap at Three\|stricter cap of 2\|2 concurrent background\|capped at \*\*3 concurrent\*\*" \
  AGENTS.md CLAUDE.md repo-governance/
```

**15 hits across 8 files** (`CLAUDE.md` carries none — it inherits the model via `@AGENTS.md`):

| File                                                           | Lines                          | Hits |
| -------------------------------------------------------------- | ------------------------------ | ---: |
| `AGENTS.md`                                                    | 266, 267                       |    2 |
| `repo-governance/development/agents/subagent-orchestration.md` | 27, 79, 83, 170, 172, 194, 196 |    7 |
| `repo-governance/development/agents/README.md`                 | 38                             |    1 |
| `repo-governance/development/README.md`                        | 154                            |    1 |
| `repo-governance/development/practice/parallel-by-default.md`  | 74, 82, 86, 139                |    4 |
| `repo-governance/workflows/plan/multi-plans-execution.md`      | 118                            |    1 |

This is the "surfaces to update" set for Phase 1 and the Phase 4 Gate's repo-wide superseded-cap
proof. Note the Phase 4 Gate uses a **wider** pattern than this baseline (it adds `cap of 2`,
`cap 3 concurrent`, `2 background`, `never more`), so its expected post-change hit count is not
simply "15 → 0" against this same command — the two greps are deliberately different instruments.

## Learning: a plan's surface inventory can miss an index that describes the file being changed

- **Context**: Phase 1's closing grep sweep, after rewriting the four concurrency surfaces.
- **Observation**: Two index READMEs carried one-line descriptions of the conventions I had just
  rewritten, and both became factually wrong the instant those rewrites landed —
  `development/agents/README.md` ("≤2 concurrent background agents") and `development/README.md`
  ("default 2 simultaneous background Agent-tool spawns; … 3 total"). The plan's Phase 4 §4a checkbox
  names `development/agents/README.md` and `development/practice/README.md`, but **not**
  `development/README.md`. That third file was in no checkbox's stated scope; without an unscoped
  sweep it would have survived to the Phase 4 Gate's repo-wide proof, or past it.
- **Why it might generalize**: when a plan enumerates "surfaces to change", the enumeration is
  naturally built from files that _state_ the rule. Files that merely _summarize_ the rule — parent
  index READMEs, catalog tables, "Related Documentation" blurbs — describe it too, and go stale in
  exactly the same way. Candidate durable fix: have `plan-maker`/`plan-checker` expand any surface
  inventory entry `X` with "every index or README that links to and characterizes `X`", derived
  mechanically by grepping for inbound links, rather than relying on the author to recall them.
- **Related**: this is the same class as the existing memory note that bulk version-string replaces
  must be followed by a grep of **all** doc files, not just the ones edited.

## Learning: appending implementation notes is not the same edit as ticking the checkbox

- **Context**: Discovered at the Phase 2 Gate, 18 items into execution.
- **Observation**: The Atomic Sync Ritual has three steps — tick the checkbox, persist the notes,
  close the task. When the tick and the notes are written as **one** `Edit` whose `old_string` is the
  tail of a multi-line checkbox (the acceptance clause), it is easy to land only the notes: the
  `old_string` anchors on text _below_ the `- [ ]` marker, so the marker is never in the replaced
  span. The edit succeeds, the notes appear, the task gets closed — and disk still says `- [ ]`.
  Nothing errors. It accumulated silently across 18 items in Phases 0-2 before a gate check happened
  to `grep -n "^- \[ \]"` and exposed it.
- **Why it might generalize**: this failure is **invisible to every signal the executor watches**.
  The Edit tool reports success, the task list looks correct, and the notes are genuinely on disk. It
  is caught only by an independent count of `- [x]` versus closed tasks. Candidate durable fixes:
  (a) make the tick its own `Edit` whose `old_string` includes the literal `- [ ]` marker, so a
  mis-anchored edit _fails loudly_ instead of silently no-op'ing; (b) have the executor assert
  `count('- [x]') == count(completed tasks)` at every phase gate rather than only at plan end.
- **Repair applied**: verified all 18 carried a `**Date**` evidence block bounded by the next
  checkbox, then flipped exactly those 18 lines and diffed to confirm the change was purely
  `[ ]` → `[x]` (18 `<` / 18 `>`, no prose touched). Ticking without that evidence check would itself
  have been a corner-cut — asserting completion from memory rather than from the record.
- **Related**: the existing memory note that the PostToolUse markdown formatter rewrites files after
  every Edit is the reason `old_string` anchors drift in this repo in the first place.

## Learning: this plan structurally worsens a preexisting instruction-size warning

- **Context**: Phase 4a, after adding the same-machine assumption and two convention cross-links to
  `AGENTS.md`.
- **Observation**: `nx run rhino-cli:instruction-size:validation` exits **0** but emits
  `[WARN] AGENTS.md is 29049 bytes (over 27000-byte warn threshold)` and
  `[WARN] resolved-tree (CLAUDE.md) is 36422 bytes (over 34000-byte warn threshold)`. Measured
  against `origin/main`, `AGENTS.md` was **already 28333 bytes** — over threshold before this plan
  touched it. This phase added 716 bytes, and Phase 4 mandates further `AGENTS.md` additions (DAG
  rule, 3-5 min cadence, PR-as-merge-point, hardened merge preconditions, and the Delta 12 merge
  default rewrite).
- **Why it might generalize**: the plan and the budget are in **structural tension** — the plan's
  whole purpose is to thread new rules through the most-loaded instruction surface in the repo, while
  the budget convention's sole sanctioned remediation is progressive disclosure. Neither is wrong;
  they were authored independently and nothing forces a plan author to notice the collision. The
  budget was not surfaced as a constraint anywhere in this plan's surface inventory or acceptance
  criteria, so an executor only encounters it by running a gate the plan does not require.
- **Not fixed here, deliberately**: remediating means restructuring `AGENTS.md` into progressive
  disclosure — a substantial refactor of the canonical instruction file, well outside this plan's
  declared scope, and directly at odds with the phases still to run. Recording it rather than
  silently absorbing it (which would hide a real trend) or scope-creeping into it (which would be a
  different, unreviewed change). Candidate follow-up: a backlog plan to move `AGENTS.md` detail
  behind progressive disclosure, sequenced **after** this plan lands so the two do not conflict.
- **Mitigation applied in-plan**: keep every remaining `AGENTS.md` addition as tight as the
  acceptance criteria allow, and prefer linking to the convention over restating it inline.

## Learning: a whole convention can be the stale surface, and a grep-count sweep will not reveal it

- **Context**: Phase 4b's sweep of hardcoded `[HUMAN]`-merge references (46 pre-edit → 20 post-edit;
  24 at Phase 4 close). The count **rose** after 4b because §4c/§4e and the checker-finding fixes each
  added new sentences of the form "a `[HUMAN]` merge gate applies only where a plan says so" — every
  one an explicit opt-in framing, i.e. exactly what the acceptance criterion wants to see. Verified
  hit-by-hit at Phase 4 close: all 24 survivors are opt-in framing, zero are stale assertions. A
  falling count was never the right signal here; a **rising** count of correctly-framed hits is the
  healthy outcome, which is itself an argument against count-based acceptance criteria.
- **Observation**: `repo-governance/development/workflow/pr-merge-protocol.md` contributed only a
  couple of matching lines, so by hit-count it looked like a minor sweep target. It is in fact an
  entire convention built on the rule Delta 12 inverts: "AI agents and automation MUST NOT merge a
  pull request without explicit user approval", "No AI agent, automation script, or workflow may
  auto-merge", "Prior approval does not carry forward", a `### The Approval Prompt` section, and a
  `FAIL: … auto-merging` worked example. Most of that text never contains the literal `[HUMAN]`, so
  the sweep's own pattern could not see it. The plan names the file in **no** checkbox.
- **Why it matters**: the sweep's acceptance ("every surviving hit is an explicit per-plan opt-in")
  was technically satisfiable while the repo still shipped a convention asserting the exact opposite
  rule in different words. A count-based sweep measures the phrasing, not the position.
- **Why it might generalize**: a governance delta that inverts a default should search for
  **documents whose thesis is the old default**, not merely lines matching the old default's
  phrasing. Candidate durable fix: when a plan declares a delta that inverts an existing rule, have
  `plan-maker`/`plan-checker` require an explicit inventory entry for every convention whose title
  or `description:` frontmatter names that rule — those files need reading, not grepping.
- **Related**: same family as the index-staleness learning above (both are surfaces the enumeration
  missed), but a strictly harder case: an index at least _links_ to the file it describes, so an
  inbound-link sweep would catch it. A competing convention has no such mechanical trace.

## Phase 4c discovery sweep — candidate surface list

Recorded 2026-07-20. Command run verbatim from the §4c checkbox:

```sh
grep -rln "cap at 2\|3 total\|2 background\|stricter cap of 2\|max-concurrency\|background agent\|worktree\|git-safety\|cleanup" \
  .claude/agents .claude/skills repo-governance/workflows
```

**36 candidate files**, matching the plan's stated expectations exactly — 20 carry
`max-concurrency`, and all 7 `repo-governance/workflows/plan/*` files appear.

- **`.claude/agents/` (8)**: `docs-file-manager`, `plan-checker`, `plan-execution-checker`,
  `plan-fixer`, `plan-maker`, `pr-review-maker`, `repo-setup-manager`, `swe-code-checker`
- **`.claude/skills/` (4)**: `plan-creating-project-plans`, `repo-defining-workflows`,
  `swe-developing-applications-common`, `swe-developing-e2e-test-with-playwright`
- **`repo-governance/workflows/` (24)**: all 7 `plan/*` (`README`, `plan-execution`,
  `plan-planning`, `plan-quality-gate`, `multi-plans-execution`,
  `plan-multi-repo-parity-planning`, `plan-multi-repo-parity-planning-and-execution`), plus
  `workflows/README.md`, `pr/pr-review-quality-gate`, 5 × `ayokoding-web/*-quality-gate`,
  `content/pdf-to-md-quality-gate`, 2 × `docs/*-quality-gate`,
  `infra/development-environment-setup`, `meta/workflow-identifier`,
  `repo/repo-harness-compatibility-quality-gate`, `repo/repo-rules-quality-gate`,
  `specs/specs-quality-gate`, `ui/ui-quality-gate`, `web/web-ux-test-fixing-planning`

Note that the sweep pattern is deliberately broad (it matches any mention of `worktree` or
`cleanup`), so a listed file is a **candidate** requiring a read, not a confirmed stale surface.

## Phase 4c-ii — the 20 `max-concurrency` files

`grep -rl "max-concurrency" repo-governance/workflows/ | sort` returns exactly **20** files, as the
plan predicted. Final state after the sweep (2026-07-20):

**Aligned to `default: 3` with N+1 wording (18)** — 5 × `ayokoding-web/*-quality-gate`,
`content/pdf-to-md-quality-gate`, `docs/docs-quality-gate`,
`docs/docs-software-engineering-separation-quality-gate`, `meta/workflow-identifier`,
`plan/multi-plans-execution`, `plan/plan-execution`, `plan/plan-quality-gate`,
`plan/plan-multi-repo-parity-planning`, `plan/plan-multi-repo-parity-planning-and-execution`,
`repo/repo-harness-compatibility-quality-gate`, `repo/repo-rules-quality-gate`,
`specs/specs-quality-gate`, `ui/ui-quality-gate`.

**Prose-only, no YAML block (1)** — `workflows/README.md` documents the parameter in a bullet list
rather than frontmatter; its `default: 2` text was updated in place.

**Deliberately preserved at `Default 1` (1)** — `web/web-ux-test-fixing-planning.md`. Left alone
**and** given an explicit justification, because its 1 is a real DAG serialization point, not a
stale cap: the three testers run exploratory → integrate → usability → integrate → design →
integrate, so each reads the plan the previous one wrote. They fail the standard independence test
(two nodes are independent only when neither reads what the other writes). Raising it to N=3 would
put three testers on one plan file concurrently.

**Deliberately excluded from the sweep (not an oversight)** — `pr/pr-review-quality-gate.md` carries
zero `max-concurrency` frontmatter and declares its cycle "strictly sequential, never parallel", so
it is out of scope by construction; verified still 0 post-sweep.

**Method note**: the sweep used a `perl -i -pe` state machine keyed on `- name: max-concurrency`
rather than a blanket `s/default: 2/default: 3/`, because several of these files use `default: 2`
for unrelated parameters. A first attempt with `for f in $FILES` silently passed the entire
newline-joined list as one argument ("File name too long") and changed nothing — caught only
because the post-sweep verification still showed `default: 2` everywhere. Verify after a bulk edit;
a loop that fails to iterate looks identical to a loop with nothing to do.

## Follow-up: the vendor-audit scanner does not know the term "Kiro"

Surfaced 2026-07-20 during Phase 4e, **not fixed here** — recording rather than silently absorbing.

Phase 4e introduces "Kiro" / "Kiro CLI" into this repo's vocabulary (the Amazon Q Developer
succession). The
[Governance Vendor-Independence Convention](../../../repo-governance/conventions/structure/governance-vendor-independence.md)
enforces vendor-neutrality in `repo-governance/**` by scanning for a fixed list of vendor terms —
`Claude Code`, `OpenCode`, `\bCursor\b`, `\bAmazon Q\b`, `\bAntigravity\b`, and so on. **"Kiro" is
not in that list.** A future Kiro mention leaking into governance prose would therefore pass the
scanner silently, which is exactly the failure the scanner exists to prevent.

**Why it was not fixed in this plan**: the scanner is implemented in
`apps/rhino-cli/src/application/repo_governance/vendor_audit.rs`, and `apps/rhino-cli/**` is
required to be **byte-identical across `ose-public`, `ose-primer`, and `ose-infra`** with zero
carve-outs, per the
[SDLC Gate Standard](../../../docs/reference/sdlc-gate-standard.md#rhino-cli-byte-identity-boundary).
Editing it from this single-repo plan would break that boundary. Editing only the convention's
documented term table would be worse: the table would then describe terms the scanner does not
actually match, so the doc would lie about the tool.

**Candidate follow-up**: a tri-repo parity plan adding `\bKiro\b` (and the `\.kiro/` path prefix) to
the vendor-term list in `vendor_audit.rs` and its companion table, landed in all three repos
together so byte-identity holds. Verified today: `grep -rn "Kiro" repo-governance/` returns
**nothing**, so there is no live leak to clean up — the gap is preventive, not corrective.

## Learning: a sweep regex with fixed term order is blind to half its own target set

- **Context**: the §4b merge-actor sweep used `\[HUMAN\][^.]*merge` — HUMAN first, `merge` after. The
  `repo-rules-checker` later found four surviving stale sites in `trunk-based-development.md` alone,
  plus the `plan-maker.md` Delivery Mode table, all of which the sweep had reported as clean.
- **Observation**: every miss shared one shape — the terms appeared in the **opposite** order
  (`merged by a human`, `Merge authority | [HUMAN]`) or were separated by a table pipe rather than
  prose. The regex was not wrong about what it matched; it was wrong about what the target set looked
  like. A table cell in particular puts the two terms in different columns, so no single-line
  same-order pattern can ever bind them.
- **Compounding factor**: the sweep's acceptance criterion was "the pattern returns only opt-in
  framing", which the pattern satisfied perfectly while missing a third of the real sites. The check
  validated the regex against itself.
- **Why it might generalize**: when sweeping for a **concept** (who merges) rather than a literal
  string, one pattern is never sufficient. Run the reverse-order pattern too
  (`merge[^.]{0,40}\[HUMAN\]`), and run a term-only pattern (`merge authority`, `by a human`) that
  makes no assumption about proximity or order at all — then read the hits rather than counting them.
  This is the same failure family as the already-recorded grep traps (`-c` counts lines, `-L` follows
  symlinks, line-based matching cannot span wrapped prose): the tool answers the question asked, and
  the question was narrower than the intent.

## Learning: fixing "the generative source" is not the same as fixing the rule

- **Context**: commit `488148eca` fixed `plan-maker.md`'s Delivery Mode table, and its own message
  justified the scope as targeting "the generative source future plans copy from". The next checker
  run found the identical stale rule alive in three more places: `plans.md`'s Executor Tagging
  section, `plan-checker.md` rule 14, and `plan-fixer.md` fix recipe 3.
- **Observation**: the maker/checker/fixer triad plus the convention they all cite carry the **same
  boilerplate** — here, the "three recurring git-mechanical steps". Updating the maker changes what
  gets authored; it does nothing to the checker that will flag correct new plans as defective, or to
  the fixer that will silently "repair" them. `plan-fixer.md`'s copy was the most dangerous of the
  four: a blanket HIGH-confidence auto-fix that would have stripped a plan's declared `[HUMAN]` merge
  opt-in and rewritten the step into a direct push to `origin main`, bypassing the PR entirely.
- **Sharpest form of the defect**: `plan-checker.md` ended up asserting two rules _exactly backwards_
  from each other — rule 14 flagged as a HIGH mis-tag the same scenario rule 19 declared valid. Both
  rules were internally coherent; only reading them together revealed the contradiction.
- **Why it might generalize**: when a governance delta changes a rule, the unit of work is **every
  copy of that rule**, not the most upstream one. For any rule stated in a convention, enumerate its
  maker/checker/fixer copies and treat them as a single atomic edit. A useful discriminator: if a
  fixer can auto-apply a change at HIGH confidence, a stale fixer copy is strictly worse than a stale
  convention copy — prose misleads a reader, a fixer recipe rewrites the repo unattended.

## Learning: three sweeps, three regexes, three different blind spots — the pattern was the bug each time

- **Context**: inverting the merge default (Delta 12) took **three** corrective rounds at the Phase 4
  Gate. Each round the edits were right and the _search_ was wrong, in a different way:
  1. `\[HUMAN\][^.]*merge` — fixed term order. Missed `merged by a human` and every markdown **table
     cell**, where the two terms sit in different columns and no same-line same-order pattern can
     bind them.
  2. Scope, not pattern: only `plan-maker.md` was swept, on the reasoning that it is "the generative
     source". The identical boilerplate lived in `plans.md`, `plan-checker.md`, and `plan-fixer.md`.
  3. `\[HUMAN\][^.]{0,40}merge|human merges` — assumed the tag is **bracketed** and the noun
     **plural**. Missed the unbracketed singular "human merge" in four places, two of which sat
     inside blocks headed `PASS: Correct behavior`, actively modelling the wrong default as right.
- **Observation**: every miss was invisible _in the same way_ — the acceptance criterion ran the same
  pattern that produced the edits, so it re-confirmed the author's own assumption about what the
  target text looks like. A sweep validated by its own regex measures phrasing coverage, never
  concept coverage. The count even **rose** legitimately between rounds (20 → 24) as correct opt-in
  framing was added, so no count-based signal could have flagged the gap either.
- **What actually found them**: an outside checker instructed to search order-independently and to
  _read_ hits rather than count them. Each round it found what the round's own acceptance had passed.
- **Why it might generalize**: for a **concept** sweep (who performs an action), a single regex is
  never an acceptance criterion — it is one sampling instrument with known blind spots. Minimum
  viable discipline: (a) search both term orders; (b) search each term alone, unbracketed and
  un-cased, accepting the noise; (c) grep the _worked examples and code comments_ specifically, since
  a stale `PASS:` example teaches the wrong rule more forcefully than stale prose states it; (d)
  enumerate every copy of the rule — convention plus maker/checker/fixer — and treat them as one
  atomic edit; (e) have something other than the editing pattern confirm convergence. Candidate
  durable fix: `plan-checker` should reject an acceptance criterion whose only evidence is the same
  regex the delivery step used to make its edits.

## Learning: round four — the paraphrase, where no shared vocabulary exists to search for

- **Context**: after three rounds of Delta-12 corrections, a fourth checker pass (briefed to search
  **paraphrases and synonyms**, explicitly forbidden from reusing any prior pattern) found two more
  survivors in `.claude/skills/`: one-line convention summaries reading
  `- [PR Merge Protocol](...) - Explicit user approval required, all quality gates must pass`.
- **Observation**: these contain neither "human" nor "merge" as the actor phrase. No variant of the
  concept sweep — bracketed, unbracketed, either term order, any plurality — could ever have matched
  them, because they state the old rule in words the old rule never used. Their correctly-fixed
  siblings (`development/README.md:109`, `development/workflow/README.md:47`) summarize the _same_
  file and _were_ swept, so this is the round-2 sibling-copy blind spot recurring in a directory the
  round-2 fix did not reach.
- **Why it matters more than a doc nit**: both are widely-loaded reference surfaces —
  `swe-developing-applications-common` serves the whole `swe-*-dev` family, `plan-creating-project-plans`
  serves `plan-maker`. A wrong one-line summary in a loaded skill steers agent behavior, and agents
  read the summary far more often than the convention it points at.
- **Why it might generalize**: a concept sweep bounded by _any_ vocabulary cannot find a paraphrase.
  The tractable move is not a better regex but a different index: enumerate every **inbound reference
  to the changed document** (`grep -rn "pr-merge-protocol.md"`) and re-read each referring sentence,
  regardless of its wording. Link targets are stable where phrasing is not. Durable fix candidate:
  when a plan rewrites a convention's thesis, require a checkbox that sweeps every file linking to
  that convention, not every file matching the old rule's phrasing.

## Learning: check the real invocation before calling a validator failure a defect

- **Context**: running `rhino-cli md mermaid validate` bare at the Phase 5 pre-push gate returned
  exit 1 with 4 violations, all inside `apps/rhino-cli/tests/fixtures/state/` — the validator's own
  **deliberately-invalid negative fixtures** (over-wide chains, over-long labels), which exist
  precisely to make the validator fail.
- **Observation**: `main-ci.yml:114` invokes the same command with
  `--exclude apps/rhino-cli/tests/fixtures`. The bare invocation was the error, not the repo. Had I
  treated it as a preexisting defect, the "fix" would have targeted `apps/rhino-cli/**` — inside the
  tri-repo byte-identity boundary — and manufactured a three-repo parity plan for a non-problem.
- **Why it might generalize**: same family as the already-recorded no-op-Nx-target trap. Before
  citing a validator's result as evidence of anything, read how CI and the git hooks actually invoke
  it; a command that looks canonical in isolation may be missing the flags that make it meaningful.
  Both failure directions are real: a missing flag can invent failures, and a no-op target can invent
  passes.

## Plan-start baseline SHAs

Recorded 2026-07-20 via `git -C <repo> rev-parse origin/main` after `git fetch origin main` in each
repo. Every later "commits this plan authored" check anchors to these (`<baseline-sha>..origin/main`),
never to reflog-relative syntax such as `origin/main@{1}`.

- ose-public: a207b66e7e59bc6fafd1f650480718fcae02f7e5
- ose-primer: 1728a6e751980289753bf93934d446b998161741
- ose-infra: edbb604e49a1c84f00bd01ea547bbd126b87b29c

---

## Triage Pass (Phase 8 — Knowledge Capture)

Run in the primary checkout on `main` after the plan merged as `60d53119b`. Every entry above, plus
nine further learnings that surfaced during execution and were not written to this log at the time,
reaches a terminal state below: **routed inline**, **filed as a backlog plan**, or **discarded with
a reason**.

## Safety Gates (both applied to every entry)

- **Secret/sensitivity gate — PASS**: no entry contains a credential, token, API key, private
  IP/hostname, or insecure implementation detail. The only concrete host paths recorded are local
  tool paths (`/opt/homebrew/bin/rg`), which are not sensitive. Commit SHAs and public branch names
  are already public in this repo.
- **Repo-relevance gate — PASS**: every routed entry is public-governance content (conventions,
  workflow docs, agent definitions). No infra-private content (Terraform, k3s, Proxmox,
  `coralpolyp`, real hostnames or inventories) appears anywhere in this log, so nothing is scoped to
  `ose-infra` alone. All routed content is eligible to propagate `ose-public` → `ose-primer` through
  the normal parity loop.

## Triage Summary

| #   | Learning                                     | Terminal state        | Home                                                                                                            |
| --- | -------------------------------------------- | --------------------- | --------------------------------------------------------------------------------------------------------------- |
| 1   | Enumeration-based guards fail open           | Routed inline         | `development/agents/anti-patterns.md` (AP-10)                                                                   |
| 2   | Zero-result search is not evidence           | Routed inline         | `development/quality/plan-anti-hallucination.md` (AP-11)                                                        |
| 3   | Completeness-diff finds what search cannot   | Routed inline         | same (AP-12)                                                                                                    |
| 4   | "Threads resolved" ≠ "findings fixed"        | Routed inline         | `workflows/pr/pr-review-quality-gate.md` + `pr-review-fixer.md`                                                 |
| 5   | `REQUEST_CHANGES` structurally unavailable   | Routed inline + filed | same + `pr-review-maker.md`; backlog `2026-07-20__pr-review-bot-identity`                                       |
| 6   | Fixed cycle count is the wrong rule          | Routed inline         | `workflows/pr/pr-review-quality-gate.md`                                                                        |
| 7   | Verification prompts must license a negative | Routed inline         | `development/agents/anti-patterns.md` (AP-11)                                                                   |
| 8   | Source-correct, render-wrong                 | Routed inline + filed | `conventions/formatting/diagrams.md`; backlog `2026-07-20__mermaid-state-label-render-clipping-warn`            |
| 9   | Size budget forced an unsafe trim            | Routed inline + filed | `conventions/structure/instruction-file-size-budget.md`; backlog `2026-07-20__agents-md-progressive-disclosure` |
| A   | Index/README staleness                       | Routed inline         | `plan-anti-hallucination.md` §concept sweeps, rule 6                                                            |
| B   | Silent checkbox no-op                        | Routed inline         | `workflows/plan/plan-execution.md` §Atomic Sync Ritual                                                          |
| C   | Instruction-size warning trend               | Filed                 | backlog `2026-07-20__agents-md-progressive-disclosure`                                                          |
| D   | A whole convention as stale surface          | Routed inline         | `plan-anti-hallucination.md` §concept sweeps, "hardest case"                                                    |
| E   | Vendor-audit does not know "Kiro"            | Filed                 | backlog `2026-07-20__vendor-audit-kiro-term`                                                                    |
| F   | Fixed-term-order regex blind spot            | Routed inline         | `plan-anti-hallucination.md` (AP-13)                                                                            |
| G   | "Generative source" ≠ the rule               | Routed inline         | same, concept-sweep rule 5                                                                                      |
| H   | Three regexes, three blind spots             | Routed inline         | same (AP-13)                                                                                                    |
| I   | Round four — the paraphrase                  | Routed inline         | same, concept-sweep rule 6                                                                                      |
| J   | Check the real invocation                    | Routed inline         | same (AP-14)                                                                                                    |
| K   | Phase 0/4c/4c-ii sweeps, baseline SHAs       | Discarded             | execution artifacts — see below                                                                                 |

---

## Entries 1–9: learnings surfaced during execution, triaged here

### 1. Enumeration-based guards fail open; placement beats enumeration

- **Context**: hardening the `[HUMAN]` merge-gate guard in `.claude/agents/plan-fixer.md`.
- **Observation**: five consecutive guards each protected the merge step correctly on the axis it
  named — tag value, then verb (write vs. delete), then delivery mode, then confidence level, then
  finding type — and each left open the next axis nobody had named. The sixth hole (deletion
  justified as removing an unverified claim) was open for the same structural reason. The durable
  fix was hoisting the invariant to the top of the file, ahead of every recipe and wired into step 2
  of Confidence Assessment, stated by WHAT IT PROTECTS rather than what it enumerates.
- **Why it generalizes**: a guard-authoring defect class, not a `plan-fixer` bug. A guard reachable
  only once the hazard is already suspected is not a guard. Standards backing: OWASP Developer Guide
  security principles (fail securely, positive security model) and NIST SP 800-207 / SP 800-167
  (deny-by-default) — denylists fail open and silently, allowlists fail closed and loudly.

**Routing**: `repo-governance/development/agents/anti-patterns.md` — **ROUTED INLINE** as
Anti-Pattern 10 plus a summary-table row.

**Justification**: the learning is about how agent instruction files are authored, which is exactly
what this existing catalog owns. `plan-fixer.md` already carries the applied instance. A new
convention would have duplicated a file that exists for this purpose.

### 2. A zero-result search is evidence only if the command could have produced non-zero

- **Context**: running absence sweeps across the governance tree during verification passes.
- **Observation**: `grep` here resolves to **`ugrep`**, which REJECTS ripgrep's `--glob`. Combined
  with `2>/dev/null`, a hard tool failure was textually indistinguishable from a clean sweep.
  Measured on one query in one tree: `--glob` + suppressed stderr → **0** hits; POSIX `--include` →
  **377**; `/opt/homebrew/bin/rg` → **69** files. Related: `ls` output carries hyperlink escapes
  that corrupt catalogue diffs — use `find -print0`.
- **Why it generalizes**: any agent citing "zero occurrences found" is exposed. Record the verbatim
  command, never suppress stderr, inspect exit status, and run a known-positive control probe before
  trusting any zero.

**Routing**: `repo-governance/development/quality/plan-anti-hallucination.md` — **ROUTED INLINE** as
a new "Absence and Completeness Claims (HARD)" section plus catalog entry **AP-11**, with the
convention's Scope broadened to bind any validating agent, not only the four plan agents.

**Justification**: this convention already owns verification rituals and the anti-pattern catalog for
agent claims; an absence claim is a claim. Its Repo-Grounding Rule covers presence claims, so the
absence/completeness mirror belongs beside it rather than in a new convention.

**Confirmed live during this very triage**: `cat learnings.md` through the command wrapper returned
only the file's scaffold header, and `ls -la` reported 487 bytes, for a file that actually held 344
lines. Re-reading with a direct file read exposed the truth. Had the triage trusted the first
result, every entry above would have been silently destroyed by an overwrite.

### 3. Completeness-diff finds what text search cannot

- **Context**: hunting blind spots in governance documents that claim to enumerate a set.
- **Observation**: three blind-spot classes (BS-13/14/15) were found ONLY by enumerating ground truth
  and diffing it against the doc claiming to describe it — never by searching text. Crucially,
  BS-15's ground truth was **not a file on disk**: it was `git branch -r`.
- **Why it generalizes**: text search finds what you thought to look for; it cannot find omissions. A
  completeness contract that assumes on-disk artifacts reproduces the very class it means to catch.

**Routing**: `repo-governance/development/quality/plan-anti-hallucination.md` — **ROUTED INLINE**
into the same new section (completeness half) plus catalog entry **AP-12**, including a table of
authoritative enumeration commands for non-on-disk ground truth.

**Justification**: same class as entry 2 — a completeness claim is an absence claim about a set.
Splitting them across two homes would separate a rule from its mirror image.

### 4. "All threads resolved" is not "all findings fixed"

- **Context**: PR review cycles on the delivering PR.
- **Observation**: a fixer correctly declined to modify a file it had been told to leave alone,
  replied to the thread, and resolved it — while the actual fix sat **uncommitted** in the working
  tree. GitHub showed 0 unresolved threads on a PR that still carried a blocking defect.
- **Why it generalizes**: thread state and fix state are independent. Any merge precondition reading
  thread counts measures the wrong thing.

**Routing**: `repo-governance/workflows/pr/pr-review-quality-gate.md` — **ROUTED INLINE**;
done-definition item 2 now requires each fix to be COMMITTED AND PUSHED, verified against the PR's
head with `git status --porcelain`, `git log origin/<branch> -1`, and `gh pr diff`. Enforcement wired
into `.claude/agents/pr-review-fixer.md` (never resolve a `fix` thread until the fix is in the PR
diff; a declined-to-touch file is a `defer`/`reject`, never a `fix`).

**Justification**: the workflow owns the done-definition, so the rule lives there. The agent edit is
the binding that makes the rule fire, not a second home.

### 5. `pr-review-maker` structurally cannot post `REQUEST_CHANGES`

- **Context**: posting blocking findings during the review cycle.
- **Observation**: `gh` authenticates as the PR author and GitHub rejects `REQUEST_CHANGES` on one's
  own PR, so blocking reviews post with STATE `COMMENT`. Anyone gating on review STATE rather than
  finding text reads a blocked PR as unblocked. Both the workflow doc and the agent definition
  previously documented `REQUEST_CHANGES` as available — a factual error.

**Routing**: `repo-governance/workflows/pr/pr-review-quality-gate.md` — **ROUTED INLINE** (Step 1
output, GitHub Reviews API Mechanics, frontmatter `termination`), with the corresponding correction
in `.claude/agents/pr-review-maker.md`. The underlying capability gap is **FILED** at
`plans/backlog/2026-07-20__pr-review-bot-identity/`.

**Justification**: two-part routing because the learning has two parts. The factual correction is a
small non-code doc edit and lands inline; provisioning a GitHub App identity is infrastructure work
well beyond this plan's scope, so it becomes a tracked backlog plan.

### 6. Three review cycles was not enough, and a fixed count is the wrong rule

- **Context**: the `*-to-pr` review-cycle gate on this plan's PR.
- **Observation**: all 3 cycles found blocking defects, and 3 further verification passes after cycle
  3 each found another. A count never once exhausted without a finding is not evidence of
  convergence.
- **Why it generalizes**: capture-recapture (Petersson et al., IEEE TSE) estimates residual defects
  but requires 4+ genuinely INDEPENDENT reviewers; one checker iterating violates independence by
  construction. Perspective-Based Reading (Basili et al., plus the Springer replication) shows
  disjoint lenses find non-overlapping defects, but merely differently-LABELED perspectives converge.
  Thematic saturation (PLOS ONE 2020, PMC7200005) validates "two consecutive clean rounds" ONLY
  alongside a tracked cumulative new-category discovery curve that has flattened.

**Routing**: `repo-governance/workflows/pr/pr-review-quality-gate.md` — **ROUTED INLINE** as a new
"Saturation, Not a Fixed Count (Loop Exit)" section: `{input.cycles}` becomes a floor, the exit
condition becomes two consecutive cycles with zero new finding CATEGORIES on a flattened tracked
discovery curve, with a per-cycle tracking table and the three research citations.

**Justification**: the workflow owns the loop-exit rule. This replaces the existing "fixed N by
design" framing at its source rather than bolting a caveat on elsewhere.

### 7. A verification prompt must license a negative finding

- **Context**: an independent verification pass over a prior fix.
- **Observation**: told to assume a prior fix had introduced a defect, one reviewer investigated,
  reported the hypothesis **WRONG**, and found a real defect elsewhere — explicitly naming agreement
  as the failure mode it was avoiding.
- **Why it generalizes**: a prompt that presupposes its conclusion measures compliance, not
  correctness. Applies to every re-review, self-check, and fixer re-validation prompt.

**Routing**: `repo-governance/development/agents/anti-patterns.md` — **ROUTED INLINE** as
Anti-Pattern 11 plus a summary-table row.

**Justification**: same home as entry 1, for the same reason — a defect in how agent prompts are
authored.

### 8. Source-correct, render-wrong is invisible to text validation

- **Context**: `stateDiagram-v2` diagrams in governance documents.
- **Observation**: edge labels clip in GitHub's renderer, so a diagram can be correct in source and
  silently wrong as displayed. No text-based validator can see this. The threshold is NOT a simple
  character count — clipping observed at **30** and **33** characters while a **40**-character label
  rendered fine. Blast radius: 31 labels over 40 chars, 202 in 31–40, 983 in 26–30, ~11,800 at or
  under 25.

**Routing**: `repo-governance/conventions/formatting/diagrams.md` — **ROUTED INLINE** as a
"Render-Fidelity Caveat" subsection under the existing State Diagram Width and Label Constraints
section. The candidate validator rule is **FILED** at
`plans/backlog/2026-07-20__mermaid-state-label-render-clipping-warn/`.

**Justification**: the convention already had a state-diagram label section asserting a ≤ 30
character rule; leaving that unqualified would keep a proxy masquerading as a guarantee, so the
caveat belongs there. The validator half routes to `apps/rhino-cli/` — a **code** home, therefore a
mandatory backlog plan under the code-routing downstream rule, never inline. The blast-radius numbers
are what force WARN over FAIL, so they are recorded in the plan.

### 9. The instruction-size budget forced a trim that broke a safety rule

- **Context**: compressing `AGENTS.md` under its 30,000-byte fail threshold.
- **Observation**: the trim replaced an inline environment-branch enumeration with a pointer to a
  table that was **not complete**, leaving three deploy targets uncovered by a "never commit
  directly" rule — one of which an agent force-pushes to. Progressive disclosure is the mandated
  remedy, but pointing at an incomplete target is a live hazard, not a compression.
- **Why it generalizes**: every `See`-link replacement carries this risk. It looks like progressive
  disclosure and is rule deletion in disguise.

**Routing**: `repo-governance/conventions/structure/instruction-file-size-budget.md` — **ROUTED
INLINE** as Forbidden Anti-Fix 4 ("Point at an incomplete target"), with the
diff-against-ground-truth recipe, the pattern-over-enumeration fix, and an explicit "never compress a
safety guardrail to save bytes" rule covering the secrets/`.env`, Git Identity, and
environment-branch guardrails.

**Justification**: that convention already names progressive disclosure as the sole sanctioned
remediation and already carries a Forbidden Anti-Fixes list; this failure is a fourth member of it.
`AGENTS.md` itself already carries the applied fix — a pattern-based rule naming `git branch -r` as
authoritative — which is both complete and shorter than the enumeration it replaced. **Verified
during this triage** by diffing `git branch -r` against the rule: all 7 `prod-*` and 4 `stag-*` refs
are covered, including `prod-web-ui`, `stag-ose-be`, and `stag-organiclever-be`, which the Web Sites
table still omits.

---

## Entries A–K: triage of the running-log entries above

### A. A plan's surface inventory can miss an index that describes the file being changed

**Routing**: `repo-governance/development/quality/plan-anti-hallucination.md` — **ROUTED INLINE** as
concept-sweep rule 6 plus the index-staleness paragraph: expand every inventory entry `X` with "every
index or README that links to and characterizes `X`", derived mechanically from inbound links rather
than the author's recall.

**Justification**: the entry's own candidate fix names `plan-maker`/`plan-checker` as the enforcers,
and this convention is the one both agents consume for verification rituals. Writing the rule into
each agent separately would have created two copies of one rule — the exact defect entry G names.

### B. Appending implementation notes is not the same edit as ticking the checkbox

**Routing**: `repo-governance/workflows/plan/plan-execution.md` §Atomic Sync Ritual — **ROUTED
INLINE**. Step 1 now requires the tick to be its own `Edit` anchored on the literal `- [ ]` marker so
a mis-anchored edit fails loudly, plus a per-gate `count('- [x]') == count(completed tasks)`
assertion and the evidence-checked repair procedure.

**Justification**: the Atomic Sync Ritual is the rule this defect violates, and it lives in this
workflow. Both of the entry's candidate fixes are implemented verbatim.

### C. This plan structurally worsens a preexisting instruction-size warning

**Routing**: **FILED** at `plans/backlog/2026-07-20__agents-md-progressive-disclosure/`.

**Justification**: the entry itself declined to fix this in-plan for correct reasons (a substantial
refactor of the canonical instruction file, outside scope). Re-measured during triage: `AGENTS.md` is
**29,995 bytes against a 30,000-byte fail threshold** — five bytes of headroom, so the next
governance addition of any size fails the gate. The plan carries entry 9's constraints as binding
execution rules so the refactor cannot reproduce the incomplete-target hazard.

### D. A whole convention can be the stale surface, and a grep-count sweep will not reveal it

**Routing**: `repo-governance/development/quality/plan-anti-hallucination.md` — **ROUTED INLINE** as
the concept-sweep section's "hardest case" paragraph: when a delta inverts an existing rule, require
an explicit inventory entry for every convention whose title or `description:` frontmatter names that
rule — those files need reading, not grepping.

**Justification**: same home and same section as A, F, G, H, I. These six entries are one rule
observed six times; routing them to one section is what keeps the rule from being restated in six
places and drifting apart.

### E. The vendor-audit scanner does not know the term "Kiro"

**Routing**: **FILED** at `plans/backlog/2026-07-20__vendor-audit-kiro-term/`.

**Justification**: the fix touches `apps/rhino-cli/**` — a **code** home inside the tri-repo
byte-identity boundary, therefore a mandatory backlog plan, never inline. The plan additionally
raises the denylist-versus-allowlist redesign question, because this gap is entry 1's
enumeration-fails-open pattern in its canonical form: a term list that fails open on every vendor
nobody has added yet. Confirmed no live leak (`grep -rn "Kiro" repo-governance/` returns nothing), so
the gap is preventive.

### F, H. Sweep-regex blind spots (fixed term order; three rounds, three patterns)

**Routing**: `repo-governance/development/quality/plan-anti-hallucination.md` — **ROUTED INLINE** as
**AP-13** plus the four-round blind-spot table and the six-point concept-sweep discipline. The entry's
own candidate durable fix is implemented as a HARD acceptance-criterion rule: an acceptance criterion
whose only evidence is the same regex the delivery step used to make its edits is invalid.

**Justification**: as with entry 2, this convention owns the anti-pattern catalog that
`plan-checker` scans against, so a rule stated here is one `plan-checker` already consumes.

### G. Fixing "the generative source" is not the same as fixing the rule

**Routing**: same file — **ROUTED INLINE** as concept-sweep rule 5, including the entry's sharpest
observation: a stale **fixer** copy is strictly worse than a stale convention copy, because prose
misleads a reader while a fixer recipe rewrites the repo unattended at HIGH confidence.

**Justification**: see D.

### I. Round four — the paraphrase, where no shared vocabulary exists to search for

**Routing**: same file — **ROUTED INLINE** as concept-sweep rule 6, stated as the only instrument
that finds paraphrases: sweep by inbound link, not by phrasing, because link targets are stable where
phrasing is not.

**Justification**: see D. This entry supplies the rule's decisive argument — no regex over the old
rule's vocabulary can ever match a paraphrase of it — so it is quoted in the convention text.

### J. Check the real invocation before calling a validator failure a defect

**Routing**: `repo-governance/development/quality/plan-anti-hallucination.md` — **ROUTED INLINE** as
**AP-14** plus a short subsection, recording both failure directions: a missing flag invents
failures, and a no-op Nx target invents passes.

**Justification**: same catalog, same consumer (`plan-checker` Step 5f). The entry explicitly ties
itself to the already-known no-op-Nx-target trap, so stating both directions in one place keeps the
pair together.

### K. Phase 0 baseline, Phase 4c and 4c-ii sweep inventories, plan-start baseline SHAs

**Routing**: **DISCARDED — not generalizable.**

**Reason**: these are execution artifacts of this specific plan — a hit-count table for one grep, two
candidate-file inventories for one sweep, and three baseline SHAs. They were load-bearing during
execution and are inert now; no durable surface would change behavior by routing them. The one
genuinely transferable observation embedded in the 4c-ii note — that a bulk edit whose loop silently
fails to iterate looks identical to a loop with nothing to do, so verify after every bulk edit — is
already covered by concept-sweep rule 4 ("read the hits; never count them") and by AP-11's
control-probe requirement. Discarding the surrounding data avoids a third copy of that rule.

---

## Terminal-State Confirmation

Every entry in this file is now routed inline, filed as a backlog plan, or discarded with a reason.
Nothing remains in an open state, and nothing that matters survives only in this file — per the
[transient-log caveat](../../../repo-governance/development/quality/knowledge-capture.md), this file
is safe to delete once the plan is archived.

**Backlog plans filed by this triage** (five):

- `plans/backlog/2026-07-20__contributing-md-trunk-guidance-and-naming-exemption/`
- `plans/backlog/2026-07-20__pr-review-bot-identity/`
- `plans/backlog/2026-07-20__mermaid-state-label-render-clipping-warn/`
- `plans/backlog/2026-07-20__agents-md-progressive-disclosure/`
- `plans/backlog/2026-07-20__vendor-audit-kiro-term/`
