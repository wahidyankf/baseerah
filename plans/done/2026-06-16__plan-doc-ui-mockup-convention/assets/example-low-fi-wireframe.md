# Example — Funnel Stage 1 (Low-Fidelity Alternatives)

ASCII / Unicode wireframes in fenced code blocks are cheap, so this is where the design **diverges**:
present several genuinely different layouts for the screen, name them, and let reviewers compare
line-by-line. All three options below target the same screen — the Salary Savings Calculator
compare-all view — and all reuse `libs/web-ui` controls (tabs, inputs, dropdown, radio, card, table).

These feed Stage 2 (hi-fi shortlist) and Stage 3 (selection) — see
[assets/README.md](./README.md) for which option was chosen and why.

## Option A — Ranked Table

Cities as sortable rows; one savings bar per row. Densest; best for scanning many cities.

```
┌────────────────────────────────────────────────────────────┐
│  💰  Salary Savings Calculator                             │
├────────────────────────────────────────────────────────────┤
│  ┏ Compare All ┓ ( Single City )                           │
│  Salary [ 4,000 USD/mo ]  Household [ Single ▼ ]  (•)Rural │
├────────────────────────────────────────────────────────────┤
│  City            Savings/mo   % of salary   ⇅              │
│  ──────────────  ──────────   ───────────                  │
│  Jakarta         $2,100       52%   ███████                │
│  Kuala Lumpur    $1,800       45%   ██████                 │
│  Singapore       $1,200       30%   ████                   │
│  Berlin          $900         22%   ███                    │
└────────────────────────────────────────────────────────────┘
```

## Option B — Card Grid

Each city is a card with a big savings number. More visual, fewer cities per screen, weaker for
precise side-by-side number comparison.

```
┌────────────────────────────────────────────────────────────┐
│  ┏ Compare All ┓ ( Single City )                           │
│  Salary [ 4,000 USD/mo ]  Household [ Single ▼ ]  (•)Rural │
├────────────────────────────────────────────────────────────┤
│  ┌── Jakarta ───────┐   ┌── Kuala Lumpur ──┐               │
│  │  Save $2,100/mo  │   │  Save $1,800/mo  │               │
│  │  52% of salary   │   │  45% of salary   │               │
│  └──────────────────┘   └──────────────────┘               │
│  ┌── Singapore ─────┐   ┌── Berlin ────────┐               │
│  │  Save $1,200/mo  │   │  Save $900/mo    │               │
│  │  30% of salary   │   │  22% of salary   │               │
│  └──────────────────┘   └──────────────────┘               │
└────────────────────────────────────────────────────────────┘
```

## Option C — Split (controls left, results right)

Two-column: controls pinned in a left rail, results table on the right. Good on wide screens; the
left rail wastes space and forces stacking on mobile.

```
┌─────────────────────┬──────────────────────────────────────┐
│ ┏ Compare All ┓     │  City         Save/mo   %            │
│ ( Single City )     │  ──────────   ───────   ───          │
│                     │  Jakarta      $2,100    52%  ███████ │
│ Salary              │  Kuala Lmpr   $1,800    45%  ██████  │
│ [ 4,000 USD/mo ]    │  Singapore    $1,200    30%  ████    │
│ Household           │  Berlin       $900      22%  ███     │
│ [ Single        ▼ ] │                                      │
│ Area                │                                      │
│ ( )Center  (•)Rural │                                      │
└─────────────────────┴──────────────────────────────────────┘
```
