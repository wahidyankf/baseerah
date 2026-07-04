# Phase 0 Audit — Command Census

Full recursive `rhino-cli … --help` walk against the canonical `ose-public` release binary
(`apps/rhino-cli/target/release/rhino-cli`, built from the current working tree,
`git_sha=17e437dca`). All commands below were probed exclusively with `--help`/`help <cmd>`
suffixes or by reading source (never bare-invoked), per the audit-only constraint on this phase.

## Method

1. `rhino-cli --help` — lists the 10 top-level groups.
2. For each group, `rhino-cli <group> --help` (or `help <group> <sub>` where needed) — lists its
   immediate subcommands.
3. Recursed into every further nested subcommand until a true leaf (a node accepting positional
   args / flags with no further `Commands:` section) was reached.

## CLI help-system quirk (drift finding, non-blocking)

`rhino-cli <group> --help` does **not** print the group's own help text. Instead it exits 2 with a
generic clap error: `error: 'rhino-cli <group>' requires a subcommand but one was not provided`
(the error body is still useful — it lists `[subcommands: ...]`). Separately,
`rhino-cli help <group>` **does** print the correct, well-formed help text (description + `Commands:`
list) but **also exits 2** — non-conventional, since clap's own `--help` flag normally exits 0. This
is a real CLI behavior quirk (likely an interaction between `subcommand_required(true)` and a custom
top-level dispatcher), worth a follow-up fix, but it did not block this census since the subcommand
lists are still fully enumerable from the error text or from `help <group>`.

## Full leaf-command tree (41 leaves across 10 groups)

| #   | Group             | Leaf command                                         | Notes                                                                                          |
| --- | ----------------- | ---------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| 1   | `test-coverage`   | `test-coverage validate <COVERAGE_FILE> <THRESHOLD>` | positional args required                                                                       |
| 2   | `repo-governance` | `repo-governance vendor validate`                    |                                                                                                |
| 3   | `repo-governance` | `repo-governance layer-coherence validate`           |                                                                                                |
| 4   | `repo-governance` | `repo-governance traceability validate`              |                                                                                                |
| 5   | `repo-governance` | `repo-governance workflows naming validate`          | 3-level nesting                                                                                |
| 6   | `repo-governance` | `repo-governance audit`                              | standalone orchestrator leaf (composes the above)                                              |
| 7   | `md`              | `md links validate`                                  |                                                                                                |
| 8   | `md`              | `md mermaid validate`                                |                                                                                                |
| 9   | `md`              | `md heading-hierarchy validate`                      |                                                                                                |
| 10  | `md`              | `md naming validate`                                 |                                                                                                |
| 11  | `md`              | `md frontmatter validate`                            |                                                                                                |
| 12  | `md`              | `md frontmatter-dates validate`                      |                                                                                                |
| 13  | `md`              | `md readme-index validate`                           |                                                                                                |
| 14  | `md`              | `md audit`                                           | standalone orchestrator leaf                                                                   |
| 15  | `convention`      | `convention emoji validate`                          |                                                                                                |
| 16  | `convention`      | `convention license validate`                        |                                                                                                |
| 17  | `convention`      | `convention audit`                                   | standalone orchestrator leaf                                                                   |
| 18  | `harness`         | `harness naming validate`                            |                                                                                                |
| 19  | `harness`         | `harness duplication validate`                       |                                                                                                |
| 20  | `harness`         | `harness claude validate`                            |                                                                                                |
| 21  | `harness`         | `harness sync validate`                              | see dead-code finding below — this is `harness sync`'s **only** subcommand                     |
| 22  | `harness`         | `harness bindings validate`                          |                                                                                                |
| 23  | `harness`         | `harness bindings generate`                          | mutating (writes `.opencode/`/`.amazonq/` bridge files); parameterized by `--harness <target>` |
| 24  | `harness`         | `harness instruction-size validate`                  |                                                                                                |
| 25  | `harness`         | `harness audit`                                      | standalone orchestrator leaf                                                                   |
| 26  | `specs`           | `specs gherkin-cardinality validate`                 |                                                                                                |
| 27  | `specs`           | `specs structure validate`                           |                                                                                                |
| 28  | `specs`           | `specs counts validate`                              |                                                                                                |
| 29  | `specs`           | `specs behavior-coverage validate`                   |                                                                                                |
| 30  | `specs`           | `specs domain-coverage validate`                     |                                                                                                |
| 31  | `specs`           | `specs clean java-imports`                           | dormant in ose-public (per its own `--help` text)                                              |
| 32  | `specs`           | `specs scaffold dart`                                | dormant in ose-public (per its own `--help` text)                                              |
| 33  | `specs`           | `specs audit`                                        | standalone orchestrator leaf (runs `specs structure validate` per project)                     |
| 34  | `lang`            | `lang java null-safety-annotations validate`         | dormant in ose-public                                                                          |
| 35  | `repo-config`     | `repo-config validate`                               |                                                                                                |
| 36  | `env`             | `env init`                                           | mutating                                                                                       |
| 37  | `env`             | `env backup`                                         | mutating                                                                                       |
| 38  | `env`             | `env restore`                                        | mutating                                                                                       |
| 39  | `env`             | `env validate`                                       | read-only                                                                                      |
| 40  | `env`             | `env staged-guard validate`                          | 3-level nesting                                                                                |
| 41  | `doctor`          | `doctor`                                             | leaf group, no subcommand                                                                      |

## Cross-check against tech-docs §1.5 and the 2026-07-03 synthesis ledger

Cross-referenced against
[tech-docs.md §1.5](../tech-docs.md#15-canonical-command-surface-aligned-with-the-2026-07-01--2026-07-03-plans)
and
[the 2026-07-03 synthesis ledger](../../../done/2026-07-03__unify-rhino-cli-sdlc-parity/audit/synthesis-ledger.md).

**Overall match**: every group and every leaf command named in tech-docs §1.5 is confirmed present
in the current binary. No group, and no documented leaf, is missing.

**Drift #1 — `harness bindings` is 2 leaves, not 1**: tech-docs §1.5's `harness` row lists `bindings`
as a single item, but the actual CLI has **two** sibling leaves under it —
`harness bindings validate` and `harness bindings generate`. This raises the actual leaf-command
count from tech-docs's implied 40 to the **41** enumerated above. Not a functional gap (both leaves
already exist and are documented elsewhere in tech-docs — `harness bindings generate` maps to the
Amazon Q Developer Binding Bridge scenarios), just an undercount in the summary table.

**Drift #2 — `harness sync`'s mutating counterpart is dead code, not a missing CLI leaf**:
tech-docs and delivery.md §1c describe `harness sync` (formerly `agents sync`) as covering the
mutating "convert `.claude/` to `.opencode/` format" behavior. The actual CLI enum
(`apps/rhino-cli/src/cli.rs:414-417`, `HarnessSyncCommands`) has **only** a `Validate` variant —
`harness sync` bare requires a subcommand and only offers `validate`. Reading
`apps/rhino-cli/tests/agents.rs:156-179` confirms the already-passing "agents sync" scenarios
(`the developer runs agents sync`, `--dry-run`, `--agents-only`) invoke
`harness bindings generate --harness opencode` instead — **not** `harness sync`. Grepping
`apps/rhino-cli/src/` for `harness_sync::` (the module implementing the old mutating `SyncArgs`)
returns **zero references** — `apps/rhino-cli/src/commands/harness_sync.rs` is dead code, never
wired into the `HarnessCommands` dispatch enum. This is not a coverage gap (the behavior is fully
covered, just under a different current-CLI leaf name); it is a dead-code finding worth a follow-up
cleanup item, out of scope for this plan's Phase 1 gap-fill (no scenario needs a new binding here).

**Drift #3 — `env staged-guard` and two other bound dirs have an undocumented rename candidate**:
tech-docs §1.5's Decision-4 rename-candidate list names 4 pairs: `system/` ↔ `doctor`,
`agent-naming/` ↔ `harness naming`, `spec-coverage/` ↔ `specs behavior-coverage`,
`repo-config-validate/` ↔ `repo-config`. Reading the bound dirs' own feature/step-def content
(see `audit/04-coverage-map.md` for the full table) surfaces **two more mismatched dirs not on that
list**: `contracts/` (tests `specs clean java-imports` + `specs scaffold dart` — stale feature
prose still says "contracts dart-scaffold"/"contracts java-clean-imports") and `java/` (tests
`lang java null-safety-annotations validate` — stale feature prose still says
"java validate-annotations"). Both already **pass** today (their step defs already invoke the
current commands; only the Gherkin prose text is stale-vocab, not the step bindings), so they are
not hollow-skips, but Phase 1's Decision-4 rename sweep should account for these two additional
dirs beyond the tech-docs-named 4, plus consider whether `contracts/` and `java/` should also move
under a `specs/`- or `lang/`-prefixed name for full command-group alignment.

**No other drift found.** The 4 unbound dirs (`ddd`, `git`, `specs`, `test-coverage`) match the
plan's claim exactly — see `audit/03-unbound-dirs.md`.
