---
name: apps-baseerah-fe-developing-content
description: Structure, tone, and accessibility rules for baseerah-fe's landing-page content (tagline, brand chip, footer, not-found/error copy). Auto-loads when authoring or validating baseerah-fe copy.
---

# Developing Content for baseerah-fe

## Purpose

`baseerah-fe` is a Next.js 16 App Router hello-world skeleton, not a blog or CMS-backed content
platform — it has no `content/` collection, no date-prefixed files, and no MDX pipeline. Its entire
content surface today is inline JSX copy inside a handful of components and routes. This Skill
documents that surface so content agents change the right files in the right way instead of
inventing a blog-style structure that doesn't exist here.

**When to use this Skill**: creating or editing copy in `apps/baseerah-fe/src/components/AppShell.tsx`,
`AppFrame.tsx`, `src/app/page.tsx`, `src/app/not-found.tsx`, `src/app/error.tsx`, or `src/app/layout.tsx`
metadata.

## Content Surface

| File                          | Content                                                                                                                               |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `src/components/AppFrame.tsx` | Shared header title (`AppHeader title="Baseerah"`) and footer copyright line                                                          |
| `src/components/AppShell.tsx` | Landing page tagline, the trilingual brand chip (بصيرة / insight / wawasan), the backend-greeting line, and the "View on GitHub" link |
| `src/app/not-found.tsx`       | 404 heading, "Back to home" link, and its own `<title>` metadata                                                                      |
| `src/app/error.tsx`           | Client-side error heading and "Try again" reset button                                                                                |
| `src/app/layout.tsx`          | Root `<html lang>`, default `<title>`/`<meta description>`                                                                            |

## Rules

- **English-first, with deliberate bilingual/trilingual moments**: the brand chip is the one place
  Arabic (`بصيرة`, `lang="ar" dir="rtl"`) and Indonesian (`wawasan`) appear alongside English
  (`insight`) — this is the brand's etymology, not a localization feature. Do not add translated
  copy anywhere else without a product decision to do so.
- **No hardcoded entities in JSX attributes**: HTML named entities (e.g. `&middot;`) decode inside
  JSX text children but not inside plain string attribute values (like `title="..."`) — use the
  literal Unicode character (`·`) directly in attributes instead.
- **Design tokens over raw values**: use the `@open-sharia-enterprise/web-ui-token` Tailwind classes
  (`text-primary`, `bg-accent`, `border-border`, etc.) already in use in these files — never introduce
  a raw hex color or arbitrary Tailwind value.
- **Shared chrome via `AppFrame`**: every route-level page (`page.tsx`, `not-found.tsx`, `error.tsx`)
  renders through `AppFrame` so the header/footer stay consistent — don't duplicate header/footer
  markup in a new route.
- **Accessibility**: exactly one `<h1>` per rendered page, landmark elements (`header`/`main`/`footer`)
  present, and any new interactive element needs a visible label or `aria-label`.
- **Coverage note**: `src/app/icon.tsx` (favicon, via `next/og`'s `ImageResponse`) is excluded from
  unit-test coverage in `vitest.config.ts` because jsdom cannot render it — verify icon changes via a
  real build or Playwright instead of chasing unit coverage there.

## Reference

- [Content Quality Principles](../../../repo-governance/conventions/writing/quality.md)
- `docs-applying-content-quality` Skill for universal markdown/content rules (this Skill only covers
  what's specific to `baseerah-fe`'s JSX content surface)
- `specs/apps/baseerah/behavior/baseerah-fe/gherkin/hello/landing-page.feature` — the Gherkin
  scenarios this content must keep satisfying
