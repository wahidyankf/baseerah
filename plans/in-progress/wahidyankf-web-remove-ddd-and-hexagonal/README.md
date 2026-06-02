# Remove DDD and Hexagonal from wahidyankf-web

> Plan type: refactor + governance + docs. Five-document multi-file layout.
> Status: in-progress. Created via the plan-establishment-execution workflow
> (macro decisions pre-resolved; grills are validation passes only).

## Context

`wahidyankf-web` is a Next.js 16 personal portfolio site (Home, CV, Personal
Projects) deployed to <https://www.wahidyankf.com/>. It is a static content
site: no Effect TS, no XState, no IO ports, and no business rules. [Repo-grounded]

The app currently carries two architectural accretions that add ceremony
without payoff:

1. **DDD accretion** — a `specs/apps/wahidyankf/ddd/` registry
   (`bounded-contexts.yaml` + `ubiquitous-language/**`) plus two `rhino-cli ddd`
   pre-push gates wired into `test:quick`. [Repo-grounded]
2. **Hexagonal `contexts/` layout** — `src/contexts/<ctx>/` with four layers
   (`domain/`, `application/`, `infrastructure/`, `presentation/`). For this app
   the `domain/` and `infrastructure/` layers are empty stubs (each `index.ts`
   is a one-line placeholder); all real code lives in `application/` and
   `presentation/`. This is the **least hexagonal** of the three sibling web
   apps. [Repo-grounded]

Because two of four layers are empty stubs, the hexagonal structure is pure
overhead here. This plan removes the DDD accretion **and** flattens the
hexagonal `contexts/` layout into plain feature folders (`src/features/<ctx>/`).

## Scope

**In scope** (three workstreams):

- **A. DDD-accretion removal** — delete `specs/apps/wahidyankf/ddd/`; remove the
  two `ddd bc`/`ddd ul` commands and the two `ddd/...` input globs from
  `apps/wahidyankf-web/project.json`; remove `"wahidyankf"` from the rhino-cli
  `apps_with_ddd()` allowlist and update its membership test.
- **B. Hexagonal flattening** — move
  `src/contexts/<ctx>/{application,infrastructure,presentation}/*` into a flat
  `src/features/<ctx>/` per context (5 contexts: app-shell, home, cv,
  personal-projects, search); rewrite all `@/contexts/<ctx>/<layer>/X` imports
  to `@/features/<ctx>/X`.
- **C. Governance + docs** — add an opt-out clause to
  `repo-governance/development/pattern/hexagonal-architecture-web.md` permitting
  flat `src/features/` for trivially-small static sites; rewrite the
  Architecture/Specs sections of `apps/wahidyankf-web/README.md`.

**Out of scope**:

- C4 specs (`specs/apps/wahidyankf/{product,system-context,containers,components}`)
  — untouched.
- Gherkin behavior specs (`specs/apps/wahidyankf/behavior/**`) and the
  `spec-coverage` gate — untouched. The gate scans the app dir
  `apps/wahidyankf-web` and `specs/.../behavior/web/gherkin`; the internal
  `contexts/`→`features/` rename does not affect its resolution (verified:
  `spec-coverage` globs `{projectRoot}/**/*.{ts,tsx}`). [Repo-grounded]
- The `rhino-cli ddd` commands themselves (only the wahidyankf entry is removed
  from the allowlist).

**Affected projects**: `wahidyankf-web` (app), `rhino-cli` (allowlist + test).

## Approach Summary

The flatten is a **behavior-preserving refactor**: the existing unit test suite
(Navigation, style, cv data, markdown, search, plus the App Router page tests)
guards behavior across the move. Each context moves in its own phase, and every
phase ends green (typecheck + lint + unit tests pass). The work is sequenced:

1. **Phase 0** — environment setup and baseline.
2. **Phase 1** — DDD-accretion removal (project.json + spec dir + rhino-cli).
3. **Phases 2–6** — flatten one context per phase (app-shell → search → cv →
   home → personal-projects), rewriting imports as part of each move.
4. **Phase 7** — governance opt-out clause + README rewrite + grep-clean
   verification + Playwright-MCP smoke + archival.

## Navigation

- [brd.md](./brd.md) — Business Requirements (WHY)
- [prd.md](./prd.md) — Product Requirements (WHAT) + Gherkin acceptance criteria
- [tech-docs.md](./tech-docs.md) — Architecture, design decisions, file impact
- [delivery.md](./delivery.md) — Phased delivery checklist (DO)

## Definition of Done

- DDD accretion removed (`specs/apps/wahidyankf/ddd/` gone; project.json and
  rhino-cli allowlist updated).
- `src/contexts/` flattened to `src/features/`; all imports updated.
- `nx affected -t typecheck lint test:quick spec-coverage` green for
  `wahidyankf-web` AND `rhino-cli`; `rhino-cli` cargo tests green.
- `nx build wahidyankf-web` passes.
- Playwright-MCP smoke (Home, /cv, /personal-projects, search) — zero console
  errors.
- Grep clean of `@/contexts` and `specs/apps/wahidyankf/ddd`.
- Governance doc opt-out clause present (vendor-neutral).
- READMEs accurate (no DDD/hexagonal/`ddd bc`/`ddd ul` references).
