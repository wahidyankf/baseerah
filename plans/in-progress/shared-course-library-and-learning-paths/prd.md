# Product Requirements — Fundamentally Strong Shared Course Library, Four Paths

## Product Overview

The "Fundamentally Strong" curriculum becomes a **shared course library** (one canonical, path-neutral
body per course, keyed by a stable course ID) composed by **four learning paths**. The three
`software-engineer` paths converge on the same software-engineering deep mastery; the fourth path
converges on a distinct AI-engineering deep mastery — convergence is a **per-role** property, not a
library-wide axiom (see [tech-docs.md DD-22](./tech-docs.md#design-decisions)):

- **`interview-ready/software-engineer`** — the **interview/job-prep-first** arc for an experienced
  engineer re-entering the market: interview prep FIRST → production-effective → deeper.
- **`immediately-effective/software-engineer`** — the **immediately-effective** arc: editor/tooling →
  one language end-to-end → **build a real app first** → then deepen.
- **`fundamentally-strong/software-engineer`** — the **university-style, fundamentals-first** arc:
  CS foundations / theory first → deeper.
- **`immediately-effective/software-engineer-to-ai-engineer`** (added 2026-07-20) — the
  **immediately-effective** arc applied to a **role transition**: assumes an already-working software
  engineer; prerequisite courses are **linked, not included**; teaches **building** AI systems (models,
  agents, evals, inference serving), not driving them (`agentic-coding` stays a separate, unrelated
  axis).

A **path is an ordered manifest** composing a **curated subset** of course IDs — not every course is in
every path, and each manifest must be a valid topological entry into the library's **prerequisite
DAG**. Courses are shared with **omit-or-create** semantics, and — as of 2026-07-20 — **course surgery**
(update/merge/split/create) is also permitted, subject to a four-path blast-radius statement per
surgery (see [tech-docs.md DD-28](./tech-docs.md#design-decisions)); a genuinely different teaching
approach is still met by a distinct **course variant**, not a body fork. This plan also delivers the
**ayokoding-www path-aware navigation UI** that makes one canonical course URL behave differently under
each path's context, under the `/en/c/learn` URL model. The library body is largely content (exempt
from `specs:coverage`); the **navigation feature is app code** and carries a `specs/` Gherkin companion
and three-level tests.

The topic content of the existing courses is unchanged — the 33 shipped topics (1–33) are **re-homed**
(moved to `courses/<course-id>/` with redirects) and the 61 transferred topics (34–94) are authored
**native** into `courses/`; all are **re-framed** (referenced by four manifests), not rewritten. This
plan additionally **authors fourteen NEW courses + nine NEW capstones** (three original plus six
DD-20 inter-topic capstones) the interview and productivity/harness/security clusters need, plus **six
further NEW AI-specific courses** (2026-07-20) the fourth path needs, for a **127-course** catalog
(121 software-engineer-role baseline + 6 AI-specific; course surgery permitted per DD-28).

## Personas (one per path)

- **Experienced engineer re-entering the job market (north-star for the
  `interview-ready/software-engineer` path)** — recently laid off, returning from a gap/sabbatical, or
  an employed senior wanting to switch. Already owns the editor workflow and deep fundamentals; needs
  to **refresh breadth fast, relearn interview technique** at mid/senior/staff level, and handle a
  **layoff / employment-gap narrative** — without walking a from-scratch curriculum. Interview/job prep
  FIRST.
- **A builder who wants to be effective fast (north-star for the
  `immediately-effective/software-engineer` path)** — wants "immediately effective" SWE: set up the
  editor, learn one language end-to-end, **ship a real app early**, then deepen into CS fundamentals,
  DS&A, algorithms, and systems. Serves both a from-scratch learner and a mid-career switcher.
- **A university-style, fundamentals-first learner (north-star for the
  `fundamentally-strong/software-engineer` path)** — wants the rigorous bottom-up route: CS
  foundations, computer architecture, paradigms, and data structures & algorithms **before** building
  apps at scale. Prefers to understand the machine and the theory first, then apply it.
- **An already-working software engineer transitioning to AI engineering (north-star for the
  `immediately-effective/software-engineer-to-ai-engineer` path, added 2026-07-20)** — already owns the
  SWE fundamentals the other three paths teach; wants to become immediately effective at **building**
  AI systems (models, agents, evals, inference serving), not at driving coding agents. Prerequisite
  courses are **linked, not included** in this path's manifest. Converges on a distinct AI-engineering
  endpoint, not the other three paths' shared software-engineering endpoint.
- **A reader who lands on a shared course by deep-link / share** — arrives at a course URL without a
  path context and must get a coherent standalone view (with its prerequisites surfaced) plus an
  obvious way to enter a path.
- **Maintainer (content strategist / frontend engineer / content author / reviewer)** — owns the
  four-path architecture, builds the navigation feature, and authors the NEW courses via the ayokoding
  maker agents.

## User Stories

- As a **builder new to software engineering**, I want an immediately-effective path that gets me
  productive and shipping a real app fast before deep theory, so that I stay motivated and learn depth
  once I feel the payoff.
- As an **experienced engineer re-entering the market**, I want an interview-ready path with real
  technique modules and a layoff/gap-narrative section, so that I get interview-ready fast at my level.
- As a **university-style learner**, I want a fundamentally-strong path that teaches CS foundations,
  architecture, paradigms, and DS&A before app-building, so that I understand the theory before I apply
  it — the same software-engineering endpoint the other two SWE paths reach, reached bottom-up.
- As an **already-working software engineer**, I want a path into AI engineering that **links** rather
  than re-teaches the SWE fundamentals I already have, so that I get straight to **building** AI
  systems without walking material I've already mastered.
- As a **reader on any path**, I want prev/next and the breadcrumb to follow **my path's order**, so
  that "next" always means the next course in the arc I chose.
- As a **reader on any course page**, I want to see the course's **prerequisites**, so that I know what
  to complete first regardless of which path (or no path) I entered from.
- As a **reader who shares or deep-links a course**, I want the course to render coherently with no
  path context, so that a shared link never breaks — and to see which paths include this course.
- As the **maintainer**, I want each course authored **once**, path-neutral, and referenced by every
  path that needs it, so that a fix or update benefits every referencing path with zero duplication.
- As the **maintainer**, I want a path to **omit** a course that does not fit and **create** a new
  course (or a distinct **variant** when a path needs a different teaching approach) only for a real
  gap, so that each path stays coherent without forking bodies.
- As the **maintainer**, I want to perform **course surgery** (update/merge/split/create) on a shared
  course when needed, stating its blast radius across all four manifests up front, so that the library
  stays coherent as it grows without silently breaking another path.
- As a **reader targeting an AI-agent-infra or security codebase**, I want the async-Python/FastAPI,
  CDP, MCP/harness, C++, and detection-engineering courses available in the library, so that any path
  can lead me to the stack skills those codebases need.
- As a **screen-reader / keyboard user**, I want the path banner, breadcrumb, prerequisite list, and
  prev/next to be fully accessible, so that path-aware navigation works without a mouse.

## UI-Design-Funnel (Path-Aware Navigation Screens)

The path-aware navigation adds/changes **three user-facing screens** in `ayokoding-www` (a Next.js
app), all under the `/en/c/learn` URL model. Each screen runs the diverge → narrow → select → justify
funnel. Low-fidelity wireframes are authored below; the two high-fidelity finalists per screen are
produced as `.excalidraw.png` assets under this plan's `assets/` during Group A (delivery steps emit
them) and embedded here. Repo-grounded **textual** hi-fi specifications for each chosen screen are
authored in [Hi-Fi Specifications](#hi-fi-specifications-textual-repo-grounded) below and are the
source of truth those PNGs render.

> **Pending assets note**: hi-fi assets are produced during execution — the six `![]()` hi-fi finalist image links
> below (two per screen, Screens 1-3) intentionally do not resolve yet. `delivery.md` Group A
> ("Produce hi-fi finalists") produces the `.excalidraw.png` files into `assets/` before the code work
> begins. A broken link here today is expected, not a mistake.

**R5 grounding note (all screens)** — before drafting, survey the existing UI to reuse rather than
reinvent: `libs/web-ui` component inventory + tokens + Storybook; the ayokoding app-shell
(`apps/ayokoding-www/src/features/app-shell/`); the existing `sidebar-tree`, `breadcrumb`, `prev-next`,
and `section-card` components [Repo-grounded — `apps/ayokoding-www/src/features/navigation/shell/` and
`.../content/shell/section-card.tsx`]. Reference the `swe-developing-frontend-ui` skill. **Net-new
components**: `PathCard`, `PathLanding`, `PathBanner`, `PathCourseLinks`, `PrerequisiteList` — all
composed from existing `libs/web-ui` primitives; named in
[tech-docs §New feature: `course-paths`](./tech-docs.md#new-feature-course-paths-functional-core--imperative-shell).

**R7 prior-art citation (all screens)** — consult, via `web-researcher` at Group-A authoring time, how
comparable learning platforms present a "path/track over shared lessons" and a "prerequisite graph"
(e.g. roadmap.sh track pages, Exercism tracks, freeCodeCamp curriculum, Coursera specialization/path
pages) so the alternatives are informed rather than invented. [Needs Verification — delegate before
authoring.]

> **Provisional-diverge note**: the R7 prior-art survey has **not** run yet — it is scheduled as a
> `delivery.md` Group A step (`web-researcher` delegation). The Screens 1-3 low-fi alternatives,
> selections, and rationales below were therefore drafted **without** prior-art input and are
> **provisional**: Group A re-runs the diverge/select stages against the R7 findings before the hi-fi
> finalists are produced, and may replace an alternative, change the selection, or add a new option if
> the survey surfaces a materially better pattern. Do not treat the "Selected:" lines below as
> prior-art-informed until Group A's R7 sweep lands (with inline excerpt + URL + access date per the
> Anti-Hallucination convention).

### Screen 1 · Paths hub ("choose your path")

Entry screen at `/en/c/learn/paths` (the paths hub) offering the four paths. The fourth path converges
on a different endpoint than the other three (per-role convergence, DD-22), so the hub's copy states
"converging within your role" rather than the earlier single-endpoint framing.

**Low-fi Option A — Path cards, 2×2 grid (Recommended)**

```text
┌────────────────────────── Fundamentally Strong · Learn ──────────────────────────┐
│  Choose your path. One library, converging within your role.                      │
│                                                                                    │
│  ┌────────────────────┐  ┌────────────────────┐                                   │
│  │ Interview-Ready SWE │  │ Immediately-Effect. │                                  │
│  │ Interview-first     │  │ Build-app-first     │                                  │
│  │ Get interview-ready │  │ Ship a real app     │                                  │
│  │ fast (re-entrant).  │  │ fast, then deepen.  │                                  │
│  │ ~N courses          │  │ ~N courses          │                                  │
│  │ [ Start → ]         │  │ [ Start → ]         │                                  │
│  └────────────────────┘  └────────────────────┘                                   │
│  ┌────────────────────┐  ┌────────────────────┐                                   │
│  │ Fundamentally Strong│  │ SWE → AI Engineer   │                                  │
│  │ Fundamentals-first  │  │ AI-transition-first │                                  │
│  │ CS theory first,    │  │ Already a SWE? Build│                                  │
│  │ then deepen.        │  │ AI systems, fast.    │                                 │
│  │ ~N courses          │  │ ~N courses           │                                 │
│  │ [ Start → ]         │  │ [ Start → ]          │                                 │
│  └────────────────────┘  └────────────────────┘                                   │
│                                                                                    │
│  Or browse the full course library →                                               │
└────────────────────────────────────────────────────────────────────────────────────┘
```

**Low-fi Option B — Stacked comparison rows**

```text
┌───────────────── Fundamentally Strong · Four Paths ──────────────────┐
│ Interview-Ready SWE (interview-first) [ Start → ]  ~N courses         │
│   interview prep → production-effective → deeper                      │
│ ────────────────────────────────────────────────────────────────────│
│ Immediately-Effective (build-app-first) [ Start → ]  ~N courses      │
│   editor → one language → BUILD APP → deepen                         │
│ ────────────────────────────────────────────────────────────────────│
│ Fundamentally Strong (fundamentals-first) [ Start → ]  ~N courses    │
│   CS foundations → architecture → paradigms → DS&A → build           │
│ ────────────────────────────────────────────────────────────────────│
│ SWE → AI Engineer (AI-transition-first) [ Start → ]  ~N courses      │
│   already a SWE → build AI systems (models, agents, evals) fast      │
└───────────────────────────────────────────────────────────────────────┘
```

**Responsive (mobile ↔ desktop)** — Option A shows a **2×2 grid** of four cards at `lg` (≥1024px),
two-up at `md` (≥768px), and **stacks to one column** below `sm`. The "Start" CTA is a full-width tap
target on mobile.

**Hi-fi finalists**: `![Paths hub — 2×2 card grid](./assets/paths-hub-option-a.excalidraw.png)`
and `![Paths hub — stacked comparison](./assets/paths-hub-option-b.excalidraw.png)`.

**Selected: Option A — Path cards, 2×2 grid.**

| Design                 | Why it won / lost                                                                          |
| ---------------------- | ------------------------------------------------------------------------------------------ |
| A — 2×2 card grid ✅   | Four equal, scannable choices; reuses `section-card`; reflows cleanly to stacked mobile    |
| B — stacked comparison | Denser, but buries the fourth path further below the fold on mobile and reads as a ranking |

### Screen 2 · Path landing page

At `/en/c/learn/paths/<path-id>` — the manifest rendered as an ordered, phase-grouped course list;
every course link carries `?path=<path-id>`. The ordering is a valid topological entry into the
prerequisite DAG.

**Low-fi Option A — Phase-grouped numbered syllabus (Recommended)**

```text
┌──────────── Interview-Ready Software Engineer · interview-first ─────────┐
│ Experienced & job-hunting? Skip the prologue → jump to Phase 1.          │
│                                                                          │
│ Prologue · Editor Foundations (skippable)                               │
│   1. Just Enough Nvim        2. Just Enough Lua     3. Extending Neovim  │
│   ▸ Capstone · Forge-Ready                                               │
│ Phase 1 · Interview Preparation                                         │
│   4. Just Enough Python …  9. Coding Interview  … 16. Behavioral        │
│   ▸ Capstone · Interview Loop                                           │
│ Phase 2 · Production-Effective …                                        │
│ Phase 3 · Deepening …                                                   │
└──────────────────────────────────────────────────────────────────────────┘
```

**Low-fi Option B — Collapsible phase accordion**

```text
┌──────────── Fundamentally Strong SWE · fundamentals-first ──────────────┐
│ ▼ Stage 1 · CS foundations & architecture       (N courses)             │
│ ▼ Stage 2 · Paradigms, DS&A, algorithms          (N courses)            │
│ ▶ Stage 3 · Build real software (collapsed)                             │
│ ▶ Stage 4 · Systems, data, security, ops (collapsed)                    │
└──────────────────────────────────────────────────────────────────────────┘
```

**Responsive (mobile ↔ desktop)** — Option A renders the numbered list full-width single-column on
mobile (each course a full-width row) and a comfortable reading column on desktop; the fast-path
callout stays pinned at the top. Phase headings are sticky sub-headers on desktop, inline on mobile.
Option B's accordion collapses all but the first stage on mobile to keep the list short.

**Hi-fi finalists**: `![Path landing — numbered syllabus](./assets/path-landing-option-a.excalidraw.png)`
and `![Path landing — phase accordion](./assets/path-landing-option-b.excalidraw.png)`.

**Selected: Option A — Phase-grouped numbered syllabus.**

| Design                   | Why it won / lost                                                                   |
| ------------------------ | ----------------------------------------------------------------------------------- |
| A — numbered syllabus ✅ | Shows the whole ordered arc at a glance; the number IS the path order; SEO-friendly |
| B — phase accordion      | Compact, but hides the arc behind collapsed sections and adds interaction cost      |

### Screen 3 · Course page in path context

A shared course body rendered with the active path's affordances: a top **path banner** (path name +
position), a path breadcrumb, a **prerequisite list**, and manifest-driven prev/next. Without `?path=`
→ canonical view (which still surfaces prerequisites).

**Low-fi Option A — Top path banner + path breadcrumb + prerequisites + bottom prev/next (Recommended)**

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ ▸ On path: Interview-Ready SWE · course 9 of N       [ view full path ]   │
│ Home / Learn / Interview-Ready SWE / Coding Interview                     │
│ Prerequisites: Data Structures & Algorithms · Advanced Algorithms         │
│                                                                          │
│ # Coding Interview                                                        │
│ …course body (unchanged, canonical, path-neutral)…                        │
│                                                                          │
│ ← Prev: Advanced Algorithms        Next: Take-Home & Live Coding →        │
│   (both links keep ?path=interview-ready/software-engineer)               │
└──────────────────────────────────────────────────────────────────────────┘
```

Canonical fallback (no `?path=`):

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Home / Learn / Courses / Coding Interview                                 │
│ Prerequisites: Data Structures & Algorithms · Advanced Algorithms         │
│ # Coding Interview … body …                                              │
│ This course is part of: [ Interview-Ready ] · [ Immediately-Effective ]   │
│                          · [ Fundamentally Strong ]                       │
└──────────────────────────────────────────────────────────────────────────┘
```

`coding-interview` shows only three badges because the `software-engineer-to-ai-engineer` path
**links** rather than includes SWE-fundamentals courses in its manifest (DD-24); the affordance
generically renders **one badge per path whose `courseOrder` actually lists the course**, so an
AI-specific course would instead show a single `[ SWE → AI Engineer ]` badge.

**Low-fi Option B — Left path rail replacing the sidebar**

```text
┌── Path rail ──┬────────────────────────────────────────────────┐
│ Interview-Rdy │ Home / … / Coding Interview                     │
│ ▸ 9 Coding ●  │ Prereqs: DS&A · Advanced Algorithms             │
│   10 Take-home│ # Coding Interview … body …                     │
│               │ ← Prev … Next → (?path kept)                    │
└───────────────┴──────────────────────────────────────────────────┘
```

**Responsive (mobile ↔ desktop)** — Option A's path banner is a full-width strip on all breakpoints;
prev/next stack vertically below `sm` and sit left/right at `sm+` (mirrors the existing `PrevNext`
component [Repo-grounded]). Option B's left rail is desktop-only and would need to collapse into a top
sheet on mobile — extra complexity, so Option A wins on mobile-first grounds.

**Hi-fi finalists**: `![Course in path — top banner](./assets/course-path-option-a.excalidraw.png)`
and `![Course in path — left rail](./assets/course-path-option-b.excalidraw.png)`.

**Selected: Option A — Top path banner + path breadcrumb + prerequisites + bottom prev/next.**

| Design             | Why it won / lost                                                                              |
| ------------------ | ---------------------------------------------------------------------------------------------- |
| A — top banner ✅  | Minimal change to the existing content layout; reuses `breadcrumb` + `prev-next`; mobile-first |
| B — left path rail | Rich, but a desktop-only pattern that fights the existing sidebar and needs a mobile sheet     |

### Hi-Fi Specifications (Textual, Repo-Grounded)

These **textual hi-fi specifications** complement the deferred `.excalidraw.png` finalists (produced
in Group A) — they pin the chosen **Option A** of each screen to concrete, existing design-system
facts so both the hi-fi PNGs and the Group-A/B build have an unambiguous target. Every primitive,
token, and class named below is **repo-grounded** in `@open-sharia-enterprise/web-ui` (barrel) /
`@open-sharia-enterprise/web-ui/primitives` and the AyoKoding token layer (`libs/web-ui-token`,
`apps/ayokoding-www/src/app/globals.css`), verified against the existing `prev-next`, `breadcrumb`,
and `section-card` components — nothing here invents a primitive or token. The provisional-diverge and
R7 caveats above still apply: if the R7 sweep changes a selection, the matching spec below is re-pinned
before its PNG is drawn.

#### Shared design legend (all three screens)

- **Import surface**: `@open-sharia-enterprise/web-ui` (composite `Button`, `Badge`, `Card*`,
  `Alert*`) and `@open-sharia-enterprise/web-ui/primitives` where a primitive is required — **not**
  `ts-web-ui`.
- **Color tokens** (Tailwind classes): surfaces `bg-background` / `bg-card` / `bg-accent`; text
  `text-foreground` / `text-muted-foreground` / `text-card-foreground` / `text-primary`; borders
  `border-border`; focus `ring-ring`. AyoKoding brand primary is **honey/amber**
  (`--color-primary: var(--hue-honey)`).
- **Per-path accent hue** (the 6-hue system with `-wash` fill / `-ink` text variants): interview-ready
  → `honey`, immediately-effective → `teal`, fundamentally-strong → `sage`,
  swe→ai-engineer → `plum` — used as `bg-[var(--hue-<h>-wash)]` fills and `text-[var(--hue-<h>-ink)]`
  accents so the four paths are colour-coded consistently across hub, landing, and banner. Hue is
  **never the sole signal** (always paired with the path name/number/icon); the final hue↔path map is
  confirmed at draw time and must hold WCAG-AA for `-ink` text on `-wash`.
- **Radius / elevation**: cards `rounded-xl` (20px on the AyoKoding scale); insets `rounded-lg`;
  `shadow-sm` at rest → `shadow-md` on hover.
- **Breakpoints**: `sm` 640 / `md` 768 / `lg` 1024 / `xl` 1280 — the only prefixes this app uses. The
  content column stays fluid `flex-1 px-6 py-8 lg:px-8` inside the `max-w-screen-2xl` content shell;
  the right TOC rail (`w-[200px]`, `hidden xl:block`) and resizable sidebar (`hidden md:block`) are
  untouched.
- **A11y baseline** (mirrors existing components): each new navigation region is a
  `<nav aria-label="…">`; lists are semantic `<ol>`/`<ul>` (this app uses semantic lists, not
  `role="list"`); the canonical focus ring is `focus-visible:ring-2 focus-visible:ring-ring`; the
  current location uses `aria-current="page"`; the global skip-link → `#main-content` is unchanged.

#### Screen 1 hi-fi — Paths hub (`/en/c/learn/paths`), Option A (2×2 card grid)

- **Container**: content column; inner `<section className="mx-auto max-w-6xl px-6 py-8 lg:px-8">`.
  Header: `<h1 className="text-4xl font-extrabold tracking-tight">` "Choose your path" +
  `<p className="mt-2 text-muted-foreground">` "One library, converging within your role."
- **Grid**: `<ul className="mt-8 grid grid-cols-1 gap-6 md:grid-cols-2">` — one column `<md`, **2×2** at
  `md+`; four `<li>`.
- **`PathCard`** (net-new, composes the existing **`SectionCard` pattern** — the whole card is a single
  `<Link className="group block focus-visible:outline-none">`, so there is **no** nested button and no
  link-in-link trap): wraps `Card`
  (`h-full rounded-xl transition-colors hover:bg-accent hover:shadow-md group-focus-visible:ring-2 group-focus-visible:ring-ring`).
  Contents — a kind `Badge` (`variant="outline"` + `hue`), `CardTitle` (`text-lg font-semibold`) = path
  name, `CardDescription` (`text-sm text-muted-foreground`) = the one-line arc
  ("interview prep → production-effective → deeper"), a course-count `Badge`
  (`variant="secondary" size="sm"`) "~N courses", and the `meta` affordance "Start →"
  (`text-sm font-medium text-primary` + lucide `ArrowRight h-3.5 w-3.5`) exactly as `SectionCard`.
- **States**: default (`bg-card border-border shadow-sm`); hover (`bg-accent shadow-md`, arrow nudges
  `group-hover:translate-x-0.5`); focus-visible (`ring-2 ring-ring` on the card). The fourth card is
  never visually de-ranked — equal weight is why Option A beat B.
- **Below the grid**: a tertiary
  `<a className="mt-6 inline-flex text-sm text-muted-foreground hover:text-foreground">` "Browse the
  full course library →" → `/en/c/learn/courses`.
- **Responsive**: 2×2 `md+`; single column `<md` (full-width cards, comfortable tap height; the "Start"
  affordance lives inside the full-card tap target).
- **A11y**: `<ul>`/`<li>`; each card `<a aria-label="Start the {path} path — {N} courses">`; the hue is
  decorative (path name carries the meaning).

#### Screen 2 hi-fi — Path landing (`/en/c/learn/paths/<path-id>`), Option A (phase-grouped numbered syllabus)

- **Container**: content column `flex-1 px-6 py-8 lg:px-8`; inner reading column `max-w-3xl`. A
  path-aware `Breadcrumb` (`Home / Learn / <Path Title>`), `<h1 className="text-4xl font-extrabold tracking-tight">`
  = path title, `<p className="text-muted-foreground">` = arc summary, framed by a hue strip
  (`bg-[var(--hue-<h>-wash)]`) matching the path's hub card.
- **Fast-path callout** (interview-ready etc.): `Alert variant="info"` above the list — "Experienced &
  job-hunting? Skip the prologue → jump to Phase 1." with an in-page anchor.
- **Syllabus**: each phase is a `<section>` with heading
  `<h2 className="mt-8 text-sm font-semibold uppercase tracking-wide text-muted-foreground">`
  (`lg:sticky lg:top-16` on desktop, inline on mobile). Courses are a **semantic ordered list**
  `<ol className="mt-3 space-y-1">` where the visible number **is** the path order; each row is a
  `<Link>` (carrying `?path=<path-id>`) styled like the sidebar-tree link
  (`rounded-md px-2 py-1 text-sm hover:bg-accent hover:text-foreground`). Capstone rows carry a `▸`
  marker + a `Badge variant="outline" hue` "Capstone".
- **States**: rows are **stateless links** (no per-user progress store in scope); the skippable prologue
  phase is dimmed (`text-muted-foreground`) with a `Badge size="sm"` "skippable".
- **Responsive**: full-width single column `<lg`; `max-w-3xl` reading column `lg+`; phase headings
  sticky only where there is vertical room (`lg+`).
- **A11y**: `<nav aria-label="{Path} syllabus">` around the phases; each phase an `<ol>` so reading
  order and "course k of N" are programmatically derivable — numbers are list semantics, not decoration.

#### Screen 3 hi-fi — Course page in path context, Option A (banner + breadcrumb + prereqs + prev/next)

The **unchanged, path-neutral course body** renders as today (`article className="min-w-0 flex-1 px-6 py-8 lg:px-8"`,
`h1 text-4xl font-extrabold`, `MarkdownRenderer`) with four affordances layered around it.

- **`PathBanner`** (net-new, above the breadcrumb, only when `?path=` is present): full-width strip
  `<div className="mb-4 flex items-center justify-between rounded-lg bg-[var(--hue-<h>-wash)] px-4 py-2 text-sm">`
  — left `▸ On path: <Path> · course <k> of <N>` (`text-[var(--hue-<h>-ink)] font-medium`), right a
  "view full path" `<Link>` (`underline-offset-2 hover:underline`) → the path landing.
- **Path-aware `Breadcrumb`**: `Home / Learn / <Path Title> / <Course Title>` (the `<Path Title>` crumb
  links to the landing with `?path=`), via the extended component below.
- **`PrerequisiteList`** (net-new, shown in **both** path and canonical views):
  `<p className="text-sm text-muted-foreground"><span className="font-medium text-foreground">Prerequisites:</span> …</p>`
  where each prerequisite is a `<Link>` (carrying `?path=` in path context) separated by `·`. **Empty
  state**: the whole line is omitted when the course has no prerequisites (never an empty
  "Prerequisites:").
- **`PrevNext` (path-aware)**: existing component, markup unchanged; `prev`/`next` come from the
  **manifest** (not `weight`) and both hrefs keep `?path=<path-id>`; bottom of article as today
  (`mt-12 border-t pt-6`).
- **Canonical fallback (no `?path=`)**: no banner; breadcrumb `Home / Learn / Courses / <Course Title>`;
  `PrerequisiteList` still shows; and a **`PathCourseLinks`** (net-new) affordance renders below the
  body: `<div className="mt-8 text-sm"><span className="text-muted-foreground">This course is part of:</span> …</div>`
  with **one `Badge` link per path whose manifest `courseOrder` actually lists this course** (hue per
  path, `variant="outline"`, wrapped in a `<Link>` to that path's landing). A course a path only
  **links** (not includes) shows no badge for it (DD-24) — `coding-interview` shows three badges; an
  AI-specific course shows a single `SWE → AI Engineer` badge.
- **States**: with-path (banner + path breadcrumb + manifest prev/next); without-path (canonical
  breadcrumb + `PathCourseLinks` + canonical neighbours or omitted prev/next); no-prereq (list
  omitted); single-path course (one `PathCourseLinks` badge).
- **Responsive**: banner full-width at all breakpoints; `PrevNext` stacks `<sm`, left/right `sm+`
  (unchanged); `PathCourseLinks` badges wrap.
- **A11y**: banner is a `<nav aria-label="Path position">`; "course k of N" is real text; prerequisite
  and path-course affordances are semantic inline link lists; hue badges always carry the path name as
  text.

#### Extended existing components (additive props, no fork)

- **`PrevNext`** (`apps/ayokoding-www/src/features/navigation/shell/prev-next.tsx`): markup unchanged
  (`<nav aria-label="Page navigation">`, `ChevronLeft/Right`, eyebrow + title). Change is
  **data-source only** — `prev`/`next` resolve from the active path manifest and both `<Link>` hrefs
  append `?path=<path-id>`; with no path context they fall back to canonical neighbours (or render
  `null` when both are null, as today).
- **`Breadcrumb`** (`.../navigation/shell/breadcrumb.tsx`): reuse `segments` + `contentHrefs` as-is; add
  optional path context so a `<Path Title>` segment is injected (linking to the landing with `?path=`)
  and downstream `href`s carry `?path=`. `showCurrent` / `aria-current="page"` behaviour unchanged.
- **`contentUrl()`** (`.../content/core/content-url.ts`): add an optional `pathId` that appends
  `?path=<path-id>`, so breadcrumb, prev/next, and prerequisite link builders all produce
  path-preserving URLs from one place.

## Acceptance Criteria (Gherkin)

Navigation-feature scenarios are the source of the `specs/` Gherkin companion (app code). Content and
path-ordering scenarios document behavior. Each scenario uses exactly one primary Given/When/Then;
extras chain with And. The scenarios below cover the `course-paths` navigation feature; course-specific
acceptance scenarios appear further down, under
[NEW Course & Capstone Specifications](#new-course--capstone-specifications).

```gherkin
Scenario: A path landing page lists its courses in manifest order
  Given the interview-ready/software-engineer path manifest is published
  When a reader opens the path landing page at /en/c/learn/paths/interview-ready/software-engineer
  Then the courses appear in the manifest's courseOrder
  And every course link carries the path context query parameter
```

```gherkin
Scenario: A path manifest is a valid topological entry into the prerequisite DAG
  Given a path manifest lists a courseOrder of course IDs
  When the manifest-integrity check runs
  Then no course appears before any of its declared prerequisites that are also in the manifest
  And every listed course ID resolves to an existing course in the library
```

```gherkin
Scenario: Every manifest course reference resolves to a real course
  Given a path manifest lists a courseOrder of course IDs
  When the manifest-integrity check runs
  Then every listed course ID resolves to an existing course in the library
  And no course ID appears more than once in the manifest
```

```gherkin
Scenario: A course page surfaces its declared prerequisites
  Given a course declares prerequisites in its canonical metadata
  When a reader opens the course page with or without a path context
  Then the page lists each prerequisite course with a link to its canonical URL
  And the prerequisite list renders even in the canonical no-path view
```

```gherkin
Scenario: Prev and next follow the active path's order
  Given a reader is on a course with an active path context
  When the reader reads the prev/next navigation
  Then prev and next are the neighboring courses in that path's manifest
  And both links preserve the path context query parameter
```

```gherkin
Scenario: The breadcrumb reflects the active path
  Given a reader is on a course with an active path context
  When the breadcrumb renders
  Then it shows Home, Learn, the path title, and the course title
  And the path crumb links to the path landing page /en/c/learn/paths/<path-id> with the path context preserved
```

```gherkin
Scenario: A course deep-linked without path context renders the canonical view
  Given a reader opens a course URL /en/c/learn/courses/<course-id> with no path context query parameter
  When the course page renders
  Then the course body renders in full with the content-tree breadcrumb and its prerequisite list
  And a "this course is part of" affordance lists every path that includes the course
```

```gherkin
Scenario: An invalid path context falls back to the canonical view
  Given a reader opens a course URL with a path context that names no known path
  When the course page renders
  Then the course renders the canonical standalone view
  And no error is shown
```

```gherkin
Scenario: A course omitted from a path shows no path nav for that path
  Given a course is not listed in a given path's manifest
  When a reader opens that course with that path's context
  Then the course renders the canonical standalone view
  And the path banner is not shown for that path
```

```gherkin
Scenario: A legacy fundamentally-strong URL redirects to the canonical course URL
  Given a re-homed course previously lived under the legacy fundamentally-strong/software-engineer content path
  When a reader requests the legacy URL
  Then the app redirects to the course's canonical /en/c/learn/courses/<course-id> URL
  And the redirect preserves any path context query parameter
```

```gherkin
Scenario: The legacy section-index browse still resolves after re-homing
  Given the 33 shipped topics have been re-homed into the course library
  When a reader browses the legacy fundamentally-strong software-engineer section index the old way
  Then every section-index entry links to live content at its /en/c/learn/courses/<course-id> URL or via a redirect
  And no legacy section-index entry resolves to a drained or missing location
```

```gherkin
Scenario: Old-way and new-way navigation coexist
  Given a course now lives at its canonical /en/c/learn/courses/<course-id> URL
  When a reader reaches it via the legacy section-index browse
  And another reader reaches it via a /en/c/learn/paths/<path-id> path landing
  Then both navigations resolve to the same single canonical course body
```

```gherkin
Scenario: The three software-engineer paths reference a shared course with no body duplication
  Given a course appears in all three of the interview-ready, immediately-effective/software-engineer, and fundamentally-strong/software-engineer manifests
  When the course library is inspected
  Then exactly one canonical path-neutral body exists for that course
  And each manifest references the course by its stable course ID
```

```gherkin
Scenario: The interview-ready MVP proves the architecture before other path work begins
  Given the interview-ready/software-engineer MVP (an architecture smoke test over already-live topics 1-33) is delivered end-to-end
  When the software-engineer-to-ai-engineer path's authoring begins
  Then the interview-ready MVP's landing page, manifest, and path-aware nav are already live in production
  And the interview cluster's remaining NEW courses are not required for that MVP to be considered shipped
```

```gherkin
Scenario: The AI path is authored before the other two manifests are composed
  Given the interview-ready MVP has shipped
  When authoring effort is allocated across the remaining paths
  Then the software-engineer-to-ai-engineer path's six net-new courses and manifest are authored first
  And the immediately-effective/software-engineer and fundamentally-strong/software-engineer manifests are composed only afterward
```

```gherkin
Scenario: The immediately-effective path is build-app-first
  Given the immediately-effective/software-engineer path manifest is published
  When a reader walks the path
  Then editor/tooling, one language end-to-end, and building a real app precede the CS-fundamentals and DS&A courses
  And the reader ships a real deployed app before any pure-theory course
```

```gherkin
Scenario: The fundamentally-strong path is fundamentals-first
  Given the fundamentally-strong/software-engineer path manifest is published
  When a reader walks the path
  Then CS foundations, computer architecture, paradigms, and DS&A precede the build-real-software courses
  And the ordering is a valid topological entry into the prerequisite DAG
```

```gherkin
Scenario: The software-engineer-to-ai-engineer path links prerequisites instead of including them
  Given the immediately-effective/software-engineer-to-ai-engineer path manifest is published
  When a reader inspects its courseOrder
  Then no shared software-engineering-fundamentals course from the other three manifests is included in courseOrder
  And the path landing page links out to those prerequisite courses' canonical pages instead
```

```gherkin
Scenario: The behavioral course covers the layoff and employment-gap narrative
  Given the behavioral-and-leadership-interviews course is authored
  When an experienced re-entrant reads its learning track
  Then it explicitly covers framing an employment gap, a layoff, or a re-entry story
  And it treats senior/staff/EM leadership rounds as core material
```

```gherkin
Scenario: The navigation feature meets accessibility requirements
  Given a reader uses a keyboard and a screen reader on a course in path context
  When they navigate the path banner, breadcrumb, prerequisite list, and prev/next
  Then each is a labelled landmark reachable and operable by keyboard with visible focus
  And the document language attribute matches the active locale
```

```gherkin
Scenario: The app builds and validates green
  Given the navigation feature and the interview-ready path are complete
  When nx run ayokoding-www:build, the three test tiers, and the link/heading validators run
  Then the build and all tiers succeed
  And link, heading-hierarchy, and markdownlint validation report no errors
```

## NEW Course & Capstone Specifications

This plan authors **twenty NEW courses + nine NEW capstones** into the library — the original
fourteen (interview + productivity/harness/security clusters) plus **six further NEW AI-specific
courses** added 2026-07-20 for the `software-engineer-to-ai-engineer` path — plus nine capstones
(three original plus six DD-20 inter-topic capstones). Full specs for the three original capstones
follow later in this section; the six DD-20 inter-topic capstones are specified inline within their
host course files per `delivery.md` Phase 10, Band 8. Each course is a full page-bundle (learning
track + drilling track) matching the sibling plan's per-topic anatomy and inheriting its cross-cutting
authoring guarantees verbatim (accuracy-verified via `web-researcher` before authoring;
follow-along-complete; typed-Python where Python; colocated runnable `code/`; exhaustive
`co-NN`/`ex-NN` enumeration; `prerequisites` metadata + navigation). Every course declares its
`prerequisites` so it takes its place in the library's prerequisite DAG. Full per-course
concept/example/capstone detail lives in the
[`syllabus/courses/` catalog](./syllabus/courses/README.md) (one file per course ID); the specs below
fix each course's purpose, register, and acceptance shape.

**Register.** The four interview-technique courses use a **refresh register** (assume prior
professional experience; reload technique, do not teach from zero). The ten productivity/harness/
security courses and the six AI-specific courses (2026-07-20) use the normal **first-learn By-Example
register**; `just-enough-cpp` is primer scope. The AI-specific courses additionally use the
**links-not-included** entry model: they assume the reader already has the SWE fundamentals the other
three paths teach (DD-24) — the courses themselves teach AI material only, they do not re-teach the
linked prerequisites.

**Principle-first framing (HARD).** Every course teaches a durable **principle**; target codebases
(`remotebrowser`, `wazuh`, `vacti*`, the ose family) are **illustrative worked-examples**, never the
subject.

**Volume-target bands** (inherited from the sibling; floor not cap):

| Course shape                                  | Concept floor (`co-NN`) | Worked-example band (`ex-NN`)         |
| --------------------------------------------- | ----------------------- | ------------------------------------- |
| By Example                                    | ≥ 10                    | 75–85 code examples                   |
| Primer (_Just Enough X_)                      | ≥ 8                     | 75–85 code examples (By-Example pace) |
| Annotated-concept, code-bearing               | ≥ 10                    | 45–60 worked examples                 |
| Annotated-concept, no-code (refresh register) | ≥ 8                     | 30–60 worked scenarios                |

### Interview-technique courses (refresh register)

- **`coding-interview`** (By Example · Python, patterns language-agnostic) — reload LeetCode-style
  pattern recognition + time-boxed problem-solving; hosts the 2026 senior interview-loop-map.
- **`take-home-and-live-coding`** (By Example · Python) — time-boxed take-home + observed live/pair
  technique: scope, test, README hygiene, thinking aloud.
- **`system-design-interview`** (Annotated-concept · no code) — the senior/staff system-design
  interview rubric + whiteboard flow; forward-links the depth course `system-design`.
- **`behavioral-and-leadership-interviews`** (Annotated-concept · no code) — STAR + senior/staff/EM
  rounds AND framing an **employment-gap / layoff / re-entry** narrative.

```gherkin
Scenario: Interview courses are written in a refresh register
  Given the four new interview-technique courses are authored
  When an experienced engineer reads them
  Then each assumes prior professional experience and focuses on interview technique and breadth refresh
  And none teaches core concepts from zero
```

### Productivity & self-hosting courses (first-learn By-Example)

- **`async-python-and-fastapi-services`** (By Example · Python) — async Python, FastAPI/Uvicorn,
  Pydantic, `uv`/`ruff`/`pyright`/`pytest-asyncio` — the `remotebrowser` + FastAPI-backend stack.
  Scoped tightly to the concrete framework + toolchain: async _concepts_ stay in
  `concurrency-and-parallelism`, framework _internals_ in `build-your-own-web-framework` — cross-linked,
  not re-derived.
- **`self-hosting-essentials`** (By Example · ops/config) — **light** on-ramp: one box, containerize,
  reverse proxy + TLS, systemd/ports, env/secrets, backups, PaaS git-push. Strictly below
  `containers-and-orchestration` / `cloud-and-iac`; distinct from `bare-metal-virtualization`.
- **`browser-automation-with-cdp`** (By Example · Python) — Chrome DevTools Protocol browser
  automation (port 9222; nodriver/zendriver family) — the core `remotebrowser` skill. Distinct from
  `software-testing`'s Playwright E2E: raw CDP automation, not a test runner.

```gherkin
Scenario: The light self-hosting course stays below clusters and IaC
  Given the self-hosting-essentials course is authored
  When a reader compares it with containers-and-orchestration and cloud-and-iac
  Then it teaches running one box, containerizing a service, a reverse proxy, and PaaS git-push deploy
  And its overview explicitly excludes clusters, Terraform/Packer/Ansible IaC, and Proxmox
```

### Harness-engineering cluster (first-learn By-Example · Python)

The five build-your-own-agentic-coding-tool courses; the MCP built in `agent-tools-and-mcp` is the same
MCP `remotebrowser` exposes; all feed `capstone-build-your-own-coding-agent`. **AI-band scope-guard**:
these build the primitives at build-your-own depth; the survey course `agentic-ai` (57) previews and
**forward-links** each primitive here and does NOT re-teach at cluster depth, and
`creating-ai-powered-apps` (56) stays at the _use-an-LLM-in-an-app_ altitude.

- **`the-agent-loop`** — the LLM read-eval-act tool-use loop, streaming, stop conditions.
- **`agent-tools-and-mcp`** — tool/function schema design; an MCP server + client; resources/prompts.
- **`agent-context-and-memory`** (Annotated-concept) — context budgeting, compaction, retrieval,
  persistent memory.
- **`agent-permissions-and-sandboxing`** — approval models, sandboxed execution, guardrails,
  fail-closed defaults.
- **`agent-orchestration-subagents-and-observability`** (Annotated-concept) — subagents, background
  tasks, hooks/skills systems, a TUI, evals + tracing/telemetry.

```gherkin
Scenario: The harness cluster builds a working agent from runnable code
  Given the five harness-engineering courses are authored
  When a reader builds an agent from them
  Then the agent loop, tools/MCP, memory, permissions, and orchestration each ship runnable typed-Python examples
  And each course names remotebrowser's bundled MCP or CDP browser only as an illustrative pickup
```

```gherkin
Scenario: The agentic-ai survey forward-links each primitive without re-teaching it
  Given the agentic-ai survey course and the five harness-cluster courses are authored
  When a reader reads the agentic-ai survey
  Then it previews the agent loop, tools/MCP, memory/context, and evals and forward-links each to its cluster course
  And it does not re-teach any primitive at build-your-own depth
```

### AI-engineering specialization courses (`software-engineer-to-ai-engineer` path, added 2026-07-20)

Six NEW courses for the fourth path, teaching **building** AI systems (not driving coding agents —
`agentic-coding` stays a separate axis, DD-21). Each is split into a **stable spine** (durable
principles) and **dated accuracy-note sidebars** (volatile SDK/model/pricing/framework specifics),
matching the pattern the existing AI-band courses already use (DD-28). **These six courses' specs are
now settled** — full concept (`co-NN`), worked-example (`ex-NN`), prerequisite-chain, and capstone
specs exist at [`syllabus/courses/`](./syllabus/README.md) (one 295-425-line file per course); the
format/language/prerequisite summaries below are drawn from those settled files, not first-pass
guesses. Author each course body **from** its `syllabus/courses/<id>.md` spec (per DD-27's build
order, this is authoring priority #1 behind the interview-ready MVP).

- **Light eval gate** (`evaluating-ai-output-essentials` — Annotated-concept, Python) — a small, early
  course sitting right after the first working LLM call and before RAG/agents; answers "how will you
  know this works?" (DD-25).
- **Statistics for evals** (`statistics-for-evaluation` — Annotated-concept, code-bearing, Python) —
  scoped tightly to what evals demand (judge concordance, significance testing), not a general
  statistics survey; `analytics-and-experimentation` (classical product A/B testing) stays a scope
  mismatch and a candidate sibling/prerequisite rather than a merge target (DD-26). Declared a **hard
  prerequisite** of deep evals, so it is authored/placed before that course (see the manifest mirror at
  `syllabus/paths/manifest-immediately-effective-software-engineer-to-ai-engineer.md`).
- **Deep evals** (`evaluating-ai-systems-in-depth` — By Example, Python) — sits after agents; error
  analysis, task-specific criteria, LLM-as-judge with measured human agreement, CI gating, judge-scope
  reliability. Absorbs the three scattered evals treatments in `creating-ai-powered-apps`, `agentic-ai`,
  and `agent-orchestration-subagents-and-observability`, which are trimmed to forward-links rather than
  duplicating a fourth treatment (DD-25, DD-28).
- **Product patterns for probabilistic systems** (`product-patterns-for-probabilistic-systems` —
  Annotated-concept, no code) — product design patterns for systems whose outputs are probabilistic
  rather than deterministic; no course owns this today (DD-28).
- **Inference serving and model deployment** (`inference-serving-and-model-deployment` — By Example,
  Python) — vLLM/TGI, KV-cache, batching, GPU considerations; entirely absent from the library today
  (DD-28).
- **Fine-tuning and adaptation** (`fine-tuning-and-adaptation` — By Example, Python) —
  fine-tuning/LoRA/PEFT versus RAG as a foil; `fine-tun*` appears once library-wide today, as a RAG
  comparison point, never its own course (DD-28).

The scope boundary between the light eval gate and deep evals is stated explicitly in both courses'
overviews, in the style of the existing AI-band scope-guard (DD-10/DD-11), to avoid reproducing that
cluster's overlap problem.

```gherkin
Scenario: The light eval gate and deep evals course do not overlap
  Given the light-eval-gate course and the deep-evals course are authored
  When a reader compares their overviews
  Then each overview states an explicit scope boundary against the other
  And neither course re-teaches the material the other owns
```

```gherkin
Scenario: The statistics-for-evals course stays scoped to what evals demand
  Given the statistics-for-evals course is authored
  When a reader compares it with analytics-and-experimentation
  Then it covers judge concordance and significance testing for evals only
  And it does not re-teach general product A/B testing, which stays analytics-and-experimentation's scope
```

### Security & systems gap-closers

- **`just-enough-cpp`** (Primer · C++) — systems-language principle on-ramp (RAII, templates/generics,
  STL, smart pointers, manual memory); prereq `just-enough-c`; Wazuh's C++ core is one illustration.
- **`detection-engineering-and-siem-operations`** (By Example · XML/rules + config + Python) —
  decoders, correlation rules, log parsing/normalization, FP tuning, dashboards, alert triage; Wazuh
  XML is the worked example. Distinct from `defensive-security` (which is **hands-on By-Example**
  generalist blue-team breadth — Sigma/ELK + IR + hardening, **not** concept-level); prereq
  `defensive-security`.

```gherkin
Scenario: Hands-on detection engineering stays distinct from generalist defensive security
  Given the detection-engineering-and-siem-operations course is authored
  When a reader compares it with the hands-on defensive-security course
  Then it has the reader author working Wazuh decoders, correlation rules, and a dashboard with false-positive tuning
  And defensive-security keeps the generalist Sigma/ELK breadth, IR, and hardening as its distinct scope
```

### NEW capstones

Capstones follow the sibling's capstone-policy shape (goal/outcome, concepts-exercised checklist,
ordered step outline, testable acceptance criteria, done bar = runnable end-to-end + web-verified).

- **`capstone-interview-loop`** (Python + prose) — a full mock interview loop (coding + system-design +
  behavioral incl. gap narrative), each round self-scored against its module rubric.
- **`capstone-build-your-own-coding-agent`** (Python) — assemble the harness cluster into a working
  minimal coding-agent CLI; bonus path drives `remotebrowser` over MCP.
- **`capstone-build-your-own-pentest-engine`** (TypeScript default) — assemble swarm orchestration +
  MCP tool arsenal + CDP browser driving + security-tool-chaining + evidence pipeline + scope
  enforcement + deterministic-prober-vs-AI-verifier into a working engine; `vacti-pentest-engine` is
  the illustration.

```gherkin
Scenario: The coding-agent capstone assembles the harness cluster into a working CLI
  Given the harness cluster and the build-your-own-coding-agent capstone are authored
  When a reader completes the capstone
  Then they have a runnable coding-agent CLI built from the agent loop, tools/MCP, memory, permissions, and orchestration courses
  And a disallowed action fails closed while every run emits a trace
```

```gherkin
Scenario: The pentest-engine capstone assembles the convergence track into a scoped engine
  Given the harness cluster, the CDP course, the security suite, and detection-engineering are authored
  When a reader completes the build-your-own-pentest-engine capstone
  Then they have a runnable engine from swarm orchestration, MCP tooling, CDP browser driving, and security-tool-chaining
  And scope enforcement refuses an out-of-scope target while the capstone uses vacti-pentest-engine only as an illustration
```

## Product Scope

**In-scope features**:

- The `course-paths` ayokoding-www feature: path manifests, path-aware prev/next + breadcrumb,
  `?path=` context, prerequisite display, graceful fallback, path landing pages, a paths hub,
  redirects, accessibility — all under the `/en/c/learn` URL model.
- Re-homing the 33 shipped topics (1–33) into `courses/<course-id>/` with redirects; native-authoring
  the 61 transferred topics (34–94) into `courses/<course-id>/`.
- The four path manifests (`interview-ready/software-engineer` interview-first,
  `immediately-effective/software-engineer` build-app-first,
  `fundamentally-strong/software-engineer` fundamentals-first, and
  `immediately-effective/software-engineer-to-ai-engineer` AI-transition-first, added 2026-07-20) as
  ordered, prerequisite-consistent course-ID lists over the library. The first three converge on the
  same software-engineering endpoint; the fourth converges on a distinct AI-engineering endpoint
  (DD-22).
- Twenty NEW courses (the original fourteen plus six AI-specific, 2026-07-20) + nine NEW capstones
  (three original plus six DD-20 inter-topic capstones) authored into the library (learning + drilling
  each), for a **127-course catalog** (121 software-engineer-role baseline + 6 AI-specific).
- Course variants authored on demand only, where a path needs a genuinely different teaching approach;
  **course surgery** (update/merge/split/create, added 2026-07-20) permitted subject to a four-path
  blast-radius statement per surgery (DD-28).
- Three-level tests (unit/integration/e2e) + a `specs/` Gherkin companion for the nav feature.
- Per-path progression-smoothness audits.

**Out-of-scope features**:

- Rewriting any existing course's subject content.
- Indonesian mirror of the section content.
- Path progress persistence, accounts, or bookmarking.
- Interactive flashcards.
- Speculative enumeration of course variants (authored on demand only).
- Teaching how to **drive** AI coding agents — that stays `agentic-coding`'s existing, unrelated scope
  (DD-21).

## Product-Level Risks

- **Order/manifest drift**: a manifest references a missing/renamed course ID, or orders a course
  before its prerequisite → broken nav / invalid DAG entry. Mitigated by a manifest-integrity check
  (gate + unit test) that validates both ID resolution and topological consistency, plus stable
  course-ID slugs.
- **Deep-link fallback gap**: a course without path context renders poorly. Mitigated by a first-class
  canonical view (with prerequisites surfaced) + Gherkin scenario + e2e test.
- **URL breakage on re-home**: mitigated by a redirect per re-homed course + redirect specs.
- **Duplication creep**: a path forks a body for framing. Mitigated by callout-only framing, a distinct
  course variant for genuine pedagogy differences, and a no-forked-body check.
- **AI-band duplication creep**: `agentic-ai` and the harness cluster re-teach the same primitives.
  Mitigated by the AI-band scope-guard cross-reference contract.
- **NEW-course quality**: interview modules must meet ayokoding pace/accuracy bars. Mitigated by the
  maker → checker → facts-checker → link-checker pipeline per course.
- **Per-role convergence confusion** (added 2026-07-20): a reader or a future author assumes the
  fourth path converges with the other three, since the plan previously asserted one global endpoint.
  Mitigated by the explicit DD-22 amendment record, cross-referenced from every prose and diagram site
  that made the original single-endpoint claim.
- **Course-surgery blast radius** (added 2026-07-20): a surgery on a shared course (e.g. trimming
  `creating-ai-powered-apps`'s evals section to a forward-link) silently breaks another path's manifest
  or prerequisite chain. Mitigated by DD-28's binding rule: every surgery states its blast radius
  across all four manifests before it is applied, and every affected manifest is re-verified
  prerequisite-consistent afterward.
