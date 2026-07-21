# Business Requirements — Fundamentally Strong Shared Course Library, Four Paths

## Business Goal

Reframe the "Fundamentally Strong" curriculum as a **shared course library** composed by **four
learning paths**, so one body of content (already-authored and being-authored) can serve four very
different readers, **converging within a role** rather than at one single global endpoint, without
any duplication:

- an **`interview-ready/software-engineer`** path — the **interview/job-prep-first** arc for an
  experienced engineer re-entering the market: interview prep FIRST → production-effective → deeper;
- an **`immediately-effective/software-engineer`** path — the **immediately-effective** principle:
  set up the editor, learn one language end-to-end, **build a real app first**, then deepen;
- a **`fundamentally-strong/software-engineer`** path — the **university-style, fundamentals-first**
  arc: CS foundations / theory first → deeper; and
- an **`immediately-effective/software-engineer-to-ai-engineer`** path (added 2026-07-20) — an
  already-working software engineer transitioning to AI engineering: assumes SWE competence
  (prerequisite courses are **linked, not included**) and teaches **building** AI systems, fast
  because it assumes competence, not because it skips depth.

Each course is a **standalone, path-neutral building block** (one topic = one course, stable ID,
single canonical body); each path is an **ordered, prerequisite-consistent manifest** composing a
**curated subset** of the library. The three `software-engineer` paths converge on the **same**
software-engineering endpoint — only the **entry point**, the **journey ordering**, and the
**teaching emphasis** differ. The fourth path converges on a **distinct** AI-engineering endpoint: the
library now serves **more than one endpoint**, one per role it serves — convergence is a per-role
property, not a library-wide axiom (see
[tech-docs.md DD-22](./tech-docs.md#design-decisions)). The business change is **architecture +
framing + a real navigation UI**, plus the thin layer of NEW courses the interview cluster, the
productivity / harness / security clusters, and (as of 2026-07-20) the AI-engineering cluster need —
not a rewrite of the existing topics' subject content.

## Why a shared library instead of four curricula

The naive alternative — author four separate curricula — would quadruplicate ~94 topics, quadruple the
maintenance surface, and let four trees drift out of sync [Judgment call]. The shared-library model
avoids that entirely:

- **Single source of truth per course.** A course body is authored once, is path-neutral, and lives
  at one canonical URL (`/en/c/learn/courses/<course-id>`). Fixing a typo, updating a version, or
  improving an example benefits **every** path that references it at once.
- **Zero duplication.** A path is a lightweight ordered list of course IDs — cheap to author, cheap to
  change, and impossible to fork a body through.
- **Prerequisite DAG keeps every path honest.** Every course declares its prerequisites, so the
  library forms one dependency graph; each path is simply a different valid **entry point and
  topological ordering** into that graph. Four paths, one DAG, per-role convergence (not one global
  endpoint — see [tech-docs.md DD-22](./tech-docs.md#design-decisions)).
- **Omit-or-create keeps each path coherent; course surgery is now permitted too.** A path omits a
  course that does not fit its arc, and a new course is created only when nothing in the library
  covers a real need — and that new course is then available to the other paths too. When a topic
  genuinely needs a different **teaching approach** for a path, a distinct **course variant** is
  authored (same topic, distinct pedagogy, distinct ID) rather than forking a shared body. As of
  2026-07-20, update/merge/split/create **course surgery** is also permitted (superseding the original
  zero-new-bodies invariant): because courses are shared, any surgery is a **four-path change** whose
  blast radius across all four manifests must be stated up front and re-verified afterward (see
  [tech-docs.md DD-28](./tech-docs.md#design-decisions)). Growth is additive and shared.
- **Four audiences, one investment.** The maintainer already invested in ~94 topics; the
  shared-library model turns that single investment into four products (an interview-ready track, a
  build-app-first track, a theory-first track, and — as of 2026-07-20 — a software-engineer-to-AI-engineer
  track) for the marginal cost of four manifests plus the interview / productivity / harness / security
  / AI-engineering NEW courses.

## Why these four paths

The maintainer's read of how the material is actually consumed identifies three distinct, high-value
entry points into the same software-engineering mastery, plus a fourth, distinct entry point into a
different, AI-engineering mastery [Judgment call]:

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
- **The already-working software engineer transitioning to AI engineering**
  (`immediately-effective/software-engineer-to-ai-engineer`, added 2026-07-20). Already owns the
  software-engineering fundamentals the first three paths teach, so re-teaching them would waste this
  reader's time; wants a short, AI-specific spine that **links** (not re-includes) those prerequisites
  and gets straight to **building** AI systems — models, agents, evals, inference serving — as fast as
  the immediately-effective principle allows for a specialization. This reader is heading toward a
  **different endpoint** than the other three (AI-engineering mastery, not software-engineering
  mastery), which is why this path does not claim to converge with them.

The first three paths share one library because the underlying **principles are the same** — only the
**order** and the **thin framing** differ, and every ordering respects the same prerequisite DAG. That
is exactly what a manifest expresses. The fourth path shares the same library **infrastructure** (the
manifest mechanism, the prerequisite DAG, the navigation UI) but not the same **destination** — it is
a second product built on the same machinery, not a fourth entry point into the same mastery.

## Why the navigation is a real UI change (not just content)

A single body served in four different orders cannot be expressed by the current model, where reading
order is carried by a single `weight` frontmatter value per page [Repo-grounded — `computePrevNext`
in `apps/ayokoding-www/src/features/content/core/tree-builder.ts` sorts siblings by `weight`]. Four
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
- An already-working software engineer who wants to move into AI engineering has no path that assumes
  their existing competence — they either re-walk SWE fundamentals they already have, or have no
  structured on-ramp into building AI systems at all (added 2026-07-20).
- Four separate curricula would quadruplicate content and quadruple maintenance.

**Expected benefits** (qualitative reasoning; no fabricated metrics):

- Four audience-fit products from one content investment, with no duplication and one maintenance
  surface.
- A reusable **course-library + path-manifest + prerequisite-DAG** capability in ayokoding-www that
  future tracks (e.g. a security track, a data track) can reuse for the marginal cost of one more
  manifest — proven out by the fourth, AI-engineering track added 2026-07-20 at exactly that marginal
  cost.
- The interview-ready path ships real technique modules; the immediately-effective path ships a
  build-app-first productive arc; the fundamentally-strong path ships a rigorous theory-first arc; the
  software-engineer-to-ai-engineer path ships a short, assumes-competence spine into building AI
  systems — each with a coherent, path-aware, prerequisite-aware reading experience.

## Affected Roles

Solo-maintainer repo — no sign-off ceremony. The maintainer wears:

- **Content strategist** — owns the four-path architecture and each path's arc/framing.
- **Frontend engineer** — builds the ayokoding-www path-aware navigation feature.
- **Content author** (via the `apps-ayokoding-www-*-maker` agents) — writes the NEW courses.
- **Content reviewer** (via the `apps-ayokoding-www-*-checker` + facts/link checkers) — validates.

Consuming agents: `apps-ayokoding-www-by-example-maker`, `apps-ayokoding-www-annotated-concept-maker`,
`apps-ayokoding-www-primer-maker` and their matching checkers, plus `apps-ayokoding-www-facts-checker`
and `apps-ayokoding-www-link-checker` [Repo-grounded]; `swe-typescript-dev` and `swe-e2e-dev` for the
navigation UI feature.

## Business-Level Success Metrics

- **One body, four orders, zero duplication** (observable, first-class signal): every course has
  exactly one canonical, path-neutral body; all four path manifests reference courses **by ID**; a
  grep confirms no course body is duplicated per path. The
  [`syllabus/courses/` catalog](./syllabus/courses/README.md) enumerates each of the 127 courses once
  (121 software-engineer-role courses + 6 net-new AI-specific courses, DD-28).
- **Prerequisite DAG is consistent** (observable): every course declares `prerequisites`; the canonical
  course page surfaces them; a manifest-integrity check confirms every path's `courseOrder` is a valid
  topological entry into the DAG (no course precedes a prerequisite). Verified by a phase gate + unit
  test.
- **Path-aware navigation works** (observable): from a path landing page, a reader walks the course
  order for that path; prev/next and breadcrumb follow the path manifest; a course deep-linked without
  `?path=` renders a coherent canonical view. Verified by unit + integration + e2e tests and the
  `specs/` Gherkin companion.
- **interview-ready MVP proves the architecture first** (observable, amended 2026-07-20): the
  interview-first path's landing page, re-homed topics 1–33, its manifest, and path-aware nav are
  complete and deployed to production as an **architecture smoke test** — before the AI path and the
  other two manifests are composed. The interview cluster's NEW courses are not part of this gate (see
  [tech-docs.md DD-27](./tech-docs.md#design-decisions)).
- **software-engineer-to-ai-engineer path is assumes-competence-first** (observable, added
  2026-07-20): its manifest **links** rather than includes SWE-fundamentals prerequisites; the six
  net-new AI-specific courses ship a learning + drilling track each; authoring this path has priority
  #1 over the immediately-effective and fundamentally-strong manifests.
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
- Teaching how to **drive** AI coding agents (added 2026-07-20) — that is `agentic-coding`'s existing,
  unrelated scope. The `software-engineer-to-ai-engineer` path teaches **building** AI systems only
  (see [tech-docs.md DD-21](./tech-docs.md#design-decisions)).

## Business Risks and Mitigations

| Risk                                                                                                                            | Mitigation                                                                                                                                                                                                                                                                        |
| ------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Re-homing the 33 shipped topics to `courses/` breaks live URLs.                                                                 | Every re-home lands with a redirect (`apps/ayokoding-www/src/redirects/`) from the legacy `fundamentally-strong/software-engineer/<slug>` URL to the new `/en/c/learn/courses/<course-id>` URL.                                                                                   |
| Native-authored topics 34–94 stall the whole plan.                                                                              | The FS-SE hard dependency is removed; an interview-ready MVP (architecture smoke test only, over the already-live re-homed 1–33) ships first; topics 34–94 backfill incrementally without blocking the MVP.                                                                       |
| A path manifest violates the prerequisite DAG (a course precedes its prerequisite).                                             | A manifest-integrity check verifies every `courseOrder` is a valid topological entry into the DAG; runs as a phase gate + unit test; course IDs are stable slugs, never renumbered.                                                                                               |
| Path context lost on share/deep-link degrades the reading experience.                                                           | Graceful canonical fallback is a first-class design requirement + a Gherkin scenario + an e2e test; a course page always names the paths that include it and surfaces its prerequisites.                                                                                          |
| Four manifests drift or reference a missing/renamed course ID.                                                                  | The manifest-integrity check (every `courseOrder` ID resolves to a library course) runs as a phase gate and a unit test.                                                                                                                                                          |
| Duplication creeps in (a path forks a body for its framing).                                                                    | Framing is limited to an optional intro/outro callout applied by the path layer; a distinct-pedagogy need is met by a separate course variant, never a body copy — enforced by review + a no-duplicate-body check.                                                                |
| AI-band courses duplicate the agent-loop/tools/MCP/memory/evals material.                                                       | The AI-band scope-guard contract: `agentic-ai` (57) is a survey that forward-links each primitive to its harness-cluster course and stops short of build-your-own depth.                                                                                                          |
| detection-engineering and defensive-security overlap.                                                                           | Explicit scope lines: `defensive-security` (re-labelled hands-on By-Example) keeps generalist Sigma/ELK breadth + IR + hardening; `detection-engineering-and-siem-operations` owns deep Wazuh decoder/rule/dashboard SIEM-ops and names `defensive-security` as its prerequisite. |
| Navigation UI regresses existing content nav (non-path readers).                                                                | The canonical (no-path) view is the existing behavior; the UI adds path-awareness without changing default nav — covered by retained navigation specs + tests.                                                                                                                    |
| Course surgery (added 2026-07-20) ripples unpredictably across the four shared manifests.                                       | Every surgery states its blast radius across all four manifests before it is applied, and every affected manifest is re-verified prerequisite-consistent afterward (DD-28) — no silent cross-path breakage.                                                                       |
| The per-role convergence amendment (added 2026-07-20) reads as a contradiction of the original "one converging endpoint" claim. | The amendment is documented explicitly, in one place (DD-22), and cross-referenced everywhere the original claim was made, rather than silently overwritten — a reader following any link lands on the current, accurate model.                                                   |
