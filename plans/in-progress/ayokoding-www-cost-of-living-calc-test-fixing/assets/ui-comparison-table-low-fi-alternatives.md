# Low-Fidelity Alternatives — Comparison Table (summary-column placement)

UI-design-funnel **diverge** stage for the primary changed screen: the Cost-of-Living Calculator
**Cost of living** tab comparison table. The defect is `UWT-004` — at 1280 px the table overflows
its ~1,120 px container (~1,564 px wide) and the right-edge **Total**, **Essentials**, **Relocation
(sunk)**, and **Liquidity reserve** columns are clipped with no scroll affordance, so the single most
important datum (Total) is off-screen and found only by accidental horizontal scroll.

All three options reuse the existing `libs/web-ui` table, badge, and tooltip primitives and the app's
existing dropdown/controls shell. The screen still answers _"how much do I need to live in each
hub?"_ and keeps every column — the alternatives differ only in **how the summary columns are kept
reachable**.

## Prior art (R7) and grounding (R5)

- **R7 prior art** — ranked cost-of-living comparison tables (Numbeo, Nomad List, Glassdoor
  cost-comparison) lead with the headline number (the index / total) on the left, next to the place
  name, then expose the breakdown to its right. The "answer first, breakdown after" idiom is the
  dominant pattern for scan-and-compare tables. This directly informs Option A.
- **R5 grounding** — the screen already composes `libs/web-ui` table/badge/tooltip primitives and the
  app's own filter/controls shell (`geo-filters.tsx`, `controls.tsx`). No net-new component is
  required for any option: Option A reorders existing columns; Option B needs a sticky-column CSS
  utility (no new component); Option C adds only a right-edge gradient/affordance element.

## Selection — Option A (Reorder summary-first, CHOSEN)

| Alternative                       | Outcome  | Reason                                                                                                      |
| --------------------------------- | -------- | ----------------------------------------------------------------------------------------------------------- |
| **A — Reorder summary-first**     | Selected | Puts Total + Essentials in the first viewport with zero new mechanics; matches the ranked-table prior art   |
| B — Sticky summary columns        | Dropped  | Sticky cells are fiddly across breakpoints and screen readers; horizontal-scroll still required to read mid |
| C — Affordance only (scroll hint) | Dropped  | Leaves the Total off-screen; only signposts the problem instead of solving it                               |

**Rationale**: Option A solves the actual harm (the answer is off-screen) by relocating the answer,
not by decorating the overflow. It requires no sticky-positioning CSS, no new component, and reflows
cleanly to the mobile stacked-card layout (where Total already leads each card). Options B and C are
the rejected alternatives captured below. A right-edge scroll affordance (from Option C) is **grafted
into Option A** as a secondary safety net for the remaining breakdown overflow.

## Option A — Reorder summary-first (SELECTED)

Column order becomes: **Country · City · Total · Essentials** (the summary), then the breakdown
**Housing · Food · Transport · Utilities · Healthcare (OOP) · Childcare · School · Lifestyle**, then
the one-time **Relocation (sunk) · Liquidity reserve**. The answer sits in the first viewport; the
breakdown trails to the right (and a right-edge fade hints there is more).

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│  💰  Cost of Living            [ Healthcare: tax-funded ]            Snapshot 2026-06        │
├──────────────────────────────────────────────────────────────────────────────────────────┤
│  ┏ Cost of living ┓ ( Savings · after expenses ) ( Minimum role · salary needed )          │
│  Region [ All ▼ ]  Country [ All ▼ ]  City [ All ▼ ]                                        │
│  ( )Single (•)Married  Pre-school[1] School-age[1]  (•)Rural                                │
├──────────────────────────────────────────────────────────────────────────────────────────┤
│  Country*  City*    TOTAL   Essen │ Hous Food Tran Util Hlth Care Schl Life │ Sunk  Reserve▸│
│  ───────   ──────   ─────   ───── │ ──── ──── ──── ──── ──── ──── ──── ──── │ ────  ─────── │
│  Indonesia Jakarta· $1,580  $1,400│ 600  250  30   120  40   180  0    180  │ $4,100 $4,800 │
│  Singapore Singapr· $5,630  $4,730│2,250 600  150  230  300 1,200 0   900  │ $9,000 $15,000│
│  Germany   Berlin·  $2,870  $2,520│1,100 400  90   180  30   320  0    350  │ $7,400 $7,800 │
│  (TOTAL + Essentials lead, in the first viewport; breakdown trails right; ▸ = more →)       │
└──────────────────────────────────────────────────────────────────────────────────────────┘
```

### Mobile reflow (Option A — selected)

On mobile (`< sm`) the table collapses to stacked cards, **Total leading each card** (it already
does), breakdown rows beneath, then the one-time relocation rows. Both city and country names stay
tappable links. This is why Option A reflows with no extra work — the mobile card already answers
"total first".

```
┌────────────────────────────┐
│ ┏Cost of living┓ ‹Sav›‹Min› │
│ [ Healthcare: tax-funded ]  │
│ Region   [ All ▼ ]          │
│ Country  [ All ▼ ]          │
│ City     [ All ▼ ]          │
├────────────────────────────┤
│ Jakarta, Indonesia  ›       │
│  Total      $1,580   ◀ lead │
│  Essentials $1,400          │
│  Housing      $600          │
│  …                          │
│  Reloc sunk $4,100 (1×)     │
│  Reserve    $4,800 (kept)   │
└────────────────────────────┘
```

The mobile↔desktop reflow differs in **layout** (multi-column table → stacked cards) but **not in
priority order** — Total leads in both, which is the whole point of the reorder.

## Option B — Sticky summary columns (dropped)

Keep the original order (breakdown first, summary last) but pin **Total** and **Essentials** as
sticky columns on the right edge so they stay visible while the user scrolls the breakdown.

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│  Country* City*  │ Hous Food Tran Util Hlth Care Schl Life │‖ Essen   TOTAL  (sticky right)│
│  ──────── ────── │ ──── ──── ──── ──── ──── ──── ──── ──── │‖ ─────   ─────                │
│  Indonesia Jakr· │ 600  250  30   120  40   180  0    180  │‖ $1,400  $1,580               │
│  …  (Sunk/Reserve scroll under the sticky pane — still hidden)                              │
└──────────────────────────────────────────────────────────────────────────────────────────┘
```

**Dropped**: sticky table cells are fragile across the three breakpoints and announce awkwardly to
screen readers; the user must still horizontal-scroll to read the breakdown, and Relocation/Liquidity
remain hidden behind the sticky pane. It decorates the overflow rather than removing it.

## Option C — Affordance only (scroll hint) (dropped)

Keep the original order entirely; add a right-edge gradient fade plus a "scroll for more →" hint so
the user knows the clipped columns exist.

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│  Country* City*  Hous Food Tran Util Hlth Care Schl Life  Essen ░░░ scroll for more → ░░░  │
│  Indonesia Jakr· 600  250  30   120  40   180  0    180   $1,400 ░░░░░░░░░░░░░░░░░░░░░░░░░  │
│  (Total / Sunk / Reserve still off-screen behind the fade)                                  │
└──────────────────────────────────────────────────────────────────────────────────────────┘
```

**Dropped**: the Total — the answer — is still off-screen on load; the affordance only signposts the
problem. Its right-edge scroll-hint idea is **grafted into Option A** as a secondary safety net for
the breakdown overflow that remains after the summary columns move left.
