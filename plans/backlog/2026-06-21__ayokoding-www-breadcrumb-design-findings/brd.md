# Business Requirements — AyoKoding Breadcrumb Design Findings

## Business Context

AyoKoding (`ayokoding.com`) is a bilingual educational platform serving readers in English and
Indonesian. The cost-of-living calculator was recently enhanced with URL-state persistence and a
new breadcrumb navigation component (`CalculatorBreadcrumb`). The breadcrumb is the first
persistent in-page navigation element a visitor encounters on the calculator page, and it appears
identically at all three breakpoints.

## Who Is Affected

- **Indonesian-language visitors** — the breadcrumb currently displays "Home / Tools / Calculator"
  in English even on `/id/` pages, breaking the bilingual experience and appearing as a localization
  defect to any Indonesian reader.
- **Brand-conscious visitors** — the plain "/" separator and the duplicated breadcrumb
  implementation present a less polished visual than the chevron-icon separator used by the rest of
  the site's navigation components.
- **Future contributors** — two competing breadcrumb implementations in the same codebase increase
  maintenance surface and risk divergence.

## Cost of Leaving Unfixed

The localization failure (DWT-B-001) is the most visible issue: Indonesian visitors see English
labels in a component that exists explicitly to help them navigate. If the calculator gains more
prominence (shared links, SEO), this becomes a credibility gap for the Indonesian locale.

The bespoke-vs-shared component split (DWT-B-004) increases long-term cost: future breadcrumb
design changes must be applied to two separate files instead of one, risking drift. The missing
`flex-wrap` (DWT-B-002) is a latent reflow risk — currently safe with short English labels but
will overflow as soon as translated labels are applied.

## Business-Level Success Metrics

- Zero breadcrumb labels rendered in English on the Indonesian locale (`/id/` prefix) — verified
  by Playwright screenshot and `html[lang]` assertion.
- All breadcrumb separator and layout behaviour consistent with the project-wide breadcrumb
  component — no visually divergent breadcrumb implementations on the same site.
- No responsive overflow at 375 px for either locale after localization is applied.
- WCAG AA contrast maintained (current 4.74:1 in light mode is compliant; fix must not regress
  below 4.5:1).

## Business Risks

- **Localization defect visible to all Indonesian visitors** — DWT-B-001 is immediately observable
  to any bilingual user and signals incomplete translation.
- **Regression risk during localization** — applying Indonesian text without also adding `flex-wrap`
  (DWT-B-002) could cause a 375 px overflow in a future commit.
- **Divergent breadcrumb implementations** — DWT-B-004 fragments the design language and increases
  the maintenance surface for breadcrumb styling.
