# Reporter/Tooling Cross-Reference — ose-infra

Short confirmation file. Full research on Jest/Vitest, Playwright, Cargo `#[ignore]`, and cucumber-rs
reporter/skip-detection tooling is done once, centrally, in the ose-public audit (same tools, shared
ecosystem across all 3 repos) — not re-researched here.

## Playwright `forbidOnly` — `coralpolyp-fe-e2e`

Confirmed present. `apps/coralpolyp-fe-e2e/playwright.config.ts:16`:

```ts
forbidOnly: !!process.env.CI,
```

Also confirmed present (asked to check as a bonus since it's the paired e2e suite) in
`apps/coralpolyp-be-e2e/playwright.config.ts:13`, same setting. Both configs additionally set
`fullyParallel: false` and `workers: 1` (shared-DB / sequential-scenario constraints), and both use
`playwright-bdd`'s `defineBddConfig` pointed at the shared coralpolyp Gherkin trees
(`specs/apps/coralpolyp/behavior/coralpolyp-be/gherkin` and `.../coralpolyp-web/gherkin`
respectively).

## Cargo `#[ignore]`-detection tooling — `coralpolyp-be`

Confirmed **absent**. Searched:

- `apps/coralpolyp-be/deny.toml` — exists, but scopes `cargo-deny` to license/advisory/dependency-ban
  policy only; no test-ignore rule.
- `.github/workflows/*.yml` — no step greps for `#[ignore]` or fails a build on its presence.
- No custom script (`scripts/`, `apps/rhino-cli`) implements `#[ignore]`-detection for
  `coralpolyp-be`'s own test suite.

This is consistent with `03-skip-inventory-infra.md`'s finding of 0 `#[ignore]` occurrences today —
there is nothing to catch yet, but also no gate that would catch one if introduced. rhino-cli's own
`.fail_on_skipped()` (used across its 5 cucumber test binaries) is a **cucumber-only** mechanism and
does not apply to `coralpolyp-be`'s plain `#[test]` surface if one is ever added alongside its
current cucumber-only `tests/unit`/`tests/integration` binaries.

## Summary

| Check                                                         | Status                              |
| ------------------------------------------------------------- | ----------------------------------- |
| `coralpolyp-fe-e2e` Playwright `forbidOnly: !!process.env.CI` | Present (`playwright.config.ts:16`) |
| `coralpolyp-be-e2e` Playwright `forbidOnly: !!process.env.CI` | Present (`playwright.config.ts:13`) |
| `coralpolyp-be` Cargo `#[ignore]`-detection tooling           | Absent — no gate exists             |
