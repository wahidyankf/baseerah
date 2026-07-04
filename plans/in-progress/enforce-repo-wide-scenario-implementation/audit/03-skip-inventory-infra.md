# Skip / Pending / Ignore Marker Inventory — ose-infra

Scope: the non-cucumber test suites that could carry skip/pending markers as new risk surface for this
plan. rhino-cli's own cucumber suite already enforces `.fail_on_skipped()` across all 5 of its test
binaries (`tests/{git_hooks,docs,spec_coverage,convention,agents}.rs`) — that is pre-existing,
out of scope here, and not re-audited.

## Jest/Vitest `.skip(` / `.only(` / `.todo(` — `coralpolyp-fe`, `ts-ui`, `ts-ui-tokens`

Command: `git grep -n -E '\.(skip|only|todo)\(' -- apps/coralpolyp-fe libs/ts-ui libs/ts-ui-tokens`
(also checked broader `xit(`/`xdescribe(`/`it.skip`/`describe.skip` variants).

| Project         | `.skip(` | `.only(` | `.todo(` | `xit`/`xdescribe` |
| --------------- | -------- | -------- | -------- | ----------------- |
| `coralpolyp-fe` | 0        | 0        | 0        | 0                 |
| `ts-ui`         | 0        | 0        | 0        | 0                 |
| `ts-ui-tokens`  | 0        | 0        | 0        | 0                 |

**Zero-count finding**: none of the three Vitest-based projects contain any skip/pending/only marker
today. This is a clean baseline — there is nothing to remediate, but also no existing tooling gate
that would catch one if introduced (see "Existing tooling" below).

## Playwright `test.skip(` / `.only(` / `.fixme(` / `test.describe.skip` — `coralpolyp-fe-e2e`, `coralpolyp-be-e2e`

Command: `git grep -n -E 'test\.(skip|only|fixme)\(|test\.describe\.skip' -- apps/coralpolyp-fe-e2e
apps/coralpolyp-be-e2e`.

| Project             | `test.skip(` | `test.only(` | `test.fixme(` | `test.describe.skip` |
| ------------------- | ------------ | ------------ | ------------- | -------------------- |
| `coralpolyp-fe-e2e` | 0            | 0            | 0             | 0                    |
| `coralpolyp-be-e2e` | 0            | 0            | 0             | 0                    |

**Zero-count finding**: no skip markers in either Playwright suite. Both suites are `playwright-bdd`
projects (`.feature` files compiled to tests via `bddgen`), so per-scenario skips would most likely
show up as a `@skip` Gherkin tag rather than inline `test.skip()` — a separate `git grep` for `@skip`
across both projects' `.feature` files (via the shared `specs/apps/coralpolyp/behavior/coralpolyp-be`
and `coralpolyp-web` glob) also returns zero.

## Cargo `#[ignore]` — `coralpolyp-be`

Command: `git grep -n '#\[ignore' -- apps/coralpolyp-be` (also a broader case-insensitive `ignore`
sweep over `apps/coralpolyp-be/**/*.rs`, excluding `generated-contracts/`).

**Result: 0 occurrences.** `coralpolyp-be` has no regular `#[test]`/`#[ignore]`-marked unit tests to
begin with in the conventional sense — its "unit" test target (`cargo test --lib --test unit`) is
itself a cucumber-rs (`harness = false`) binary driven by the shared `.feature` file via
Given/When/Then step attributes (see `01-scenario-census-infra.md` §3-4 and `04-vacuity-infra.md`),
not a tree of individually-ignorable `#[test] fn ...() {}` functions. There is no `#[ignore]` usage
anywhere in the crate today.

## Summary table

| Category                                             | Count | Zero-count? |
| ---------------------------------------------------- | ----- | ----------- |
| Jest/Vitest `.skip`/`.only`/`.todo` (3 projects)     | 0     | Yes         |
| Playwright `test.skip`/`.only`/`.fixme` (2 projects) | 0     | Yes         |
| Gherkin `@skip` tag (coralpolyp `.feature` files)    | 0     | Yes         |
| Cargo `#[ignore]` (`coralpolyp-be`)                  | 0     | Yes         |

## Existing guard-rail context (see `05-reporters-infra.md` for full detail)

Both Playwright configs (`apps/coralpolyp-be-e2e/playwright.config.ts:13`,
`apps/coralpolyp-fe-e2e/playwright.config.ts:16`) already set `forbidOnly: !!process.env.CI`, which
would fail CI if a `.only(` were ever introduced — this is a real, load-bearing guard already in
place for the two Playwright suites. No equivalent guard exists for Vitest `.only(`/`.skip(` in
`coralpolyp-fe`/`ts-ui`/`ts-ui-tokens`, and no tooling detects Cargo `#[ignore]` in `coralpolyp-be`
(confirmed by grepping `.github/workflows/*.yml` and the crate's `deny.toml`, which covers
license/dependency policy only, not test-ignore detection).
