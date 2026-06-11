# BRD — Standardize App Spec Trees

## Business Goal

Make the `specs/` layout uniform across every app family so contributors and AI agents can
locate, validate, and maintain specifications by one predictable rule — `apps/<family>-*` maps to
`specs/apps/<family>/` — with **zero exceptions**. Today OSE breaks that rule by spanning two
spec trees (`ose-app`, `ose-platform`), forcing everyone to learn a one-off mental model and
making cross-family tooling (spec-coverage, specs-checker, the ose-primer sync) special-case OSE.

## Why Now

- The repo already documents the one-family-one-tree rule in
  [`specs-directory-structure.md`](../../../repo-governance/conventions/structure/specs-directory-structure.md),
  but OSE silently violates it. A documented-but-unenforced rule erodes trust in the convention.
- A second deployable family (`ose-app-mobile`) is already foreseen in `AGENTS.md`. Fixing the
  layout before more `ose-*` apps land avoids compounding the divergence.
- The user has asked that the consolidated shape become the **standard for all `apps/`
  projects**, not a one-off cleanup — so the convention must gain a first-class
  "multi-deployable family" rule rather than leaving OSE as an undocumented special case.

## Business Impact

- **Lower cognitive load**: one rule for spec location across the whole repo.
- **Cheaper tooling**: spec-coverage commands, `specs-checker`, and ose-primer sync stop needing
  OSE-specific branches.
- **Convention integrity**: the written standard and the on-disk reality agree, so the
  convention can be enforced rather than aspirational.

## Affected Roles

- **Contributors / AI agents** — gain a single deterministic spec-location rule.
- **Spec authors (`specs-maker`)** — get an explicit template for multi-deployable families.
- **Spec validators (`specs-checker`)** — gain an enforceable rule covering the multi-deployable
  case and the `be` perspective name.
- **OSE app maintainers** — all `ose-*` projects reference one tree.

## Business-Level Success Metrics

- **Single-tree coverage**: every `apps/ose-*` project references `specs/apps/ose/` and nothing
  references `specs/apps/ose-app/` or `specs/apps/ose-platform/` (observable: repo-wide grep
  returns zero stale references). _[Judgment call: binary pass/fail, not a tuned KPI.]_
- **Convention–reality agreement**: `specs/apps/` contains exactly one directory per app family;
  `specs-checker` passes with the amended rule active.
- **No regression**: affected `nx affected -t spec-coverage`, `test:quick`, and e2e suites stay
  green through every phase.

## Business-Scope Non-Goals

- No change to product behavior, deployment topology, or app naming.
- No new product scope, features, or Gherkin scenarios.
- No reorganization of non-OSE families beyond a read-only conformance check.

## Business Risks

| Risk                                                              | Mitigation                                                                                                       |
| ----------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Broken spec-coverage / e2e wiring after path moves                | Phased delivery with a green gate after each phase; rewrite refs atomically per surface                          |
| Lost git history on moved spec files                              | Use `git mv` for every relocation so blame/history follow the file                                               |
| Convention amendment contradicts existing single-deployable rule  | Amend as an additive "multi-deployable family" subsection; re-validate with `specs-checker` / repo-rules-checker |
| Stale references survive in generated artifacts (`.features-gen`) | Regenerate playwright-bdd artifacts and grep for residue in the Phase gates                                      |
