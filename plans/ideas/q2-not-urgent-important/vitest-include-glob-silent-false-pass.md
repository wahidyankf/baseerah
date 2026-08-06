# A test file outside every Vitest include glob runs zero times and still reports green

One-line summary: a test file can be well-written, fully correct, and provide exactly zero
protection if its path matches no configured Vitest `include` glob — nothing reports the miss, so
the file reads as covered in every status report while never executing once.

> Demoted from a full `backlog/` plan to a two-pager on 2026-08-05. The full plan carried the
> standard five documents — `README.md`, `brd.md`, `prd.md`, `tech-docs.md`, and a two-phase
> `delivery.md` with Gherkin acceptance criteria, phase gates, and a Knowledge Capture phase — all
> of which is folded into the sections below. It originated in a sibling repository, not this one:
> during the `ayokoding-www-tools-ai-benchmark` PR #122 cycle-3 review, `pr-review-integrity-maker`
> filed HIGH finding F2 after proving that the EWT-003 regression test
> (`benchmark-content.test.tsx`, added under `src/app/[locale]/tools/ai-benchmark/`) matched neither
> of `apps/ayokoding-www/vitest.config.ts`'s two named projects — `unit` (Node environment,
> `test/unit/be-steps/**/*.steps.ts` plus `**/*.unit.{test,spec}.{ts,tsx}`) nor `unit-fe` (jsdom,
> `test/unit/fe-steps/**/*.steps.{ts,tsx}` plus `src/features/**/*.test.{ts,tsx}`). The reviewer
> Renamed from vitest-glob-coverage-guard.md on 2026-08-06 by plan-ideas-grooming.
> reverted the actual code fix and re-ran the full suite; it still passed 144/144 test files with
> the bug fully reintroduced. That repo fixed its own config inline by widening `unit-fe` to also
> cover `src/app/**/*.test.{ts,tsx}`; the defect class was never fixed anywhere.

## Problem / context

This is the **silent-false-pass class**: a check that cannot fail certifies nothing. A test whose
path misses every `include` glob produces no error and no warning — just a zero-execution count
indistinguishable from success. `passWithNoTests: true` deepens it, because zero files matched
reports as zero files failed.

Re-deriving the exposure against this repository's own configuration changes the picture but does
not eliminate it. `beaver-nest` has exactly three Vitest configs, and **none of them uses a named
`projects` array** — each declares a single `test` block, so the originating incident's exact
multi-project shape cannot occur here:

| Config                                 | Name    | Environment | `include`                                           | `passWithNoTests` |
| -------------------------------------- | ------- | ----------- | --------------------------------------------------- | ----------------- |
| `apps/beaver-nest-fe/vitest.config.ts` | `unit`  | jsdom       | `src/**/*.{test,spec}.{ts,tsx}`                     | `true`            |
| `libs/web-ui/vitest.config.ts`         | (unset) | jsdom       | `src/**/*.test.{ts,tsx}`, `src/**/*.steps.{ts,tsx}` | unset (default)   |
| `libs/web-ui-token/vitest.config.ts`   | (unset) | node        | `src/**/*.test.{ts,tsx}`, `src/**/*.steps.{ts,tsx}` | unset (default)   |

Three concrete findings follow from that table:

- **The glob sets have diverged.** Both libs match `*.steps.{ts,tsx}`; `beaver-nest-fe` does not.
  Any `.steps.ts` or `.steps.tsx` file placed under `apps/beaver-nest-fe/src/` runs zero times,
  while the identically-named pattern one directory over in `libs/web-ui` runs normally. That is
  precisely the trap the class describes, and it is live today.
- **There is already an unmatched file.** `apps/beaver-nest-fe/src/test/landing.steps.ts` matches
  no `include` glob in this repo. It is deliberate rather than accidental — its own header comment
  states it is a literal-text registry existing only so every Gherkin step in
  `specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature` has a matching
  call for the spec-coverage checker to find, with the real assertions living in
  `src/app/page.test.tsx`. It even defines its own no-op local `Given`/`When`/`Then`/`And`. So a
  naive guard would fire on it on day one; the file is a design input, not a bug.
- **`passWithNoTests: true` appears exactly once repo-wide**, in `apps/beaver-nest-fe`. The two libs
  leave it at the Vitest default, so a _total_ zero-match fails loudly there. Neither behaviour
  helps with a _single_ stray file — that stays silent in all three projects.

Everything else currently matches: `beaver-nest-fe`'s four test files under `src/app/` and `src/lib/`
all match; `libs/web-ui`'s `*.unit.test.tsx` files match `src/**/*.test.{ts,tsx}` via the leading
wildcard; `libs/web-ui-token`'s single `src/tokens-export.steps.ts` matches. No test-shaped file in
any of the three projects sits outside `src/`. The class is therefore **narrower here than upstream,
but not impossible** — and the fe/lib glob divergence means the next `.steps.tsx` file added to the
front end will hit it.

## Why now

The upstream instance survived a full green CI run and a passing PR review; it was caught only
because a specialist manually reverted a fix and re-ran the suite. Nothing automated caught it, and
nothing automated would catch it here either. The window is favourable precisely because this repo
is small — three configs, one already-known exempt file — so a guard can be designed, proven, and
adopted against a tractable surface before the app count grows and the glob divergence between
`apps/beaver-nest-fe` and `libs/web-ui` propagates into new projects by copy-paste.

## Prior art / precedents

- [acceptance-clause-vacuity](https://github.com/wahidyankf/ose-public/blob/main/plans/ideas/q1-urgent-important/acceptance-clause-vacuity.md) — the direct sibling in the
  silent-false-pass family: clauses that pass no matter what the world looks like. Its central rule,
  "verify the check could have failed", is the same rule this guard mechanizes for test discovery.
- [mermaid-validator-does-not-check-syntax](https://github.com/wahidyankf/ose-public/blob/main/plans/ideas/q1-urgent-important/mermaid-validator-does-not-check-syntax.md) — another
  same-class instance: a validator wired into pre-commit, pre-push, and CI, routinely cited as the
  correctness gate, that never actually parses what it claims to check.
- [Regression Test Mandate](../../../repo-governance/development/quality/regression-test-mandate.md) —
  requires every bug fix to land with a reproducing test. That mandate is only as strong as the
  guarantee the test actually runs, which is exactly what this guard supplies.
- [Feature-Change Completeness](../../../repo-governance/development/quality/feature-change-completeness.md)
  — the existing `specs:coverage` machinery already reasons about which files satisfy which
  requirement; it is the closest structural precedent for a path-to-glob reconciliation check.
- [Maker-Checker-Fixer pattern](../../../repo-governance/development/pattern/maker-checker-fixer.md) —
  the natural home if the guard lands as a checker enhancement rather than a standalone script.

## Proposed direction (sketch)

Enumerate every project carrying a Vitest config, read its `include` globs, then enumerate every
test-shaped file (`*.test.*`, `*.spec.*`, `*.steps.*`) under that project's root and report any file
matching no glob. Two design questions dominate and were left open by the original plan: where the
guard lives — a lightweight script wired into an Nx target, or an enhancement to the existing
[`ci-checker`](../../../.claude/agents/ci-checker.md) or
[`swe-code-checker`](../../../.claude/agents/swe-code-checker.md) agent — and whether it blocks CI or
merely reports. A deliberate-exemption mechanism is mandatory from the start, because
`landing.steps.ts` is a legitimate unmatched file. The guard must be proven both ways: zero findings
against the repo as it stands today, and exactly one finding against a synthetic reintroduction of
the glob gap.

## Rough scope & non-goals

**In scope**: a durable automated guard that fails CI or a checker report when a test file exists
outside every configured test-project's glob, plus the investigation phase that designs and places
it. Candidate coverage is every `apps/*` and `libs/*` project with a Vitest config exposing
`include` globs, though the investigation may start with one project and expand once proven.

**Out of scope** (carried forward verbatim from the source plan):

- Does not re-litigate the specific `unit-fe` glob fix already merged in
  `ayokoding-www-tools-ai-benchmark`'s PR #122.
- Does not change test content or assertions — only detects glob-coverage gaps.

## Risks & open questions

- The guard's home is undecided between a new script plus Nx target and an enhancement to
  `ci-checker` or `swe-code-checker`, and its failure mode is undecided between CI-blocking and
  checker-report. The two choices interact: a blocking gate needs a robust exemption mechanism that
  a report does not. (open)
- Scope is undecided between every Vitest-configured project and a narrow start. With only three
  configs here, "all of them" is cheap — but the same guard in a sibling repo faces a much larger
  surface. (open)
- How to exempt `apps/beaver-nest-fe/src/test/landing.steps.ts` without creating a hole wide enough
  to swallow real misses. An inline pragma, an allowlist file, and a naming convention all have
  different failure modes. (open)
- Whether the guard should additionally flag _divergence_ between sibling configs — `beaver-nest-fe`
  omitting the `*.steps.{ts,tsx}` pattern that both libs carry — rather than only unmatched files.
  Divergence is the upstream cause; unmatched files are only its symptom. (open)
- Whether Playwright configs (`apps/beaver-nest-fe-e2e`, `apps/beaver-nest-be-e2e`,
  `libs/web-ui/e2e`) belong in the same guard or need their own. (open)
- An adjacent gap in the same family, found while re-deriving: `libs/web-ui-token`'s `test:unit`
  greps for `\.(skip|todo)\(` while `beaver-nest-fe` and `web-ui` both grep
  `\b(it|test|describe)\.(skip|only|todo)\(`, so a `.only` narrowing is unguarded in that one
  project. Worth deciding whether this guard absorbs it or it is filed separately.
- Re-litigating the already-merged upstream glob fix remains a standing temptation and an explicit
  non-goal.

## What success looks like + promotion signal

Success is zero silently-uncovered test files across every Vitest-configured project, verified by an
automated check rather than manual glob review — and, critically, a guard demonstrated to fail
against a seeded counterexample, not merely to pass against the current tree. Secondary success:
`apps/beaver-nest-fe` and the two libs converge on one deliberate glob set, or their divergence
becomes an explicit, documented decision rather than an accident.

**Promotion signal**: promote when either (a) a second `apps/*` project gains a Vitest config —
copy-paste divergence becomes likely enough that the guard pays for itself — or (b) the first
`.steps.{ts,tsx}` file is proposed under `apps/beaver-nest-fe/src/`, which would hit the live gap
immediately. Answering the guard's home and failure mode is the one piece of design work that must
precede promotion; the exemption question can be resolved inside the plan.
