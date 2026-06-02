# Technical Documentation — ayokoding-web Remove DDD

## Current state (verified on the working commit)

### DDD spec subtree — `specs/apps/ayokoding/ddd/` [Repo-grounded]

10 files:

```text
specs/apps/ayokoding/ddd/
├── README.md
├── bounded-context-map.md
├── bounded-contexts.yaml
└── ubiquitous-language/
    ├── README.md
    ├── app-shell.md
    ├── content.md
    ├── health.md
    ├── i18n.md
    ├── navigation.md
    └── search.md
```

### `apps/ayokoding-web/project.json` `test:quick` target [Repo-grounded]

- `inputs` (current) includes these two DDD globs:
  - `{workspaceRoot}/specs/apps/ayokoding/ddd/bounded-contexts.yaml`
  - `{workspaceRoot}/specs/apps/ayokoding/ddd/ubiquitous-language/**/*.md`
- `options.commands` (current) includes these two DDD commands (first two array entries):
  - `(cd ../../apps/rhino-cli && cargo run --release --quiet -- ddd bc ayokoding)`
  - `(cd ../../apps/rhino-cli && cargo run --release --quiet -- ddd ul ayokoding)`
- Everything else (the vitest + coverage-82 command, the `ayokoding-cli links check` command,
  the `generate-indexes --validate` command, `parallel: false`, `cwd`, `dependsOn`) stays.

### `apps/rhino-cli/src/internal/allowlist.rs` [Repo-grounded]

- Module `//!` doc block (lines 8–12) lists five apps including
  `//!   - ayokoding:    bounded-contexts.yaml + feature files present`.
- `apps_with_ddd()` slice (lines 20–26) contains five entries:
  `"organiclever", "wahidyankf", "ose-platform", "ayokoding", "ose-app"`.
- `membership` test (lines 35–42): `assert_eq!(v.len(), 5);` plus
  `assert!(v.contains(&"organiclever"));`, `assert!(v.contains(&"ayokoding"));`,
  `assert!(v.contains(&"ose-app"));`.

> **Cross-plan note**: `wahidyankf` is also present in the current slice, and two sibling plans
> (`ose-web-remove-ddd` → `ose-platform`, `wahidyankf-web-remove-ddd-and-hexagonal` →
> `wahidyankf`) edit this same file and `membership` test. Express this plan's edits
> **relatively** so they remain correct under any execution order.

### Empty `domain/` layers [Repo-grounded]

Each `apps/ayokoding-web/src/contexts/<ctx>/domain/` (ctx ∈ {app-shell, content, health, i18n,
navigation, search}) is an empty folder whose only entry is a **blank** `index.ts` barrel.
`grep -rn "contexts/[a-z-]*/domain"` outside the barrels themselves returns **zero** importers,
so deletion is safe.

### `apps/ayokoding-web/README.md` DDD/BC language [Repo-grounded]

- `## Source Layout (BC-organized)` section describes the structure by "bounded context" with a
  per-BC table and asserts the DDD registry is the source of truth and that
  `rhino-cli ddd bc/ul` enforce it on every `test:quick`.
- `## Related` section links `specs/apps/ayokoding/` as "C4 + DDD + Gherkin specifications".

## Target state

| Artifact                       | Change                                                                                                          |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------- |
| `specs/apps/ayokoding/ddd/`    | Deleted entirely.                                                                                               |
| `project.json` `test:quick`    | Two DDD commands removed; two DDD `inputs` globs removed; rest unchanged.                                       |
| `allowlist.rs`                 | `ayokoding` removed from slice + `//!` doc; `membership` `len` decremented by 1; `ayokoding` assertion removed. |
| `src/contexts/*/domain/`       | Six folders + blank barrels deleted.                                                                            |
| `apps/ayokoding-web/README.md` | Architecture sections rewritten as hexagonal feature modules (3 layers); DDD/registry/`ddd` references removed. |

## Design decisions

### DD-1: Relative allowlist test edit (order-independence)

**Decision**: Edit the `membership` test by decrementing the asserted `len` by exactly one and
removing only the `ayokoding`-specific lines — never by hard-coding `assert_eq!(v.len(), 4)`.

**Rationale**: Sibling plans concurrently remove `ose-platform` and `wahidyankf`. A hard-coded
absolute count would be wrong whenever a sibling runs first. The execution agent must read the
current `len` literal, subtract one, and write the result; and delete only the `ayokoding`
slice entry, doc line, and assertion. [Repo-grounded — `wahidyankf` confirmed present]

### DD-2: Keep the `rhino-cli ddd` subcommands

**Decision**: Remove only ayokoding's allowlist entry and its two invocations; leave the `ddd bc`
/ `ddd ul` subcommand implementations intact.

**Rationale**: `organiclever-be`, `organiclever-web`, and `ose-app-be` still rely on these
subcommands and on `apps_with_ddd()`. Removing the subcommands is out of scope and would break
those apps. [Repo-grounded]

### DD-3: Delete empty `domain/` layers rather than populate them

**Decision**: Delete the six empty `domain/` folders and barrels.

**Rationale**: They contain no code and have no importers; keeping them implies a structure that
does not exist. The hexagonal-web governance lists `domain/` as an available layer, not a
mandatory one, so an app with no domain logic legitimately omits it. [Repo-grounded]

### DD-4: README describes three real layers, links governance

**Decision**: Replace "bounded context" framing with "hexagonal feature module" framing, list the
three layers that actually exist (`application`, `infrastructure`, `presentation`), and link
`hexagonal-architecture-web.md`. Use the
[Dynamic Collection References Convention](../../../repo-governance/conventions/writing/dynamic-collection-references.md)
— it is acceptable to enumerate the six feature modules, but the structural authority is the
linked governance doc, not a hard-coded count.

**Rationale**: Aligns docs with governance and removes the governance/doc conflict.

## File-impact summary

| File                                                                                                    | Operation |
| ------------------------------------------------------------------------------------------------------- | --------- |
| `specs/apps/ayokoding/ddd/**` (10 files)                                                                | Delete    |
| `apps/ayokoding-web/project.json`                                                                       | Edit      |
| `apps/rhino-cli/src/internal/allowlist.rs`                                                              | Edit      |
| `apps/ayokoding-web/src/contexts/{app-shell,content,health,i18n,navigation,search}/domain/index.ts` (6) | Delete    |
| `apps/ayokoding-web/src/contexts/{...}/domain/` (6 dirs)                                                | Delete    |
| `apps/ayokoding-web/README.md`                                                                          | Edit      |

## Testing strategy

This is a deletion/config plan; the "tests" are guard assertions written/adjusted first:

| Acceptance criterion (prd.md) | Guard / test level                                                                    |
| ----------------------------- | ------------------------------------------------------------------------------------- |
| DDD subtree deleted (US-1)    | `grep` assertion: zero `specs/apps/ayokoding/ddd` matches outside `plans/`            |
| test:quick DDD removed (US-2) | `nx run ayokoding-web:test:quick` passes; `grep` shows no `ddd bc/ul` in project.json |
| allowlist updated (US-3)      | Unit test `membership` (`cargo test --lib`) — natural RED→GREEN                       |
| domain layers deleted (US-4)  | `nx run ayokoding-web:typecheck` exits 0; `grep` shows no `*/domain/` folders         |
| README accurate (US-5)        | `grep` assertion: no `bounded context` / `DDD registry` / `ddd` references            |
| full gate green (DoD)         | `nx affected -t typecheck lint test:quick spec-coverage`                              |
| dev server renders (DoD)      | Manual Playwright MCP smoke (home + one content page, zero console errors)            |

## Harness-neutrality

This plan edits app source (`ayokoding-web`), tooling source (`rhino-cli` Rust), and an app
README — all **vendor-neutral** app/tooling code. It does **not** edit `repo-governance/`,
`.claude/`, or `.opencode/`. The README rewrite must not introduce vendor-specific instructions
(no Claude Code / OpenCode-specific directives); describe architecture in tool-agnostic terms.

## Rollback

Every change is a deletion or a localized edit on `main`. To roll back, `git revert` the phase
commits in reverse order. No data migration, no external state, no irreversible operation is
involved.

## Dependencies

- `rhino-cli` is a pre-push dependency for other apps; it must build and test clean after the
  allowlist edit. The plan rebuilds and retests `rhino-cli` in its own phase before final gates.
- No new external libraries, versions, or APIs are introduced (nothing to verify via web).
