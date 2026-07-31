# Phase 4 — WCAG AA Contrast Verification for `libs/web-ui-token/src/baseerah.css`

Every foreground/background semantic color pairing defined by the new Baseerah brand token file,
computed from its OKLCH values via the standard OKLCH → linear-sRGB → relative-luminance → WCAG
contrast-ratio pipeline (Björn Ottosson's OKLab conversion matrices; WCAG 2.x relative luminance and
contrast formulas). All pairs are body text, so the required threshold is **4.5:1** (no large-text
pair — 3:1 — is used by this token set).

## Method

Computed with a standalone Node script (no external dependency), converting each `oklch(L% C H)` or
`#hex` value through:

1. OKLCH → OKLab → linear sRGB (Ottosson's matrices)
2. Linear sRGB → relative luminance (`0.2126·R + 0.7152·G + 0.0722·B`)
3. Contrast ratio `(L1 + 0.05) / (L2 + 0.05)`, lighter over darker

## Results

### Light mode

| Pair                                     | Background             | Foreground             | Ratio     | Result   |
| ---------------------------------------- | ---------------------- | ---------------------- | --------- | -------- |
| `background` / `foreground`              | `oklch(99% 0.004 265)` | `oklch(18% 0.016 262)` | 18.28 : 1 | **PASS** |
| `card` / `card-foreground`               | `#ffffff`              | `oklch(18% 0.016 262)` | 18.81 : 1 | **PASS** |
| `popover` / `popover-foreground`         | `#ffffff`              | `oklch(18% 0.016 262)` | 18.81 : 1 | **PASS** |
| `primary` / `primary-foreground`         | `oklch(55% 0.17 265)`  | `#ffffff`              | 5.00 : 1  | **PASS** |
| `secondary` / `secondary-foreground`     | `oklch(94% 0.008 265)` | `oklch(18% 0.016 262)` | 15.77 : 1 | **PASS** |
| `muted` / `muted-foreground`             | `oklch(94% 0.008 265)` | `oklch(48% 0.016 265)` | 5.49 : 1  | **PASS** |
| `accent` / `accent-foreground`           | `oklch(95% 0.03 205)`  | `oklch(36% 0.1 205)`   | 8.63 : 1  | **PASS** |
| `destructive` / `destructive-foreground` | `oklch(56% 0.15 45)`   | `#ffffff`              | 4.95 : 1  | **PASS** |

### Dark mode

| Pair                                     | Background             | Foreground             | Ratio     | Result   |
| ---------------------------------------- | ---------------------- | ---------------------- | --------- | -------- |
| `background` / `foreground`              | `oklch(18% 0.012 262)` | `oklch(96% 0.008 265)` | 16.74 : 1 | **PASS** |
| `card` / `card-foreground`               | `oklch(22% 0.014 262)` | `oklch(96% 0.008 265)` | 15.41 : 1 | **PASS** |
| `popover` / `popover-foreground`         | `oklch(22% 0.014 262)` | `oklch(96% 0.008 265)` | 15.41 : 1 | **PASS** |
| `primary` / `primary-foreground`         | `oklch(54% 0.18 265)`  | `#ffffff`              | 5.25 : 1  | **PASS** |
| `secondary` / `secondary-foreground`     | `oklch(26% 0.016 262)` | `oklch(96% 0.008 265)` | 13.83 : 1 | **PASS** |
| `muted` / `muted-foreground`             | `oklch(26% 0.016 262)` | `oklch(70% 0.014 265)` | 5.82 : 1  | **PASS** |
| `accent` / `accent-foreground`           | `oklch(32% 0.06 205)`  | `oklch(86% 0.12 205)`  | 8.37 : 1  | **PASS** |
| `destructive` / `destructive-foreground` | `oklch(56% 0.16 45)`   | `#ffffff`              | 4.97 : 1  | **PASS** |

All 16 pairs (8 light + 8 dark) pass WCAG AA (≥4.5:1).

## Defects Found and Fixed

The first draft of `baseerah.css` failed 3 of the 16 pairs:

| Pair                                           | Original L | Original ratio  | Fixed L | Fixed ratio |
| ---------------------------------------------- | ---------- | --------------- | ------- | ----------- |
| light `destructive` / `destructive-foreground` | 64%        | 3.56 : 1 (FAIL) | 56%     | 4.95 : 1    |
| dark `primary` / `primary-foreground`          | 68%        | 2.99 : 1 (FAIL) | 54%     | 5.25 : 1    |
| dark `destructive` / `destructive-foreground`  | 70%        | 2.83 : 1 (FAIL) | 56%     | 4.97 : 1    |

All three used white (`#ffffff`) foreground text over a hue whose lightness (`L`) was too high for
sufficient contrast — a variant of the same failure mode recorded in the
[`web-ui-alert-destructive-dark-contrast`](../../../ideas/web-ui-alert-destructive-dark-contrast.md)
two-pager for other brands' tokens. Fixed by lowering each hue's `L` value (keeping chroma and hue
angle unchanged) until the ratio cleared 4.5:1 with a safety margin, then re-verified against the
full pair set above and against `npx nx run web-ui-token:test:quick --skip-nx-cache` (still passing).

## Scope

Only `libs/web-ui-token/src/baseerah.css`'s semantic color pairs are in scope — the neutral-only
`tokens.css` defaults (unbranded fallback) and the retained-but-unused legacy brand files
(`organiclever.css`, `ose.css`, `ayokoding.css`, `wahidyankf.css`) are out of scope for this Phase 4
verification, since no app in this repository currently imports them.
