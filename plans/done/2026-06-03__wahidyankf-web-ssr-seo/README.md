# wahidyankf-web SSR + SEO Refactor

Refactor `wahidyankf-web` pages to use server-side rendering (SSR) via Next.js 15+ `searchParams`
prop, eliminating client-side `useSearchParams` hooks and their mandatory `<Suspense>` wrappers.
Adds per-page `export const metadata` for `/cv` and `/personal-projects`, and removes the
`output: "standalone"` Dockerfile configuration that is sub-optimal for Vercel native deployments.

## Status

In Progress

## Scope

**In scope** (all changes are to `apps/wahidyankf-web/` and its supporting test/config files):

- `src/app/page.tsx` — add `searchParams` prop, remove `<Suspense>` wrapper
- `src/app/cv/page.tsx` — add `searchParams` prop, `export const metadata`, remove `<Suspense>` wrapper
- `src/app/personal-projects/page.tsx` — add `searchParams` prop, `export const metadata`, remove
  `<Suspense>` wrapper
- `src/features/app-shell/Navigation.tsx` — add explicit `"use client"` directive
- `src/features/home/HomeContent.tsx` — accept `initialSearchTerm` prop, remove `useSearchParams`
- `src/features/cv/CvContent.tsx` — accept `initialSearchTerm` + `scrollTop` props, remove
  `useSearchParams`
- `src/features/personal-projects/PersonalProjectsContent.tsx` — accept `initialSearchTerm` prop,
  remove `useSearchParams`, remove internal `<Suspense>`
- `apps/wahidyankf-web/next.config.ts` — remove `output: "standalone"`
- `apps/wahidyankf-web/Dockerfile` — update final stage to use `next start` instead of standalone
  `server.js`
- Unit test files for the changed components

**Out of scope:**

- Any changes outside `apps/wahidyankf-web/` (no changes to shared libs, other apps, or E2E tests
  beyond what is needed for the unit-test updates)
- SEO changes to `layout.tsx` (already has site-wide metadata)
- Performance tuning or adding ISR/caching
- Changing the search feature logic itself (`search.ts`, `filterItems`)

## Documents

- [Business Requirements](./brd.md) — WHY: SEO impact, crawlability rationale, business value
- [Product Requirements](./prd.md) — WHAT: user stories, Gherkin acceptance criteria, scope
- [Technical Documentation](./tech-docs.md) — HOW: architecture, design decisions, file impacts
- [Delivery Checklist](./delivery.md) — DO: phased implementation steps with TDD structure

## Definition of Done

- All 3 pages render full HTML without a `Loading...` fallback when crawled (server renders the
  page shell _and_ content in a single pass)
- Search interactivity (filter, highlight, URL sync) still works in the browser after SSR
- `output: "standalone"` removed from `next.config.ts`
- Dockerfile updated for standard `next start`
- `export const metadata` added for `/cv` and `/personal-projects`
- All unit tests pass (`npx nx run wahidyankf-web:test:quick` exits 0)
- CI E2E tests pass (`npx nx run wahidyankf-web-fe-e2e:test:e2e` exits 0 in CI)
