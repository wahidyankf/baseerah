# ayokoding-web — Remove DDD (Keep Hexagonal)

> **Status**: In Progress
> **Scope tier**: B — "DDD removal + tidy" (keep hexagonal feature modules; imports and runtime
> logic untouched)
> **Affected projects**: `ayokoding-web`, `rhino-cli` (allowlist source + tests)

## Context

`ayokoding-web` is a Next.js 16 content site (TypeScript, tRPC). Its `src/contexts/<ctx>/`
directory currently carries Domain-Driven Design (DDD) "accretions" added by the earlier
[`plans/done/2026-05-10__ayokoding-web-ddd-and-specs-format/`](../../done/2026-05-10__ayokoding-web-ddd-and-specs-format/)
plan: a DDD bounded-context spec registry, an `apps_with_ddd()` allowlist entry in `rhino-cli`,
two pre-push `rhino-cli ddd` validation commands, six empty `domain/` layer folders, and
DDD/"bounded context" language in the app README.

The governance doc
[`repo-governance/development/pattern/hexagonal-architecture-web.md`](../../../repo-governance/development/pattern/hexagonal-architecture-web.md)
[Repo-grounded] already states that the web `contexts/` directory follows the Effect.ts
`Context.Tag` naming convention and that **"DDD applies only to backend apps"**. Removing DDD
from this web app therefore **realigns it with existing governance** rather than diverging from
it. [Repo-grounded]

This plan removes the DDD accretions and re-describes the structure as **hexagonal feature
modules**. No `contexts/` runtime code, no imports, and no dependency directions change.

## Scope

### In scope (the only changes)

1. Delete the DDD spec subtree `specs/apps/ayokoding/ddd/` entirely (10 files: `README.md`,
   `bounded-context-map.md`, `bounded-contexts.yaml`, and `ubiquitous-language/**`).
   [Repo-grounded]
2. Edit `apps/ayokoding-web/project.json`: remove the two `rhino-cli ddd bc/ul` pre-push
   commands from the `test:quick` target and the two `inputs` globs pointing at the DDD spec
   files. [Repo-grounded]
3. Edit `apps/rhino-cli/src/internal/allowlist.rs`: remove `"ayokoding"` from `apps_with_ddd()`,
   update the `membership` test relatively (decrement expected `len` by 1; remove the
   `ayokoding` assertion), and update the module `//!` doc block. Rebuild + retest `rhino-cli`.
   [Repo-grounded]
4. Delete the six empty `src/contexts/*/domain/` folders (app-shell, content, health, i18n,
   navigation, search) and their empty `domain/index.ts` barrels. [Repo-grounded]
5. Rewrite the Architecture-related sections of `apps/ayokoding-web/README.md` to drop DDD /
   "bounded context" / "DDD registry" language and describe the structure as hexagonal feature
   modules (three layers now: `application/`, `infrastructure/`, `presentation/`). [Repo-grounded]

### Out of scope (do NOT touch)

- The `contexts/` source folders' `application/`, `infrastructure/`, `presentation/` code —
  keep current imports; do **not** flip dependency direction or move ports.
- The C4 spec trees `specs/apps/ayokoding/{product,system-context,containers,components}`.
- The Gherkin `specs/apps/ayokoding/behavior/**` and the `spec-coverage` gate.
- The `rhino-cli ddd` subcommands themselves — still used by `organiclever-be`,
  `organiclever-web`, `ose-app-be` (only the `ayokoding` allowlist entry and this app's two
  invocations are removed).

## Approach summary

This is a deletion-and-tidy plan, not a feature build. Every change is guarded by a test or a
`grep` assertion written/adjusted **first** (RED), then the change is applied (GREEN), then the
tree is tidied (REFACTOR). The `rhino-cli` allowlist test is a natural RED→GREEN: adjust the
`membership` assertion to its post-removal shape (it fails against the current slice), then
remove the slice entry to make it pass.

Because two sibling plans (`ose-web-remove-ddd` and `wahidyankf-web-remove-ddd-and-hexagonal`)
edit the **same** `allowlist.rs` file and `membership` test and may run in any order, the
allowlist edits in this plan are expressed **relatively** ("decrement expected `len` by 1";
"remove the `ayokoding` assertion / entry / doc line") so they stay correct regardless of
execution order. [Repo-grounded — `wahidyankf` is also present in the current slice]

## Document map

| Document                         | Purpose                                                           |
| -------------------------------- | ----------------------------------------------------------------- |
| [`brd.md`](./brd.md)             | WHY — business rationale, impact, affected roles, business risks  |
| [`prd.md`](./prd.md)             | WHAT — personas, user stories, Gherkin acceptance criteria, scope |
| [`tech-docs.md`](./tech-docs.md) | HOW — architecture, design decisions, file-impact, rollback       |
| [`delivery.md`](./delivery.md)   | DO — phased `[AI]`/`[HUMAN]` checklist with gates                 |

## Definition of done

- All five in-scope change groups applied.
- `nx affected -t typecheck lint test:quick spec-coverage` green for `ayokoding-web` AND
  `rhino-cli`; `rhino-cli` cargo tests green.
- `nx build ayokoding-web` succeeds and the dev server renders (manual Playwright-MCP smoke:
  home page + one content page, zero console errors).
- No dangling references to `specs/apps/ayokoding/ddd` anywhere in the repo (`grep` clean,
  excluding `plans/`).
- `apps/ayokoding-web/README.md` Architecture description is accurate (no DDD/BC language).

## Git workflow

Trunk Based Development — commit directly to `main`, no PR (no PR was requested). Thematic
Conventional Commits per phase. See
[Trunk Based Development Convention](../../../repo-governance/development/workflow/trunk-based-development.md).
