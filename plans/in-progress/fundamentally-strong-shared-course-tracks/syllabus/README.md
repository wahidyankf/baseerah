# Syllabus — Fundamentally Strong Shared Course Library + Three Paths

This `syllabus/` folder is the design surface for a **shared course library** and the **three learning
paths** built over it. It has three parts:

- **[courses/](./courses/README.md)** — the **per-course-block detail layer**: the index of the
  **121-course catalog** and one **`<course-id>.md`** detail file per course (concepts, worked examples,
  capstone spec). Courses have **no single order** here — order is a per-path property.
- **[paths/](./paths/README.md)** — the **three path manifests**, each an ordered, prerequisite-consistent
  list of course IDs over the shared library.
- **This root README** — the architecture: how a course (building block) and a path (ordered manifest)
  relate, the three paths at a glance, and the library-wide guarantees.

**Source of truth**: the [tech-docs §Course Library Catalog](../tech-docs.md#course-library-catalog)
(course ID, format, language, short summary) and [tech-docs §Path Manifests](../tech-docs.md#path-manifests)
(the three orderings) are authoritative. The [prd.md](../prd.md) holds the product spec (personas, user
stories, Gherkin, NEW-course specs). This folder adds the dimension the tables cannot hold: per course,
the concrete **Concepts** (`co-NN`), **Worked examples** (`ex-NN`), and **Capstone spec**; and per path,
the concrete ordered manifest.

## Core model — one shared library, three composing paths

```mermaid
flowchart TD
    LIB["Course Library<br/>(one body per course-id,<br/>prerequisite DAG)"]:::lib
    IR["manifest ·<br/>interview-ready/<br/>software-engineer<br/>interview-first"]:::ir
    IE["manifest ·<br/>immediately-effective/<br/>software-engineer<br/>shipping-first"]:::ie
    FS["manifest ·<br/>fundamentally-strong/<br/>software-engineer<br/>fundamentals-first"]:::fs
    IR -->|ordered course-ids| LIB
    IE -->|ordered course-ids| LIB
    FS -->|ordered course-ids| LIB

    classDef lib fill:#0072B2,stroke:#000,color:#fff
    classDef ir fill:#E69F00,stroke:#000,color:#000
    classDef ie fill:#009E73,stroke:#000,color:#fff
    classDef fs fill:#CC79A7,stroke:#000,color:#000
```

- **Course = standalone, path-neutral building block.** A course is a self-contained topic module
  (learning + drilling track) with a stable **course ID** (its kebab-case slug). One canonical body,
  one canonical URL (`/en/c/learn/courses/<course-id>`), authored once, never forked. Rendering a course
  with no path context shows its canonical standalone view.
- **Path = ordered manifest composing course-ids.** A path lists course IDs in a chosen order over a
  curated selection of the library; a course page reads the active path context (`?path=<path-id>`) and
  its prev/next + breadcrumb follow that path's order. Path landings live at
  `/en/c/learn/paths/<path-id>`. See
  [tech-docs §Path-Aware Navigation UI](../tech-docs.md#path-aware-navigation-ui-ayokoding-www).
- **Prerequisite DAG.** Every course declares `prerequisites: [course-id, ...]` in its canonical
  metadata; the library forms a **prerequisite DAG**. Each path manifest MUST be a valid topological
  entry into that DAG (a prerequisite-consistent ordering). The three paths are three different entry
  points and orderings into the one DAG that converges on the same endpoint.
- **Omit-or-create.** A path omits a course that does not fit its arc and creates a new course only for
  a real gap (added to the library, available to every path). Optional per-path framing is a
  lightweight intro/outro callout, never a body fork. When a path needs a genuinely different teaching
  approach for a topic, author a separate course _variant_ (distinct course-id); the default is still
  one shared, path-neutral block.

## The three paths

All three end at the **same deep mastery** — only the **entry point, journey ordering, and teaching
emphasis** differ. `fundamentally-strong` is **both** the library/section brand and path #3's id.

- **[`interview-ready/software-engineer`](./paths/manifest-interview-ready-software-engineer.md)
  (interview-first)** — for an **experienced engineer re-entering the job market**: interview/job prep
  **first** → production-effective → deeper. Ships first. (Formerly `job-seeking`.)
- **[`immediately-effective/software-engineer`](./paths/manifest-immediately-effective-software-engineer.md)
  (shipping-first)** — for a **builder who wants to be effective fast**: editor → one language → build a
  real app **first** → then deepen. (Formerly the `fundamentally-strong` shipping-first path.)
- **[`fundamentally-strong/software-engineer`](./paths/manifest-fundamentally-strong-software-engineer.md)
  (fundamentals-first)** — university-style: CS theory and fundamentals **first** → breadth →
  application → deeper. (New path.)

## Skip / fast-path affordances (per path)

- **Interview-first** — skip the editor prologue; start at the stand-alone Phase 1; refresh register
  (re-ground a working engineer, not first-teach); skip any `just-enough-<lang>` primer you already own;
  phase-boundary bridges soften the two sharp transitions.
- **Shipping-first** — "already fluent in a language? jump straight to the build-an-app stage"; a
  Stage-2→Stage-3 bridge ("you shipped it; now understand why it worked") softens the shipping → CS-depth
  transition.
- **Fundamentals-first** — the editor prologue is skippable; primers are skippable ("if you already know
  X, jump to Y"); a Stage-8→Stage-9 bridge softens the internals-builds → application-development
  transition.

See [tech-docs §Smoothness Architecture](../tech-docs.md#smoothness-architecture-per-path).

## Principle-transfer productivity note (proof-of-transfer, NOT repo tutorials)

The library teaches **durable principles**; the seven target codebases
(`ose-public`/`ose-primer`/`ose-infra`, `remotebrowser`, `wazuh/wazuh`, `vacti`, `vacti-pentest-engine`)
are **evidence the principles transfer**, never subject matter. No course names any repo as its subject.
See [tech-docs §Productive in Target Codebases](../tech-docs.md#productive-in-target-codebases-proof-of-transfer-outcome-anchor).
This anchor justifies the **library** and is inherited by all three paths.

---

Next: [courses/README.md — the 121-course catalog](./courses/README.md) ·
[paths/README.md — the three path manifests](./paths/README.md)
