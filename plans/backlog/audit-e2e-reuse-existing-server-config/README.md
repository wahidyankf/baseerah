# Audit `reuseExistingServer` Across `*-e2e` Playwright Configs

> **Status**: Backlog (not started). Filed from a Knowledge Capture learning surfaced during
> `ayokoding-www-tools-ai-benchmark`'s Phase 10 Rule-15 retest.

## Context

While running a scoped `ayokoding-www-fe-e2e` subset, a long-lived `next dev` process (started
hours earlier, before that session's code changes) was already listening on the app's port.
Playwright's `reuseExistingServer: true` found it and skipped running the configured
`webServer.command` entirely — so the e2e run silently exercised stale dev-mode code instead of the
production build the config actually specifies (`NODE_ENV: "production"`, a standalone server,
e2e-specific env vars such as `AYOKODING_WEB_MANIFESTS_DIR`). A later full e2e run against the same
stale server produced a wall of unrelated-looking failures, all traced back to the reused server
never having the e2e fixture manifests directory wired in.

A repo-wide grep of every `*-e2e` project's `playwright.config.ts` (2026-07-30) showed the setting
hardcoded `true` unconditionally in six configs, not gated on `!process.env.CI`:
`apps/ayokoding-www-fe-e2e`, `apps/ayokoding-www-be-e2e`, `apps/organiclever-www-fe-e2e`,
`apps/wahidyankf-www-fe-e2e`, `apps/ose-www-fe-e2e`, `apps/ose-www-be-e2e` — plus one config that
already gated it correctly, `apps/organiclever-app-web-e2e` (`reuseExistingServer:
!process.env.CI`). **All seven of those apps were deleted from this repo by the Baseerah repo
reset** (see `plans/in-progress/baseerah-repo-reset/`), so none of the originally-listed files
exist anymore.

**Rescoped 2026-07-31**: a fresh repo-wide search for `playwright.config.ts` now finds exactly one
surviving config with the same unconditional-`true` pattern:

- `libs/web-ui/e2e/playwright.config.ts` (line 19: `reuseExistingServer: true`, not gated on
  `!process.env.CI`)

The reference "already gates it correctly" example (`organiclever-app-web-e2e`) no longer exists
either; the correct pattern to match is simply `reuseExistingServer: !process.env.CI`.

This is a Playwright-documented, common local-dev convenience setting: it silently reuses ANY
process already bound to the target port, including one from an unrelated earlier session/purpose,
with no warning that the configured `webServer.command` (and its env vars) was skipped.

## Scope

**In scope**: `libs/web-ui/e2e/playwright.config.ts` (the sole surviving offender); a decision on
whether `reuseExistingServer` needs a CI-conditional gate, doc caveat, or automated check.

**Out of scope**: any change to the e2e test scenarios themselves; the already-fixed
`ayokoding-www-tools-ai-benchmark` incident this was filed from; the six deleted apps' configs,
which are moot now that the apps are gone.

## Navigation

- [brd.md](./brd.md) — WHY: business rationale, impact, risk.
- [prd.md](./prd.md) — WHAT: user story, Gherkin acceptance criteria, product scope.
- [tech-docs.md](./tech-docs.md) — HOW: the proposed investigation and the open remedy decision the
  investigation phase resolves.
- [delivery.md](./delivery.md) — DO: phased, gated delivery checklist, quality gates, verification.
- [learnings.md](./learnings.md) — Knowledge Capture running log for this plan's own execution.

## Delivery Mode

`worktree-to-pr` (the repo default) — this is a config/tooling change spanning multiple `apps/`
projects, so it is filed as its own plan per the code-homed-learnings-are-never-landed-inline rule
rather than folded into any single app's plan.
