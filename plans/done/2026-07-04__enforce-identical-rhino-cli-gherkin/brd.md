# Business Requirements — Enforce Identical, Fully-Enforcing rhino-cli Gherkin

## Business Goal

`rhino-cli` is the **enforcement tool** for the three-repo OSE platform — it runs the pre-commit,
pre-push, and CI quality gates that keep `ose-public`, `ose-primer`, and `ose-infra` governed by one
set of rules. Its source is already byte-identical across all three. This plan closes the last gap: make
its **behaviour** genuinely identical and **actually enforced** everywhere, by making its Gherkin
behaviour specification byte-identical across the three repos and ensuring every scenario in it truly
executes.

The business value is **trustworthy governance**: when a contributor (human or agent) works in any of
the three repos, the same rules are enforced the same way, and a green rhino-cli run means the behaviour
was really checked — not silently skipped.

## Why Now

Two prior plans ([2026-07-01](../../done/2026-07-01__standardize-rhino-cli-sdlc-parity/README.md),
[2026-07-03](../../done/2026-07-03__unify-rhino-cli-sdlc-parity/README.md)) both **claimed** an
`"identical"` end-state and both **left the Gherkin tree diverged** — the second one even documented the
skip-by-data shortcut in its own audit notes as acceptable to keep CI green. The result today:

- **53% of rhino-cli behaviour scenarios do not execute** (`121 / 228` skipped-by-data in `ose-public`),
  including the **entire** `repo-governance` and `workflows` surfaces. [Repo-grounded]
- **`ose-primer` ships a different, staler behaviour spec** than `ose-public`/`ose-infra`. [Repo-grounded]

This is a governance-integrity problem: the gate reports "pass" for behaviour it never ran. Left alone,
it recurs on every command rename — the tree is outside any identity gate. Fixing it now, plus adding
the anti-drift gate, ends the recurrence.

## Impact & Affected Roles

| Role                          | Impact                                                                                      |
| ----------------------------- | ------------------------------------------------------------------------------------------- |
| Repo maintainer (solo)        | One canonical, fully-enforcing behaviour spec to reason about; drift can't silently return. |
| AI coding agents              | Working cross-repo is truly identical — same commands, same enforced behaviour, same specs. |
| CI / quality gates            | A green rhino-cli run now means behaviour was actually exercised, not skipped.              |
| Downstream `ose-primer` users | The public scaffolding template ships the same enforced governance as the source repo.      |

## Business Success Metrics

- **Zero skipped-by-data scenarios** in the rhino-cli cucumber suite in **all three** repos
  (observable: `cargo test --manifest-path apps/rhino-cli/Cargo.toml -p rhino-cli` output reports `0 skipped` across every binary). [Judgment call:
  target derived directly from the user directive "all behaviour should be enforced"]
- **Byte-identical Gherkin tree** across all three repos (observable: `diff -rq` of the
  `.feature` + behaviour-`README.md` set reports no differences; md5 manifests match).
- **Every leaf rhino-cli command maps to ≥ 1 executing scenario** (observable: command-census ↔
  feature-coverage map in Phase 0 audit shows no uncovered leaf command).
- **Recurrence closed** — the SDLC gate standard + parity workflow explicitly cover the Gherkin tree
  (observable: the boundary doc lists the path; the workflow has a verification step).

## Business-Scope Non-Goals

- Not changing what any validator decides (no rule/logic changes) — this is a parity + enforcement plan.
- Not unifying the C4 architecture prose (`product/`, `system-context/`, `components/`, `containers/`).
- Not building new runtime tooling for drift detection — the anti-drift mechanism reuses the existing
  cross-repo parity gate.
- No time estimates (per repo policy) — success is defined by the observable outcomes above.

## Business Risks

| Risk                                                                            | Severity | Mitigation                                                                                                                                                                                                       |
| ------------------------------------------------------------------------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| De-hollowing surfaces **real, previously-hidden validator failures** in a repo. | Medium   | Expected and desirable — Phase 0 records the baseline; each surfaced failure is fixed at root cause per policy before its phase gate.                                                                            |
| Touching byte-identical `tests/*.rs` risks re-introducing source drift.         | High     | Single canonical edit in public, propagated verbatim; golden-master + `diff -rq apps/rhino-cli` gate every repo.                                                                                                 |
| A repo lacks a toolchain a de-hollowed scenario needs.                          | Low      | Ported validators are pure-Rust (no external toolchain at validation time — verified by predecessor); tag `@requires-<x>` + skip-by-data only if a genuine toolchain dependency is found, documented explicitly. |
| Pushing three `main` branches at once.                                          | Medium   | Per-repo phase gates (each repo passes its own full gate before its push); staged, explicit-path commits; CI watched per repo.                                                                                   |
