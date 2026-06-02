# Technical Documentation — Remove DDD and Hexagonal from wahidyankf-web

## Architecture: Before and After

### Before (hexagonal `contexts/`)

```text
apps/wahidyankf-web/src/
├── app/                          # Next.js App Router (thin page wrappers)
│   ├── page.tsx                  # → HomeContent
│   ├── cv/page.tsx               # → CvContent
│   └── personal-projects/page.tsx# → PersonalProjectsContent
└── contexts/
    ├── app-shell/
    │   ├── domain/index.ts            # empty stub
    │   ├── application/index.ts       # empty stub
    │   ├── infrastructure/index.ts    # empty stub
    │   └── presentation/              # Navigation.tsx, style.ts (+ unit tests)
    ├── cv/
    │   ├── domain/index.ts            # empty stub
    │   ├── infrastructure/index.ts    # empty stub
    │   ├── application/               # data.ts, markdown.tsx (+ unit tests)
    │   └── presentation/CvContent.tsx
    ├── home/
    │   ├── domain|application|infrastructure/index.ts   # empty stubs
    │   └── presentation/HomeContent.tsx
    ├── personal-projects/
    │   ├── domain|infrastructure/index.ts   # empty stubs
    │   ├── application/projects.ts
    │   └── presentation/PersonalProjectsContent.tsx
    └── search/
        ├── domain|infrastructure/index.ts   # empty stubs
        ├── application/search.ts (+ unit test)
        └── presentation/SearchSection.tsx
```

[Repo-grounded — full file inventory verified via `find`.]

### After (flat `features/`)

```text
apps/wahidyankf-web/src/
├── app/                          # unchanged routing shell (imports rewritten)
└── features/
    ├── app-shell/   # Navigation.tsx, Navigation.unit.test.tsx, style.ts, style.unit.test.ts
    ├── cv/          # data.ts, data.unit.test.ts, markdown.tsx, markdown.unit.test.tsx, CvContent.tsx
    ├── home/        # HomeContent.tsx
    ├── personal-projects/  # projects.ts, PersonalProjectsContent.tsx
    └── search/      # search.ts, search.unit.test.ts, SearchSection.tsx
```

The empty `domain/` and `application/index.ts` and `infrastructure/index.ts`
stubs are **deleted** (not moved) — they export nothing and are imported by no
one. [Repo-grounded — verified: no file imports `@/contexts/<ctx>/domain` or
`@/contexts/<ctx>/infrastructure`; the only barrel/index imports are the layer
`presentation`/`application` deep paths shown below.]

## Design Decisions

### DD-1: Flatten to `features/`, not keep a 2-layer `contexts/`

The user resolved scope to remove BOTH DDD and the hexagonal layout. A flat
`features/<ctx>/` directory is the minimum-viable structure: one folder per page
concern, files named by role (`data.ts`, `markdown.tsx`, `search.ts`,
`Navigation.tsx`, `CvContent.tsx`, etc.). No barrel `index.ts` files are
introduced — imports point directly at the concrete file. This matches the
**Simplicity Over Complexity** principle. [Repo-grounded — principle in AGENTS.md]

### DD-2: Filename mapping (flat targets)

Files keep their existing basenames; only the layer directory is removed:

| From (`src/contexts/...`)                                    | To (`src/features/...`)                         |
| ------------------------------------------------------------ | ----------------------------------------------- |
| `app-shell/presentation/Navigation.tsx`                      | `app-shell/Navigation.tsx`                      |
| `app-shell/presentation/Navigation.unit.test.tsx`            | `app-shell/Navigation.unit.test.tsx`            |
| `app-shell/presentation/style.ts`                            | `app-shell/style.ts`                            |
| `app-shell/presentation/style.unit.test.ts`                  | `app-shell/style.unit.test.ts`                  |
| `cv/application/data.ts`                                     | `cv/data.ts`                                    |
| `cv/application/data.unit.test.ts`                           | `cv/data.unit.test.ts`                          |
| `cv/application/markdown.tsx`                                | `cv/markdown.tsx`                               |
| `cv/application/markdown.unit.test.tsx`                      | `cv/markdown.unit.test.tsx`                     |
| `cv/presentation/CvContent.tsx`                              | `cv/CvContent.tsx`                              |
| `home/presentation/HomeContent.tsx`                          | `home/HomeContent.tsx`                          |
| `personal-projects/application/projects.ts`                  | `personal-projects/projects.ts`                 |
| `personal-projects/presentation/PersonalProjectsContent.tsx` | `personal-projects/PersonalProjectsContent.tsx` |
| `search/application/search.ts`                               | `search/search.ts`                              |
| `search/application/search.unit.test.ts`                     | `search/search.unit.test.ts`                    |
| `search/presentation/SearchSection.tsx`                      | `search/SearchSection.tsx`                      |

Empty stubs deleted: every `<ctx>/domain/index.ts`, `<ctx>/infrastructure/index.ts`,
and the standalone `app-shell/application/index.ts`, `home/application/index.ts`,
`personal-projects/domain/index.ts`, etc. (all empty barrels). [Repo-grounded]

### DD-3: `@/` path alias requires no tsconfig change

`apps/wahidyankf-web/tsconfig.json` maps `"@/*": ["./src/*"]`. [Repo-grounded —
read]. There is **no** `@/contexts`-specific alias, so `@/features/...` resolves
automatically. tsconfig is left untouched. Verify with a typecheck.

### DD-4: Import rewrite map (23 `@/contexts` import statements across 10 files)

[Repo-grounded — every site verified via `grep -rn "@/contexts"`.]

In `src/contexts/home/presentation/HomeContent.tsx`:

- `@/contexts/cv/application/data` → `@/features/cv/data`
- `@/contexts/app-shell/presentation/Navigation` → `@/features/app-shell/Navigation`
- `@/contexts/search/application/search` → `@/features/search/search`
- `@/contexts/cv/application/markdown` → `@/features/cv/markdown`

In `src/contexts/personal-projects/application/projects.ts`:

- `@/contexts/search/application/search` → `@/features/search/search`

In `src/contexts/personal-projects/presentation/PersonalProjectsContent.tsx`:

- `@/contexts/app-shell/presentation/Navigation` → `@/features/app-shell/Navigation`
- `@/contexts/personal-projects/application/projects` → `@/features/personal-projects/projects`

In `src/contexts/cv/presentation/CvContent.tsx`:

- `@/contexts/app-shell/presentation/Navigation` → `@/features/app-shell/Navigation`
- `@/contexts/search/application/search` → `@/features/search/search`
- `@/contexts/cv/application/data` → `@/features/cv/data`
- `@/contexts/cv/application/markdown` → `@/features/cv/markdown`

In `src/app/page.tsx`:

- `@/contexts/home/presentation/HomeContent` → `@/features/home/HomeContent`

In `src/app/cv/page.tsx`:

- `@/contexts/cv/presentation/CvContent` → `@/features/cv/CvContent`

In `src/app/personal-projects/page.tsx`:

- `@/contexts/personal-projects/presentation/PersonalProjectsContent` →
  `@/features/personal-projects/PersonalProjectsContent`

In `src/app/page.unit.test.tsx` (mocks + import):

- `@/contexts/search/application/search` → `@/features/search/search`
- `@/contexts/app-shell/presentation/Navigation` → `@/features/app-shell/Navigation`
- `@/contexts/cv/application/data` → `@/features/cv/data`

In `src/app/cv/page.unit.test.tsx` (mocks):

- `@/contexts/app-shell/presentation/Navigation` → `@/features/app-shell/Navigation`
- `@/contexts/search/application/search` → `@/features/search/search`
- `@/contexts/cv/application/data` → `@/features/cv/data`

In `src/app/personal-projects/page.unit.test.tsx` (mocks):

- `@/contexts/app-shell/presentation/Navigation` → `@/features/app-shell/Navigation`
- `@/contexts/search/application/search` → `@/features/search/search`

> Note: `apps/wahidyankf-web/test/unit/steps/*` step files do **not** import
> `@/contexts`. [Repo-grounded — `grep` returned no matches]. No edits needed
> there.

### DD-5: rhino-cli allowlist edit

`apps/rhino-cli/src/internal/allowlist.rs` [Repo-grounded — read]:

- Remove `"wahidyankf",` from `apps_with_ddd()` (currently 5 entries:
  organiclever, wahidyankf, ose-platform, ayokoding, ose-app).
- In `mod tests::membership`: change `assert_eq!(v.len(), 5)` →
  `assert_eq!(v.len(), 4)`. There is **no** `assert!(v.contains(&"wahidyankf"))`
  assertion currently (the test asserts organiclever, ayokoding, ose-app), so
  none needs removing — but the plan checks for it defensively.
- Remove the `//!   - wahidyankf: ...` rustdoc line from the top doc-comment.

> **Sequencing caveat**: sibling plans may edit this same file/test. Express the
> edit RELATIVELY — decrement whatever `v.len()` value is present at execution
> time by one and remove the wahidyankf line/entry/assertion if present, rather
> than hardcoding 5→4. At authoring time the value is 5. [Repo-grounded]

### DD-6: project.json edit

`apps/wahidyankf-web/project.json` `test:quick` target [Repo-grounded — read]:

- Remove input glob lines for `specs/apps/wahidyankf/ddd/bounded-contexts.yaml`
  and `specs/apps/wahidyankf/ddd/ubiquitous-language/**/*.md`.
- Remove the two `commands[]` entries that run `ddd bc wahidyankf` and
  `ddd ul wahidyankf`, leaving the vitest+coverage-80 command as the sole entry.
- Keep `dependsOn: ["rhino-cli:build"]` (the remaining command still invokes
  `rhino-cli test-coverage validate`).
- Keep the `behavior/web/gherkin/**/*.feature` input glob.
- `spec-coverage` and `test:unit`/`test:integration` targets untouched.

### DD-7: Governance opt-out clause

Add an `## Exemptions` section to
`repo-governance/development/pattern/hexagonal-architecture-web.md` (191 lines;
insert before `## Related`). [Repo-grounded — line count + structure verified].
The clause states: trivially-small static content sites with no IO ports and no
business rules MAY use a flat `src/features/<name>/` layout instead of the
hexagonal `contexts/` layout, documented per-app. Vendor-neutral wording — the
doc lives under `repo-governance/` and has no "Platform Binding Examples"
heading, so the harness-neutrality scan applies to the whole file.

## Harness-Neutrality Note

This plan edits `repo-governance/development/pattern/hexagonal-architecture-web.md`.
The harness-neutrality scan applies. The new clause must avoid vendor-specific
references (no Claude Code / OpenCode / Amazon Q / tool-brand names) and must not
sit under any "Platform Binding Examples" heading (the file has none).

## Dependencies

- No new packages. No version changes.
- `rhino-cli` rebuild required after the Rust edit (`rhino-cli:build` is already a
  `dependsOn` of `test:quick`). [Repo-grounded]

## Testing Strategy

| Acceptance criterion (prd.md)        | Test level / verification                                   |
| ------------------------------------ | ----------------------------------------------------------- |
| DDD spec tree removed                | `test ! -d specs/apps/wahidyankf/ddd`                       |
| project.json no DDD gates            | `grep` assertions on `project.json`                         |
| rhino-cli allowlist drops wahidyankf | `cargo test` (membership unit test) + `grep`                |
| contexts flattened                   | `test -d features`, `test ! -d contexts`, `grep @/contexts` |
| imports resolve                      | `nx run wahidyankf-web:typecheck` (unit-level, existing)    |
| behavior preserved                   | `nx run wahidyankf-web:test:quick` (existing unit suite)    |
| spec-coverage resolves               | `nx run wahidyankf-web:spec-coverage`                       |
| governance clause present            | `grep` for the exemptions heading + key phrase              |
| READMEs accurate                     | `grep` for forbidden terms                                  |
| full gate green                      | `nx affected -t typecheck lint test:quick spec-coverage`    |
| visual smoke                         | Playwright-MCP manual assertion (Home, /cv, /pp, search)    |

Tests are not newly written — the existing unit suite is the behavior guard. The
refactor's RED/GREEN/REFACTOR cycle uses "tests currently green → move files +
rewrite imports → tests green again" as the loop.

## Rollback

Each phase is a self-contained, independently-revertable commit (or small set of
thematic commits). If a phase gate fails and cannot be fixed forward quickly,
`git revert` the phase's commits to return to the last green phase boundary. The
DDD removal (Phase 1) and each context flatten (Phases 2–6) are orthogonal and
revert cleanly in isolation.
