# Lo-fi: Select chrome consistency (DWT-002 / DWT-003)

The household selects (Adults / Preschool / School-age) and the min-role currency/ref selects render with
the **native OS dropdown arrow** (`appearance:auto`), while the geo selects (Region / Country / City) use
the design-system `SelectField` (`appearance-none` + custom `ChevronDown`). Two dropdown chromes on one page.

## Before (the defect)

```
 Region            Adults
 ┌──────────────▼┐  ┌──────────────⌄┐   ← geo = custom chevron (▼ token icon)
 │ All regions   │  │ 1             │     household = native OS arrow (⌄)
 └───────────────┘  └───────────────┘
   styled chrome       inconsistent native chrome
```

## After (target — one chrome everywhere)

```
 Region            Adults
 ┌──────────────▼┐  ┌──────────────▼┐   ← all selects: appearance-none + ChevronDown
 │ All regions   │  │ 1             │     identical border, radius, padding (pr-8 pl-3), 44px
 └───────────────┘  └───────────────┘
```

- Wrap the household selects (`controls.tsx`) and the min-role currency/ref selects (`min-role.tsx`) in the
  same `SelectField` primitive / `GEO_SELECT_CLASS` the geo selects use.
- All selects: `appearance-none`, custom `ChevronDown` overlay, `min-h-[44px]`, `border-border`,
  `bg-background`, `rounded-md`, `pr-8 pl-3`.

## Responsive

Identical at all breakpoints — selects are full-width within their field column; the chevron overlay scales
with the control. No layout change beyond the chrome swap.

## Token usage

`border-border` · `bg-background` · `text-foreground` · `ChevronDown` (currentColor). No raw hex, no native
arrow.
