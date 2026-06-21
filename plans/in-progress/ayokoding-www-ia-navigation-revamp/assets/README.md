# Hi-Fi Design Assets — IA & Navigation Revamp

This directory holds the **selected** high-fidelity mockups that serve as the
**visual-parity ground truth** for the rule-15 delivery sign-off. The diverge-stage
low-fi alternatives live in [`ui-low-fi-alternatives.md`](./ui-low-fi-alternatives.md);
the narrow → select → justify reasoning lives in [`../prd.md`](../prd.md#ui-design-funnel).

## Selected direction — Option A (all three screens)

After comparing the three low-fi alternatives per screen, **Option A** was selected for
every screen because the three Option-A layouts share one card vocabulary, which keeps the
landing, the `/c` browse index, and the chrome visually consistent and directly solves the
"calculator is cut off from the flow" problem (the Tools teaser is a first-class band, not a
buried chip).

| Screen | Selected | Why (vs the dropped alternatives) |
| --- | --- | --- |
| Landing `/[locale]` | **A — Hero + section-card grid + Tools teaser** | B's two-rail split under-sells the curated content and stacks weakly on mobile; C's latest-feed re-buries Tools into a chip (re-creating the discoverability bug) and leans on per-page dates the `id` locale lacks. |
| `/c` browse index | **A — Restyled section-card grid** | B is literally today's bare `SidebarTree` (the thing we're replacing); C's two-pane explorer duplicates the in-content sidebar and is heavier than a browse index needs. |
| Header + footer | **A — Inline header links + multi-column footer** | B's single-row footer can't group Learn/Tools/About legibly; C's mega-dropdown is over-built for two nav items and adds a focus-trap a11y burden. |

## Design tokens used (light theme — verified against source)

Pulled from `libs/web-ui-token/src/tokens.css` + `apps/ayokoding-www/src/app/globals.css`:

| Token | Value | Hex (approx) | Use in mockups |
| --- | --- | --- | --- |
| `--color-background` | `hsl(0 0% 100%)` | `#ffffff` | page + card fill |
| `--color-foreground` | `hsl(0 0% 3.9%)` | `#0a0a0a` | headings, nav, body |
| `--color-muted-foreground` | `hsl(0 0% 45.1%)` | `#737373` | blurbs, captions |
| `--color-border` | `hsl(0 0% 89.8%)` | `#e5e5e5` | card + header/footer borders |
| `--color-accent` / `--color-muted` | `hsl(0 0% 96.1%)` | `#f5f5f5` | hover/keycap surfaces |
| `--color-primary` (ayokoding brand) | `hsl(221.2 83.2% 53.3%)` | `#2563eb` | CTAs, active nav, Tools band |
| `--color-primary-foreground` | `hsl(210 40% 98%)` | `#ffffff` | text on primary |

The Tools-teaser tint (`#eff4ff` fill / `#dbe6ff` border) is a low-opacity wash of the brand
primary — implement as `bg-primary/5 border-primary/15` rather than a new token.

## File index

Each screen is rendered at the four supported breakpoints. `320px` is the overflow-proof
floor (regression target from the prerequisite plan's UWT-008); `375` = small mobile,
`768` = tablet, `1280` = desktop. The `320` layout is identical to `375` (single column) and
is rendered only for the landing top-fold to prove no horizontal overflow.

| Screen | 320 | 375 | 768 | 1280 |
| --- | --- | --- | --- | --- |
| Landing `/[locale]` | [`landing-320.svg`](./landing-320.svg) | [`landing-375.svg`](./landing-375.svg) | [`landing-768.svg`](./landing-768.svg) | [`landing-1280.svg`](./landing-1280.svg) |
| `/c` browse index | (= 375) | [`browse-375.svg`](./browse-375.svg) | [`browse-768.svg`](./browse-768.svg) | [`browse-1280.svg`](./browse-1280.svg) |
| Header + footer chrome | (= 375) | [`chrome-375.svg`](./chrome-375.svg) | (= 1280 inline) | [`chrome-1280.svg`](./chrome-1280.svg) |

`chrome-375.svg` also shows the open **MobileNav** drawer (the hamburger target) so the
mobile-nav parity requirement has a visual ground truth.

## Locale note

The mockups render the `en` locale. The `id` locale uses the same layout, token, and
component structure with translated strings and **locale-specific slugs** — header "Learn"
points at `/id/c`, section cards derive from `belajar` / `celoteh` / `konten-video`, and the
footer links `Tentang AyoKoding` / `Syarat & Ketentuan` at their top-level `id` URLs. See
[`../tech-docs.md` §Locale Slug Asymmetry](../tech-docs.md). Phase 6 captures real screenshots
of both locales at all four breakpoints into [`../evidence/`](../evidence/).

## Status

These SVGs are the committed visual-parity ground truth. They are wireframe-fidelity hi-fi
(layout, spacing, type scale, real tokens) — not pixel-final renders. During Phase 4/6 the
built pages are diffed against these; any intentional deviation is recorded in `../evidence/`.
