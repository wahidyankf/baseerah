# Business Requirements: Cross-Repo Port Registry

## Problem Statement

Port allocation is currently a manual, prose-based process: an engineer or agent adding a new app
must read every sibling repo's `docs/reference/monorepo-structure.md` table by eye and pick an
unclaimed port. Nothing enforces that the chosen port is actually free across all four repos, and
nothing catches drift if a table goes stale. The failure mode is silent — two apps in different
repos both claiming the same port only surfaces when someone tries to run both at once.

## Impact

**Affected roles**: any engineer or AI agent scaffolding a new app anywhere under
`/Users/wkf/ose-projects/` (`ose-public`, `ose-primer`, `ose-private`, `baseerah`), since all four
can run concurrently on the same machine.

## Success Metrics

Zero manual cross-repo table review required to allocate a new port; a collision is caught by an
automated check rather than by a developer noticing a bind failure at runtime.

## Risks

- **Undecided ownership could stall the fix.** The registry's home (a shared `repo-config.yml` key,
  a dedicated file, or a `rhino-cli` validator) is left open for the Phase 1 investigation to
  resolve before any code lands — the risk is scheduling drift, not a wrong technical choice.
- **Four-repo coordination.** A registry that spans repos needs either a shared location (e.g. a
  file synced via the existing `ose-public`/`ose-primer` parity loop) or a convention each repo
  independently checks against — Phase 1 must pick one before implementation.
