# organiclever-www-fe-e2e

End-to-end frontend tests for [`apps/organiclever-www`](../organiclever-www/) using
Playwright and `playwright-bdd`. Consumes the Gherkin feature files at
`specs/apps/organiclever/behavior/organiclever-www/gherkin/`.

## Commands

```bash
# Install Chromium for Playwright
nx run organiclever-www-fe-e2e:install

# Run all E2E scenarios headlessly (Playwright starts the production server)
nx run organiclever-www-fe-e2e:test:e2e

# Run with Playwright UI
nx run organiclever-www-fe-e2e:test:e2e:ui

# View last run report
nx run organiclever-www-fe-e2e:test:e2e:report

# Pre-push quick gate (typecheck + lint; e2e runs nightly / on demand)
nx run organiclever-www-fe-e2e:test:quick
```

## Features consumed

- `home/home.feature` — marketing landing page rendering
- `accessibility/accessibility.feature` — WCAG AA compliance

## Default base URL

`http://localhost:3200` — override with `BASE_URL` env var for staging /
production smoke runs.
