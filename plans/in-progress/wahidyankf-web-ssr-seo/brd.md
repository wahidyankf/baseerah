# Business Requirements Document — wahidyankf-web SSR + SEO Refactor

## Business Goal

Improve the crawlability and search-engine indexability of `wahidyankf.com` so that the portfolio
pages (Home, CV, Personal Projects) appear in search results with accurate titles, descriptions,
and content rather than a blank `Loading...` shell.

## Business Impact

### Pain Points

The current architecture wraps page content in `<Suspense fallback={<div>Loading...</div>}>` because
`useSearchParams` (a client-side hook) requires a Suspense boundary. Web crawlers and social-media
link-preview bots do not execute JavaScript; they see only the fallback. Concrete effects:

- Google, LinkedIn, and other crawlers receive a page with a single `<div>Loading...</div>` body —
  no headings, no content, no structured data beyond what `layout.tsx` provides.
- LinkedIn post previews for the `/cv` and `/personal-projects` URLs will show only the generic
  site-level title and description, not page-specific titles.
- The portfolio owner (Wahidyan Kresna Fridayoka) cannot be reliably discovered by recruiters or
  potential collaborators who search for his name or skills.

### Expected Benefits

- All three pages return full HTML content on the first server response — crawlable and indexable
  immediately.
- `/cv` and `/personal-projects` get distinct `<title>` and `<meta description>` tags, enabling
  page-specific rich snippets.
- Removing `output: "standalone"` aligns the Docker image with how Vercel actually runs Next.js,
  reducing build complexity and potential divergence between CI and production.

### Business-Level Success Metrics

_Judgment call:_ the primary measurable outcome is "no `Loading...` fallback in rendered HTML when
the page is fetched with `curl -s https://wahidyankf.com/ | grep -i loading`". Secondary: Google
Search Console shows the three routes as indexable (no "Crawled — currently not indexed" with
empty content).

## Affected Roles

This is a solo-maintainer repository. The only role is the owner/maintainer (Wahidyan Kresna
Fridayoka), who wears multiple hats:

- **Portfolio owner** — benefits from improved discoverability
- **Developer** — performs the implementation
- **DevOps** — maintains CI/CD and Docker configuration

No sign-off ceremonies or stakeholder approvals are required.

## Business-Scope Non-Goals

- Improving page-load speed beyond what SSR naturally provides — out of scope.
- Adding structured data / JSON-LD schema markup — out of scope.
- Rewriting the search or filtering logic — out of scope.
- Changes to `layout.tsx` site-wide metadata — already in place, not touched.
- Adding a sitemap or `robots.txt` — out of scope.

## Business Risks and Mitigations

| Risk                                                                                                         | Likelihood | Impact | Mitigation                                                                                                    |
| ------------------------------------------------------------------------------------------------------------ | ---------- | ------ | ------------------------------------------------------------------------------------------------------------- |
| SSR `searchParams` opt-in changes route from static to dynamic, increasing Vercel serverless cold-start cost | Low        | Low    | Pages are low-traffic portfolio pages; dynamic rendering cost is negligible                                   |
| Removing `output: "standalone"` breaks the CI Docker image used by E2E tests                                 | Medium     | Medium | Dockerfile update is a required delivery step (Phase 3); E2E gate validates Docker before plan is marked done |
| `useEffect` sync of `initialSearchTerm` → `searchTerm` misses the update on navigation                       | Low        | Low    | Canonical Next.js pattern; covered by E2E tests that exercise search interactivity                            |
| Per-page `export const metadata` titles conflict with layout-level metadata                                  | Low        | Low    | Next.js page-level metadata merges with layout metadata; page title overrides layout title by design          |
