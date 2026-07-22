# AyoKoding Learning-Path Programme

Shared context for the seven `ayokoding-learning-path-*` plans in
[`./README.md`](./README.md). This file is **not a plan** — it holds the programme-level structure
and decisions the seven plans share, so that no single plan owns a document the other six depend on.

That placement is deliberate. These decisions bind all seven plans, so putting them inside any one
plan would put a shared document on a cross-plan seam — the same reasoning that produced `A3`. They
do not belong in [`./README.md`](./README.md) either, which is an index of everything in `backlog/`
and not a home for one programme's internals.

## Programme structure

Plans `01`-`05` are the **five-way split** of the retired
[`shared-course-library-and-learning-paths`](../done/2026-07-21__shared-course-library-and-learning-paths/README.md)
plan and cover the **`careers/`** category only. Plans `06` and `07` add the **`skills/`** category,
which that retired plan never scoped.

The `NN-` prefix **is the execution sequence**, and it encodes a three-wave dependency DAG:

- **Wave 1** — `01`, `02`. Start immediately, in parallel.
- **Wave 2** — `03`, `04`, `06`. Need both Wave 1 plans merged.
- **Wave 3** — `05`, `07`. Each needs its own Wave 2 predecessor merged.

Each plan is a separate `worktree-to-pr` delivery with its own PR.

The two category branches are independent after Wave 1 — nothing in `05` waits on `06`/`07`, and
nothing in `07` waits on `05`. The one cross-branch edge is `07`'s dependency on `06`, which is
**soft overall and hard at specific wave gates**: ERP courses with no accounting prerequisite are
authorable while `06` is still in flight. Plans `06` and `07` own the current shape of that edge,
which they express at **stage granularity** rather than by course number, so that renumbering a
corpus cannot silently break it.

## Programme decisions (`R*` / `A*`)

The seven plans cite these ids throughout (`R7`, `A3`, and so on). They are **programme-scope
decisions, not governance rule ids** — nothing under
[`../../repo-governance/`](../../repo-governance/README.md) defines them, and they bind only this
programme. Each plan additionally encodes the rules affecting it as its own numbered `DD-*`
decisions, where the full rationale lives; the table below is definitional.

`A*` amendments are **later than** the `R*` rules and **win on conflict**.

| Id  | Decision                                                                                                                                                                                     |
| --- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| R0  | Remove the `/c/` content namespace by **inverting** `contentNamespaceRedirects` — supplementing it would 308-loop                                                                            |
| R1  | URL grammar is `/en/learn/paths/{careers,skills}/…` over six paths (raised to **eight** by `A10`)                                                                                            |
| R2  | `pathId` is **variable-depth by design** — `careers/<arc>/<role>` is 3 segments, `skills/<subject>` is 2; nothing may key on segment count                                                   |
| R3  | The fourth careers path targets a distinct AI-engineering endpoint (superseded in part by `A1`)                                                                                              |
| R4  | Ownership split: plans 01-05 are `careers/`-only; the `skills/` category is separate (revised by `A2`)                                                                                       |
| R5  | The full skills corpus is authored **in this programme**, not deferred                                                                                                                       |
| R6  | The paths hub is **redesigned** around the two categories, not relabelled                                                                                                                    |
| R7  | **Every URL segment must render** — no orphan segments                                                                                                                                       |
| R8  | Every `skills/` path uses the **immediately-effective** arc, always                                                                                                                          |
| R9  | Every plan declares its **UI-gate and API-gate posture explicitly**; a plan bearing neither surface is _not_ thereby exempt and must state why                                               |
| A1  | `careers/immediately-effective/ai-engineer` assumes **no** prior software-engineering competence; prerequisites are included in `courseOrder`, not linked                                    |
| A2  | The skills category splits into **two** plans — 06 (accounting) and 07 (ERP), the latter `blockedBy` the former                                                                              |
| A3  | Plan 01 owns **every structural `_index.md`** under `paths/`; plans 05-07 own only their path landings, manifests and corpora                                                                |
| A4  | Research verification status is carried forward verbatim — an `[Unverified]` claim must never be restated as fact                                                                            |
| A5  | Plan 03 owns **all** design assets; a `.png` is a baked render and desynchronises silently when its `.html` changes                                                                          |
| A6  | Plans 06-07 teach the **domain to build-founding depth** — enough to implement the software — but contain **no system-building courses**; building is out of scope for a path                |
| A7  | ERP's buyer/consultant courses are **replaced** by domain-depth courses; evaluation, selection and implementation-methodology material leaves the corpus                                     |
| A8  | **Strict clean-room licensing, programme-wide** — binds all seven plans, not only 06-07; nothing copyrighted is reproduced, and every concept is restated in original words with a citation  |
| A9  | Both corpora **expand past 20 courses** as the domain requires; every derived count follows                                                                                                  |
| A10 | The skills category carries **four** paths — `conventional-accounting`, `sharia-accounting`, `conventional-erp`, `sharia-erp`; each Sharia path covers the basics too, and `A11` governs how |
| A11 | Shared courses are **referenced by both manifests, authored once** — a Sharia path's `courseOrder` interleaves shared and Sharia-specific ids rather than duplicating files                  |
| A12 | Every syllabus is **independently authored, then externally confirmed** — a published curriculum may corroborate coverage but must never supply the structure being written                  |

## A6 — the build-founding-depth line

`A6` draws a line that is easy to misread in both directions, so it is stated positively and
negatively:

- **In scope**: the domain knowledge an implementer needs — double-entry mechanics, the
  subledger-to-general-ledger relationship, costing methods, period close, document state machines,
  posting rules, the failure modes each of these produces. Architecture is domain knowledge here: you
  cannot found an implementation without knowing how a ledger is structured.
- **Out of scope**: building it. No capstone that constructs a system, no "implement X" exercise, no
  scaffolded codebase the reader extends. A course may describe how a ledger system is architected;
  it may not ask the reader to build one.

The four courses this removes are `capstone-build-a-general-ledger-system`,
`capstone-sharia-compliant-ledger`, `capstone-build-a-minimal-erp-core`, and
`capstone-stand-up-and-integrate-an-open-source-erp`. The first three fail the build test; the fourth
fails `A7` as well, being buyer-competence material.

## A8 — licensing binds the whole programme

`A8` originally read as a plan-06/07 concern because the standards bodies are most visibly
restrictive there. That scoping was wrong: **every plan in the programme authors teaching material,
and teaching material is where copyright exposure concentrates.** The careers corpus carries its own
distinct hazards, and they are easy to miss precisely because programming content feels free:

- **Code examples** copied from documentation, tutorials, blog posts or Stack Overflow. Stack
  Overflow contributions are CC-BY-SA — attribution _and_ share-alike, which is a licence most course
  material cannot satisfy. Author examples originally.
- **Documentation prose** from a framework's official docs. Being free to read is not permission to
  reproduce; most project docs carry their own licence, and it is frequently copyleft.
- **Figures, diagrams and screenshots** lifted from vendor or project sites.
- **Book and course structure.** Reproducing a well-known book's chapter progression, or a paid
  course's module sequence, is the same derivative-work risk as `A12` addresses for syllabi.
- **Trademarks.** Language, framework and vendor names may be used nominatively but never in a course
  title, path segment, or anything that implies endorsement or affiliation.
- **Datasets and sample data** — author them; do not lift a dataset whose licence is unexamined.

The `A8` posture is therefore uniform across all seven plans: **describe, cite and link; never
reproduce.** Where a reader needs the source text, send them to the source.

## A12 — how a syllabus may and may not be confirmed

`A12` exists because the confirmation step introduces the exact risk the rest of `A8` guards against.
Published curricula — ACCA, CPA, CIMA, ASCM/APICS CPIM and CSCP, university course catalogues — are
**copyrighted works**, and several are commercial products whose syllabus _is_ the product. Checking
a syllabus against one is legitimate; deriving a syllabus from one is not.

The order of operations is what keeps this clean, and it is not optional:

1. Author the syllabus from domain reasoning and the plan's own research grounding.
2. **Then** research externally to ask whether the coverage is right — what a practitioner would
   expect that a draft omits, and what it includes that the field does not recognise.
3. Treat the answer as **evidence about coverage**, never as a structure to adopt. A finding is
   actionable as "this topic is missing"; it is never actionable as "reorder to match theirs."

Confirmation must never reproduce a curriculum's text, its module titles, or its sequence. Naming a
body as corroboration ("the topic appears in ASCM's CPIM outline") is nominative use and is fine;
transcribing its outline is not.
