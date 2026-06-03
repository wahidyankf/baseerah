# Product Requirements Document — wahidyankf-web SSR + SEO Refactor

## Product Overview

Refactor three Next.js page routes (`/`, `/cv`, `/personal-projects`) in `wahidyankf-web` so that
search-param–driven content is resolved on the server and passed as props to client components.
This eliminates the `useSearchParams` hook from all content components, removes the mandatory
`<Suspense>` wrappers, and makes the routes dynamically server-rendered (SSR) so web crawlers
receive full HTML.

The refactor also adds per-page `export const metadata` on `/cv` and `/personal-projects` for
page-specific SEO titles and descriptions, and updates the infrastructure (Dockerfile, next.config.ts)
to match the Vercel-native deployment model.

## Personas

- **Portfolio visitor / recruiter** — searches for the owner's name or skills; expects full page
  content to load, be indexed, and appear in search results with accurate titles.
- **Owner / developer** — wants robust SSR-first pages without sacrificing client-side search
  interactivity.
- **Search-engine crawler** — receives the server response; must see full HTML (headings, content,
  metadata) on the first HTTP response, with no `Loading...` placeholder.

## User Stories

1. As a **search-engine crawler**, I want to receive the full page HTML for `/`, `/cv`, and
   `/personal-projects` on the first HTTP response, so that the content is indexable.
2. As a **portfolio visitor**, I want to navigate to `/cv?search=TypeScript` and see the CV
   filtered immediately (no flash of `Loading...`), so that deep-linked URLs are shareable and
   bookmarkable.
3. As a **recruiter** sharing a link to `/cv` on LinkedIn, I want the link preview to show
   the title "Curriculum Vitae — Wahidyan Kresna Fridayoka" and a page-specific description,
   so that the preview is informative.
4. As a **portfolio visitor** clicking a skill pill on the Home page, I want to be taken to
   `/cv?search=React&scrollTop=true` and land at the top of the CV page with the search term
   pre-applied, so that the cross-page navigation feels seamless.
5. As the **owner / developer**, I want the CI Docker image to build and start without `output:
"standalone"`, so that the Docker configuration matches the Vercel-native runtime.

## Acceptance Criteria (Gherkin)

### Scenario 1: Home page renders full HTML on the server

```gherkin
Scenario: Home page delivers full HTML to a crawler
  Given the wahidyankf-web Next.js app is running
  When an HTTP GET request is made to "http://localhost:3201/"
  Then the response body contains the text "Welcome to My Portfolio"
  And the response body does not contain the text "Loading..."
  And the response status is 200
```

### Scenario 2: CV page renders full HTML on the server

```gherkin
Scenario: CV page delivers full HTML to a crawler
  Given the wahidyankf-web Next.js app is running
  When an HTTP GET request is made to "http://localhost:3201/cv"
  Then the response body contains the text "Curriculum Vitae"
  And the response body does not contain the text "Loading..."
  And the response status is 200
```

### Scenario 3: Personal Projects page renders full HTML on the server

```gherkin
Scenario: Personal Projects page delivers full HTML to a crawler
  Given the wahidyankf-web Next.js app is running
  When an HTTP GET request is made to "http://localhost:3201/personal-projects"
  Then the response body contains the text "Personal Projects"
  And the response body does not contain the text "Loading..."
  And the response status is 200
```

### Scenario 4: Search term from URL is applied on first render

```gherkin
Scenario: CV page pre-populates search from URL query parameter
  Given the wahidyankf-web Next.js app is running
  When a browser navigates to "http://localhost:3201/cv?search=TypeScript"
  Then the search input contains "TypeScript"
  And CV entries matching "TypeScript" are visible
  And the page does not show a "Loading..." fallback at any point during load
```

### Scenario 5: CV page per-page metadata is present

```gherkin
Scenario: CV page has a page-specific title tag
  Given the wahidyankf-web Next.js app is running
  When an HTTP GET request is made to "http://localhost:3201/cv"
  Then the response HTML contains a <title> tag with "CV" in it
  And the response HTML contains a <meta name="description"> tag specific to the CV page
```

### Scenario 6: Personal Projects page per-page metadata is present

```gherkin
Scenario: Personal Projects page has a page-specific title tag
  Given the wahidyankf-web Next.js app is running
  When an HTTP GET request is made to "http://localhost:3201/personal-projects"
  Then the response HTML contains a <title> tag with "Personal Projects" in it
  And the response HTML contains a <meta name="description"> tag specific to the Personal Projects page
```

### Scenario 7: Cross-page navigation from skill pill lands at top of CV with search applied

```gherkin
Scenario: Clicking a skill pill on Home navigates to CV with search pre-applied at top
  Given a browser is on "http://localhost:3201/"
  When the user clicks a skill pill (e.g. "TypeScript")
  Then the browser navigates to "/cv?search=TypeScript&scrollTop=true"
  And the CV page is scrolled to the top
  And the search input contains "TypeScript"
  And the "scrollTop" query parameter is removed from the URL after landing
```

### Scenario 8: Client-side search interactivity still works after SSR

```gherkin
Scenario: User types a new search term in the browser after SSR load
  Given a browser has loaded "http://localhost:3201/cv"
  When the user types "React" into the search input
  Then the CV entries are filtered to show only "React"-matching entries
  And the URL updates to "/cv?search=React"
  And no full page reload occurs
```

## Product Scope

### In Scope

- SSR via `searchParams` prop on all three `page.tsx` server components
- Removing `useSearchParams` from `HomeContent`, `CvContent`, and `PersonalProjectsContent`
- Removing `<Suspense fallback={<div>Loading...</div>}>` wrappers from all three page files and
  from the internal `ProjectsContent` wrapper inside `PersonalProjectsContent.tsx`
- Adding `"use client"` directive to `Navigation.tsx`
- Adding `export const metadata` to `/cv/page.tsx` and `/personal-projects/page.tsx`
- Updating unit tests for all changed components
- Removing `output: "standalone"` from `next.config.ts`
- Updating the Dockerfile final stage to use `next start` instead of standalone `server.js`

### Out of Scope

- Structured data / JSON-LD markup
- Sitemap or `robots.txt`
- Layout-level metadata changes in `layout.tsx`
- Changes to the search feature logic (`search.ts`, `filterItems`, `SearchComponent`)
- Changes to any app outside `wahidyankf-web`
- Performance optimization beyond what SSR naturally provides

## Product-Level Risks

| Risk                                                                                      | Impact | Notes                                                                                               |
| ----------------------------------------------------------------------------------------- | ------ | --------------------------------------------------------------------------------------------------- |
| `useEffect` sync `[initialSearchTerm]` dependency creates a re-render on every navigation | Low    | Expected — this is the canonical Next.js SSR + client-state sync pattern; the flicker is negligible |
| `scrollTop` prop `useEffect` fires on every render if not guarded                         | Low    | Guard with `if (scrollTop)` check; `router.replace` removes the param so the effect fires only once |
| Unit tests that mock `useSearchParams` need to be updated to pass props directly          | Low    | Covered in Phase 2 delivery steps; tests become simpler after the refactor                          |
