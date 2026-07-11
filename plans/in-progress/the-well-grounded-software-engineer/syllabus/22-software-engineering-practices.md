# 22 · Software Engineering Practices (Annotated-concept, Python \*)

**prd row**: Pass 2 · Solidify the Core · Annotated-concept · Python \* · Learn 122 / Drill 222 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the professional practices that turn code into engineering — version control (the Git
model + trunk-based development), testing discipline, code quality, CI/CD, collaboration/process, and
debugging/observability basics. Testing mechanics come from
[`13-software-testing`](./13-software-testing.md); this topic is the surrounding workflow, driven from the
`git` CLI (DD-17). It underpins the whole repo's own conventions.

## Prerequisites

- **Prior topics**: [topic 05 Just Enough Bash](./05-just-enough-bash.md) (terminal + `git` CLI),
  [topic 13 Software Testing](./13-software-testing.md) (the testing discipline this workflow wraps), and a
  working app from Pass 1 (e.g. [topic 09 Backend Essentials](./09-backend-essentials.md)) to practice on.
- **Tools & environment**: a macOS/Linux terminal; **`git`**; **Python 3.x** with a linter/formatter
  (`ruff`/`black`) and `pytest`; a CI runner concept (GitHub Actions YAML shown, run locally where possible).
- **Assumed knowledge**: basic `git` add/commit; running tests from the CLI; editing YAML.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: Conventional Commits is at a stable final **v1.0.0** (no newer major). `git` CLI
  and GitHub Actions YAML schema broadly stable — standard spot-check at authoring. (conventionalcommits.org)
- 2026-07-12 — verified (CORRECTION of framing): as of 2026 (**Ruff 0.15**), `ruff format` is a credible
  full **Black replacement** (>99.9% output-identical) and Ruff is the de facto single consolidated tool
  (replaces flake8 + isort + Black + pyupgrade in one binary). Prefer "**ruff** (formatter + linter,
  Black-compatible)" over teaching `ruff` + `black` side by side. Not a hard error (both still function).
  (astral.sh / docs.astral.sh/ruff)

## Items

- Version control: the Git model (commits/branches/merges/rebases), trunk-based development, conventional
  commits, PR review — driven from the `git` CLI.
- Testing discipline: the test pyramid, TDD, unit/integration/e2e, coverage & its limits, test doubles
  (cross-ref `software-testing`).
- Code quality: linting/formatting, code review, refactoring, technical debt, readability.
- CI/CD: pipelines, quality gates, artifact/build, release strategies (blue-green, canary).
- Collaboration & process: Agile/Scrum/Kanban intuition, estimation pitfalls, documentation, ADRs.
- Debugging & observability basics; incident hygiene.

## Worked examples

Colocated under `software-engineering-practices/learning/code/`; each a runnable workflow artifact
(DD-20/DD-30).

- **tdd-cycle** — walk a feature through TDD red → green → refactor with a real Python test.
- **clean-history** — a commit-history worked example: messy history → clean conventional-commit history
  via the `git` CLI (rebase/squash).
- **ci-pipeline** — a minimal CI pipeline (lint → test → build) annotated stage by stage with a
  WCAG-accessible Mermaid flow, runnable locally.
- **adr** — an ADR worked example (decision, context, consequences) for a small technical choice.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: take a small Python feature and run it through a full professional workflow: TDD it on a
  feature branch, produce a clean conventional-commit history, wire a CI pipeline (lint → test → build)
  that gates the change, and record an ADR — ending with a green, reviewable change and its decision trail.
- **Concepts exercised**: [ ] TDD red→green→refactor [ ] a feature branch + clean conventional commits
  [ ] linting/formatting gate [ ] a CI pipeline (lint→test→build) [ ] an ADR [ ] a self-review pass.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — TDD the feature: failing test first, then implement. Verify the test
     goes red→green and `ruff`/`black` are clean.
  2. Craft the history: a feature branch with conventional commits; squash/rebase a messy sequence into a
     clean one. Verify `git log` shows a clean, conventional history.
  3. `ci.yml` — a lint→test→build pipeline. Verify each stage runs (locally via `act` or documented run)
     and a deliberately broken commit fails the gate.
  4. `adr-0001.md` — record the decision, context, consequences. Verify it references the actual change.
- **Acceptance criteria**: the feature is TDD-built and passing; history is clean + conventional; the CI
  pipeline gates green and fails on a bad commit; the ADR documents the real decision.
- **Done bar**: runnable end-to-end (pipeline gates the change) + produces the ADR + web-verified.

---

← Previous: [21 · Advanced Networking](./21-advanced-networking.md) · Next: [23 · Advanced SQL & Query Performance](./23-advanced-sql-and-query-performance.md) →
