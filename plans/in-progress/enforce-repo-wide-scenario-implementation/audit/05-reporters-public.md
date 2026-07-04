# Reporter/Tooling Cross-Reference — ose-public

Central research file for all 3 repos (per `02-covers-adoption-infra.md`'s note: this ecosystem is
shared, so it is researched once here rather than re-derived per repo). Covers the 4 test tools
relevant to `ose-public`: cucumber-rs (rhino-cli), Vitest (unit, all 6 Next.js apps — no Jest is used
anywhere in this repo), Playwright (e2e, all 11 `*-e2e` apps), and xunit.v3 via `dotnet test`
(F# unit + integration, `organiclever-be`/`ose-be`/`crane-cli`/`fsharp-crane-core`). Every claim below
was verified directly (`--help` output from the exact pinned version, or official docs/crate metadata
fetched live) — none is asserted from training-data recall alone.

## 1. cucumber-rs 0.23.0 (pinned in `apps/rhino-cli/Cargo.toml:35`, `cucumber = "0.23.0"`, default

features only)

| Capability                            | Status                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Machine-readable report format        | **Exists but not enabled.** `writer::Json` (feature-gated `output-json`) and `writer::JUnit` (feature-gated `output-junit`) both exist (confirmed via `docs.rs/cucumber/0.23.0/cucumber/writer` and the crate's feature list). Neither `output-json` nor `output-junit` is a default feature, and `apps/rhino-cli/Cargo.toml` declares only `cucumber = "0.23.0"` with no `features = [...]` — so only the default `macros` feature is active today. **To unlock JSON output: add `features = ["output-json"]` (or `"output-junit"`) to the Cargo.toml dependency line.**                                                      |
| Skip/undefined/pending → fail the run | **Exists and is already in active use.** `writer::FailOnSkipped` "transforms skipped steps into failed steps" (confirmed via `docs.rs`). Exposed as the `.fail_on_skipped()` builder convenience method on `World::cucumber()`. **All 18 of rhino-cli's cucumber test binaries already call `.fail_on_skipped()`** (verified: `grep -c fail_on_skipped` across all 18 files in `apps/rhino-cli/tests/` returns `1` each) before `.run_and_exit(...)` — see `apps/rhino-cli/tests/spec_coverage.rs:78-83` for the exact pattern. This capability is core (not feature-gated) — it compiles and runs with only default features. |

## 2. Vitest 4.1.0 (pinned identically across all 6 Next.js apps' `package.json`)

Verified via `rtk proxy npx vitest --help` (bypassing the `rtk` output filter to get the raw CLI help)
from `apps/ayokoding-www`.

| Capability                     | Status                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Machine-readable report format | **Yes.** `--reporter <name>` accepts `json` directly (also `junit`, `tap`, `tap-flat`, `blob`, among others), paired with `--outputFile <path>` to write it to disk. Example: `vitest run --reporter=json --outputFile=results.json`.                                                                                                                                                                                                                                                                          |
| Skip/todo → fail the run       | **No built-in flag.** The only related flag is `--allowOnly` ("Allow tests and suites that are marked as only", default `!process.env.CI`) — that governs `.only()`, not `.skip()`/`.todo()`. No flag in the full `--help` output converts a skipped/todo test into a failure. **Would require a custom guard**: either a static grep (as done in `03-skip-inventory-public.md`) or a script that parses the `--reporter=json` output for any result with `"state":"skip"`/`"mode":"skip"` and exits non-zero. |

## 3. Playwright 1.60.0 (pinned identically across all 11 `*-e2e` apps' `package.json`)

Verified via `rtk proxy npx playwright test --help` from `apps/organiclever-app-web-e2e`.

| Capability                              | Status                                                                                                                                                                                                                                                                                                                                                                                                |
| --------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Machine-readable report format          | **Yes.** `--reporter <reporter>` accepts `json` directly (also `junit`, `html`, `blob`, comma-separated for multiple).                                                                                                                                                                                                                                                                                |
| Skip/fixme → fail the run               | **No built-in flag.** `--forbid-only` ("Fail if test.only is called") is the closest related flag, but it only governs `.only()`. No flag fails the run because a test was skipped or marked `.fixme()`. **Would require a custom guard**: parse the JSON reporter's output for any test with `"status":"skipped"` and exit non-zero, or a repo-wide static grep as in `03-skip-inventory-public.md`. |
| Existing partial guard already in place | All 11 `*-e2e` apps' `playwright.config.ts` set `forbidOnly: !!process.env.CI` (verified by direct `grep -rn forbidOnly apps/*-e2e/playwright.config.ts` — 11/11 hits, one per app). This is a real, load-bearing guard against accidental `.only()` in CI today, but it does nothing for `.skip()`/`.fixme()`.                                                                                       |

## 4. xunit.v3 3.2.2 (pinned identically in `organiclever-be`/`ose-be`'s `.fsproj` files via

`xunit.v3` + `xunit.runner.visualstudio`; `crane-cli`/`fsharp-crane-core` also on F#/xunit)

Verified via `dotnet test --help` (.NET SDK 10.0.300) and `xunit.net`'s official v3 getting-started
documentation (fetched live).

| Capability                     | Status                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Machine-readable report format | **Yes.** `dotnet test --logger trx` (or `--logger "trx;LogFileName=<name>.trx"`) produces the standard VSTest TRX (XML) format — documented directly in `dotnet test --help`'s `-l, --logger` section. This is a `dotnet test`-level (VSTest) capability, available regardless of the test framework underneath.                                                                                                                                                                                                                                                                                                                           |
| Skip/"Not Run" → fail the run  | **No.** Per xunit.net's own v3 getting-started docs: there is no CLI flag, `xunit.runner.json` setting, or reporter option that converts a `[Fact(Skip = "...")]`/`[Theory(Skip = "...")]`-skipped test (or a "Not Run" result) into a failure. A `dotnet test` run's summary line reports Skipped/Not-Run counts separately from Failed, and the process exit code is driven by the Failed count only. **Would require a custom guard**: a post-processing script that parses the TRX output for `outcome="NotExecuted"` (or greps source for `Skip\s*=\s*"` as in `03-skip-inventory-public.md`) and fails the pipeline if any is found. |

## Cross-tool summary

| Tool        | Version | JSON/TRX report                                          | Skip → fail mechanism                                                            | Currently wired in this repo?                                                             |
| ----------- | ------- | -------------------------------------------------------- | -------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| cucumber-rs | 0.23.0  | Exists, not enabled (needs `features = ["output-json"]`) | Exists, **already used** (`.fail_on_skipped()`)                                  | Skip-fail: yes, all 18 rhino-cli binaries. JSON: no.                                      |
| Vitest      | 4.1.0   | Yes (`--reporter=json`)                                  | None — custom guard required                                                     | Neither wired today (no reporter config found in any `vitest.config.ts`)                  |
| Playwright  | 1.60.0  | Yes (`--reporter=json`)                                  | None — custom guard required (only `--forbid-only` for `.only()`, already wired) | `forbidOnly` wired (11/11); skip-fail not wired (none exists)                             |
| xunit.v3    | 3.2.2   | Yes (`--logger trx`)                                     | None — custom guard required                                                     | Neither wired today (`dotnet test` invoked with no `--logger` flag in any `project.json`) |

## Key finding

Every one of the 4 tools **can** produce a machine-readable report (JSON for cucumber-rs/Vitest/
Playwright once the right flag/feature is enabled, TRX for xunit natively via `dotnet test --logger
trx`). Only **one** of the 4 — cucumber-rs — has a built-in, already-active mechanism to make a
skipped/pending step fail the run (`.fail_on_skipped()`, live in all 18 rhino-cli binaries today).
Vitest, Playwright, and xunit.v3 have **no equivalent native flag**; enforcing "no skip may exist" for
these three tools repo-wide (as this plan intends to generalize) will require a custom guard — most
practically a small script/rhino-cli subcommand that either (a) statically greps for
`.skip(`/`.only(`/`.todo(`/`test.skip(`/`Skip\s*=\s*"` (the exact approach `03-skip-inventory-public.md`
used for this audit, which is currently the _only_ mechanism catching this repo-wide), or (b) parses
each tool's JSON/TRX report for a skip/pending status and fails the pipeline. Building this guard is
new work this plan must scope — it does not already exist for Vitest, Playwright, or xunit anywhere in
`ose-public`.
