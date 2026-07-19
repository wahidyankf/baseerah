# Business Requirements — Fundamentally Strong Shared Course Library, Three Paths

## Business Goal

Reframe the "Fundamentally Strong" curriculum as a **shared course library** composed by **three
learning paths**, so one body of content (already-authored and being-authored) can serve three very
different readers **converging on the same deep mastery** without any duplication:

- an **`interview-ready/software-engineer`** path — the **interview/job-prep-first** arc for an
  experienced engineer re-entering the market: interview prep FIRST → production-effective → deeper;
- an **`immediately-effective/software-engineer`** path — the **immediately-effective** principle:
  set up the editor, learn one language end-to-end, **build a real app first**, then deepen; and
- a **`fundamentally-strong/software-engineer`** path — the **university-style, fundamentals-first**
  arc: CS foundations / theory first → deeper.

Each course is a **standalone, path-neutral building block** (one topic = one course, stable ID,
single canonical body); each path is an **ordered, prerequisite-consistent manifest** composing a
**curated subset** of the library. All three paths converge on the **same endpoint** — only the
**entry point**, the **journey ordering**, and the **teaching emphasis** differ. The business change is
**architecture + framing + a real navigation UI**, plus the thin layer of NEW courses the interview
cluster and the productivity / harness / security clusters need — not a rewrite of the existing
topics' subject content.

## Why a shared library instead of three curricula

The naive alternative — author three separate curricula — would triplicate ~94 topics, triple the
maintenance surface, and let three trees drift out of sync [Judgment call]. The shared-library model
avoids that entirely:

- **Single source of truth per course.** A course body is authored once, is path-neutral, and lives
  at one canonical URL (`/en/c/learn/courses/<course-id>`). Fixing a typo, updating a version, or
  improving an example benefits **all three** paths at once.
- **Zero duplication.** A path is a lightweight ordered list of course IDs — cheap to author, cheap to
  change, and impossible to fork a body through.
- **Prerequisite DAG keeps every path honest.** Every course declares its prerequisites, so the
  library forms one dependency graph; each path is simply a different valid **entry point and
  topological ordering** into that graph. Three paths, one DAG, one converging endpoint.
- **Omit-or-create keeps each path coherent.** A path omits a course that does not fit its arc, and a
  new course is created only when nothing in the library covers a real need — and that new course is
  then available to the other paths too. When a topic genuinely needs a different **teaching approach**
  for a path, a distinct **course variant** is authored (same topic, distinct pedagogy, distinct ID)
  rather than forking a shared body. Growth is additive and shared.
- **Three audiences, one investment.** The maintainer already invested in ~94 topics; the
  shared-library model turns that single investment into three products (an interview-ready track, a
  build-app-first track, and a theory-first track) for the marginal cost of three manifests plus the
  interview / productivity / harness / security NEW courses.

## Why these three paths

The maintainer's read of how the material is actually consumed identifies three distinct, high-value
entry points into the same mastery [Judgment call]:

- **The experienced engineer re-entering the job market** (`interview-ready/software-engineer`). Lands
  days-to-weeks before a senior loop, already owns the editor workflow and the deep fundamentals, and
  needs to **refresh breadth fast, relearn interview technique, and get interview-ready** at
  mid/senior/staff level — including framing a layoff / employment-gap narrative. The interview-first
  arc leads with the most time-pressured, highest-stakes use.
- **The builder who wants to be effective fast** (`immediately-effective/software-engineer`). Does not
  want a spiral or a theory-first march; wants to set up the editor, pick up one language, **ship a
  real working app early**, and only then go deep. Shipping-first sequencing matches how motivated
  self-learners stay engaged — momentum from a working artifact, then depth once the payoff is felt.
- **The university-style, fundamentals-first learner** (`fundamentally-strong/software-engineer`).
  Wants the rigorous bottom-up route: CS foundations, computer architecture, paradigms, data
  structures and algorithms **before** building apps at scale. Prefers to understand the machine and
  the theory first, then apply it — the classic degree-program ordering.

One library serves all three because the underlying **principles are the same** — only the **order**
and the **thin framing** differ, and every ordering respects the same prerequisite DAG. That is
exactly what a manifest expresses.

## Why the navigation is a real UI change (not just content)

A single body served in three different orders cannot be expressed by the current model, where reading
order is carried by a single `weight` frontmatter value per page [Repo-grounded — `computePrevNext`
in `apps/ayokoding-www/src/features/content/core/tree-builder.ts` sorts siblings by `weight`]. Three
orders over one body require the **order to move out of the body and into the path manifest**, and the
course page's prev/next + breadcrumb to **resolve against the active path**. The course page must also
**surface each course's declared prerequisites**. That is a genuine frontend change to ayokoding-www
(a Next.js app) — routing under the `/en/c/learn` URL model, a `?path=` context, manifest-driven
navigation, prerequisite display, and a graceful fallback when a course is deep-linked without path
context. The maintainer explicitly asked that this UI be **planned properly**, with a design funnel,
accessibility, and unit/integration/e2e tests plus a `specs/` Gherkin companion.

## Business Impact

**Pain points addressed**:

- Today the curriculum has exactly one arc; an interview-ready re-entrant, a productive-fast builder,
  and a theory-first learner are all forced through the same order, and none is optimally served.
- There is no interview-technique material, no shipping-first productive on-ramp, and no explicit
  fundamentals-first ordering as first-class paths.
- Three separate curricula would triplicate content and triple maintenance.

**Expected benefits** (qualitative reasoning; no fabricated metrics):

- Three audience-fit products from one content investment, with no duplication and one maintenance
  surface.
- A reusable **course-library + path-manifest + prerequisite-DAG** capability in ayokoding-www that
  future tracks (e.g. a security track, a data track) can reuse for the marginal cost of one more
  manifest.
- The interview-ready path ships real technique modules; the immediately-effective path ships a
  build-app-first productive arc; the fundamentally-strong path ships a rigorous theory-first arc —
  each with a coherent, path-aware, prerequisite-aware reading experience.

## Affected Roles

Solo-maintainer repo — no sign-off ceremony. The maintainer wears:

- **Content strategist** — owns the three-path architecture and each path's arc/framing.
- **Frontend engineer** — builds the ayokoding-www path-aware navigation feature.
- **Content author** (via the `apps-ayokoding-www-*-maker` agents) — writes the NEW courses.
- **Content reviewer** (via the `apps-ayokoding-www-*-checker` + facts/link checkers) — validates.

Consuming agents: `apps-ayokoding-www-by-example-maker`, `apps-ayokoding-www-annotated-concept-maker`,
`apps-ayokoding-www-primer-maker` and their matching checkers, plus `apps-ayokoding-www-facts-checker`
and `apps-ayokoding-www-link-checker` [Repo-grounded]; `swe-typescript-dev` and `swe-e2e-dev` for the
navigation UI feature.

## Business-Level Success Metrics

- **One body, three orders, zero duplication** (observable, first-class signal): every course has
  exactly one canonical, path-neutral body; all three path manifests reference courses **by ID**; a
  grep confirms no course body is duplicated per path. The
  [`syllabus/courses/` catalog](./syllabus/courses/README.md) enumerates each of the 121 courses once.
- **Prerequisite DAG is consistent** (observable): every course declares `prerequisites`; the canonical
  course page surfaces them; a manifest-integrity check confirms every path's `courseOrder` is a valid
  topological entry into the DAG (no course precedes a prerequisite). Verified by a phase gate + unit
  test.
- **Path-aware navigation works** (observable): from a path landing page, a reader walks the course
  order for that path; prev/next and breadcrumb follow the path manifest; a course deep-linked without
  `?path=` renders a coherent canonical view. Verified by unit + integration + e2e tests and the
  `specs/` Gherkin companion.
- **interview-ready path ships first, end-to-end** (observable): the interview-first path — landing
  page, its re-homed topics 1–33, the interview cluster, its manifest, path-aware nav — is complete and
  deployed to production **before** the other two paths' manifests are composed.
- **immediately-effective path is build-app-first** (observable): its manifest places editor/tooling →
  one language end-to-end → **build a real app** ahead of CS-fundamentals/DS&A/algorithms/systems
  depth; it reuses the shared courses with zero body duplication.
- **fundamentally-strong path is theory-first** (observable): its manifest places CS
  foundations/architecture/paradigms/DS&A ahead of build-at-scale courses; it reuses the shared
  courses with zero body duplication.
- **Interview coverage** (observable): the four NEW interview modules ship a learning + drilling track
  each, in a **refresh register**, and pass their checker + facts-checker + link-checker; the
  behavioral module covers the **layoff / employment-gap narrative**.
- **Productive in target codebases** (observable, retained from the prior scope): the productivity /
  harness / security NEW courses (`async-python-and-fastapi-services`, `browser-automation-with-cdp`,
  the harness cluster, `just-enough-cpp`, `detection-engineering-and-siem-operations`, the
  build-your-own capstones) fill the stack gaps for the target codebases; see
  [tech-docs §Productive in Target Codebases](./tech-docs.md#productive-in-target-codebases-proof-of-transfer-outcome-anchor).
- **Progression smoothness** (observable): each path reads smoothly for its persona — prereq-chaining,
  monotonic-ish difficulty, skip/fast-path affordances, and (for the interview-ready path) refresh
  register — verified by a per-path smoothness audit before archival.
- **No regressions** (observable): `nx run ayokoding-www:build` renders green; `test:unit` /
  `test:integration` / `test:e2e`, heading-hierarchy, markdownlint, and link validation pass across the
  app and the section; legacy `fundamentally-strong/software-engineer/<slug>` URLs redirect to
  `/en/c/learn/courses/<course-id>`.

## Business-Scope Non-Goals

- Re-writing the pedagogy or depth of any existing topic (only re-homing/native-authoring +
  re-framing + re-ordering; and authoring a course variant only where a path genuinely needs a
  different teaching approach).
- Adding an Indonesian mirror of the section content — deferred (the nav UI still handles all app
  locales correctly).
- Building path-level progress persistence, accounts, or bookmarking — the path context is
  URL/client-state only for this plan (a future enhancement).
- Interactive/JS flashcards — drilling stays static markdown, matching the sibling.
- Enumerating speculative course variants — variants are authored on demand only.

## Business Risks and Mitigations

| Risk                                                                                | Mitigation                                                                                                                                                                                                                                                                        |
| ----------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Re-homing the 33 shipped topics to `courses/` breaks live URLs.                     | Every re-home lands with a redirect (`apps/ayokoding-www/src/redirects/`) from the legacy `fundamentally-strong/software-engineer/<slug>` URL to the new `/en/c/learn/courses/<course-id>` URL.                                                                                   |
| Native-authored topics 34–94 stall the whole plan.                                  | The FS-SE hard dependency is removed; interview-ready ships first over the re-homed 1–33 + interview cluster; topics 34–94 backfill incrementally without blocking the MVP.                                                                                                       |
| A path manifest violates the prerequisite DAG (a course precedes its prerequisite). | A manifest-integrity check verifies every `courseOrder` is a valid topological entry into the DAG; runs as a phase gate + unit test; course IDs are stable slugs, never renumbered.                                                                                               |
| Path context lost on share/deep-link degrades the reading experience.               | Graceful canonical fallback is a first-class design requirement + a Gherkin scenario + an e2e test; a course page always names the paths that include it and surfaces its prerequisites.                                                                                          |
| Three manifests drift or reference a missing/renamed course ID.                     | The manifest-integrity check (every `courseOrder` ID resolves to a library course) runs as a phase gate and a unit test.                                                                                                                                                          |
| Duplication creeps in (a path forks a body for its framing).                        | Framing is limited to an optional intro/outro callout applied by the path layer; a distinct-pedagogy need is met by a separate course variant, never a body copy — enforced by review + a no-duplicate-body check.                                                                |
| AI-band courses duplicate the agent-loop/tools/MCP/memory/evals material.           | The AI-band scope-guard contract: `agentic-ai` (57) is a survey that forward-links each primitive to its harness-cluster course and stops short of build-your-own depth.                                                                                                          |
| detection-engineering and defensive-security overlap.                               | Explicit scope lines: `defensive-security` (re-labelled hands-on By-Example) keeps generalist Sigma/ELK breadth + IR + hardening; `detection-engineering-and-siem-operations` owns deep Wazuh decoder/rule/dashboard SIEM-ops and names `defensive-security` as its prerequisite. |
| Navigation UI regresses existing content nav (non-path readers).                    | The canonical (no-path) view is the existing behavior; the UI adds path-awareness without changing default nav — covered by retained navigation specs + tests.                                                                                                                    |
