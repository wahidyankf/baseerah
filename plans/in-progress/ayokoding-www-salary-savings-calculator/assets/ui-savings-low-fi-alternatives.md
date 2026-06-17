# Example — Funnel Stage 1 (Low-Fidelity Alternatives) — Savings Screen

ASCII / Unicode wireframes in fenced code blocks are cheap, so this is where the design **diverges**:
present several genuinely different layouts for the screen, name them, and let reviewers compare
line-by-line. Both options below target the same screen — the Salary Savings Calculator **Savings**
tab — and all reuse `libs/web-ui` controls (tabs, input, dropdown, card, table, badge).

The screen answers: _"For my gross salary, where do I save the most across cities?"_ The user enters a
**gross salary** — **monthly and annual** (enter one, the tool shows both; annual = 12 × monthly).
For each city the tool converts gross to **net take-home** via the country's federal banded effective
tax rate plus any city sub-national rate, subtracts the modeled essentials, and shows **two savings
figures** — savings after essentials (`E.Save`) and savings after lifestyle (`L.Save`) — with
percentages, sortable. A separate **non-salary comp** column shows the typical RSU/equity + bonus for
that role × country as informational total-comp context (NOT folded into the deterministic monthly
savings math). The always-on **healthcare funding-scheme badge** is shown too.

**Geographic filters (all tabs)** — three cascading **Region / Country / City** filters sit above the
table (region narrows countries; country narrows cities). Every row **always shows both Country and
City** — a **Country column immediately to the LEFT of the City column** (mobile cards read "City,
Country"). **Both the Country and the City name are links** in every row: clicking a **City name**
navigates to that city's single-city **Cost-of-living detail** view (`?tab=cost&city=<id>`); clicking a
**Country name** switches to the **Cost-of-living tab filtered to that country** (`?tab=cost&country=<id>`).

**Roles** are **software-engineering roles (IC + management tracks)** — a caption/badge states this so
the gross figures and non-salary comp are read in that context.

## Selection — Option A (stakeholder-selected)

The stakeholder selected **Option A — Net/Savings Table**. Rationale: it makes the gross → net →
essentials → savings chain transparent in one row across many cities (the proven Numbeo ranked-table
idiom), and has room for the dual gross (monthly + annual), the non-salary-comp column, and the
Country+City columns. The card grid (Option B) hides the essentials/net chain and shows too few cities
for a worldwide savings scan.

| Alternative               | Outcome  | Reason                                                                                        |
| ------------------------- | -------- | --------------------------------------------------------------------------------------------- |
| **A — Net/Savings Table** | Selected | Transparent gross→net→savings chain in one row; room for dual gross + non-salary comp columns |
| B — Savings Card Grid     | Dropped  | Hides essentials/net chain; too few cities per screen for the worldwide savings scan          |

Refinements folded into the selected Option A: (1) **Region / Country / City** cascading filters;
(2) a **Country column immediately left of City** + **both Country and City names as links** (City →
single-city detail `?tab=cost&city=<id>`; Country → Cost-of-living filtered to that country
`?tab=cost&country=<id>`); (3) gross
shown **monthly AND annual**; (4) a **non-salary comp** column and a derived **total compensation**
(base + non-salary comp, informational — for salary-negotiation context, not in the savings math);
(5) a **"Roles: software-engineering (IC + management)"** caption.

> Note: this low-fi tier stays at sketch fidelity — the non-salary-comp / total-comp columns are
> shown abbreviated (e.g. "+$10k RSU") in the ASCII; the hi-fi pass adds the explicit Total-comp
> column. All money figures also render in local currency, converted via the in-repo `fx.ts` table.

## Option A — Net/Savings Table (SELECTED)

A sortable table: a Country column left of City, gross (monthly + annual), the typical non-salary comp
(RSU/equity + bonus), the income band + effective tax %, net take-home (after federal + sub-national
tax), modeled essentials, both savings figures, and their %. Densest; makes the gross → net →
essentials → savings chain transparent in one row.

```
┌────────────────────────────────────────────────────────────────────────────────────────────┐
│  💰  Savings              [ Healthcare: tax-funded ]   Roles: software-engineering (IC + mgmt)│
├────────────────────────────────────────────────────────────────────────────────────────────┤
│  ( Cost of living ) ┏ Savings ┓ ( Minimum role )                                            │
│  Region [ All ▼ ] Country [ All ▼ ] City [ All ▼ ]                                           │
│  Gross [ 8,000 USD/mo ] (= 96,000 USD/yr)  ( )Single (•)Married Pre[1] Sch[1]  (•)Rural      │
├────────────────────────────────────────────────────────────────────────────────────────────┤
│  Country* City*    Gross/mo  Gross/yr  Non-salary  Band Tax%  Net    Essen  E.Save E% L.Save │
│  ───────  ──────   ───────   ───────   ─────────   ──── ────  ─────  ─────  ────── ── ────── │
│  Indon.   Jakarta· $8,000    $96,000   +$10k RSU   mid  15%   $6,800 $1,400 $5,400 79% $5,220 │
│  Malays.  K.Lmpr·  $8,000    $96,000   +$8k  RSU   mid  19%   $6,500 $1,665 $4,835 74% $4,625 │
│  Portugal Lisbon·  $8,000    $96,000   +$5k  RSU   high 33%   $5,400 $2,080 $3,320 61% $3,020 │
│  Germany  Berlin·  $8,000    $96,000   +$6k  RSU   high 35%   $5,200 $2,520 $2,680 52% $2,360 │
│  (City* = link → single-city Cost-of-living detail; Country* = link → Cost-of-living filtered │
│   to that country; Non-salary = typical RSU/equity + bonus, informational total-comp only,   │
│   NOT in the savings math; E.Save = ranking figure; all $=+local)                            │
└────────────────────────────────────────────────────────────────────────────────────────────┘
```

## Option B — Savings Card Grid (dropped)

Each city is a card (heading "City, Country") with both savings numbers, the net figure, and the
non-salary-comp line. More visual, fewer cities per screen; hides the essentials column and makes the
net-of-tax chain harder to read at a glance.

```
┌──────────────────────────────────────────────────────────────────────┐
│  ( Cost of living ) ┏ Savings ┓ ( Minimum role )  [ Healthcare: OOP ] │
│  Region [ All ▼ ] Country [ All ▼ ] City [ All ▼ ]                    │
│  Gross [ 8,000 USD/mo ] (= 96,000/yr) ( )Single (•)Married (•)Rural   │
├──────────────────────────────────────────────────────────────────────┤
│  ┌── Jakarta, Indonesia ┐  ┌── K.Lumpur, Malaysia┐                    │
│  │ Net   $6,800     │      │ Net   $6,500     │                       │
│  │ +$10k RSU (info) │      │ +$8k RSU (info)  │                       │
│  │ E.Save $5,400/mo │      │ E.Save $4,835/mo │                       │
│  │ L.Save $5,220/mo │      │ L.Save $4,625/mo │                       │
│  └──────────────────┘      └──────────────────┘                      │
│  ┌── Lisbon, Portugal ┐    ┌── Berlin, Germany ┐                      │
│  │ E.Save $3,320/mo │      │ E.Save $2,680/mo │                       │
│  └──────────────────┘      └──────────────────┘                      │
└──────────────────────────────────────────────────────────────────────┘
```

## Mobile reflow (Option A — selected)

On mobile (`< sm`) the net/savings table collapses to **stacked cards**, one city per card (heading
"City, Country"; both the city name (→ detail) and the country name (→ Cost-of-living filtered to that
country) remain tappable links), the gross input (monthly + derived annual) and the three cascading
filters full-width above:

```
┌────────────────────────────┐
│ ‹Cost of living›┏Savings┓‹Min›│
│ [ Healthcare: tax-funded ]  │
│ Roles: SWE (IC + mgmt)      │
│ Region  [ All ▼ ]          │
│ Country [ All ▼ ]          │
│ City    [ All ▼ ]          │
│ Gross [ 8,000 USD/mo ]      │
│       (= 96,000 USD/yr)     │
│ ( )Single (•)Married        │
│ Pre-school[1] School-age[1] │
│ Area  ( )Center (•)Rural    │
│ Sort  [ E.Savings ▼ ]       │
├────────────────────────────┤
│ Jakarta, Indonesia  ›      │
│  Net          $6,800       │
│  Non-salary   +$10k RSU    │
│  Essentials   $1,400       │
│  E.Save       $5,400 (79%) │
│  L.Save       $5,220 (77%) │
├────────────────────────────┤
│ K. Lumpur, Malaysia  ›     │
│  Net          $6,500       │
│  E.Save       $4,835 (74%) │
└────────────────────────────┘
```
