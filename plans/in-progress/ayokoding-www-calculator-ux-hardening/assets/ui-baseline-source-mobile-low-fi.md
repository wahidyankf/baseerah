# Lo-fi: Baseline-source segmented control at mobile (DWT-004)

Min-role tab. The 3-option segmented control ("Savings target / Match a role / My salary") currently wraps
its third option inside a fixed-height row at ≤375px, ballooning the box to 56px and breaking the 44px
control rhythm + bottom-alignment.

## Before (≤375px — the defect)

```
 Baseline source
 ┌──────────────────────────────────────┐  ← box grows to 56px
 │ [ Savings target ][ Match a role ][ My│
 │  salary ]                             │  ← "My salary" wraps INSIDE the fixed row
 └──────────────────────────────────────┘
```

## After — Option A (DEFAULT): flex-wrap, each option keeps 44px

```
 Baseline source
 ┌──────────────────────────────────────┐
 │ [ Savings target ] [ Match a role ]   │  ← row 1, each pill 44px tall
 │ [ My salary ]                         │  ← row 2, still 44px tall, gap-aligned
 └──────────────────────────────────────┘
   each pill min-h-[44px]; container wraps with consistent row gap
```

## After — Option B (fallback): vertical stack at mobile

```
 Baseline source
 ┌──────────────────────────────────────┐
 │ [ Savings target                    ] │  44px
 │ [ Match a role                      ] │  44px
 │ [ My salary                         ] │  44px
 └──────────────────────────────────────┘
```

**Chosen default: Option A** (flex-wrap) — preserves the segmented feel, keeps each option at 44px (also
helping EWT-002), and is the smallest change to `SegmentedControl`. Option B is the fallback if wrapped
pills read awkwardly in review. At ≥768px the control stays a single 44px row (unchanged).

## Token usage

`bg-muted` wrapper · `bg-primary text-primary-foreground` active pill · `min-h-[44px]` per option. No raw hex.
