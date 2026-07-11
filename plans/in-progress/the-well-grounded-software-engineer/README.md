# The Well-Grounded Software Engineer

A cross-cutting **relearn-and-drill** tutorial section on ayokoding-www that lets a working
software engineer re-ground themselves across the breadth of the discipline — computer science
through IT security — via two parallel tracks: **learning** (by-example-depth per topic) and
**drilling** (active-recall practice). Breadth across the field, **by-example pace within each
topic**.

## Context

Working engineers routinely need to _re-load_ a topic they once knew but haven't touched in a
while — before an interview, before joining a new team, before a design review, or just to close a
nagging knowledge gap. The existing `learn/software-engineering/` tree on ayokoding-www is
excellent for _first-time deep learning_ (Diátaxis tutorials, by-example, in-the-field), but it is
optimized for depth, not for a fast, breadth-first refresh. There is no single place a practitioner
can go to _quickly_ re-ground themselves across the whole field and then _test_ that the knowledge
actually stuck.

This matters most **in the age of AI and LLMs**: when assistants generate more and more of the code,
the engineer's durable edge is a solid grasp of the fundamentals needed to judge, review, and
correct that output. This section is the human-in-the-loop's reference for staying well-grounded
across the field rather than deferring blindly to generated answers.

This plan adds **"The Well-Grounded Software Engineer"** — a new section under
`learn/software-engineering/` — that is deliberately breadth-first, self-contained, and split into a
learning track and a drilling track covering the same topics in the same order.

## Scope

**In scope** (all under `apps/ayokoding-www/content/en/learn/software-engineering/the-well-grounded-software-engineer/`):

- A section landing (`_index.md`) + `overview.md`.
- A **learning** track: a by-example-depth learning subtree per topic (**hybrid format** — code-centric
  topics use the ayokoding **By Example** format; concept-centric topics use an equal-density
  **annotated-concept** format with worked examples + accessible Mermaid diagrams). See
  [prd.md](./prd.md) for the per-topic format assignment and volume targets.
- A **drilling** track: one active-recall page per topic (same topics, same order) with four drill
  forms (recall Q&A, applied scenarios, code katas, self-check checklist).
- **32 topics** (topic set still **open** — more may be added before lock), identical ordering across
  both tracks, sequenced as a **seven-level, interview/job-relevance-first journey** (what a SWE
  interview most tests + what makes you job-ready first → deeper/specialized/niche later;
  app-building before OS-level depth). See [prd.md](./prd.md) for the canonical topic table with
  per-topic level, format, primary language, and weights, and [syllabus.md](./syllabus.md) for every
  item and example inside each topic. In journey order:
  1. Data Structures & Algorithms _(L1 · Interview Core)_
  2. Computer Science Foundations _(L1)_
  3. Computer Networking _(L1)_
  4. Object-Oriented Programming _(L1)_
  5. Programming Paradigms _(L1)_ — survey framing the paradigm topics
  6. Functional Programming _(L1)_
  7. Concurrency & Parallelism _(L1)_
  8. Software Engineering Practices _(L2 · Build & Ship)_
  9. Data Storage (Databases) _(L2)_
  10. Backend Development _(L2)_
  11. Frontend Development _(L2)_
  12. Android App Development _(L2)_
  13. iOS App Development _(L2)_
  14. Software Architecture _(L3 · Design at Scale)_
  15. System Design _(L3)_
  16. Windows App Development _(L4 · Broaden Delivery)_
  17. Linux App Development _(L4)_
  18. Cloud, Containers & IaC _(L4)_
  19. Data Engineering _(L4)_
  20. Creating AI-Powered Apps _(L4)_
  21. IT Security _(L4)_
  22. Linux OS _(L5 · Systems & Language Depth)_
  23. Windows OS _(L5)_
  24. System Programming _(L5)_
  25. Lisp _(L5)_
  26. Type Systems (Hindley–Milner) _(L5)_
  27. Compilers, Parsers & Transpilers _(L5)_
  28. Site Reliability Engineering _(L6 · Advanced Ops)_
  29. IT Governance (IT GRC) _(L7 · Leadership & Product)_
  30. Project Management _(L7)_
  31. Software Product Engineering _(L7)_
  32. Engineering Management _(L7)_
- Wiring the new section into the `learn/software-engineering/_index.md` and `learn/_index.md`
  navigation.
- English only.

**Out of scope**:

- Indonesian (`content/id/...`) mirror — deferred; may follow later.
- Any change to the existing `learn/software-engineering/` deep-content subtrees (system-design,
  algorithms-and-data-structures, etc.). This section is **self-contained** — it does NOT link into
  or restructure existing content.
- Any application/component/code change under `apps/ayokoding-www/src/` — this is a **content-only**
  plan (markdown under `content/`).
- Interactive/JS-driven flashcards — drilling uses static markdown with `<details>` collapsibles
  (already supported in existing content).

## Approach Summary

Two parallel tracks under one section root, topics in identical order in both:

```
learn/software-engineering/the-well-grounded-software-engineer/
  _index.md            # section landing (nav list)
  overview.md          # what this is, how to use the two tracks
  learning/            # by-example-depth per topic (subtree per topic, weights 101..132)
  drilling/            # active-recall practice (one page per topic, same order, weights 201..232)
```

Each learning topic is authored at **by-example pace** (annotation density 1.0–2.25 comments/line,
incremental, five-part examples for code topics; equal-density annotated worked examples + diagrams
for concept topics). Each **drilling** page follows one fixed anatomy combining all four drill forms
(recall Q&A / applied scenarios / code katas / self-check checklist).

**Concrete, single-primary-language rule**: any topic whose content uses code uses a **real
programming language** (never pseudocode), and one **primary language** is used across as many topics
as possible for consistency. A topic deviates only where the platform mandates it (e.g. Swift for iOS,
Kotlin for Android, C for low-level system programming, a JS/TS-family language for frontend). The
primary language is fixed in [tech-docs.md](./tech-docs.md) (DD-7). See [prd.md](./prd.md) for the
per-topic language column.

Content authored via `apps-ayokoding-www-by-example-maker` (code topics) and
`apps-ayokoding-www-general-maker` (concept topics + scaffolding); validated via the matching
checker plus `apps-ayokoding-www-facts-checker` + `apps-ayokoding-www-link-checker`. Delivered as
`main-to-origin-main` (primary checkout, direct `[AI]` push to `origin main`, no worktree, no PR).

## Navigation

- [Business Requirements (brd.md)](./brd.md) — WHY this section exists, who it serves, success signals.
- [Product Requirements (prd.md)](./prd.md) — WHAT the pages contain, personas, user stories, Gherkin
  acceptance criteria, page anatomies.
- [Syllabus (syllabus.md)](./syllabus.md) — every item and worked example inside each of the 32 topics.
- [Technical Docs (tech-docs.md)](./tech-docs.md) — HOW: content-tree layout, weights, frontmatter,
  drilling markup, file-impact, diagrams.
- [Delivery Checklist (delivery.md)](./delivery.md) — DO: phased, executable checklist.
- [Learnings (learnings.md)](./learnings.md) — Knowledge-capture running log.

## Delivery Mode

`main-to-origin-main` — primary checkout (no worktree), direct `[AI]` push to `origin main`, no PR
and no human-merge gate. See [delivery.md](./delivery.md) for the `## Worktree` and `## Delivery Mode`
declarations.
