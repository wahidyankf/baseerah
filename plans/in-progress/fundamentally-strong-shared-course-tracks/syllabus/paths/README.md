# Path Manifests — Three Curated Orderings Over the Shared Library

This folder holds the **three path manifests** — the human-readable mirrors of the machine-consumed
ordering data. A **path** is an ordered, prerequisite-consistent list of **course IDs** over the
[shared course library](../courses/README.md); it composes existing course building blocks in a chosen
order and adds **zero new bodies**. All three paths **converge on the same deep-mastery endpoint** (the
AI/harness cluster, internals builds, distributed systems, and the security capstone) — only the entry
point, journey ordering, teaching emphasis, and **which courses are curated into the spine** differ. See
the [syllabus root README](../README.md) for the full course-vs-path architecture.

## Curated + converge (LOCKED decision, 2026-07-19)

The three paths are **not all comprehensive** — not every course appears in every path. Each path is a
**curated subset ordering** over the one prerequisite DAG:

- **`fundamentally-strong/software-engineer`** is the **complete-mastery** path: it includes **all 121**
  courses in a theory-first ordering and is the only path that omits nothing.
- **`interview-ready/software-engineer`** teaches an **interview + core + production spine**, then offers
  the deep-systems / OS / kernel / compilers / internals-builds / niche courses as an explicit optional
  **"Go deeper" tail** — reachable, but never required for interview-readiness.
- **`immediately-effective/software-engineer`** teaches a **build-first spine** (ship a real app first),
  then defers the heavy theory (CS foundations, type systems, advanced algorithms, paradigms, computer
  architecture, and the rest of the CS/systems depth) into a later **Deepening band**.

The curated paths **genuinely omit** a small, curriculum-judged set of niche courses (never a
prerequisite of anything they include, so each manifest stays **prerequisite-closed**). Every course a
path includes appears **after all of its prerequisites** — a property machine-verified for all three
manifests.

**DD-20 addendum (2026-07-19)**: the 121-course catalog includes seven inter-topic capstones
reconciled in from other topics' embedded specs (`capstone-solid-core` plus six new capstones — see
[tech-docs DD-20](../../tech-docs.md#design-decisions)). All seven are included in all three manifests
(none is genuinely omitted); each is placed at its earliest prerequisite-safe position.

Each manifest is the **human-readable mirror**. The **machine-consumed source of truth** is a standalone
data file at `apps/ayokoding-www/src/features/course-paths/manifests/<path-id>.yaml` (nested to mirror
the slash-form path id — e.g. `manifests/interview-ready/software-engineer.yaml`). Path landing pages are
served at `/en/c/learn/paths/<path-id>`; a course page reads path context via `?path=<path-id>` and its
prev/next + breadcrumb follow that path's ordering.

## The three paths

| Path id                                   | Persona                                         | Shape                                                                      | Manifest                                                          |
| ----------------------------------------- | ----------------------------------------------- | -------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `interview-ready/software-engineer`       | Experienced engineer re-entering the job market | Interview/production **spine** + optional **Go deeper tail** (116 courses) | [manifest](./manifest-interview-ready-software-engineer.md)       |
| `immediately-effective/software-engineer` | Builder who wants to be effective fast          | Build-first **spine** + **Deepening band** (119 courses)                   | [manifest](./manifest-immediately-effective-software-engineer.md) |
| `fundamentally-strong/software-engineer`  | Learner who wants university-style depth        | Theory-first, **all 121 courses** (complete mastery)                       | [manifest](./manifest-fundamentally-strong-software-engineer.md)  |

- **`interview-ready`** was formerly `job-seeking`.
- **`immediately-effective`** was formerly the `fundamentally-strong` shipping-first path.
- **`fundamentally-strong`** is the new university-style path, and is also the library/section brand.

## How to read a manifest

Each manifest is an **ordered list of course IDs** grouped into phases/stages, with a short composition
rationale, an ordered spine, an optional-tail / deepening-band section for the deferred-or-deeper courses,
and smoothness notes (RD-16). Order is a per-path property; it is **not** a catalog property (the
[catalog](../courses/README.md) is order-neutral).

Every manifest is **prerequisite-consistent**: it is a valid topological entry into the library's
prerequisite DAG, so every `just-enough-<lang>` primer — and every prerequisite course — precedes its
first use within that path. A course a path relegates to its optional tail or deepening band is still
present in that path (just out of the required spine); a course a path genuinely omits appears only in
the paths that include it. Each course's own **"In which paths"** section names the exact section it sits
in for every path that carries it.

---

← Back to the [syllabus root README](../README.md)
