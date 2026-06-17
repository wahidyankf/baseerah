# Example — Funnel Stage 1 (Low-Fidelity Alternatives) — Minimum-Role Screen

ASCII / Unicode wireframes in fenced code blocks are cheap, so this is where the design **diverges**:
present several genuinely different layouts for the screen, name them, and let reviewers compare
line-by-line. All three options below target the same screen — the Salary Savings Calculator
**Minimum Engineering Role** mode (the third tab) — and all reuse `libs/web-ui` controls (tabs,
inputs, dropdown, radio, card, table, badge, alert).

The screen answers: _"Given this savings bar, what is the **minimum** engineering role (anywhere in
the world) whose typical salary saves at least as much, in absolute terms?"_ The user sets a
**baseline** three ways — their own salary, a reference city + role, or a raw savings target — and
the screen ranks the canonical IC + management role ladder, marking the lowest rung that clears the
bar.

These feed Stage 2 (hi-fi shortlist) and Stage 3 (selection) — see the
[UI Design — Minimum-Role Screen section in `prd.md`](../prd.md#ui-design--minimum-role-screen-design-funnel)
for the hi-fi finalists, the named selection, and the rationale.

## Option A — Ladder Table

The full role ladder as sortable rows (least → most senior). Each row shows the best (cheapest)
city where that role clears the bar and its savings in USD / local / your currency. A highlighted
threshold line marks the **minimum qualifying role**; rungs below it are dimmed (cannot clear the
bar anywhere). Densest; shows how far above/below the bar every tier sits.

```
┌──────────────────────────────────────────────────────────────────┐
│  💰  Salary Savings Calculator                                    │
├──────────────────────────────────────────────────────────────────┤
│  ( Compare All ) ( Single City ) ┏ Minimum Role ┓                 │
│  Baseline: ( My salary ) (•)Reference role ( Savings target )     │
│  City [ Jakarta ▼ ]  Role [ Senior SWE ▼ ]  → bar = $2,100/mo     │
│  Show in: [ USD ▼ ]  Household [ Single ▼ ]  (•)Rural             │
├──────────────────────────────────────────────────────────────────┤
│  Role               Best city      Saves/mo (USD · local) · ⇅     │
│  ─────────────────  ────────────   ─────────────────────────      │
│  ░ SWE I            Hanoi          $1,150  · ₫29.0m   (below)      │
│  ░ SWE II           Jakarta        $1,780  · Rp28.4m  (below)      │
│  ▶ Senior SWE       Jakarta        $2,310  · Rp36.9m  ✓ MINIMUM    │
│    Eng. Manager     Bangkok        $2,640  · ฿94k     ✓            │
│    Staff SWE        Manila         $3,520  · ₱204k    ✓            │
│    Director         Kuala Lumpur   $4,910  · RM23k    ✓            │
└──────────────────────────────────────────────────────────────────┘
```

## Option B — Banner + List

A big answer banner up top states the minimum role plainly; below it, a flat ranked list of the
qualifying role+city combinations sorted by savings. Most direct for the one-shot question, but
hides the rungs that fail (less context on how close the near-misses are).

```
┌──────────────────────────────────────────────────────────────────┐
│  ( Compare All ) ( Single City ) ┏ Minimum Role ┓                 │
│  Baseline savings bar: $2,100 / mo   Show in: [ USD ▼ ]           │
├──────────────────────────────────────────────────────────────────┤
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Minimum role to match $2,100/mo:                          │  │
│  │     Senior SWE  in  Jakarta   →  saves $2,310/mo           │  │
│  └────────────────────────────────────────────────────────────┘  │
│  Other ways to clear the bar (ranked by savings):                 │
│   • Staff SWE      · Manila        $3,520/mo                       │
│   • Eng. Manager   · Bangkok       $2,640/mo                       │
│   • Senior SWE     · Jakarta       $2,310/mo  ← minimum            │
│   • Senior SWE     · Hanoi         $2,180/mo                       │
└──────────────────────────────────────────────────────────────────┘
```

## Option C — Split (baseline controls left, ladder right)

Two-column: baseline + display controls pinned in a left rail, the role ladder on the right. Good
on wide screens; the left rail wastes space and forces stacking on mobile (consistent with the
Compare-All Option C trade-off).

```
┌─────────────────────────┬────────────────────────────────────────┐
│ ( Compare ) ( Single )   │  Role           Best city    Saves/mo  │
│ ┏ Minimum Role ┓         │  ───────────    ──────────   ───────── │
│                          │  ░ SWE I        Hanoi        $1,150     │
│ Baseline                 │  ░ SWE II       Jakarta      $1,780     │
│ (•) Reference role       │  ▶ Senior SWE   Jakarta      $2,310  ✓  │
│ City [ Jakarta      ▼ ]  │    Eng. Manager Bangkok      $2,640  ✓  │
│ Role [ Senior SWE   ▼ ]  │    Staff SWE    Manila       $3,520  ✓  │
│ bar = $2,100/mo          │    Director     K. Lumpur    $4,910  ✓  │
│ Show in [ USD       ▼ ]  │                                        │
│ Household [ Single  ▼ ]  │                                        │
│ Area ( )Center (•)Rural  │                                        │
└─────────────────────────┴────────────────────────────────────────┘
```
