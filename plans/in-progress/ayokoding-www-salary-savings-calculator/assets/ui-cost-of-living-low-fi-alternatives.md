# Example — Funnel Stage 1 (Low-Fidelity Alternatives) — Cost-of-Living Screen

ASCII / Unicode wireframes in fenced code blocks are cheap, so this is where the design **diverges**:
present several genuinely different layouts for the screen, name them, and let reviewers compare
line-by-line. All three options below target the same screen — the Salary Savings Calculator
**Cost of living** tab — and all reuse `libs/web-ui` controls (tabs, dropdown, command, card, table,
badge).

The screen answers: _"How much do I need to live in each hub?"_ It has **no salary input** and **no
savings columns** — only the modeled monthly expense breakdown (housing, food, transport, utilities,
healthcare-OOP, childcare, school, lifestyle) with an essentials subtotal and a total, a separate
one-time **relocation sunk-cost** line, and a separately labelled **liquidity reserve** (the cash
cushion the user keeps). Every layout also shows the always-on **healthcare funding-scheme badge** for
the selected city/country, plus an **on-screen legend explaining the "Healthcare (OOP)" column header —
OOP = out-of-pocket**, the healthcare you pay yourself on top of any tax-funded or insurance coverage.

**Geographic filters (all tabs)** — three cascading filters sit above the table on every tab:
**Region** narrows the **Country** list, and **Country** narrows the **City** list. Every row
**always shows both Country and City** — a **Country column immediately to the LEFT of the City
column** in the table (on mobile cards the heading reads "City, Country"). **Both the Country and the
City name are links** in every row: clicking a **City name** navigates to that city's single-city
**Cost-of-living detail** view (deep-linkable as `?tab=cost&city=<id>`); clicking a **Country name**
switches to the **Cost-of-living tab filtered to that country** (its cities as a list, deep-linkable as
`?tab=cost&country=<id>`).

## Selection — Option A (stakeholder-selected)

The stakeholder selected **Option A — Category Table**. Rationale: it is the densest worldwide-scan
layout (matches the Numbeo ranked-index idiom from the prior-art survey), keeps the full per-category
breakdown inline for direct cross-city comparison, and reflows cleanly to stacked cards on mobile. The
two card-based runners-up (Option B, Option C) show too few cities per screen for the worldwide scan
the tool is built for.

| Alternative                | Outcome  | Reason                                                                                                |
| -------------------------- | -------- | ----------------------------------------------------------------------------------------------------- |
| **A — Category Table**     | Selected | Densest worldwide scan; full per-category breakdown inline; Country+City columns; clean mobile reflow |
| B — Category Cards         | Dropped  | Too few cities per screen; weak for precise side-by-side category comparison                          |
| C — Country Drill (2-pane) | Dropped  | Single-country pane too narrow for the worldwide scan; left rail stacks awkwardly on mobile           |

Refinements folded into the selected Option A: (1) **Region / Country / City** cascading filter row;
(2) a **Country column immediately left of City**; (3) **both Country and City names are links** —
a City link opens the single-city **Cost-of-living detail** view (`?tab=cost&city=<id>`), a Country
link opens the **Cost-of-living tab filtered to that country** (`?tab=cost&country=<id>`); (4) the
**school** column is shown inline alongside childcare.

## Option A — Category Table (SELECTED)

Cities as sortable rows; a **Country column immediately to the left of the City column**, then one
column per expense category (incl. childcare and school), then an essentials subtotal, a total, a
separate relocation sunk-cost column, and a liquidity-reserve column. Densest; best for scanning many
cities across categories. The three cascading **Region / Country / City** filters sit above the table.

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│  💰  Cost of Living            [ Healthcare: tax-funded ]                                  │
├──────────────────────────────────────────────────────────────────────────────────────────┤
│  ┏ Cost of living ┓ ( Savings ) ( Minimum role )                                          │
│  Region [ All ▼ ]  Country [ All ▼ ]  City [ All ▼ ]                                       │
│  ( )Single (•)Married  Pre-school[1] School-age[1]  (•)Rural                               │
├──────────────────────────────────────────────────────────────────────────────────────────┤
│  Country*   City*     Hous Food Trans Util Hlth Care Schl Life Essen  Total  Sunk  Reserve │
│  ────────   ───────   ──── ──── ───── ──── ──── ──── ──── ──── ─────  ─────  ────  ─────── │
│  Indonesia  Jakarta·  600  250  30    120  40   180  0    180  $1,400 $1,580 $4,100 $4,800  │
│  Malaysia   K.Lumpur· 720  300  35    140  50   210  0    220  $1,665 $1,885 $5,000 $5,500  │
│  Germany    Berlin·  1,100 400  90    180  30   320  0    350  $2,520 $2,870 $7,400 $7,800  │
│  Portugal   Lisbon·   950  350  45    160  35   270  0    300  $2,080 $2,380 $6,200 $6,600  │
│  (City* = click → single-city Cost-of-living detail; Country* = click → Cost-of-living     │
│   filtered to that country; Sunk = relocation sunk cost; Reserve = liquidity reserve kept; │
│   all money also shown in USD)                                                             │
└──────────────────────────────────────────────────────────────────────────────────────────┘
```

### Single-city Cost-of-living detail (drill-down from a city link)

Clicking any **city name** anywhere (any tab) opens the **Cost-of-living tab scoped to that one city**
(deep-linkable as `?tab=cost&city=<id>`): the full per-category breakdown, essentials subtotal, total,
healthcare scheme badge, and the split relocation (sunk + liquidity reserve), all dual-currency
(local + USD). A back affordance returns to the full table. Clicking a **country name** instead opens
the **Cost-of-living tab filtered to that country** (`?tab=cost&country=<id>`) — the full table scoped
to that country's cities (a filtered list, not a single-city detail); a city click always wins when
both a `country` and a `city` param are present (a city implies its country).

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ‹ Back to all cities      Jakarta, Indonesia   [ Healthcare: out-of-pocket ] │
├──────────────────────────────────────────────────────────────────────────────┤
│  Housing      Rp 9.0m   ($600)      Childcare    Rp 4.5m   ($300)             │
│  Food         Rp 3.75m  ($250)      School (pub) Rp 0       ($0)              │
│  Transport    Rp 0.45m  ($30)       Lifestyle    Rp 2.7m   ($180)            │
│  Utilities    Rp 1.8m   ($120)      ───────────────────────────────         │
│  Healthcare   Rp 0.6m   ($40)       Essentials   Rp 21.0m  ($1,400)          │
│                                     Total        Rp 23.7m  ($1,580)          │
│  ── Relocation (one-time) ──────────────────────────────────────────        │
│  Sunk cost    Rp 61.5m  ($4,100)    Liquidity reserve  Rp 72.0m  ($4,800)    │
└──────────────────────────────────────────────────────────────────────────────┘
```

## Option B — Category Cards (dropped)

Each city is a card with the categories stacked, an essentials subtotal, a total, a relocation
sunk-cost line, and a separate liquidity-reserve line; the card heading reads "City, Country". More
visual, fewer cities per screen, weaker for precise side-by-side comparison of the categories.

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ┏ Cost of living ┓ ( Savings ) ( Minimum role )   [ Healthcare: out-of-pocket ]│
│  Region [ All ▼ ]  Country [ All ▼ ]  City [ All ▼ ]                          │
│  ( )Single (•)Married  Pre-school[1] School-age[1]  (•)Rural                  │
├──────────────────────────────────────────────────────────────────────────────┤
│  ┌── Jakarta, Indonesia ┐   ┌── K. Lumpur, Malaysia┐                          │
│  │ Housing      $600   │    │ Housing      $720   │                           │
│  │ Childcare    $300   │    │ Childcare    $350   │                           │
│  │ … Essen   $1,400    │    │ … Essen   $1,665    │                           │
│  │ Total     $1,580    │    │ Total     $1,885    │                           │
│  │ Reloc sunk  $4,100  │    │ Reloc sunk  $5,000  │                           │
│  │ Reserve     $4,800  │    │ Reserve     $5,500  │                           │
│  └─────────────────────┘    └─────────────────────┘                          │
└──────────────────────────────────────────────────────────────────────────────┘
```

## Option C — Country Drill (filter left, one country's cities right) (dropped)

Two-column: a country filter pinned in a left rail, the chosen country's cities and their category
breakdown on the right. Good for comparing cities within one country; the single-country pane shows
too few cities for the worldwide scan, and the left rail stacks awkwardly on mobile. Its
cascading-filter idea is **grafted into Option A** as the Region / Country / City filter row.

```
┌─────────────────────┬────────────────────────────────────────────────────────┐
│ ┏ Cost of living ┓  │  Jakarta, Indonesia [ Healthcare: out-of-pocket ]       │
│ ( Savings )         │  Housing $600  Food $250  Trans $30  Util $120         │
│ ( Minimum role )    │  Hlth $40  Childcare $300  Life $180  · Essen $1,400    │
│                     │  Total $1,580 · Reloc sunk $4,100 · Reserve $4,800      │
│ Region [ ASEAN  ▼ ] │  ─────────────────────────────────────────────────     │
│ Country [ Indon. ▼ ]│  Bandung, Indonesia                                     │
│  • Jakarta          │  Housing $420  Childcare $220  … · Total $1,180         │
│  • Bandung          │  Reloc sunk $3,200 · Reserve $3,600                     │
│  • Surabaya         │  ─────────────────────────────────────────────────     │
│ ( )Single (•)Married│  Surabaya, Indonesia                                    │
│ Pre[1] Sch[1]       │  Housing $380  Childcare $200  … · Total $1,090         │
│ Area ()Ctr (•)Rural │  Reloc sunk $2,900 · Reserve $3,300                     │
└─────────────────────┴────────────────────────────────────────────────────────┘
```

## Mobile reflow (Option A — selected)

On mobile (`< sm`) the category table collapses to **stacked cards**, one city per card (heading
"City, Country" — both the city name (→ detail) and the country name (→ Cost-of-living filtered to
that country) remain tappable links), controls full-width above; the three cascading filters stack:

```
┌────────────────────────────┐
│ ┏Cost of living┓ ‹Sav›‹Min› │
│ [ Healthcare: tax-funded ]  │
│ Region   [ All ▼ ]         │
│ Country  [ All ▼ ]         │
│ City     [ All ▼ ]         │
│ ( )Single (•)Married        │
│ Pre-school[1] School-age[1] │
│ Area  ( )Center (•)Rural    │
├────────────────────────────┤
│ Jakarta, Indonesia  ›      │
│  Housing      $600         │
│  Food         $250         │
│  Transport    $30          │
│  Utilities    $120         │
│  Healthcare   $40          │
│  Childcare    $300         │
│  School       $0           │
│  Lifestyle    $180         │
│  Essentials $1,400         │
│  Total      $1,580         │
│  Reloc sunk $4,100 (1×)    │
│  Reserve    $4,800 (kept)  │
├────────────────────────────┤
│ K. Lumpur, Malaysia  ›     │
│  Total      $1,885         │
│  Reloc sunk $5,000 (1×)    │
│  Reserve    $5,500 (kept)  │
└────────────────────────────┘
```
