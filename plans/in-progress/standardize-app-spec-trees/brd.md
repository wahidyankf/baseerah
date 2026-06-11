# BRD — Standardize App Spec Trees

## Business Goal

Make the `specs/` layout uniform across every app family so contributors and AI agents can locate,
validate, and maintain specifications by one predictable rule. Two predictable rules must hold with
**zero exceptions**:

1. **One tree per family** — `apps/<family>-*` maps to exactly one `specs/apps/<family>/` tree.
2. **One naming scheme for behavior dirs** — every behavior surface is
   `specs/apps/<family>/behavior/<product>-<surface>/gherkin/` (the **flat product-surface**
   scheme), with `be` as the only backend-HTTP perspective name (never `api`).

Today OSE breaks rule 1 by spanning two trees (`ose-app`, `ose-platform`), and several families
break rule 2: the convention currently names behavior dirs bare-perspective
(`behavior/be|web|cli/gherkin/`) and two families still use `api` (`ose-platform`, `ayokoding`).
This forces a one-off mental model and makes cross-family tooling (spec-coverage, `specs-checker`,
the ose-primer sync) special-case OSE and tolerate naming drift. `[Repo-grounded]`

## Why Now

- The one-family-one-tree rule is documented in
  [`specs-directory-structure.md`](../../../repo-governance/conventions/structure/specs-directory-structure.md)
  but OSE silently violates it. A documented-but-unenforced rule erodes trust in the convention.
- A second OSE deployable family (`ose-app-mobile`) is already foreseen in `AGENTS.md`.
  `[Repo-grounded]` Fixing the layout before more `ose-*` apps land avoids compounding divergence.
- The user has asked the standard apply to **all `apps/` families across three sibling repos**
  (ose-public, ose-primer, ose-infra) so the on-disk reality matches the written convention
  everywhere and the convention can be enforced rather than aspirational.

## Business Impact

- **Lower cognitive load** — one rule for spec location and one naming scheme across the whole repo
  and the wider ecosystem.
- **Cheaper tooling** — spec-coverage commands, `specs-checker`, and ose-primer sync stop needing
  OSE-specific branches and stop tolerating `api`/`be` drift.
- **Convention integrity** — the written standard and the on-disk reality agree, so the convention
  can be enforced by `specs-checker`.
- **Cross-repo parity** — the convention amendment text is byte-identical between ose-public and
  ose-primer (bidirectional/identity in the primer-sync classifier), so the standard propagates
  without manual reconciliation.

## Affected Roles

The maintainer wears several hats here; AI agents consume the resulting files:

- **Contributor / AI agent** — gains a single deterministic spec-location rule and one naming scheme.
- **Spec authors (`specs-maker`)** — gets an explicit template for the flat product-surface scheme,
  including the multi-product (OSE) and single-product (organiclever) cases.
- **Spec validators (`specs-checker`)** — gains an enforceable rule covering one-tree-per-family,
  flat product-surface dirs, and the `be`-over-`api` perspective name.
- **App maintainers** — every `apps/<family>-*` project references one tree with consistent dirs.

## Business-Level Success Metrics

- **Single-tree coverage**: every `apps/ose-*` project references `specs/apps/ose/`; repo-wide grep
  for `specs/apps/ose-app` or `specs/apps/ose-platform` returns zero hits outside `plans/`.
  _[Judgment call: binary pass/fail, not a tuned KPI.]_
- **Uniform naming**: every behavior dir matches `behavior/<product>-<surface>/gherkin/`; repo-wide
  grep for the bare-surface forms (`behavior/be/gherkin`, `behavior/web/gherkin`,
  `behavior/cli/gherkin`, `behavior/api/gherkin`, `behavior/build-tools/gherkin`) returns zero hits
  outside `plans/`. _[Judgment call: binary pass/fail.]_
- **Convention–reality agreement**: `specs/apps/` contains exactly one directory per app family;
  `specs-checker` passes with the amended rule active.
- **No regression**: affected `nx affected -t spec-coverage`, `test:quick`, and e2e suites stay
  green through every phase.

## Business-Scope Non-Goals

- No change to product behavior, deployment topology, or `apps/*` project names.
- No new product scope, features, or Gherkin scenarios.
- No restructuring of sibling repos (ose-primer, ose-infra) — each owns its own parity plan.

## Business Risks

| Risk                                                              | Mitigation                                                                                                    |
| ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| Broken spec-coverage / e2e wiring after path moves                | Phased delivery with a green gate after each phase; rewrite refs atomically per surface                       |
| Lost git history on moved spec files                              | Use `git mv` for every relocation so blame/history follow the file                                            |
| rhino-cli Rust source carries hardcoded default spec paths        | Treat rhino source-default updates as TDD-shaped code steps (RED→GREEN→REFACTOR), not blind string swaps      |
| Convention amendment contradicts existing single-deployable rule  | Amend as an additive flat product-surface subsection; re-validate with `specs-checker` / repo-rules-checker   |
| Stale references survive in generated artifacts (`.features-gen`) | Regenerate playwright-bdd artifacts and grep for residue in the Phase gates                                   |
| Divergence from sibling repos' convention text                    | Author the amendment so ose-primer can adopt it byte-identical; record the deviation matrix in `tech-docs.md` |
