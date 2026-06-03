# BRD — ose-web-remove-ddd

## Business Goal

Realign `apps/ose-web/` with the repository's own architecture governance by removing the
Domain-Driven Design (DDD) scaffolding that was added to a content/marketing site where DDD does
not apply, while preserving the hexagonal feature-module structure that does.

## Business Rationale

The repo's governance document `repo-governance/development/pattern/hexagonal-architecture-web.md`
[Repo-grounded] states plainly that the web `contexts/` directory follows Effect.ts `Context.Tag`
naming and that "DDD applies only to backend apps". `ose-web` nonetheless carries DDD artifacts
left over from `plans/done/2026-05-10__oseplatform-web-ddd-and-specs-format/`:

- a DDD bounded-context spec registry (`specs/apps/ose-platform/ddd/`),
- an `apps_with_ddd()` allowlist entry plus two pre-push `rhino-cli ddd` validators,
- seven empty `domain/` layer folders (every `domain/index.ts` is the stub `export {};`), and
- DDD-framed README prose ("bounded contexts", "Per-BC layout", DDD registry references).

This is governance drift: the codebase asserts one architecture for web apps and the app
implements another. The drift costs nothing to remove (the `domain/` layers are empty and
unimported) and removing it eliminates a recurring source of confusion for any agent or human
reading the app expecting DDD semantics that do not exist.

## Business Impact

### Pain points addressed

- **Governance contradiction** [Judgment call]: `ose-web` claims DDD bounded contexts while
  governance says web apps do not use DDD. A reader cannot trust the app's own documentation.
- **Dead validation cost** [Repo-grounded]: every `ose-web:test:quick` run shells out to two
  `rhino-cli ddd` validators against a registry that describes empty domain layers — work that
  validates DDD constraints the app does not actually express in code.
- **Misleading scaffolding** [Repo-grounded]: seven empty `domain/` folders imply a domain layer
  that has zero content and zero importers, inviting misplaced future code.

### Expected benefits

- Documentation and code agree on a single architecture (hexagonal feature modules).
- Pre-push `test:quick` for `ose-web` drops two `rhino-cli ddd` subprocess invocations. [Repo-grounded]
- `rhino-cli`'s DDD allowlist reflects reality (only genuine DDD apps remain). [Repo-grounded]

## Affected Roles

This is a solo-maintainer repository; the maintainer wears several hats here, and several agents
consume the affected files:

- **Architecture owner** (maintainer hat): owns the governance/code alignment decision.
- **`rhino-cli` maintainer** (maintainer hat): owns the `apps_with_ddd()` allowlist and its tests.
- **Consuming agents**: `apps-ose-web-deployer`, `apps-ose-web-content-maker`, and any plan-execution
  agent that reads `apps/ose-web/README.md` or runs `ose-web:test:quick`.

No sign-off ceremony applies (solo-maintainer repo).

## Business-Level Success Metrics

- **Governance alignment** (observable): `grep` finds zero `ose-platform/ddd` references in
  tracked `ose-web` source/config and in `apps/ose-web/README.md` after the change. [Repo-grounded]
- **No regression** (observable): `nx affected -t typecheck lint test:quick spec-coverage` stays
  green for `ose-web` and `rhino-cli`; `nx build ose-web` passes. [Repo-grounded]
- **Reduced pre-push surface** (observable): `ose-web:test:quick` no longer contains the two
  `rhino-cli ddd bc/ul ose-platform` command lines. [Repo-grounded]

## Business-Scope Non-Goals

- Not changing `ose-web` runtime behavior, routes, content, or rendered output.
- Not removing or weakening the C4 spec tree or the Gherkin `spec-coverage` gate.
- Not removing the `rhino-cli ddd` subcommands (other apps still use them).
- Not refactoring the `application/`/`infrastructure/`/`presentation/` layers or their imports.

## Business Risks and Mitigations

| Risk                                                                              | Likelihood | Mitigation                                                                                                             |
| --------------------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------- |
| Allowlist edit conflicts with sibling plans editing the same file                 | Medium     | Express edits **relatively** (decrement `len`, remove `ose-platform` entry); never assert absolute final counts.       |
| Deleting `domain/` breaks an import                                               | Low        | Verified [Repo-grounded] that nothing outside the `domain/index.ts` barrels imports from `domain/`; barrels are stubs. |
| Breaking `rhino-cli` (a pre-push dependency for other apps)                       | Low        | Rebuild + retest `rhino-cli` (cargo test + `nx build`) inside this plan before declaring done.                         |
| Removing a `test:quick` `inputs` glob silently changes cache keys for other tests | Low        | Remove only the two `ddd/...` globs; keep all behavior/web, behavior/api, src, content, vitest globs intact.           |
