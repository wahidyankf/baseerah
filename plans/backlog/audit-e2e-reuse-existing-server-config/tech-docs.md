# Technical Design: Audit `reuseExistingServer` Across `*-e2e` Playwright Configs

## Proposed Investigation

- Confirm whether the CI runner for `libs/web-ui/e2e/playwright.config.ts` (the sole surviving
  hardcoded-`true` config; the plan's originally-cited six configs were deleted along with their
  apps by the Baseerah repo reset) ever actually risks a port collision (fresh runner per job vs. a
  shared/reused runner where a stray process could persist).
- If CI runners are ephemeral per job, the risk is local-development-only — the fix might be
  documentation (a caveat on developers running e2e locally) rather than a config change.
- If CI runners are shared/persistent (self-hosted runners are used in this repo — see
  `plans/done/2026-07-19__fundamentally-strong-software-engineer/learnings.md`
  ("OpenAPI Generator CLI jar download race is a second concurrency-flake class"),
  which records a shared-tool-cache concurrency race on a co-triggered self-hosted runner and
  generalizes it as "any tool with a shared local cache, invoked by 2+ workflows the same push
  fires concurrently"), `libs/web-ui/e2e/playwright.config.ts` should likely gain a
  `reuseExistingServer: !process.env.CI` gate.
- Consider whether a lightweight guard (e.g. a `ci-checker` rule, or a comment convention) should
  flag any new `*-e2e` Playwright config that sets `reuseExistingServer: true` unconditionally.
