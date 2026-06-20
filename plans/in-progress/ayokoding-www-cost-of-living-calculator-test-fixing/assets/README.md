# UI Assets — Cost-of-Living Calculator Fix

This is a **fidelity-restoration** plan. The design ground truth already exists as committed hi-fi mockups;
this folder holds (1) a pointer to that ground truth and (2) lo-fi wireframes for the only **net-new** UI —
the two empty-state prompts.

## Tier-2 ground truth (existing, by reference)

The restore-fidelity clusters (dual currency, mobile card country, styled input, segmented baseline control,
locale names, tab labels) target these already-committed finalists — re-rendering them here would duplicate
maintained assets:

- [`../../../done/2026-06-19__ayokoding-www-salary-savings-calculator/assets/ui-cost-of-living-option-a-category-table.png`](../../../done/2026-06-19__ayokoding-www-salary-savings-calculator/assets/ui-cost-of-living-option-a-category-table.png) (desktop) + `-tablet.png` + `-mobile.png`
- [`../../../done/2026-06-19__ayokoding-www-salary-savings-calculator/assets/ui-savings-option-a-net-savings-table.png`](../../../done/2026-06-19__ayokoding-www-salary-savings-calculator/assets/ui-savings-option-a-net-savings-table.png) (desktop) + `-tablet.png` + `-mobile.png`
- [`../../../done/2026-06-19__ayokoding-www-salary-savings-calculator/assets/ui-min-role-option-a-ladder-table.png`](../../../done/2026-06-19__ayokoding-www-salary-savings-calculator/assets/ui-min-role-option-a-ladder-table.png) (desktop) + `-tablet.png` + `-mobile.png`

These mockups already show: dual-currency cells (`Rp 9.4M / $600`), the "City, Country" mobile card header,
the bordered salary input, and the segmented baseline control — i.e. the corrected design.

## Tier-1 net-new lo-fi (this folder)

- [`ui-empty-states-low-fi.md`](./ui-empty-states-low-fi.md) — ASCII wireframes for the Savings and
  Minimum-role empty states, mobile + desktop.
- [`ui-empty-states-low-fi-alternatives.md`](./ui-empty-states-low-fi-alternatives.md) — the design-funnel
  divergence (which lo-fi advances to hi-fi).

## Tier-2 net-new hi-fi (produced during execution — F11 Residual)

> **F11 Residual**: the hi-fi `.excalidraw.png` finalists for the two net-new empty states are the
> one plan-checker finding that cannot be remediated by the fixer agent, because binary PNG files
> cannot be auto-generated. This is a documented, intentional residual — not silently passed.

The hi-fi `.excalidraw.png` finalists are produced as an explicit `delivery.md` step **7.0**
(`[HUMAN]` sign-off gate) **before** the empty-state code lands in step 7.1. The Phase 7 Gate in
`delivery.md` explicitly blocks code until both files are committed here:

- `assets/ui-empty-states-savings-option-a.excalidraw.png` (mobile + desktop frames)
- `assets/ui-empty-states-min-role-option-a.excalidraw.png` (mobile + desktop frames)

Both must be grounded in `libs/web-ui` with design-token colors only (`bg-muted`,
`text-muted-foreground`, `text-foreground`), never raw hex.
