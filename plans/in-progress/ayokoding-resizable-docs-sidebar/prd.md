# PRD: Resizable Docs Sidebar (ayokoding-www)

## Product Overview

The ayokoding-www docs surface gains a **resizable navigation rail**. On desktop/tablet (`≥ md`,
≥ 768 px) the reader drags a vertical handle on the sidebar's right edge — or focuses it and uses
arrow keys — to set the rail width within a relative band (15%–35% of the viewport). The width
persists across sessions. When a nav label or nested tree is wider than the current width, the rail
content scrolls horizontally rather than clipping or wrapping. Below `md` the sidebar remains a
`Sheet` overlay drawer (`mobile-nav.tsx`); that drawer gains a small set of fixed preset widths.

The resize mechanic lives in a new shared `libs/web-ui` primitive, `resizable-panel`, so other apps
can reuse it.

## Personas

Solo-maintainer repo — personas are the hats the maintainer wears plus consuming agents:

- **Reader (end user)** — reads AyoKoding docs; wants to tune sidebar width to content depth/screen.
- **Design-system owner** — maintains `libs/web-ui`; wants a minimal, tested, reusable primitive.
- **App owner (ayokoding-www)** — wires the primitive into the content layout and mobile drawer.
- **Consuming agents** — `swe-ui-maker`/`-checker`/`-fixer`, `swe-typescript-dev`, `specs-maker`.

## User Stories

- **US-1** — As a reader on a wide screen, I want to widen the docs sidebar so I can read long,
  nested navigation labels without opening each section.
- **US-2** — As a reader on a narrow `md`-range screen, I want to narrow the docs sidebar so I can
  reclaim reading width for the article.
- **US-3** — As a keyboard-only reader, I want to resize the sidebar with the keyboard so the control
  is operable without a pointer.
- **US-4** — As a returning reader, I want my chosen sidebar width to persist across page loads and
  sessions so I do not re-adjust it every visit.
- **US-5** — As a reader whose nav labels exceed the current width, I want the sidebar content to
  scroll horizontally so no label is clipped or awkwardly wrapped.
- **US-6** — As a reader on mobile, I want the nav drawer to offer a couple of preset widths so the
  overlay fits my screen and content.
- **US-7** — As the design-system owner, I want the resize mechanic as a reusable `libs/web-ui`
  primitive so future apps adopt it without re-implementing drag/keyboard/persistence.
- **US-8** — As the repo maintainer, I want the whole feature built with zero new external packages
  so the dependency surface (and its supply-chain/soak burden) does not grow for an ergonomics change.

## Acceptance Criteria (Gherkin)

Each scenario obeys the step-keyword cardinality HARD rule (one primary `Given`/`When`/`Then`;
extras chained with `And`/`But`).

### Core width model (pure)

```gherkin
Scenario: Clamp a requested width above the maximum
  Given a viewport width of 1000 pixels and a max of 35 percent
  When a requested sidebar width of 500 pixels is clamped
  Then the resolved width is 350 pixels
```

```gherkin
Scenario: Clamp a requested width below the minimum
  Given a viewport width of 1000 pixels and a min of 15 percent
  When a requested sidebar width of 80 pixels is clamped
  Then the resolved width is 150 pixels
```

```gherkin
Scenario: Keep a requested width already inside the band
  Given a viewport width of 1000 pixels with a 15 to 35 percent band
  When a requested sidebar width of 250 pixels is clamped
  Then the resolved width is 250 pixels
```

```gherkin
Scenario: Reject an unparseable persisted value
  Given a persisted sidebar-width string of "not-a-number"
  When the persisted value is parsed
  Then the parser returns no width
  And the caller falls back to the default width
```

### Primitive — drag resize

```gherkin
Scenario: Widen the panel by dragging the handle right
  Given a resizable panel rendered at 250 pixels with a 150 to 350 pixel band
  When the user drags the separator handle 60 pixels to the right
  Then the panel width becomes 310 pixels
```

```gherkin
Scenario: Dragging past the maximum stops at the maximum
  Given a resizable panel rendered at 340 pixels with a 150 to 350 pixel band
  When the user drags the separator handle 100 pixels to the right
  Then the panel width stops at 350 pixels
```

### Primitive — keyboard resize and a11y

```gherkin
Scenario: Widen the panel with the ArrowRight key
  Given the separator handle is focused on a panel at 250 pixels
  When the user presses ArrowRight
  Then the panel width increases by the keyboard step
  And the handle exposes the new width via aria-valuenow
```

```gherkin
Scenario: The handle exposes separator semantics
  Given a resizable panel is rendered
  When the accessibility tree is inspected
  Then the handle has role "separator"
  And the handle has aria-orientation "vertical"
```

### ayokoding-www consumption — persistence, scope, horizontal scroll

```gherkin
Scenario: Persist the chosen width across a reload
  Given the reader has resized the docs sidebar to 320 pixels on a desktop viewport
  When the reader reloads the page
  Then the docs sidebar renders at 320 pixels
```

```gherkin
Scenario: Hide the resizable rail below the md breakpoint
  Given the docs page is open at a 375 pixel viewport
  When the layout renders
  Then the resizable aside is not displayed
  And navigation is available through the mobile drawer
```

```gherkin
Scenario: Scroll the sidebar horizontally when a label overflows
  Given a docs sidebar narrowed to 150 pixels containing a nav label wider than 150 pixels
  When the reader views the sidebar
  Then the sidebar content area is horizontally scrollable
  And the label is not clipped or wrapped
```

### Mobile drawer preset widths

```gherkin
Scenario: Apply a preset width to the mobile nav drawer
  Given the mobile nav drawer is open at a 375 pixel viewport
  When the reader selects the wider preset
  Then the drawer renders at the wider preset width
```

### Zero new dependencies (US-8)

```gherkin
Scenario: Ship the feature without adding any external package
  Given the plan's changes are staged for the PR
  When package.json, libs/web-ui/package.json, apps/ayokoding-www/package.json, and package-lock.json are diffed against origin/main
  Then no dependency or devDependency key is added in any of the three package.json files
  And package-lock.json introduces no new external package
```

## UI-Design-Funnel

> This plan is **UI-bearing** (adds a `libs/web-ui` component and changes `apps/ayokoding-www`
> screens). Per the UI Mockups in Plan Docs convention, the funnel below documents diverge → narrow
> → select → justify. Low-fidelity ASCII wireframes are inline; the two high-fidelity
> `.excalidraw.png` finalists are produced during delivery (Phase 1) and saved under
> `./assets/`, then embedded here.

### R5 grounding note (survey existing UI before drafting)

Before drafting, survey and reuse:

- `libs/web-ui/src/primitives/` [Repo-grounded] — existing primitives (`separator`, `scroll-area`,
  `sheet`, `tabs`, …) follow a `radix-ui` + `cn` + CVA pattern with `data-slot` attributes
  (see `scroll-area.tsx`). The new `resizable-panel` MUST match this pattern and reuse tokens.
- `apps/ayokoding-www/src/app/[locale]/(content)/layout.tsx` [Repo-grounded] — the current fixed
  `<aside>` shell (sticky, `overflow-y-auto`, `border-r border-border`) is the layout to preserve.
- `apps/ayokoding-www/src/features/navigation/shell/sidebar-tree.tsx` [Repo-grounded] — the tree
  content whose container gains `overflow-x-auto`.
- `libs/web-ui/src/components/theme-toggle/theme-toggle.tsx` [Repo-grounded] — the raw `localStorage`
  persistence pattern to mirror for the width value.

**Net-new component**: `resizable-panel` (primitive). No existing primitive provides a draggable
separator; `separator` is decorative only. Reference the `swe-developing-frontend-ui` skill during
implementation.

### R7 prior-art citation

Delegate a `web-researcher` survey (Phase 1 delivery step) of how comparable docs sites solve a
resizable side rail (e.g. VS Code side bar, Docusaurus/Nextra sidebars, `react-resizable-panels`
handle semantics) to inform the divergent alternatives, returning `[Verified]`/`[Needs Verification]`
cited findings. _Prior-art specifics: [Unverified] until the Phase 1 research step runs._

### Diverge — low-fidelity alternatives (≥ 2 named, genuinely different)

**Option A — Edge drag handle (thin gutter on the rail's right border)** _(Selected)_

The handle is a thin vertical strip sitting on the existing `border-r`; hover shows a `col-resize`
cursor; focus shows a ring. Minimal chrome, closest to the current layout.

```text
Desktop / tablet (>= md)                          Mobile (< md)
+-------------------+---------------------------+   +---------------------------+
| SIDEBAR       |  ||  ARTICLE CONTENT          |   | [=]  AyoKoding      [theme]|
| (nav tree)    |  ||                           |   +---------------------------+
|  > Section A  |  ||   # Page title            |   |  (article content, full   |
|    > Item 1   |  || <- handle (grab/arrows)   |   |   width; no side rail)     |
|    > Item 2   |  ||                           |   |                           |
|  > Section B  |  ||   Body text ...           |   |  Drawer (Sheet) opens over |
| [<-- overflow |  ||                           |   |  content with preset width |
|  scrolls -->] |  ||                           |   |  chooser inside header.    |
+-------------------+---------------------------+   +---------------------------+
   ^ width = 15%..35% of viewport, drag/keys      ^ preset widths, not free drag
```

**Option B — Explicit rail footer control (drag handle + a small width control in the rail footer)**

Same edge handle, plus a footer row with "narrow / default / wide" buttons and a reset. More
discoverable for non-drag users, but adds persistent chrome to every docs page and duplicates the
keyboard affordance the handle already provides.

```text
+-------------------+---------------------------+
| SIDEBAR       |  ||  ARTICLE CONTENT          |
|  > Section A  |  ||                           |
|  > Section B  |  ||                           |
|               |  ||                           |
| [narrow][def] |  ||                           |
| [wide] [reset]|  ||                           |
+-------------------+---------------------------+
   ^ extra footer control row (more chrome)
```

**Option C — Floating collapse+resize rail (overlay handle with a collapse toggle)**

The rail can fully collapse to an icon strip and expand on hover, with the drag handle only visible
on hover. Powerful but a larger behavioral change (collapse state, hover-expand) beyond this plan's
"make width adjustable" scope; more surface to test and to get wrong on touch/hybrid devices.

```text
+--+----------------------------------+        +-------------------+-------------+
|▤ |  ARTICLE (rail collapsed)        |  <-->  | SIDEBAR (expanded)|  ARTICLE    |
|▤ |                                  |        |  > Section A   |  ||           |
|▤ |                                  |        |  > Section B   |  ||           |
+--+----------------------------------+        +-------------------+-------------+
   ^ collapsed icon strip / hover-expand + drag (bigger scope)
```

### Narrow — hi-fi finalists

The two strongest alternatives carried to high fidelity as `.excalidraw.png` (produced in Phase 1):

- **Finalist 1 — Option A (Edge drag handle)**: `![Hi-fi mockup of the edge drag-handle resizable docs sidebar on desktop and mobile](./assets/resizable-sidebar-option-a.excalidraw.png)`
- **Finalist 2 — Option B (Rail footer control)**: `![Hi-fi mockup of the resizable docs sidebar with an explicit footer width control](./assets/resizable-sidebar-option-b.excalidraw.png)`

Dropped: **Option C (Floating collapse+resize rail)** — collapse/hover-expand is a larger behavioral
change than "adjustable width" and risks touch/hybrid regressions; out of scope for this plan.

### Select

**Selected: Option A — Edge drag handle.** It delivers drag + keyboard resize with the least new
chrome, preserves the current layout (`border-r`, sticky, `overflow-y`), and maps cleanly onto a
minimal `resizable-panel` primitive. The keyboard-operable `role="separator"` handle satisfies the
non-drag/accessibility need that Option B's footer buttons were meant to cover, so Option B's extra
chrome is unnecessary.

### Justify — decision record

| Design                         | Outcome    | Why                                                                                                                |
| ------------------------------ | ---------- | ------------------------------------------------------------------------------------------------------------------ |
| Option A — Edge drag handle    | **Winner** | Minimal chrome; preserves current layout; keyboard `separator` handle covers the non-drag path; smallest primitive |
| Option B — Rail footer control | Runner-up  | Discoverable, but adds persistent chrome to every docs page and duplicates the handle's keyboard affordance        |
| Option C — Floating collapse   | Dropped    | Collapse/hover-expand exceeds "adjustable width" scope; touch/hybrid regression surface too large for this plan    |

### Responsive strategy (mobile-first, per breakpoint)

- **Mobile (`< md`, < 768 px)**: no resizable rail. Navigation is the existing `Sheet` overlay
  drawer (`mobile-nav.tsx`), which reflows from the side-rail column into a full-height left sheet.
  The drawer offers **fixed preset widths** (e.g. a default and a wider preset) — no free drag,
  since an overlay drawer does not compete with content width the way a persistent column does.
- **Tablet (`md`, ≥ 768 px)**: the resizable `<aside>` appears (`md:block`) and is fully
  drag + keyboard resizable within the 15%–35% band. This is the lower edge of the resizable range.
- **Desktop (`lg`, ≥ 1024 px)**: identical resizable behavior; the relative 15%–35% band means the
  usable pixel range scales up with the viewport, so ultra-wide screens get a proportionally wider
  allowable rail without a hard pixel cap.
- **Reflow summary**: side rail (≥ md) → overlay sheet with preset widths (< md). The article
  content column is `min-w-0 flex-1` so it always absorbs the remaining space; sidebar content uses
  `overflow-x-auto` at every breakpoint so narrow widths never clip labels.

## Product Scope

**In-scope features**: drag resize, keyboard resize, width persistence, relative min/max clamp,
horizontal scroll of nav content, mobile preset widths, reusable `resizable-panel` primitive with
story + tests + specs. **Hard constraint**: everything is built with ZERO new external packages
(runtime or dev) — React, the existing `libs/web-ui` primitives/tokens, the existing Radix/CVA
deps, and the existing test tooling only (see `tech-docs.md` DD-2).

**Out-of-scope features**: SSR/cookie width, multi-pane split group, sidebar visual redesign,
collapse-to-icon-strip behavior, wiring the primitive into any other app.

## Product Risks

- **Hydration flash** at the default width before `localStorage` applies — accepted; mitigated by
  effect-based read (see `tech-docs.md`).
- **Touch drag ergonomics** on hybrid `md` tablets — the handle must have an adequate hit area;
  covered by the Storybook a11y check and E2E.
- **Persisted width becomes invalid** if the viewport shrinks (e.g. window resize) — the clamp is
  re-applied against the current viewport so a stored 35%-of-wide value is re-clamped on a narrow
  screen.
