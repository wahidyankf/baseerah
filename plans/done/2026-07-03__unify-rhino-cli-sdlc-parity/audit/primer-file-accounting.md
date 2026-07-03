# Synthesis Ledger — File Accounting (primer-only files)

Read-only investigation, 2026-07-02. Enumerates every file under `apps/rhino-cli/src/` and
`apps/rhino-cli/tests/` that exists in `ose-primer` but NOT at the same relative path in
`ose-public` (85 files, as pre-enumerated by the requester), and records a disposition for each:
`ported` (pull into public as a new standalone file), `merged` (its logic is redundant with a file
that is _also_ being ported or that already exists in public — do not copy as a second standalone
file), or `dropped-with-reason` (dead/orphaned/superseded — do not port).

Method: for every candidate, traced Rust module reachability from `src/lib.rs` down through
`internal.rs` / `application/mod.rs` / `commands.rs`'s literal `pub mod` declarations (not
assumptions), cross-checked with `grep -rl` for external references, and — where the flat-file-vs-
directory shadowing pattern mattered (`foo.rs` co-existing with `foo/`) — confirmed empirically with
`cargo check`/`cargo build` in the primer checkout. Cross-referenced the plan's own prior audit
(`audit/01-rhino-cli-src-diff.md`, `audit/05-cucumber-sweep.md`) and `delivery.md`'s unchecked Phase 1
task list, which independently corroborate every finding below.

## Summary

| Disposition           |  Count | Notes                                                                                                                                           |
| --------------------- | -----: | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `ported`              |     31 | New standalone files public genuinely lacks and Phase 1 wants                                                                                   |
| `merged`              |     11 | `internal/testcoverage/*` — redundant with the `application/testcoverage/*` copy that is itself being `ported`; do not also copy this directory |
| `dropped-with-reason` |     43 | Dead/orphaned/superseded — confirmed unreachable or strictly superseded by code already live in public                                          |
| **Total**             | **85** | Matches the requester's pre-enumerated count                                                                                                    |

## Flagged for human / follow-up-agent double-check (not guessed)

1. **The entire `test-coverage` command surface is currently 100% unreachable in primer itself** —
   not merely "less tested." `internal.rs` does not declare `pub mod testcoverage;` at all (confirmed
   by reading the file — it lists agents, allowlist, bcregistry, contracts, docs, doctor, envbackup,
   envinjection, envvalidate, git, glossary, java, naming, repo_governance, severity, speccoverage,
   specs — no `testcoverage`). `application/mod.rs` likewise never declares `pub mod testcoverage;`.
   `internal/testcoverage.rs` (flat shim) and `application/testcoverage/mod.rs` glob-re-export
   **from each other** (`pub use crate::application::testcoverage::*;` / `pub use
crate::internal::testcoverage::*;`) — a circular pair that would fail to compile if either were
   ever actually wired in. `cli.rs` has an explicit regression test
   (`test_coverage_validate_no_longer_parses`, "§2a-cov RED: test-coverage validate command removed")
   proving the CLI surface was deliberately decommissioned. Despite this, `delivery.md`'s Phase 1 task
   list (lines 108-109, unchecked) explicitly plans to pull this module in and wire it into the union
   command surface. This ledger marks the relevant files `ported`/`merged` in line with that documented
   intent, **but flags that file-copy alone is insufficient** — landing this feature also requires: (a)
   adding `pub mod testcoverage;` to `application/mod.rs`, (b) giving `application/testcoverage/mod.rs`
   real `pub mod cobertura;` etc. declarations instead of its current circular re-export, (c) adding
   `pub mod testcoverage;`/`pub mod test_coverage_validate;` to `commands.rs`, (d) adding CLI enum
   variants in `cli.rs`, and (e) fixing the two command files' `use crate::internal::testcoverage::*`
   imports to point at `crate::application::testcoverage::*` (or wherever it lands). None of that
   wiring work is captured by this file-accounting ledger — it belongs to the delivery plan's
   implementation tasks.

2. **Public's own `internal/testcoverage.rs`** (already present at that path in both repos, so it is
   NOT one of the 85 files in scope) is _also_ a circular shim pointing at a non-existent
   `application::testcoverage` module. It is a pre-existing latent dangling reference in public, not
   something this port creates — but once `application/testcoverage/*` is ported and wired per item 1,
   this existing shim file should be reviewed (its `pub use` will finally resolve, which is the
   intended fix, but worth a explicit look rather than assuming it "just works").

3. **The 9 cucumber `tests/*.rs` files marked `ported` require adaptation, not verbatim copy.** Primer
   pins `cucumber = "0.22.1"`; public/infra are already on the canonical `0.23.0`. `delivery.md`
   explicitly calls for "migrating the harness code from cucumber 0.22.1 to the canonical 0.23.0 API"
   and reconciling the `.feature` directory tree to the **union** of all three repos' trees (Decision 15) — not a verbatim copy, since a verbatim copy would delete public's own `ddd`/`specs`/`workflows`
   `.feature` dirs that primer lacks. This ledger's `ported` disposition for these 9 files means "the
   step-definition Rust source is worth pulling in," not "copy these bytes unchanged into public."

4. **`internal/testcoverage/*` vs `application/testcoverage/*` — confirmed genuine near-duplicate, not
   identical.** Diffed `cobertura.rs` from both: same struct shapes and parsing logic, but
   `application/testcoverage/cobertura.rs` carries full rustdoc (`//!`/`///`) comments explicitly
   labeled "Byte-for-byte port of `apps/rhino-cli/internal/testcoverage/cobertura_coverage.go`" while
   `internal/testcoverage/cobertura.rs` has only terse `//` line comments and no per-field docs — i.e.
   `application/` is a later, more-documented evolution of the same logic, not a fork with different
   behavior. High confidence in `merged` disposition for the `internal/` copy, but a follow-up agent
   doing the actual port should still diff all 11 file-pairs (not just `cobertura.rs`) before assuming
   zero unique logic in the `internal/` copies.

5. **`internal/agents/naming.rs`'s `AGENT_ROLES` list (6 entries: maker, checker, fixer, dev,
   deployer, manager) is narrower than the live `commands/harness_validate_naming.rs`'s local
   `AGENT_ROLES` (8 entries, adds `tester`, `researcher`).** This is additional evidence the dead file
   is a stale predecessor, not a parallel-but-different feature — but flagging the discrepancy in case
   a follow-up agent wants to confirm no role was _lost_ in the transition (unlikely, since the live
   list is a superset, but worth a glance).

## `src/application/testcoverage/*` (11 files) — ported

| File             | Disposition | Reason                                                                                                                                                                                                                |
| ---------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cobertura.rs`   | ported      | Real, richly-documented Cobertura XML parser; canonical (more-evolved) copy of the coverage-parsing logic. Not yet declared in `application/mod.rs` in either repo — needs wiring (see flag 1).                       |
| `detect.rs`      | ported      | Coverage-format auto-detection logic; same "not yet wired" status as its siblings.                                                                                                                                    |
| `diff.rs`        | ported      | Diff-coverage computation, referenced by primer's (currently dead) `commands/testcoverage.rs`.                                                                                                                        |
| `exclude.rs`     | ported      | Exclude-pattern matching for coverage files; referenced by primer's dead `commands/testcoverage.rs` and `test_coverage_validate.rs`.                                                                                  |
| `go_coverage.rs` | ported      | Go `cover.out` format parser; referenced by primer's dead `commands/test_coverage_validate.rs`.                                                                                                                       |
| `jacoco.rs`      | ported      | JaCoCo XML format parser.                                                                                                                                                                                             |
| `lcov.rs`        | ported      | LCOV format parser.                                                                                                                                                                                                   |
| `merge.rs`       | ported      | Coverage-map merge logic, referenced by primer's dead `commands/testcoverage.rs`.                                                                                                                                     |
| `mod.rs`         | ported      | Currently only a circular `pub use crate::internal::testcoverage::*;` re-export (see flag 1) — porting requires rewriting it to declare `pub mod cobertura;` etc. directly, not copying the current content verbatim. |
| `reporter.rs`    | ported      | Coverage-report formatting (text/json).                                                                                                                                                                               |
| `types.rs`       | ported      | Shared `FileResult`/`Format`/`Result` domain types used by all the above.                                                                                                                                             |

## `src/commands/*.rs` (4 files) — 2 ported, 2 dropped

| File                        | Disposition         | Reason                                                                                                                                                                                                                                                                                                                                                                                                                               |
| --------------------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `git.rs`                    | dropped-with-reason | Not declared in `commands.rs` (`pub mod git_pre_commit;` is declared, not `pub mod git;`) — confirmed unreachable. Duplicates the already-live `commands/git_pre_commit.rs` (already present at the same path in public), which itself calls the live `application::git::pre_commit::run`. This file instead calls the dead `internal::git::runner`. Redundant, superseded, would collide in purpose with the existing live command. |
| `specs_validate_links.rs`   | dropped-with-reason | Pre-confirmed by the task: undeclared anywhere, referenced only by `specs_validate_links_no_longer_parses` (a CLI test asserting the command was intentionally removed). Also independently corroborated by `delivery.md` line 108, which explicitly excludes this exact file for this exact reason.                                                                                                                                 |
| `test_coverage_validate.rs` | ported              | Implements the `test-coverage validate` command body; currently unreachable (`commands.rs` doesn't declare it) but `delivery.md` explicitly plans to revive it as part of the union command surface (see flag 1). Imports need retargeting from `internal::testcoverage::*` to wherever the ported module lands.                                                                                                                     |
| `testcoverage.rs`           | ported              | Implements the `test-coverage {validate,diff,merge}` family; same unreachable-but-wanted status and import-retargeting caveat as `test_coverage_validate.rs`.                                                                                                                                                                                                                                                                        |

## `src/internal/agents/*` (13 files) — all dropped

`internal.rs` declares `pub mod agents;`, which resolves to the flat `internal/agents.rs` (3 lines:
`pub use crate::application::agents::*;`) because Rust's module resolution prefers `foo.rs` over
`foo/` when both exist for the same `mod foo;` declaration (empirically confirmed: `cargo check`
succeeds, and `internal/agents.rs` declares no submodules, so nothing in `internal/agents/` is ever
referenced by any `mod` statement anywhere in the crate). `application/agents/*` — the live target of
the shim — already exists at the same path in public (not one of the 85 files), so none of these
files add anything public lacks.

| File                    | Disposition         | Reason                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ----------------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_validator.rs`    | dropped-with-reason | 3-line re-export shim to `application::agents::agent_validator::*`, which already lives in public at the same path.                                                                                                                                                                                                                                                                                                                         |
| `bindings.rs`           | dropped-with-reason | Same pattern, target `application::agents::bindings`.                                                                                                                                                                                                                                                                                                                                                                                       |
| `claude_validator.rs`   | dropped-with-reason | Same pattern, target `application::agents::claude_validator`.                                                                                                                                                                                                                                                                                                                                                                               |
| `converter.rs`          | dropped-with-reason | Same pattern, target `application::agents::converter`.                                                                                                                                                                                                                                                                                                                                                                                      |
| `detect_duplication.rs` | dropped-with-reason | Same pattern, target `application::agents::detect_duplication`.                                                                                                                                                                                                                                                                                                                                                                             |
| `frontmatter.rs`        | dropped-with-reason | Same pattern, target `application::agents::frontmatter`.                                                                                                                                                                                                                                                                                                                                                                                    |
| `naming.rs`             | dropped-with-reason | NOT a tiny shim (8.5K, real `validate_naming()` orchestration + local `AGENT_ROLES`) but still unreachable via the directory-shadowing rule above. Its logic is superseded by the live `commands/harness_validate_naming.rs` (already present at the same path in public), which implements the same `.claude`/`.opencode` walk more completely (repo-config-driven tier resolution, 8-entry `AGENT_ROLES` vs. this file's 6 — see flag 5). |
| `reporter.rs`           | dropped-with-reason | Same shim pattern, target `application::agents::reporter`.                                                                                                                                                                                                                                                                                                                                                                                  |
| `skill_validator.rs`    | dropped-with-reason | Same pattern, target `application::agents::skill_validator`.                                                                                                                                                                                                                                                                                                                                                                                |
| `sync_validator.rs`     | dropped-with-reason | Same pattern, target `application::agents::sync_validator`.                                                                                                                                                                                                                                                                                                                                                                                 |
| `sync.rs`               | dropped-with-reason | Same pattern, target `application::agents::sync`.                                                                                                                                                                                                                                                                                                                                                                                           |
| `types.rs`              | dropped-with-reason | Same pattern, target `application::agents::types`.                                                                                                                                                                                                                                                                                                                                                                                          |
| `yaml_formatting.rs`    | dropped-with-reason | Same pattern, target `application::agents::yaml_formatting`.                                                                                                                                                                                                                                                                                                                                                                                |

## `src/internal/cliout/*` (2 files) — all dropped

`internal.rs`'s `pub mod` list has no `cliout` entry at all (not even shadowed — genuinely absent),
so this directory is 100% unreachable from the crate root. Only referenced by other already-dead files
(`internal/docs/*`, `internal/naming/reporter.rs`, `internal/mermaid/reporter.rs`,
`commands/testcoverage.rs`) — zero references from any live file. The live equivalent,
`domain/cliout.rs` (same sealed `OutputFormat` enum, same parse/code/Display logic), already exists at
that path in public.

| File        | Disposition         | Reason                                                                                                                                   |
| ----------- | ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `gojson.rs` | dropped-with-reason | Not declared anywhere in `internal.rs`; only referenced by other dead files. HTML-escaping helper, no live callers.                      |
| `mod.rs`    | dropped-with-reason | Not declared anywhere; the `OutputFormat` enum it defines is fully superseded by the live `domain/cliout.rs`, already present in public. |

## `src/internal/contracts/*` (4 files) — all ported

`internal.rs` declares `pub mod contracts;`; no flat `internal/contracts.rs` file exists to shadow the
directory, so `internal/contracts/mod.rs` is the direct, live resolution. Confirmed live callers:
`commands/lang_java_validate_null_safety.rs`, `commands/specs_clean_java_imports.rs`,
`commands/specs_scaffold_dart.rs` all reference `internal::contracts::*`. Public already has these
three command files but they are explicit **"Dormant in ose-public — ships for union CLI parity"**
stubs whose doc comments say they "activate when Java/Dart contract codegen lands" — porting this
directory is exactly what would let those stubs become real. (`internal/contracts/reporter.rs` was
already deleted upstream as superseded — correctly absent from both the 85-file list and this table.)

| File                    | Disposition | Reason                                                                                                               |
| ----------------------- | ----------- | -------------------------------------------------------------------------------------------------------------------- |
| `dart_scaffold.rs`      | ported      | Real Dart-scaffold-generation logic backing the currently-dormant `commands/specs_scaffold_dart.rs` stub in public.  |
| `java_clean_imports.rs` | ported      | Real Java-import-cleaning logic backing the currently-dormant `commands/specs_clean_java_imports.rs` stub in public. |
| `mod.rs`                | ported      | Module doc + `pub mod` declarations wiring the two files above; needed for the directory to compile.                 |
| `types.rs`              | ported      | Shared `JavaCleanImportsOptions`/`JavaCleanImportsResult` (etc.) domain types used by the two logic files.           |

## `src/internal/docs/*` (9 files) — all dropped

`internal.rs` declares `pub mod docs;`, which resolves to the flat `internal/docs.rs` (3-line shim
re-exporting `application::docs::*`) — the directory is shadowed exactly like `internal/agents/`.
The live `application/docs/{frontmatter,heading_hierarchy,links,naming}.rs` (4 files, all already
present in public) is a genuinely different, evolved rewrite — confirmed by diffing `heading_hierarchy.rs`
(26.8K dead vs. 24.0K live) and `links.rs` (43.7K dead vs. 45.5K live): close but non-identical sizes,
consistent with a rewrite rather than a straight copy. The dead copy's other 5 concerns (categorizer,
fences, headings, scanner, validator — no live counterpart by name) have no separate replacement file;
their functionality is folded into the 4 live files. This matches the task's pre-established finding
("internal/docs/\* is 100% dead") — independently re-confirmed here via `grep -rl` (only dead files
reference `internal::docs::`, e.g. `internal/git/runner.rs`, plus the 4 live `commands/md_validate_*.rs`
files which resolve `internal::docs::X` through the shim to the _live_ `application::docs::X`, not to
this directory).

| File                   | Disposition         | Reason                                                                                                                                                                                |
| ---------------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `categorizer.rs`       | dropped-with-reason | Unreachable (directory shadowed by flat `internal/docs.rs`); no live counterpart by name — folded into the live `application/docs/*` rewrite.                                         |
| `fences.rs`            | dropped-with-reason | Same shadowing; no live counterpart by name.                                                                                                                                          |
| `heading_hierarchy.rs` | dropped-with-reason | Same shadowing; superseded by the live, differently-sized `application/docs/heading_hierarchy.rs` (already in public).                                                                |
| `headings.rs`          | dropped-with-reason | Same shadowing; no live counterpart by name.                                                                                                                                          |
| `links.rs`             | dropped-with-reason | Same shadowing; superseded by the live, differently-sized `application/docs/links.rs` (already in public, previously confirmed modified/live per the task's own prior-fix reference). |
| `reporter.rs`          | dropped-with-reason | Same shadowing; no live counterpart by name.                                                                                                                                          |
| `scanner.rs`           | dropped-with-reason | Same shadowing; no live counterpart by name.                                                                                                                                          |
| `types.rs`             | dropped-with-reason | Same shadowing; no live counterpart by name.                                                                                                                                          |
| `validator.rs`         | dropped-with-reason | Same shadowing; no live counterpart by name.                                                                                                                                          |

## `src/internal/git/runner.rs` (1 file) — dropped

| File        | Disposition         | Reason                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ----------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `runner.rs` | dropped-with-reason | `internal/git.rs` declares only `pub mod root;` (making `internal/git/root.rs` live and already present in public), not `pub mod runner;` — confirmed unreachable, matching the task's pre-confirmed finding. 813-line legacy pre-commit orchestrator that itself imports from the dead `internal::agents::*`, `internal::docs::*`, and `internal::mermaid::*` trees (a self-contained dead island). Fully superseded by `application/git/pre_commit.rs` (already present in public, 22K, explicitly documented as "Reproduces the pipeline originally in `internal/git.rs`, now as a hexagonal use case with injected I/O"), which imports the same functionality from the _live_ `application::docs`/`application::agents`/`domain::mermaid` paths instead. `internal/git/root.rs` (the sibling that IS live) is a thin 4-line delegate to `infrastructure::git::root::find_root` — not a duplicate implementation, and already present in public, so out of scope for this ledger. |

## `src/internal/java/*` (5 files) — all ported

`internal.rs` declares `pub mod java;`; no flat `internal/java.rs` shadows it, so `internal/java/mod.rs`
is the direct, live resolution. Sole live caller: `commands/lang_java_validate_null_safety.rs`, whose
public counterpart is an explicit **"Dormant in ose-public — no JVM source here yet... activates when a
JVM app lands"** stub. Porting this directory is exactly what lets that stub become real.

| File           | Disposition | Reason                                                                           |
| -------------- | ----------- | -------------------------------------------------------------------------------- |
| `mod.rs`       | ported      | Module doc + `pub mod` declarations wiring the 4 files below.                    |
| `reporter.rs`  | ported      | Violation-report formatting for the null-safety validator.                       |
| `scanner.rs`   | ported      | Java package/directory scanner (finds `.java` files, `package-info.java`).       |
| `types.rs`     | ported      | `ViolationType` and related domain types.                                        |
| `validator.rs` | ported      | Core null-safety-annotation validation logic backing the dormant stub in public. |

## `src/internal/mermaid/*` (7 files) — all dropped

`internal.rs`'s `pub mod` list has no `mermaid` entry at all (genuinely absent, not shadowed) — 100%
unreachable, matching the task's pre-established finding. Only referenced by the already-dead
`internal/git/runner.rs`. Fully superseded by the live `domain/mermaid/*` (validator, graph, types,
etc.), `infrastructure/mermaid/reporter.rs`, and `application/mermaid/*` — all already present in
public (confirmed via `commands/md_validate_mermaid.rs`'s imports, which pull from
`crate::domain::mermaid::*` and `crate::infrastructure::mermaid::reporter::*`, not from
`internal::mermaid`).

| File           | Disposition         | Reason                                                                                                                                                        |
| -------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extractor.rs` | dropped-with-reason | Not declared anywhere in `internal.rs`; superseded by the live `domain::mermaid`/`application::mermaid` trees.                                                |
| `graph.rs`     | dropped-with-reason | Same — not declared anywhere; superseded by live `domain::mermaid::graph`-equivalent logic.                                                                   |
| `mod.rs`       | dropped-with-reason | Same — the directory's entry point is never reached by any `mod` statement.                                                                                   |
| `parser.rs`    | dropped-with-reason | Same — superseded by live parsing in `domain::mermaid`.                                                                                                       |
| `reporter.rs`  | dropped-with-reason | Same — superseded by the live `infrastructure/mermaid/reporter.rs` (confirmed import target of `commands/md_validate_mermaid.rs`, already present in public). |
| `types.rs`     | dropped-with-reason | Same — superseded by live `domain::mermaid` types.                                                                                                            |
| `validator.rs` | dropped-with-reason | Same — superseded by live `domain::mermaid::validator`.                                                                                                       |

## `src/internal/naming/reporter.rs` (1 file) — dropped

| File          | Disposition         | Reason                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `reporter.rs` | dropped-with-reason | `internal.rs` declares `pub mod naming;`, resolving to the flat `internal/naming.rs` shim (`pub use crate::application::naming::*;`, already present in public). Because that shim's glob re-export brings in `application::naming`'s own `pub mod reporter;`, any live caller's `crate::internal::naming::reporter::*` path resolves through the shim to the _live_ `application/naming/reporter.rs` (already present in public) — never to this directory file, which sits at a filesystem path that merely _looks_ like it should back that import but is actually orphaned. Confirmed by reading `commands/harness_validate_naming.rs`'s imports, which use exactly that shim-mediated path. |

## `src/internal/speccoverage/*` (7 files) — all dropped

`internal.rs` declares `pub mod speccoverage;`, which resolves to the flat `internal/speccoverage.rs`
(3-line shim, `pub use crate::application::speccoverage::*;` — already present in public, byte-identical
to primer's copy) because that flat file wins over the same-named directory, exactly like the
`agents`/`docs` cases above. Every file in this directory is _itself_ just another 3-line re-export shim
to the corresponding `application::speccoverage::X` submodule — i.e. these are shims-shadowed-by-a-shim,
doubly redundant. `application/speccoverage/*` already exists in public at the same path (confirmed via
directory listing on both sides); this ledger's scope is only the 85 primer-only files, and none of
these 7 add anything.

| File               | Disposition         | Reason                                                                                                                                                                                                                          |
| ------------------ | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `checker.rs`       | dropped-with-reason | 3-line re-export shim to `application::speccoverage::checker`, unreachable (directory shadowed by flat `internal/speccoverage.rs`), and the target already exists in public.                                                    |
| `cucumber_expr.rs` | dropped-with-reason | Same pattern, target `application::speccoverage::cucumber_expr` (already in public, though its content there currently differs from primer's — a separate, already-tracked "5-file delta" concern, not a missing-file concern). |
| `extractors.rs`    | dropped-with-reason | Same pattern, target `application::speccoverage::extractors` (same already-tracked delta caveat as above).                                                                                                                      |
| `parser.rs`        | dropped-with-reason | Same pattern, target `application::speccoverage::parser`.                                                                                                                                                                       |
| `reporter.rs`      | dropped-with-reason | Same pattern, target `application::speccoverage::reporter`.                                                                                                                                                                     |
| `types.rs`         | dropped-with-reason | Same pattern, target `application::speccoverage::types`.                                                                                                                                                                        |
| `util.rs`          | dropped-with-reason | Same pattern, target `application::speccoverage::util`.                                                                                                                                                                         |

## `src/internal/testcoverage/*` (11 files) — all merged

Same directory-shadowed-by-flat-file pattern as `internal/speccoverage/` — except here `internal.rs`
does **not** declare `pub mod testcoverage;` at all (unlike `speccoverage`), so both the flat
`internal/testcoverage.rs` shim AND this directory are unreachable (see flag 1). Diffed against the
`application/testcoverage/*` files being ported (see that section): same underlying logic, but
`application/`'s copy carries fuller rustdoc and is explicitly labeled a "byte-for-byte port" — i.e.
this directory is the earlier, less-documented predecessor of the exact same feature. Rather than
porting it as a second, redundant standalone copy, its content is treated as already absorbed into the
`application/testcoverage/*` files being ported.

| File             | Disposition | Reason                                                                                                                                                                   |
| ---------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `cobertura.rs`   | merged      | Near-duplicate of the `application/testcoverage/cobertura.rs` file being ported (same parse logic, terser comments, no per-field rustdoc) — don't copy as a second file. |
| `detect.rs`      | merged      | Near-duplicate of `application/testcoverage/detect.rs` being ported.                                                                                                     |
| `diff.rs`        | merged      | Near-duplicate of `application/testcoverage/diff.rs` being ported.                                                                                                       |
| `exclude.rs`     | merged      | Near-duplicate of `application/testcoverage/exclude.rs` being ported.                                                                                                    |
| `go_coverage.rs` | merged      | Near-duplicate of `application/testcoverage/go_coverage.rs` being ported.                                                                                                |
| `jacoco.rs`      | merged      | Near-duplicate of `application/testcoverage/jacoco.rs` being ported.                                                                                                     |
| `lcov.rs`        | merged      | Near-duplicate of `application/testcoverage/lcov.rs` being ported.                                                                                                       |
| `merge.rs`       | merged      | Near-duplicate of `application/testcoverage/merge.rs` being ported.                                                                                                      |
| `mod.rs`         | merged      | Circularly re-exports from `application::testcoverage::*` (see flag 1) — not a real standalone module to copy.                                                           |
| `reporter.rs`    | merged      | Near-duplicate of `application/testcoverage/reporter.rs` being ported.                                                                                                   |
| `types.rs`       | merged      | Near-duplicate of `application/testcoverage/types.rs` being ported.                                                                                                      |

## `tests/*.rs` (10 files) — 9 ported, 1 dropped

All 9 non-`env_validate` files are cucumber-rs (`cucumber = "0.22.1"` in primer) integration suites
driving the compiled binary via `assert_cmd`. They test command surfaces that are either already live
in both repos (docs, doctor, env, repo_governance/workflows, spec_coverage) or are being `ported` in
this same synthesis (contracts, java). Public currently has zero cucumber-based `tests/*.rs` files
(only `cli_smoke.rs`, `env_validate_integration.rs`, `golden_master.rs`, `mermaid_golden_corpus.rs`),
and `delivery.md`'s Phase 1 task list explicitly plans to adopt this harness (see flag 3 — requires a
0.22.1→0.23.0 cucumber API migration and a `.feature`-tree union, not a verbatim copy).

| File                 | Disposition         | Reason                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| -------------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agents.rs`          | ported              | Cucumber suite for `harness bindings generate/validate`, `harness claude validate`, `harness sync validate`, `harness naming validate` — all already-live commands in public; public has no cucumber coverage of them today.                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `contracts.rs`       | ported              | Cucumber suite for `specs clean java-imports` / `specs scaffold dart` — the exact command surface being activated by porting `internal/contracts/*` above.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `docs.rs`            | ported              | Cucumber suite for `docs validate-links` / `docs validate-mermaid` / `docs validate-heading-hierarchy` — already-live commands in public.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `doctor.rs`          | ported              | Cucumber suite for the `doctor` command — already live in public.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `env.rs`             | ported              | Cucumber suite for `env init/backup/restore` — already live in public.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `env_validate.rs`    | dropped-with-reason | Non-cucumber plain integration test. Diffed function-for-function against public's existing `tests/env_validate_integration.rs`: public's file contains all 5 of this file's test functions verbatim (`integration_matching_typescript_surface_exits_clean`, `integration_matching_rust_surface_exits_clean`, `integration_declared_but_unread_exits_nonzero_and_names_key`, `integration_read_but_undeclared_exits_nonzero_and_names_key`, `integration_warn_only_does_not_fail_on_drift`) **plus 2 more** (fixture-based manifest-consistency tests) and richer shared helpers. Public's file is a confirmed strict superset — nothing to port. |
| `java.rs`            | ported              | Cucumber suite for `lang java null-safety-annotations validate` — the exact command surface being activated by porting `internal/java/*` above.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `repo_governance.rs` | ported              | Cucumber suite for `repo-governance vendor validate` and `specs gherkin-cardinality validate` — already-live commands in public.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `spec_coverage.rs`   | ported              | Cucumber suite for `specs behavior-coverage validate` — already-live command in public.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `workflows.rs`       | ported              | Cucumber suite for `repo-governance workflows naming validate` — already-live command in public.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
