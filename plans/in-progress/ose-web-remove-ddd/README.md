# ose-web-remove-ddd

> **Status**: In Progress
> **Affected projects**: `ose-web`, `rhino-cli` (allowlist source + tests)
> **Worktree**: `worktrees/ose-web-remove-ddd/`

## Context

`apps/ose-web/` is a Next.js 16 content/marketing site (TypeScript, tRPC). During the
`plans/done/2026-05-10__oseplatform-web-ddd-and-specs-format/` work it accreted a set of
Domain-Driven Design (DDD) artifacts — a bounded-context spec registry, an `apps_with_ddd()`
allowlist entry in `rhino-cli`, two pre-push `rhino-cli ddd` validation commands, seven empty
`domain/` layer folders, and DDD-framed README prose.

These DDD accretions contradict the repo's own governance. `repo-governance/development/pattern/hexagonal-architecture-web.md`
[Repo-grounded] states that the web `contexts/` directory name follows the Effect.ts `Context.Tag`
naming convention and that "DDD applies only to backend apps". Removing DDD from `ose-web`
**realigns** the app with governance rather than diverging from it.

This plan removes only the DDD scaffolding. It keeps the hexagonal feature-module layout intact —
no application/infrastructure/presentation source code, imports, or dependency directions change.

## Scope

### In scope (exactly five change groups)

1. Delete `specs/apps/ose-platform/ddd/` entirely (bounded-contexts.yaml + ubiquitous-language/\*\*, 11 files). [Repo-grounded]
2. Edit `apps/ose-web/project.json`: remove the two `rhino-cli ddd bc/ul` pre-push commands and the two `ddd` `inputs` globs.
3. Edit `apps/rhino-cli/src/internal/allowlist.rs`: remove `"ose-platform"` from `apps_with_ddd()`; update the `membership` test **relatively** (decrement expected `len` by 1); update the module `//!` doc block. Rebuild + retest `rhino-cli`.
4. Delete the seven empty `apps/ose-web/src/contexts/*/domain/` folders + their `domain/index.ts` barrels. [Repo-grounded]
5. Rewrite the Architecture/Specs/Bounded-Contexts sections of `apps/ose-web/README.md` to drop DDD language and describe hexagonal feature modules per `hexagonal-architecture-web.md`.

### Out of scope (do NOT touch)

- `contexts/` `application/`, `infrastructure/`, `presentation/` source code and imports — no dependency-direction changes.
- C4 specs: `specs/apps/ose-platform/{product,system-context,containers,components}`.
- Gherkin behavior specs `specs/apps/ose-platform/behavior/**` and the `spec-coverage` gate.
- The `rhino-cli ddd` subcommands themselves — still used by `organiclever`, `ayokoding`, `ose-app`. Only the `ose-platform` allowlist entry and this app's two pre-push command lines are removed.

> **Spec slug note**: ose-web's spec app identifier is `ose-platform` (not "ose-web"). The spec
> tree lives at `specs/apps/ose-platform/`. [Repo-grounded]

## Coordination With Sibling Plans

Two sibling plans — `ayokoding-web-remove-ddd` (already in-progress) and
`wahidyankf-web-remove-ddd-and-hexagonal` — edit the **same** `allowlist.rs` file and the same
`membership` test, and may run in any order. To avoid order-dependent merge conflicts, this plan
expresses the allowlist edits **relatively** ("remove the `ose-platform` entry"; "decrement the
expected `len` by 1") rather than with absolute counts or absolute final-state assertions.
[Repo-grounded]

> **Current `membership` test note**: the test today asserts `assert_eq!(v.len(), 5)` and
> `contains` for `organiclever`, `ayokoding`, `ose-app` — it does **not** assert `ose-platform`.
> So the load-bearing edit is the `len` decrement; there is no `ose-platform` `contains`
> assertion to remove. [Repo-grounded]

## Approach Summary

This is a deletion-and-tidy plan, not a feature build. Code/config steps use a deletion-shaped
RED→GREEN→REFACTOR cycle: RED = a guarding grep-assertion or unit test still asserts the old DDD
behavior (or the old artifact still exists); GREEN = the artifact is deleted / the config or
allowlist edited; REFACTOR = surrounding prose and comments are tidied. The `rhino-cli` allowlist
`membership` test is a natural RED→GREEN: it goes red after the entry is removed and green after
the `len` is decremented.

`rhino-cli` is a pre-push dependency for other apps (it powers `test:quick`, `spec-coverage`, and
the `ddd` validators), so it is rebuilt and retested as part of this plan.

## Document Map

| Document                       | Purpose                                                         |
| ------------------------------ | --------------------------------------------------------------- |
| [brd.md](./brd.md)             | WHY — business rationale, impact, affected roles, risks         |
| [prd.md](./prd.md)             | WHAT — product scope, user stories, Gherkin acceptance criteria |
| [tech-docs.md](./tech-docs.md) | HOW — architecture, design decisions, file-impact map, rollback |
| [delivery.md](./delivery.md)   | DO — phased, gated, TDD-shaped delivery checklist               |

## Definition of Done

- All five change groups applied.
- `nx affected -t typecheck lint test:quick spec-coverage` green for `ose-web` AND `rhino-cli`.
- `rhino-cli` cargo tests green; `nx build ose-web` passes.
- Dev-server smoke via Playwright MCP (landing `/`, `/updates`, `/about`) — zero console errors.
- `grep` shows no dangling `specs/apps/ose-platform/ddd` references in tracked source/config/docs.
- `apps/ose-web/README.md` accurately describes the hexagonal feature-module architecture.
