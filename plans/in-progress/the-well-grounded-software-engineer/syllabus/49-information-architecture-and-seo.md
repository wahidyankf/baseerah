# 49 · Information Architecture & SEO (Annotated-concept, ‡ HTML †)

**prd row**: Pass 3 · Build for the Real World · Annotated-concept · ‡ HTML † · Learn 149 / Drill 249 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: making content findable and legible to both machines and people — taxonomy and URL
design, semantic HTML, sitemaps, structured data (schema.org), and Core Web Vitals as an SEO signal.
An annotated-concept topic: the deliverables are markup and structure, exercised against real
crawler/validation tooling rather than an application build. `‡ HTML †`: the worked artifacts are
semantic HTML and structured-data markup.

## Why this exists · the big idea

- **The problem before the solution**: content that humans can read is often invisible to machines —
  a beautiful page with `<div>` soup, opaque URLs, and no structured data can't be crawled, ranked,
  or understood by a search engine or assistive tech, so it doesn't get found. "Looks fine in a
  browser" is not the same as "legible to the systems that decide who sees it".
- **Keep-this-if-you-forget-everything**: structure _is_ meaning — a clear taxonomy, honest URLs,
  semantic HTML, and explicit structured data let machines and people navigate the same content, and
  findability follows from that shared structure, not from keyword tricks.
- **Big ideas touched**: `coupling-vs-cohesion` (a good information architecture groups what belongs
  together and separates what doesn't, so navigation and URLs stay stable as content grows),
  `layering-and-leaks` (the same page is read by a browser, a crawler, and a screen reader — semantic
  markup is the layer that serves all three, and where it's missing, each consumer's view diverges).

## Prerequisites

- **Prior topics**: [topic 14 Frontend Essentials](./14-frontend-essentials.md) (HTML semantics,
  accessibility, the document outline) and [topic 47 Advanced Frontend](./47-advanced-frontend.md)
  (rendering models — CSR/SSR/SSG — and their crawlability and performance consequences).
- **Tools & environment**: a macOS/Linux terminal; a browser with Lighthouse/DevTools; a structured-
  data validator (Schema.org / Rich Results test) and a Core Web Vitals measurement tool; the ability
  to serve static HTML locally; Neovim/VSCode with an HTML LSP (DD-17).
- **Assumed knowledge**: writing semantic HTML and understanding accessibility roles (topic 14); how
  server- vs client-rendering affects what a crawler sees (topic 47); serving files from the CLI
  (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: **Core Web Vitals** remain a Google ranking signal and the metric set is left
  correctly version-unpinned — the specific thresholds and the constituent metrics (notably the LCP/
  CLS pair and the interaction-responsiveness metric that replaced First Input Delay) shift over time,
  so measure with current tooling rather than hard-coding numbers.
- 2026-07-12 — verified: **schema.org** structured data (JSON-LD as the recommended embedding) and
  **sitemaps.org** XML sitemaps are stable, standard, and left unpinned. Validate against the current
  Rich Results / structured-data test at drafting time, as eligible rich-result types change.

## Items

- Information architecture fundamentals: taxonomy, hierarchy, labeling, and navigation that match how
  users actually look for things (findability, information scent).
- URL design: readable, stable, hierarchical URLs; canonicalization; and why URLs are a long-lived
  public contract, not an implementation detail.
- Semantic HTML and the document outline: landmarks, headings, and roles that serve browsers,
  crawlers, and assistive tech from one markup source.
- Machine legibility: XML sitemaps, `robots.txt`, canonical tags, and metadata (title/description,
  Open Graph) that tell crawlers what exists and what's authoritative.
- Structured data: schema.org vocabularies embedded as JSON-LD, and validating for rich-result
  eligibility.
- Core Web Vitals as an SEO signal: measuring loading, interactivity, and layout stability, and the
  rendering choices (topic 47) that move them.

## Tensions & trade-offs — when NOT to reach for this

- **SEO is not content quality**: perfect markup on thin, unhelpful content still loses. Structured
  data and Core Web Vitals are amplifiers, not substitutes — chasing them for a page nobody wants is
  effort spent on the wrong layer.
- **Structured data has a maintenance and honesty cost**: schema.org markup that drifts from the
  visible page is worse than none — it risks manual penalties and misleads users. Only mark up what's
  genuinely on the page, and only the types you'll keep accurate.
- **Not every surface needs it**: an internal admin tool, a gated app, or an authenticated dashboard
  gains nothing from sitemaps, rich results, or crawl optimization. IA discipline still helps
  navigation, but the SEO machinery is for public, discoverable content only.

## Lineage — why it beat the alternative

- Information architecture matured as library-science practice for the web (the "polar bear book"),
  and early SEO was an arms race of keyword stuffing and link tricks. Search engines responded by
  rewarding what actually helps users: semantic structure, honest metadata, explicit structured data,
  and fast, stable pages. The durable approach won because it aligns the machine's incentives with the
  reader's — structure content well and both crawlers and people benefit, with no penalty risk. This
  hands well-structured, crawlable, performant public surfaces to the operational concerns of
  [topic 47 Advanced Frontend](./47-advanced-frontend.md) (rendering for crawlability and vitals) and
  builds directly on the semantic-HTML foundation of
  [topic 14 Frontend Essentials](./14-frontend-essentials.md).

## Worked examples

Colocated under `information-architecture-and-seo/learning/code/`; each artifact is real markup served
locally and checked against crawler/validation tooling (DD-20/DD-30).

- **beginner** — take a `<div>`-soup page and restructure it into semantic HTML with a correct heading
  outline and landmarks; verify the accessibility/document outline in DevTools.
- **intermediate** — design a small site's taxonomy and URL scheme, add an XML sitemap, `robots.txt`,
  canonical tags, and title/description/Open Graph metadata; verify a crawler sees the intended
  structure.
- **advanced** — add schema.org JSON-LD structured data for a content type and measure Core Web Vitals;
  verify rich-result eligibility in the validator and a passing vitals measurement.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: take a small multi-page site and make it machine-legible end to end — a coherent taxonomy
  and URL scheme, semantic HTML with a correct outline, an XML sitemap + `robots.txt` + canonical
  tags, schema.org JSON-LD for its main content type, and a measured Core Web Vitals pass — validated
  with real tooling.
- **Concepts exercised**: [ ] taxonomy + URL scheme [ ] semantic HTML + document outline [ ] sitemap +
  robots + canonical [ ] title/description/Open Graph metadata [ ] schema.org JSON-LD [ ] Core Web
  Vitals measurement.
- **Ordered steps**:
  1. `.../learning/capstone/site/` — restructure pages into semantic HTML with a correct heading
     outline and landmarks. Verify the document outline and accessibility tree in DevTools.
  2. `.../learning/capstone/site/sitemap.xml` + `robots.txt` + canonical tags — declare structure and
     authority. Verify a crawler/validator reads the sitemap and honors canonicals.
  3. `.../learning/capstone/site/structured-data.html` — schema.org JSON-LD for the main content type.
     Verify it passes the structured-data / rich-results validator with no errors.
  4. Measure Core Web Vitals with current tooling and record the result. Verify the primary metrics
     pass and note which rendering choice moved them.
- **Acceptance criteria**: markup is semantic with a correct outline; sitemap/robots/canonicals are
  valid and consistent; JSON-LD validates and matches the visible content; Core Web Vitals pass under
  current tooling.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Information Architecture for the Web and Beyond** — Louis Rosenfeld, Peter Morville, Jorge Arango
  (4th ed., 2015). The standard reference text for information architecture, often called "the polar
  bear book".
- **Don't Make Me Think, Revisited** — Steve Krug (3rd ed., 2014). The classic, widely read primer on
  usability and navigation design.

**Papers & articles**

- **Google Search Central Documentation** — Google (ongoing). The authoritative technical SEO reference
  maintained directly by the search engine it documents. <https://developers.google.com/search/docs>
- **Information Architecture: Study Guide** — Nielsen Norman Group (ongoing). Widely cited
  practitioner-research hub on IA fundamentals, navigation, and findability research methods.
  <https://www.nngroup.com/articles/ia-study-guide/>

---

← Previous: [48 · Build Your Own Reactive UI](./48-build-your-own-reactive-ui.md) · Next: [50 · Containers & Orchestration](./50-containers-and-orchestration.md) →
