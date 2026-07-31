# Phase 3 — `rhino-cli` Coupling Audit

Read in full, before any change, per delivery.md's Phase 3 "Establish what is actually hardcoded
before changing anything" step. Records, per source file, whether each occurrence of a retired app
name is production behaviour, a test fixture, or a doc comment.

## `apps/rhino-cli/src/commands/specs_validate_counts.rs`

**Classification: test fixture only — no production hardcode, no behaviour change.**

`run_at_root` reads its default area list from
`repo_config::load_or_default(repo_root).specs.ddd_areas` — i.e. from `repo-config.yml`'s
`specs.ddd-areas` key, which Phase 2 already emptied. The `["organiclever", "ose"]` literal lived
only inside the unit test `resolve_folders_default_reads_config_areas`, whose own doc comment
already stated the default is config-supplied rather than hardcoded.

**Action taken**: renamed the fixture strings to `["baseerah"]` and updated the expected
`specs/apps/organiclever` / `specs/apps/ose` assertions to `specs/apps/baseerah`. Confirmed via
`git diff` that every changed line is inside the `#[cfg(test)]` module — no production line touched.

## `apps/rhino-cli/src/application/repo_governance/frontmatter_audit.rs`

**Classification: production behaviour — `WEBSITE_APP_PREFIXES` is an exemption (skip) list.**

`is_website_app` returned `true` for paths under `apps/ayokoding-www/`, `apps/ose-www/`,
`apps/organiclever-app-web/`, `apps/wahidyankf-www/`, and `audit_frontmatter` excluded matching
paths from the frontmatter/date-metadata audit. All four prefixes name deleted apps, so the list was
entirely dead. Emptying it is a real behaviour change (the audit now applies everywhere — strictly
more coverage), and is bound to a Gherkin scenario per Specs & Gherkin Completeness.

**Action taken** (RED → GREEN → REFACTOR):

- RED: added `no_application_path_is_exempt_from_the_audit` (asserts `is_website_app` is `false` for
  both a future `apps/baseerah-fe/` path and the old `apps/ayokoding-www/` path — the latter is the
  genuine failing assertion against the pre-change list) and rewrote `skips_website_apps` (renamed
  `no_longer_skips_former_website_apps`) to assert the audit now reports a finding instead of
  skipping. Bound to a new scenario "No application path is exempt from the frontmatter audit" in
  the already-existing (not newly created) feature file
  `specs/apps/rhino/behavior/rhino-cli/gherkin/md/repo-governance-frontmatter-audit.feature`
  (tag `@repo-governance-frontmatter-audit`, mapped to the `repo-governance frontmatter-audit`
  command via `apps/rhino-cli/src/commands/md_validate_frontmatter_dates.rs`). The delivery.md step
  guessed a different file path (`gherkin/repo-governance/frontmatter-audit.feature`); the actually
  -bound file was used instead per Root Cause Orientation, since duplicating coverage in a
  freshly-invented file would fragment the 1:1 command↔feature mapping.
- GREEN: emptied `WEBSITE_APP_PREFIXES` to `&[]`; updated its doc comment.
- REFACTOR: kept `is_website_app` and the const rather than inlining/removing them — documented as
  the extension point for a future Baseerah content tree that might legitimately need an exemption.
- End-to-end: ran `repo-governance frontmatter audit docs repo-governance` — see delivery.md's Phase
  3 entry for the result and any surfaced findings.

## End-to-end frontmatter audit run — disposition of the 21 findings

Ran the corrected invocation (the plan's guessed `repo-governance frontmatter audit docs
repo-governance` does not exist — `repo-governance` subcommands are
`[vendor, layer-coherence, traceability, workflows, audit, help]`; traced via `cli.rs` to the actual
command):

```text
cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- \
  md frontmatter-dates validate docs repo-governance
```

21 findings, all in `repo-governance/conventions/README.md:125`,
`repo-governance/conventions/structure/README.md:37`,
`repo-governance/conventions/structure/no-date-metadata.md` (17 hits), and
`repo-governance/conventions/structure/no-last-updated.md:19`.

**Disposition: pre-existing false positives, unrelated to this plan's `WEBSITE_APP_PREFIXES` change,
out of scope for Phase 3.** Two independent checks:

1. `WEBSITE_APP_PREFIXES` (before or after this Phase's edit) never covered `repo-governance/` or
   `docs/` — only `apps/ayokoding-www/`, `apps/ose-www/`, `apps/organiclever-app-web/`,
   `apps/wahidyankf-www/`. Emptying it could not have changed the audit's behaviour on these paths;
   the same 21 findings would have surfaced on the pre-Phase-3 binary too.
2. Read every flagged line: all 21 are inline single-backtick code spans (`` `**Last Updated**` ``,
   `` `updated:` ``, `` `- **Last Updated**: date` ``) inside prose that is _documenting_ the forbidden
   pattern in `no-date-metadata.md`/`no-last-updated.md` and cross-referencing it in the two README
   indexes — not actual manually-added date metadata. `audit_frontmatter`'s matcher is plain-text
   regex with no markdown code-span/fence awareness, so it cannot distinguish "this file names the
   banned string as a documentation example" from "this file contains the banned string." Confirmed
   via `grep -rn "frontmatter-dates\|frontmatter_dates\|frontmatter-audit" nx.json project.json
apps/rhino-cli/project.json .husky/ .github/workflows/` (zero hits) and a broader
   `--include="*.json" --include="*.sh" --include="*.yml"` sweep (only an unrelated golden-master test
   fixture) that this exact command has never been wired into any Nx target, Husky hook, or CI
   workflow — so these findings were never live-enforced, before or after Phase 3.

Delivery.md's Phase 3 step text assumed the widened audit would surface findings the exemption had
been _hiding_; that premise doesn't hold here since the exemption never reached these paths. Making
the checker markdown-code-span-aware is a real, separate improvement but is unrelated to pruning the
agent fleet/governance/docs for deleted apps — not undertaken in this plan. No fix applied; findings
recorded here for traceability should a future plan want to pick this up.

## Remaining fixtures/doc comments sweep

Ran `rg -n -i 'ayokoding|organiclever|wahidyankf|crane|ose-www|ose-app|ose-be'
apps/rhino-cli/src/ apps/rhino-cli/tests/`. Since Phase 2 deleted every app directory except
`rhino-cli` itself (confirmed via `ls apps/` → only `README.md` and `rhino-cli`), every hit names a
retired app. Classified each into production-behaviour (fixed) vs. pure test-fixture/doc-example
(left as-is, matching the precedent `tests/ddd.rs`'s own doc comment already sets for using a
plausible-but-arbitrary app name as synthetic test data):

**Production behaviour — fixed:**

- `src/domain/git/staged_files.rs` — `STAGED_SKIP_PREFIXES` carried dead
  `"apps/ayokoding-www/content"` / `"apps/ose-www/content"` entries (neither path can exist).
  Removed both; updated the dependent test fixture in the same file to exercise the still-live
  `"apps/rhino-cli/tests/fixtures"` prefix instead.
- `src/application/git/pre_commit.rs` — `step4_stage_ayokoding` unconditionally ran
  `git add apps/ayokoding-www/content/` on every pre-commit invocation, silently no-oping (exit
  code discarded) since the path can't exist. Deleted the step and its call site; renumbered the
  module doc comment from "8-step" to "7-step" pipeline. Also dropped the same two dead paths from
  `step7_validate_links`'s `skip_paths`. No dedicated test covered step4's presence, so removal is a
  pure dead-code deletion, not a behaviour change requiring new Gherkin coverage; confirmed via
  `cargo test` that `step7_excludes_plans_done_broken_link` (the one test touching that skip list)
  still passes since it only exercises `plans/done`.
- `src/commands/specs_coverage.rs` — the integration test `run_domain_runs_full_scan_for_eligible_project`
  asserted `is_ok()` against `"ose-be"`, commented "IS listed in repo-config.yml's specs.domain-areas".
  Phase 2 emptied `specs.domain-areas` (task #94), so the test now silently exercises the _skip_
  branch instead of the eligible-project branch it claims to cover — a test-integrity regression, not
  a cosmetic staleness issue, since `result.is_ok()` passes on both branches and gave false
  confidence. Verified via `cargo test ... run_domain_runs_full_scan_for_eligible_project -- --nocapture`
  before fixing: output showed `"specs domain-coverage validate: skipped — \"ose-be\" is not listed..."`
  yet the test still reported `ok`. Deleted the test (replaced with a comment explaining why) rather
  than fabricating an isolated temp-git-repo fixture to keep exercising a three-line if/else: the skip
  branch is already covered by `run_domain_skips_project_not_in_domain_areas`, and the `is_eligible`
  predicate itself is unit-tested independently of any real `repo-config.yml` in
  `application::domain_coverage::tests`.
- `src/application/domain_coverage/mod.rs` — renamed its `is_eligible` unit-test fixtures from
  `organiclever-be`/`ose-be` to `baseerah-be`/`example-be` (pure string-parsing test, no behaviour
  change, done for consistency with the app names actually planned for this repo).
- `src/application/doctor/tools.rs` — `global_json` still points at the now-nonexistent
  `apps/ose-be/global.json`; `read_dotnet_v()` already degrades gracefully via `unwrap_or_default()`
  (same pattern as `read_node_v`/`read_npm_v`), so this was never a functional break — Phase 0/1/2
  CI was green throughout. Updated the doc comment only, to record that no app currently owns this
  path and that a future .NET backend would need to re-point it; left the runtime path unchanged
  since inventing a config-driven path registry for a not-yet-scaffolded backend is out of Phase 3's
  scope.
- `tests/cargo_target_share.rs` — an illustrative code comment referenced `nx run ayokoding-cli:build`
  (a project name that was never a real Rust crate in this repo) as a hypothetical example of Nx
  cache-hit verification methodology. Repointed the example at `rhino-cli`, the one real surviving
  Rust crate.
- `src/application/docs/naming.rs` — a regression-test doc comment named the now-deleted
  `apps-ayokoding-www-general-maker` agent as the historical trigger of a real incident. Generalized
  to "a content-maker agent" — the narrative point (a scaffolded `_index.md` first tripped the rule)
  doesn't depend on which specific agent did the scaffolding.

**Pure test-fixture/doc-example — left unchanged** (arbitrary plausible app-name strings with no
correctness implication, same category `tests/ddd.rs`'s own doc comment (lines 29-33) already
justifies as intentional synthetic data): `tests/agents.rs:1278`, `src/application/env/validate.rs`,
`src/commands/md_validate_heading_hierarchy.rs`, `tests/repo_config_data_driven.rs:64` (explicitly
historical/accurate, same pattern as `repo_config/mod.rs`'s own regression comment), `src/commands/env_init.rs`,
`src/commands/md_validate_links.rs:84`, `src/application/env/injection.rs`,
`src/application/bcregistry.rs`, `src/application/glossary.rs`,
`src/application/agents/detect_duplication.rs:456`, `src/application/repo_config/mod.rs:173`
(historical explanation, accurate), `tests/specs_tree.rs`, `tests/ddd.rs`.

**Verification**: `cargo test --manifest-path apps/rhino-cli/Cargo.toml` (full workspace: unit tests +
all cucumber BDD suites) — exit code 0, zero failures.

## `repo-governance/` prose sweep (task #144) — final disposition

Ran twice (5 background agents per round; round 1 crashed to a transient `529 Overloaded` API error,
round 2 re-scoped to the exact files still showing hits and completed clean), plus a direct pass over
`repo-governance/conventions/README.md` and `repo-governance/development/pattern/README.md` (whose
descriptions had drifted stale mid-sweep because their target files were renamed/annotated by other
concurrent batches). `repo-governance/principles/` excluded throughout per Decision 13 (must stay
byte-identical to upstream `ose-public`).

Final consolidated sweep —
`rg -n 'ayokoding|organiclever|wahidyankf|crane-cli|ose-www|ose-app-web|ose-be|ose-cli' repo-governance/ --glob '!repo-governance/principles/**'`
— returns 201 hits across 28 files. Every remaining hit falls into one of five justified categories:

1. **Real identity, not an app name**: `wahidyankf` as GitHub org/licensor (`structure/licensing.md`,
   `formatting/linking.md`, `workflow/reproducible-environments.md`,
   `infra/development-environment-setup.md`) — this is the actual person/org, unaffected by app deletion.
2. **Historical banners already added by this sweep**: explicit "Historical note" / "(Historical)" /
   "Known blocker (regression...)" / "Current State" sections in `programming-language-docs-separation.md`,
   `tutorials/programming-language-content.md`, `infra/ci-conventions.md`,
   `infra/github-actions-workflow-naming.md`, `infra/vercel-deployment.md`, `quality/code.md`,
   `pattern/hexagonal-architecture-cli.md`, `frontend/design-tokens.md`, `frontend/styling.md`,
   `content/pdf-to-md-quality-gate.md`, `quality/specs-application-sync.md`,
   `structure/specs-directory-structure.md`, `infra/development-environment-setup.md` — each
   explicitly states the app was removed and why the mention is retained (reference/calibration/changelog).
3. **Changelog / version-history entries**: `structure/app-readme-vs-specs.md`'s dated changelog table,
   `quality/fixer-confidence-levels.md`'s "Version History" section — accurate historical record of a
   past state, not a claim about current state. Left unedited, matching how git history itself is never
   rewritten for accuracy.
4. **Empirical calibration data in tutorial conventions**: `tutorials/by-concept.md`,
   `tutorials/swe-by-example.md`, `tutorials/in-the-field.md` cite measured production stats from
   `ayokoding-www` (annotation-density ratios, example counts, word counts) as the empirical basis for
   numeric standards the conventions still codify (e.g. "75-85 examples", "1.0-2.25 ratio"). The
   standards apply to any future content-education app; the calibration data's origin doesn't change
   because its source app was removed — same treatment as a style guide citing "derived from analysis
   of N production codebases" after those codebases are archived.
5. **References to plans/ folders that still exist**: `structure/learning-plan-syllabus.md`'s
   `ayokoding-learning-path-0{2,4,5,6,7}-*` corpus examples and `web/web-ux-test-fixing-planning.md`'s
   link into `plans/done/2026-06-19__ayokoding-www-*` point at real, currently-present plan folders
   under `plans/backlog/`, `plans/in-progress/`, and `plans/done/` — not at deleted `apps/` source.
   These are in scope for the pending plan-archive tasks (#153 in-progress triage, #154 backlog triage,
   #152 `plans/done` deletion), not this prose sweep; `learning-plan-syllabus.md` itself will need no
   edit regardless since it only describes the folder-naming/shape convention, not any specific corpus's
   fate.
6. **`conventions/README.md`'s own two "Historical"/"Partially historical" description bullets** (lines
   130, 142) — correctly retain the app name because they are _describing_ the historical scope, exactly
   mirroring what the linked convention files themselves now say.

No further edits required. `repo-governance/` prose sweep converged.
