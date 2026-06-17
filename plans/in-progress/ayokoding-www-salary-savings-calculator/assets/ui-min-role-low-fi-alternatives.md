# Example — Funnel Stage 1 (Low-Fidelity Alternatives) — Minimum-Role Screen

ASCII / Unicode wireframes in fenced code blocks are cheap, so this is where the design **diverges**:
present several genuinely different layouts for the screen, name them, and let reviewers compare
line-by-line. All three options below target the same screen — the Salary Savings Calculator
**Minimum Engineering Role** mode (the third tab) — and all reuse `libs/web-ui` controls (tabs,
inputs, dropdown, radio, card, table, badge, alert).

The screen answers: _"Given this savings bar, what is the **minimum** software-engineering role
(anywhere in the world) whose typical salary saves at least as much **essential savings** (lifestyle
excluded), in absolute terms?"_ The user sets a **baseline** three ways — their own salary, a
reference city + role, or a raw savings target — and the screen ranks the canonical
**software-engineering** role ladder (**IC + management tracks**) on **essential savings**. The ladder
is **reordered so qualifying roles are grouped above the minimum and non-qualifying ("below minimum")
roles are grouped below a divider**. The always-on **healthcare funding-scheme badge** is shown too.

**Roles are software-engineering roles (IC + management).** A caption/badge states this so the ranking
is read in context.

**Role salary is a per-role × COUNTRY distribution** — each role × country stores **p25 (bottom 25%),
median, and p75 (top 25%)**; cities inherit their country's distribution (a documented simplification —
role salary is national-level, not city-level). The **median** is the representative salary used for
ranking and for the reference-role baseline; **all three percentiles are displayed**. A typical
**non-salary comp** (RSU/equity + bonus) per role × country is also shown as informational total-comp
context (not folded into the savings math).

**Geographic filters (all tabs)** — three cascading **Region / Country / City** filters sit above the
ladder; they **scope the candidate cities** (each role's best city is chosen within the filtered set).
Every qualifying/non-qualifying row **always shows both Country and City** (a **Country column to the
left of the City column**; mobile cards read "City, Country"). Clicking any **city name** opens the
single-city **Cost-of-living detail** (`?tab=cost&city=<id>`).

## Selection — Option A (stakeholder-selected)

The stakeholder selected **Option A — Ladder Table**. Rationale: it shows the full ranked ladder with
the qualifying/non-qualifying split, the per-role best city + country, the p25/median/p75 distribution,
and the dual+ currencies in one dense, scannable surface — the proven levels.fyi/Numbeo ranked-table
idiom. The banner (Option B) hides the near-miss context and the split-rail layout (Option C) wastes
width and forces awkward mobile stacking.

| Alternative          | Outcome  | Reason                                                                                  |
| -------------------- | -------- | --------------------------------------------------------------------------------------- |
| **A — Ladder Table** | Selected | Full ranked ladder with qualifying/non-qualifying split, best city+country, p25/med/p75 |
| B — Banner + List    | Dropped  | Hides the failing rungs / near-miss context that the ladder surfaces                    |
| C — Split (2-pane)   | Dropped  | Left rail wastes width; forces awkward mobile stacking                                  |

Refinements folded into the selected Option A: (1) **reorder** so qualifying roles sit high→low down
to the marked MINIMUM, then a **divider**, then dimmed non-qualifying roles below; (2) a **Country
column** (best city + its country); (3) **p25 / median / p75** shown per role × country; (4) a
**non-salary comp** line and a derived **total compensation** (base + non-salary comp, informational —
for salary-negotiation context, not in the ranking); (5) **Region / Country / City** filters scope the
candidate cities; (6) a **"Roles: software-engineering (IC + management)"** caption; (7) city names
link to the detail.

> Note: this low-fi tier stays at sketch fidelity — non-salary-comp / total-comp are shown on tap in
> the ASCII; the hi-fi pass surfaces the explicit Total-comp figure. All money figures render in USD +
> local (+ chosen display currency), converted via the in-repo `fx.ts` table.

## Option A — Ladder Table (SELECTED)

The role ladder as sortable rows, **reordered around the minimum**: qualifying roles are listed
high→low down to the marked **MINIMUM** qualifier, then a divider, then the **dimmed non-qualifying
("below minimum")** roles grouped below. Each row shows the best (cheapest-qualifying) city **and its
country**, the role × country **p25 / median / p75** distribution, and the resulting **essential
savings** in USD / local / your currency.

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│  💰  Salary Savings Calculator   [ Healthcare: OOP ]  Roles: SWE (IC + management) │
├──────────────────────────────────────────────────────────────────────────────────┤
│  ( Cost of living ) ( Savings ) ┏ Minimum role ┓                                  │
│  Baseline: ( My salary ) (•)Reference role ( Savings target )                     │
│  Region [ All ▼ ] Country [ All ▼ ] City [ All ▼ ]                                │
│  Ref City [ Jakarta ▼ ]  Ref Role [ Senior SWE ▼ ]  → bar = $2,100/mo            │
│  Show in: [ USD ▼ ]  ( )Single (•)Married Pre[1] Sch[1]  (•)Rural                 │
├──────────────────────────────────────────────────────────────────────────────────┤
│  Role            Country   Best city  Salary p25/med/p75   E.Save/mo (USD·local) ⇅│
│  ─────────────   ────────  ─────────  ──────────────────   ─────────────────────  │
│  ── Qualifies (best-city essential savings ≥ bar) ──────────────────────────────  │
│  Director        Malaysia  K.Lumpur·  $9k / $12k / $16k    $4,910 · RM23k    ✓    │
│  Staff SWE       Philipp.  Manila·    $6k / $8k  / $11k    $3,520 · ₱204k    ✓    │
│  Eng. Manager    Thailand  Bangkok·   $5k / $7k  / $9k     $2,640 · ฿94k     ✓    │
│  ▶ Senior SWE    Indonesia Jakarta·   $4k / $6k  / $8k     $2,310 · Rp36.9m  ✓ MIN│
│  ── Below minimum (cannot clear the bar anywhere) ──────────────────────────────  │
│  ░ SWE II        Indonesia Jakarta·   $3k / $4k  / $5k     $1,780 · Rp28.4m       │
│  ░ SWE I         Vietnam   Hanoi·     $2k / $3k  / $4k     $1,150 · ₫29.0m        │
│  (Salary p25/med/p75 = role×country distribution, median ranks; E.Save = essential │
│   savings, lifestyle excluded; City* links to detail; non-salary comp shown on tap)│
└──────────────────────────────────────────────────────────────────────────────────┘
```

## Option B — Banner + List (dropped)

A big answer banner up top states the minimum role plainly; below it, a flat ranked list of the
qualifying role+city+country combinations sorted by essential savings, then the below-minimum rungs.
Most direct for the one-shot question, but hides how close the near-misses are.

```
┌──────────────────────────────────────────────────────────────────┐
│  ( Cost of living ) ( Savings ) ┏ Minimum role ┓ [ HC: OOP ]      │
│  Roles: SWE (IC + mgmt)  Baseline bar: $2,100/mo  Show: [ USD ▼ ] │
├──────────────────────────────────────────────────────────────────┤
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Minimum role to match $2,100/mo essential savings:        │  │
│  │     Senior SWE  in  Jakarta, Indonesia → saves $2,310/mo   │  │
│  │     (Indonesia p25/med/p75: $4k / $6k / $8k)               │  │
│  └────────────────────────────────────────────────────────────┘  │
│  Qualifies (ranked by essential savings):                         │
│   • Staff SWE   · Manila, Philippines   $3,520/mo                  │
│   • Eng. Manager· Bangkok, Thailand     $2,640/mo                  │
│   • Senior SWE  · Jakarta, Indonesia    $2,310/mo  ← minimum       │
│  Below minimum:                                                    │
│   ░ SWE II      · Jakarta, Indonesia    $1,780/mo                  │
└──────────────────────────────────────────────────────────────────┘
```

## Option C — Split (baseline controls left, ladder right) (dropped)

Two-column: baseline + display controls pinned in a left rail, the reordered role ladder on the right.
Good on wide screens; the left rail wastes space and forces stacking on mobile.

```
┌─────────────────────────┬────────────────────────────────────────────┐
│ ( Cost ) ( Savings )     │  Role         Country  Best city  E.Save/mo │
│ ┏ Minimum role ┓         │  ──────────   ──────   ────────   ───────── │
│ [ HC: OOP ]              │  Director     MY       K.Lumpur   $4,910 ✓  │
│ Roles: SWE (IC+mgmt)     │  Staff SWE    PH       Manila     $3,520 ✓  │
│ Baseline                 │  Eng. Mgr     TH       Bangkok    $2,640 ✓  │
│ (•) Reference role       │  ▶Senior SWE  ID       Jakarta    $2,310 ✓MIN│
│ Region [ All       ▼ ]   │  ── below minimum ───────────────────────── │
│ Country [ All      ▼ ]   │  ░ SWE II     ID       Jakarta    $1,780     │
│ City  [ Jakarta    ▼ ]   │  ░ SWE I      VN       Hanoi      $1,150     │
│ Role  [ Senior SWE ▼ ]   │                                            │
│ bar = $2,100/mo          │                                            │
│ Show in [ USD      ▼ ]   │                                            │
└─────────────────────────┴────────────────────────────────────────────┘
```

## Mobile reflow (Option A — selected)

On mobile (`< sm`) the ladder collapses to **stacked cards** (one role per card, heading "City,
Country"; the city name links to the detail view), baseline + filter controls full-width above. The
qualifying cards come first (threshold card emphasised), a divider, then the dimmed below-minimum
cards:

```
┌────────────────────────────┐
│ ‹Cost›‹Savings›┏Min role┓   │
│ [ Healthcare: OOP ]         │
│ Roles: SWE (IC + mgmt)      │
│ Baseline (•)Reference role  │
│ Region/Country/City [ All ] │
│ bar = $2,100/mo  Show[USD▼] │
├──── Qualifies ─────────────┤
│ ▶ Senior SWE        ✓ MIN  │
│   Jakarta, Indonesia  ›    │
│   p25/med/p75 $4k/$6k/$8k  │
│   E.Save $2,310 · Rp36.9m  │
├────────────────────────────┤
│   Eng. Manager        ✓    │
│   Bangkok, Thailand   ›    │
│   E.Save $2,640            │
├──── Below minimum ─────────┤
│ ░ SWE II                   │
│   Jakarta, Indonesia  ›    │
│   E.Save $1,780            │
└────────────────────────────┘
```
