# A `syllabus/` folder convention for learning-bearing plans

One-line summary: UI-bearing plans already have a mandated design-funnel-plus-`assets/` convention;
learning-bearing plans have grown an equivalent `syllabus/` folder **three times by imitation, with no
convention behind it** — so the format has no template, custody has no rule, and the first plan to
guess differently forks a 127-file corpus.

> Captured 2026-07-22 while amending the `ayokoding-learning-path-*` programme, where two plans were
> about to author syllabi in invented formats and had to be redirected by hand.

## Problem / context

The repo governs the UI case well. [Plans Organization Convention §Plan Contents](../../repo-governance/conventions/structure/plans.md)
requires that any UI-bearing plan record a full design funnel in `prd.md` — Diverge (≥ 2 named
alternatives), Narrow (high-fidelity mockups referenced from the plan's `assets/` folder), Select
(the named choice), Justify (a rationale table). Ten-plus plans across `backlog/` and `done/` carry
an `assets/` folder as a result, and the rule is enforceable because it is written down.

Learning-bearing plans have no such rule, yet the same need clearly exists. Three plans have
independently grown a `syllabus/` folder:

- `ayokoding-learning-path-02-schema-and-prerequisite-dag/syllabus/` — `courses/` and `paths/`, a
  **127-course catalog** (120 standalone files plus 7 capstones embedded in other courses), each
  file roughly 350 lines.
- `ayokoding-learning-path-06-skills-accounting/syllabus/` and
  `ayokoding-learning-path-07-skills-erp/syllabus/` — added 2026-07-22; plan 07 alone contributes 29
  per-course files plus 2 path-manifest mirrors.

The format is genuinely rich — a header line carrying course id, format and language; a short
summary; a scope note; a "Why this exists · the big idea" section built from problem-before-solution,
keep-this-if-you-forget-everything, and big-ideas-touched framing; then prerequisites. **None of it
is specified anywhere.** It exists only as 127 worked examples, and is transmitted by whoever
happens to read one first.

That transmission failed under load on 2026-07-22. Two agents authoring plans 06 and 07 concurrently
were both about to invent their own syllabus templates; both had to be told by hand to go read
`syllabus/courses/actor-model-concurrency.md` and mirror it. They complied — but the corpus was one
unreviewed step from carrying three incompatible formats, and nothing in the repo would have failed.

Custody has the same shape of gap. The programme resolved "who owns the syllabus corpus when several
plans share it" with a per-plan `DD-*` decision invented in the moment, because no convention
answers it.

## Why now

Three plans already carry the folder, and the count is rising rather than stable — two of the three
appeared this week. A convention written now describes what exists; written after a fourth plan
guesses differently, it becomes a migration.

There is also a precedent for plan-folder artifacts becoming machine-consumed ground truth: the
`web-design-tester` agent already evaluates live pages against "committed plan-folder mockups" as one
of its five ground-truth sources. The moment a content checker wants to do the equivalent for
courses — verify a shipped course against its syllabus — it needs a predictable location and shape.
That is much cheaper to have before the checker is written than after.

## Prior art / precedents

- **The UI-design-funnel convention itself** — the direct model to copy, including the part most
  worth copying: it mandates not just the artifact folder but the _decision record_ (candidates,
  selection, rationale). [plans.md](../../repo-governance/conventions/structure/plans.md),
  [diagrams.md §Placement](../../repo-governance/conventions/formatting/diagrams.md#placement--the-ui-lives-in-prdmd-hard-rule)
- **The `plan-doc-ui-mockup-convention` plan** (`plans/done/2026-06-16__plan-doc-ui-mockup-convention/`)
  — the completed precedent showing this exact kind of gap being closed for UI, and a template for
  how to scope the work.
- **Diátaxis** — the repo's existing answer to "different content types need different structures",
  already governing `docs/`. [diataxis-framework.md](../../repo-governance/conventions/structure/diataxis-framework.md)
- **Architecture Decision Records** — the general pattern of recording a decision plus its rejected
  alternatives beside the work, which is what the funnel's Select/Justify stages are.
  [adr.github.io](https://adr.github.io/)

## Proposed direction (sketch)

Write the learning-plan counterpart to the UI rule, deliberately mirroring its shape rather than
inventing a parallel vocabulary:

- Define when a plan is **learning-bearing** (it authors or restructures course/tutorial content),
  the trigger analogous to "UI-bearing".
- Require a `syllabus/` folder, and specify its internal split — the existing `courses/` and `paths/`
  division is the candidate, since it already carries a per-course and a per-path artifact.
- Specify the per-course syllabus **shape**, derived from the 127 existing files rather than designed
  fresh, and ship it as a template so the next author copies a spec instead of a sample.
- State a **custody rule** for a corpus shared across plans — the question the programme had to
  answer ad hoc.
- Decide whether the funnel's decision-recording half has a learning analogue. It plausibly does:
  a course list is a selection among candidate scopes, and today that reasoning is invisible.

## Rough scope & non-goals

In scope: one convention document, a syllabus template, the custody rule, and updating the plans
convention to reference it as it already references the UI rule.

Out of scope: retrofitting the 127 existing files to any newly-specified shape — the convention
should be derived from them, so they are the reference, not the debt. Also out of scope: a validator.
A `rhino-cli` check for syllabus conformance is plausible later but should follow a settled format,
not precede it. And explicitly **not** in scope: anything about `assets/` or UI mockups, which are
already governed — this brief exists to close the asymmetry, not to restate the half that works.

## Risks & open questions

- **Does a 127-file corpus belong in `plans/` at all?** The largest open question. Plan folders are
  planning artifacts that eventually archive to `done/`; a course catalog is long-lived reference
  material that outlives the plan delivering it. The convention may need to say where a syllabus
  _goes_ on archival, and the answer might be "out of `plans/` entirely". (open)
- **Custody when plans share a corpus** — plan 02 custodies the `careers/` syllabi while plan 04
  authors bodies from them, and plans 06/07 each carry their own. Whether that split is the intended
  model or an accident of sequencing is unresolved. (open)
- Over-specifying the format could make it brittle for non-course learning content (a tutorial series,
  a workshop) that does not fit a course/path split. (open)
- A convention that merely describes today's shape without a template repeats the current failure
  mode — transmission by example is exactly what broke.

## What success looks like + promotion signal

Success: a new learning-bearing plan produces a syllabus in the right place and right shape without
anyone intervening, and the shared-corpus custody question has a written answer instead of a
per-plan ruling.

Promotion signal: ripe once the two open questions above have provisional answers — specifically
whether a syllabus corpus survives archival inside the plan folder or moves elsewhere, since that
determines whether this is a `plans/` convention or something larger. A fourth learning-bearing plan
appearing, or any plan forking the format, should promote it immediately regardless.
