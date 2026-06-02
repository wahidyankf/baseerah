# Business Requirements Document — ayokoding-web Remove DDD

## Business goal

Realign `ayokoding-web` with the repository's own architecture governance by removing the
Domain-Driven Design (DDD) scaffolding that was layered onto a web app where DDD does not apply.
The result is a smaller, more honest codebase whose documentation and tooling match the
[Hexagonal Architecture — Web Apps](../../../repo-governance/development/pattern/hexagonal-architecture-web.md)
convention, which explicitly states **"DDD applies only to backend apps"**. [Repo-grounded]

## Business rationale (WHY)

The earlier `2026-05-10__ayokoding-web-ddd-and-specs-format` plan introduced DDD artifacts
(a bounded-context registry, ubiquitous-language glossary files, an `apps_with_ddd()` allowlist
entry, two pre-push validation commands, and six empty `domain/` layers) to a Next.js content
site. These artifacts impose ongoing maintenance and validation cost without delivering value:

- The six `domain/` layers are **empty** — every `domain/index.ts` barrel is blank and no file
  outside those barrels imports from `domain/`. [Repo-grounded — `grep` found zero importers]
- The DDD pre-push commands (`rhino-cli ddd bc ayokoding`, `rhino-cli ddd ul ayokoding`) run on
  every `test:quick`, adding `cargo run` startup cost and a class of failure unrelated to the
  app's actual behavior. [Repo-grounded — present in `test:quick` command array]
- The DDD vocabulary in the app README contradicts the governance doc that governs this app,
  creating a documentation/governance conflict that confuses any contributor (human or agent)
  who reads both. [Repo-grounded]

Removing these accretions is a net simplification that brings the app back into compliance.

## Business impact

| Dimension            | Before                                                         | After                                                             |
| -------------------- | -------------------------------------------------------------- | ----------------------------------------------------------------- |
| Governance alignment | README + tooling assert DDD on a web app (contradicts policy)  | README + tooling describe hexagonal feature modules (compliant)   |
| Pre-push cost        | Two extra `cargo run` DDD validations per `test:quick`         | Those two validations removed for this app                        |
| Spec surface         | `specs/apps/ayokoding/ddd/` (10 files) maintained              | DDD subtree deleted; C4 + Gherkin specs retained                  |
| Source clarity       | Six empty `domain/` layers imply structure that does not exist | Three real layers (`application`/`infrastructure`/`presentation`) |

All impacts are qualitative simplifications. No user-facing behavior changes. [Judgment call]

## Affected roles

This is a solo-maintainer repository; the maintainer wears multiple hats, and several agents
consume the affected files:

- **Maintainer (architecture hat)** — owns the governance/structure decision.
- **Maintainer (web-app hat)** — maintains `ayokoding-web` source and README.
- **`rhino-cli` consumers** — `organiclever-be`, `organiclever-web`, `ose-app-be` still rely on
  `rhino-cli ddd` and on `apps_with_ddd()`; the allowlist edit must keep them working.
- **Agents that read the README/governance** — any planning or content agent reading
  `apps/ayokoding-web/README.md` should see consistent, non-DDD language.

No sign-off ceremony applies (solo-maintainer repo).

## Business-level success metrics

- **Governance conflict eliminated**: zero DDD/"bounded context" assertions remain in
  `apps/ayokoding-web/README.md` after the rewrite. _Observable check_: `grep -i "bounded
context\|DDD" apps/ayokoding-web/README.md` returns nothing (excluding any historical-context
  prose explicitly retained). [Repo-grounded — verifiable post-change]
- **No dangling DDD references**: `grep -rn "specs/apps/ayokoding/ddd" .` (excluding `plans/`)
  returns zero matches. _Observable check_. [Repo-grounded — verifiable post-change]
- **No regression**: `nx affected -t typecheck lint test:quick spec-coverage` stays green for
  `ayokoding-web` and `rhino-cli`. _Observable check_.

## Business-scope non-goals

- Not re-architecting the `contexts/` modules or changing dependency directions.
- Not removing DDD from other apps (`organiclever`, `wahidyankf`, `ose-platform`, `ose-app`).
- Not touching the C4 or Gherkin spec trees, nor the `spec-coverage` gate.
- Not removing the `rhino-cli ddd` subcommands (other apps depend on them).

## Business risks and mitigations

| Risk                                                                       | Likelihood | Mitigation                                                                                                                                                 |
| -------------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Allowlist edit collides with sibling plans editing the same file/test      | Medium     | Express the `allowlist.rs` test edit **relatively** (decrement `len` by 1; remove only the `ayokoding` lines) so order-independence holds. [Repo-grounded] |
| Removing DDD `inputs` globs leaves a stale `test:quick` cache key mismatch | Low        | Phase gate re-runs `test:quick` after the edit; Nx recomputes the cache from the new `inputs`.                                                             |
| Deleting `domain/` folders breaks a hidden importer                        | Very low   | `grep` already confirmed zero importers; Phase gate runs `typecheck` which would catch any break.                                                          |
| README rewrite introduces vendor-specific or inaccurate architecture prose | Low        | Rewrite cites `hexagonal-architecture-web.md`; README is app/tooling doc (vendor-neutral), no harness content.                                             |
