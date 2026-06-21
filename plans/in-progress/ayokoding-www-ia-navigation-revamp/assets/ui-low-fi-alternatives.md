# Low-Fidelity UI Alternatives — IA & Navigation Revamp

This document holds the **diverge** stage of the UI-design-funnel: ≥3 genuinely different low-fi
ASCII wireframes per screen (landing homepage, `/c` browse index, header/footer nav), showing the
mobile↔desktop reflow. The **narrow → select → justify** stages and the hi-fi `.png`/`.svg`
finalists live in [`../prd.md`](../prd.md#ui-design-funnel) and the `assets/` `*.svg`/`*.png` files.

Grounding (R5): all layouts are built from `libs/web-ui` primitives (`Button`, `Breadcrumb`, card
patterns) + Tailwind tokens already used in `apps/ayokoding-www`. No net-new primitive is required;
section cards reuse the existing card/border/`bg-accent` token vocabulary. See
[swe-developing-frontend-ui skill]. Prior art (R7) consulted via `web-researcher`: developer-content
homepages (MDN, web.dev, Tailwind docs) for the hero + curated-cards + tools-teaser pattern.

---

## Screen 1 — Landing homepage `/[locale]`

### Option A — Hero + Section-Card Grid + Tools Teaser (Recommended)

Desktop (`lg` ≥ 1024 px):

```text
+----------------------------------------------------------------------+
| [AyoKoding]            Learn   Tools        [Search] [EN/ID] [theme]  |  <- header nav
+----------------------------------------------------------------------+
|                                                                      |
|   AyoKoding — learn software engineering in public                   |  <- hero H1
|   Practical, battle-tested notes from building a real platform.      |  <- hero intro
|   [ Browse Learn ]   [ Open Tools ]                                  |  <- hero CTAs
|                                                                      |
+----------------------------------------------------------------------+
|  Explore                                                             |
|  +----------------+ +----------------+ +----------------+            |
|  | [icon] Software | | [icon] Security| | [icon] AI      |           |  <- auto section cards
|  | Engineering     | | ...            | | ...            |           |
|  | blurb...        | | blurb...       | | blurb...       |           |
|  +----------------+ +----------------+ +----------------+            |
|  +----------------+ +----------------+ +----------------+            |
|  | Business        | | Personal Dev   | | Rants          |           |  <- rants = first-class card
|  +----------------+ +----------------+ +----------------+            |
+----------------------------------------------------------------------+
|  +--------------------------------------------------------------+    |
|  | TOOLS  ·  Cost of Living Calculator                          |    |  <- Tools teaser card
|  | Compare living costs across cities.   [ Open calculator -> ] |    |
|  +--------------------------------------------------------------+    |
+----------------------------------------------------------------------+
| [footer: Learn | Tools | About/Terms columns]                        |
+----------------------------------------------------------------------+
```

Mobile (`< sm`, 320–375 px) — cards stack to a single column, hero CTAs stack, header nav collapses
into the hamburger `MobileNav`:

```text
+--------------------------+
| [=] AyoKoding [search][T]|  <- hamburger + logo + search + theme
+--------------------------+
| AyoKoding — learn        |
| software engineering     |
| in public                |
| [ Browse Learn ]         |  <- CTAs stacked full-width
| [ Open Tools ]           |
+--------------------------+
| Explore                  |
| +----------------------+ |
| | [icon] Software Eng  | |  <- one card per row
| | blurb...             | |
| +----------------------+ |
| +----------------------+ |
| | [icon] Security      | |
| +----------------------+ |
|   ... (rants card too)   |
+--------------------------+
| +----------------------+ |
| | TOOLS                | |  <- Tools teaser full-width
| | Cost of Living Calc  | |
| | [ Open calculator -> ]| |
| +----------------------+ |
+--------------------------+
| [footer nav stacked]     |
+--------------------------+
```

### Option B — Hero + Two-Column Split (Learn list rail + Tools rail)

Desktop: hero on top; below, a two-pane split — left pane a vertical Learn section list, right pane
a Tools panel. Reflows to single column on mobile (Learn list first, Tools below).

```text
+----------------------------------------------------------------------+
| hero (as A)                                                          |
+----------------------------------------------------------------------+
| Learn                              | Tools                           |
| - Software Engineering             | +-----------------------------+ |
| - Information Security             | | Cost of Living Calculator   | |
| - Artificial Intelligence         | | [ Open -> ]                 | |
| - Business                        | +-----------------------------+ |
| - Personal Development            | (room for future tools)         |
| - Rants                           |                                 |
+----------------------------------------------------------------------+
```

Mobile: the two panes stack (Learn list, then Tools).

### Option C — Hero + Featured-Latest Feed + compact nav cards

Desktop: hero; then a "Latest / Featured" content feed (most-recent pages) as the primary body, with
small Learn/Tools nav chips above it. Section cards demoted to chips; Tools is a chip, not a teaser
card.

```text
+----------------------------------------------------------------------+
| hero                                                                 |
+----------------------------------------------------------------------+
| [Software Eng] [Security] [AI] [Business] [Rants] [Tools]            |  <- compact chips
+----------------------------------------------------------------------+
| Latest                                                               |
| - <page title>            <date>                                     |
| - <page title>            <date>                                     |
| - <page title>            <date>                                     |
+----------------------------------------------------------------------+
```

Mobile: chips wrap; latest feed is a single-column list.

**Drop reasons** — Option B: the Tools rail competes with the Learn rail for primary attention and
under-sells the curated content story; weaker on mobile where the split just stacks. Option C: a
latest-feed buries the Tools teaser into a chip (re-creating the discoverability problem this plan
exists to fix) and depends on reliable per-page dates that the `id` locale largely lacks.

---

## Screen 2 — `/c` content browse index `/[locale]/c`

### Option A — Restyled Section-Card Grid (Recommended)

Desktop: the same section-card grid as the landing's Explore block, but exhaustive (every top-level
section) and titled "Browse" with a breadcrumb `Home > Browse`.

```text
+----------------------------------------------------------------------+
| Home > Browse                                                        |  <- Breadcrumb primitive
| Browse                                                              |
| +----------------+ +----------------+ +----------------+            |
| | Software Eng   | | Security       | | AI             |            |
| +----------------+ +----------------+ +----------------+            |
| | Business       | | Personal Dev   | | Rants          |            |
| +----------------+ +----------------+ +----------------+            |
+----------------------------------------------------------------------+
```

Mobile: cards stack single-column.

### Option B — Collapsible Tree (today's `SidebarTree`, full-width)

Reuse the existing `SidebarTree` component full-width as the browse index — expandable nested tree.

```text
+--------------------------+
| Home > Browse            |
| v Software Engineering   |
|   - Programming Languages|
|   - Data                 |
| > Information Security   |
| > Artificial Intelligence|
| > Rants                  |
+--------------------------+
```

### Option C — Two-Pane Explorer (section rail + child list)

Desktop: left rail of sections, right pane lists the selected section's children. Reflows to a
single accordion on mobile.

**Drop reasons** — Option B reproduces the bare-tree feel this revamp is replacing (it is literally
today's sidebar). Option C two-pane explorer is heavier than the browse index needs and duplicates
the in-content sidebar that already exists on content pages.

---

## Screen 3 — Header + Footer navigation

### Option A — Inline header links + multi-column footer (Recommended)

Header desktop: logo, then inline `Learn  Tools` links, spacer, search, language, theme. Mobile:
hamburger opens `MobileNav` with the same `Learn / Tools` links plus language/theme.

```text
Header (desktop):
[AyoKoding]  Learn  Tools            [Search...]  [EN/ID]  [theme]

Header (mobile):
[=]  [AyoKoding]                     [search] [theme]
  (hamburger -> MobileNav: Learn, Tools, language, theme)

Footer (desktop) — multi-column:
+-----------------------------------------------------------+
| Learn            Tools            About                    |
| - Browse all     - Calculator     - About AyoKoding        |
| - Software Eng                    - Terms & Conditions     |
| - Rants                                                    |
+-----------------------------------------------------------+
| (c) AyoKoding · FSL-1.1-MIT · Source-Available Project     |
+-----------------------------------------------------------+
```

### Option B — Centered nav bar + simple footer row

Header: centered `Learn | Tools` nav, logo left, controls right. Footer: single row of links (no
columns).

### Option C — Dropdown "mega" header nav + multi-column footer

Header: `Learn ▾` opens a dropdown of section links; `Tools ▾` opens a tool list. Footer multi-column
as A.

**Drop reasons** — Option B's single-row footer cannot group Learn/Tools/About legibly and the
centered header competes with the logo for the left edge. Option C's mega-dropdown is over-built for
two nav items and adds keyboard-focus-trap a11y burden with little payoff at this catalog size.

---

## Responsive strategy (applies to the selected Option A across all three screens)

- **Mobile (`< sm`, 320/375 px)**: section-card grid → single column; hero CTAs → stacked
  full-width; header inline nav → hamburger `MobileNav`; footer columns → stacked sections. No
  horizontal overflow at 320 px (regression target from the prerequisite plan's UWT-008).
- **Tablet (`md` ≥ 768 px)**: section cards in 2 columns; header nav inline; footer 2–3 columns.
- **Desktop (`lg` ≥ 1280 px)**: section cards in 3 columns; full inline header nav; footer 3
  columns; Tools teaser full-width band.
