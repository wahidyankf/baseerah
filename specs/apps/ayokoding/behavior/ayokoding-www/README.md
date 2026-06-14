# ayokoding-www — Behavior Surface

UI-semantic Gherkin scenarios for the AyoKoding browser UI surface (Next.js 16).

## Contents

- **[gherkin/](./gherkin/README.md)** — Feature files organized by bounded context domain.
  Consumed by `apps/ayokoding-www-fe-e2e` (Playwright FE E2E).

## Background step

All scenarios use: `Given the app is running`

## Domains

- **app-shell/** — Responsive layout + accessibility chrome
- **content/** — Article and content-list rendering
- **search/** — Search dialog and results
- **i18n/** — Locale switcher (English / Indonesian)
- **navigation/** — Top-level navigation, sidebar, breadcrumb

## Related

- [Parent behavior README](../README.md)
- [Gherkin specs](./gherkin/README.md)
