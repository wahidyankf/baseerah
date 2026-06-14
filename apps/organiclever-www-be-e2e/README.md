# organiclever-www-be-e2e

Backend E2E slot for [`apps/organiclever-www`](../organiclever-www/) using
Playwright and `playwright-bdd`.

`organiclever-www` is a pure Next.js static marketing site with no tRPC route
handlers or dedicated backend API. This project exists to satisfy the
standardized `{app}-be-e2e` + `{app}-fe-e2e` reusable workflow pair. The
`be-e2e` slot is tolerated-absent in CI (called with `|| true`).

## Commands

```bash
# Install Chromium for Playwright
nx run organiclever-www-be-e2e:install

# Run E2E (placeholder suite — no real scenarios)
nx run organiclever-www-be-e2e:test:e2e

# Pre-push quick gate (typecheck + lint)
nx run organiclever-www-be-e2e:test:quick
```

## Features consumed

- `specs/apps/organiclever/behavior/organiclever-www-be/gherkin/placeholder/placeholder.feature`
  — structural placeholder, no real backend scenarios

## Default base URL

`http://localhost:3200` — override with `BASE_URL` env var.
