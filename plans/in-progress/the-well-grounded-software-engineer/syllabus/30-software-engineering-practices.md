# 30 · Software Engineering Practices (Annotated-concept, Python \*)

**prd row**: Pass 2 · Depth, Design & Craft · Annotated-concept · Python \* · Learn 130 / Drill 230 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the professional practices that turn code into engineering — version control (the Git
model + trunk-based development), testing discipline, code quality, CI/CD, collaboration/process, and
debugging/observability basics. Testing mechanics come from
[`15-software-testing`](./15-software-testing.md); this topic is the surrounding workflow, driven from the
`git` CLI (DD-17). It underpins the whole repo's own conventions.

## Why this exists · the big idea

- **The problem before the solution**: code that works on your machine isn't engineering — without version
  control, tests, review, and CI, a growing team and codebase regress faster than they progress.
- **Keep-this-if-you-forget-everything**: the practices exist to make change _safe and reversible_ — small
  commits, green tests, and an automated gate let you move fast _because_ you can always undo.
- **Big ideas touched**: `correctness-vs-pragmatism` (CI gates, coverage, and review are risk management,
  not bureaucracy), `coupling-vs-cohesion` (trunk-based dev and small PRs cut the merge coupling between people).

## Prerequisites

- **Prior topics**: [topic 5 Just Enough Bash](./05-just-enough-bash.md) (terminal + `git` CLI),
  [topic 15 Software Testing](./15-software-testing.md) (the testing discipline this workflow wraps), and a
  working app from Pass 1 (e.g. [topic 11 Backend Essentials](./11-backend-essentials.md)) to practice on.
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

## Tensions & trade-offs — when NOT to reach for this

- **Process vs velocity**: every gate — review, CI, required coverage — trades throughput for safety. On a
  solo throwaway prototype the full ceremony is pure drag; on a shared production system skipping it is how
  you get a 3am incident. The skill is dialing ceremony to the blast radius, not maxing or zeroing it.
- **Branching models**: trunk-based development optimizes for integration frequency and small diffs;
  long-lived feature branches / GitFlow optimize for release isolation but pay in merge hell. Team size,
  release cadence, and review culture decide — neither is universally right.
- **Coverage as a target**: a coverage number is a proxy, and chasing 100% tests trivia and breeds
  assertion-free tests. Goodhart's law bites — the moment a metric becomes a target it stops measuring what
  you wanted.

## Lineage — why it beat the alternative

- These practices are scar tissue from specific, expensive failures. Version control (SCCS → CVS → SVN → Git)
  grew because coordinating shared code by hand lost work; continuous integration (Kent Beck / XP, late 1990s)
  answered the "integration hell" of big-bang merges; conventional commits and trunk-based dev answered the
  review-and-conflict costs that long branches produced at scale; DevOps / CI-CD (from ~2009) collapsed the
  dev↔ops wall that made releases rare and terrifying. The through-line: each practice removed one class of
  recurring failure — so adopt a practice for the failure it prevents, not because it's on a checklist. This
  is the ground the repo's own conventions and [`32-software-product-engineering`](./32-software-product-engineering.md) /
  [`09-project-management`](./09-project-management.md) build on.

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

## Read more

**Books**

- **The Pragmatic Programmer** — David Thomas & Andrew Hunt (1999; 20th anniversary ed. 2019). Foundational collection of practical software-craftsmanship heuristics.
- **Clean Code: A Handbook of Agile Software Craftsmanship** — Robert C. Martin (2008). Widely read standard reference on naming, functions, and code-level craftsmanship.
- **Code Complete** — Steve McConnell (1993; 2nd ed. 2004). Comprehensive handbook of software construction practices grounded in empirical research.
- **Working Effectively with Legacy Code** — Michael Feathers (2004). The standard reference for safely modifying untested, poorly structured existing code.
- **Refactoring: Improving the Design of Existing Code** — Martin Fowler (1999; 2nd ed. 2018). Canonical catalog of code smells and refactorings for continuous code improvement.

**Papers & articles**

- **How to Do a Code Review (Google Engineering Practices)** — Google (continually maintained). Widely adopted industry-standard guide to code review culture, mechanics, and reviewer/author responsibilities. <https://google.github.io/eng-practices/review/>

---

← Previous: [29 · Advanced Networking](./29-advanced-networking.md) · Next: [31 · Agentic Coding](./31-agentic-coding.md) →
