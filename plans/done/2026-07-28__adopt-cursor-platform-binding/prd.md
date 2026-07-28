# Product Requirements — Adopt a Cursor Platform Binding

## Product Overview

A third generated platform binding, delivered into **each of the three sibling repositories**:
`.cursor/agents/`, emitted from that repository's own `.claude/agents/` by
`rhino-cli harness bindings generate` and guarded by `rhino-cli harness naming validate` and
`rhino-cli harness bindings validate`, in which every agent's `model:` field is rewritten from the
Anthropic capability-tier alias into the same Cursor model ID that pins the non-fast Composer 2.5
toggle — full tier collapse across thinking, execution, and fast grades. The emitter never writes
`composer-2.5-fast`.

The product is the emitter, the guard, and the governance record — not a promise about Cursor's
interactive UI.

**Repository-shaped, not repository-specific.** The emitter and its Gherkin are byte-identical across
`ose-public`, `ose-primer`, and `ose-infra`; the generated output and the governance amendments are
per repo, because the agent rosters and the governance documents genuinely differ. Every acceptance
scenario below is therefore written to be **roster-agnostic** — no scenario names a count, a filename,
or a document structure that holds in only one of the three trees.

## Personas

Solo-maintainer repository; these are hats the maintainer wears plus the automated consumers.

| Persona                                | Type      | Need                                                                                |
| -------------------------------------- | --------- | ----------------------------------------------------------------------------------- |
| Maintainer-in-Cursor                   | Human hat | Delegated subagents run on the intended model in whichever repo is open             |
| Maintainer-as-bill-payer               | Human hat | Subagent inference is not silently billed at the fast tier, in any repo             |
| Maintainer-as-governance-owner         | Human hat | Every surface in each repo stating the Cursor rule agrees with the others in it     |
| Maintainer-as-parity-owner             | Human hat | The shared `rhino-cli` and Gherkin stay byte-identical after every landing          |
| `rhino-cli harness bindings` command   | Agent     | One registry entry and one converter per generated binding, no special-casing       |
| `rhino-cli harness naming` command     | Agent     | A registry-declared generated tier it can mirror-check without new code             |
| `repo-harness-compatibility-checker`   | Agent     | A catalog row it can parse, plus a named drift axis for the model pin, in each repo |
| `ose-primer` / `ose-infra` maintainers | Human hat | A complete, documented binding in their own tree — not just the shared source       |

## User Stories

**US-1** — As the maintainer-as-bill-payer, I want every delegated Cursor subagent to declare the
non-fast Composer 2.5 toggle, so that subagent inference is not billed at six times the standard
input and output rate by default.

**US-2** — As the maintainer-in-Cursor, I want `.cursor/agents/` generated from `.claude/agents/`
rather than hand-written, so that adding or editing an agent updates the Cursor binding
automatically instead of drifting.

**US-3** — As the maintainer-as-governance-owner, I want a hand-edit to any generated Cursor file to
fail the pre-push gate, so that the binding cannot silently diverge from its source.

**US-4** — As the maintainer-as-governance-owner, I want the Cursor binding declared as data in
`repo-config.yml` rather than as a hard-coded directory list, so that the harness registry stays the
single place a harness is described.

**US-5** — As the maintainer-in-Cursor, I want a written record of which Cursor surfaces the binding
does **not** reach, so that I do not assume an interactive session is covered when it is not.

**US-6** — As the maintainer-as-bill-payer, I want the model pin verified against a live subagent
rather than trusted from the frontmatter, so that a known Cursor defect does not silently defeat it.

**US-7** — As the maintainer-as-parity-owner, I want the identical `rhino-cli` source and Gherkin tree
in all three repositories, so that the byte-identity boundary in the SDLC Gate Standard holds.

**US-8** — As the `ose-primer` / `ose-infra` maintainer, I want my repository to get its own generated
`.cursor/agents/` **and** its own catalog row, tier amendment, and out-of-reach note, so that the
emitter arriving via byte-identity propagation does not leave an undocumented generated directory
behind.

**US-9** — As the maintainer-as-governance-owner, I want the emitter to mirror whatever roster its
repository actually has, so that a repo with 53 agents and a repo with 90 both get a correct binding
from the same code with no per-repo configuration.

## Acceptance Criteria

All scenarios below observe the step-keyword cardinality rule: exactly one primary `Given`, one
`When`, and one `Then` per scenario, with every additional step chained via `And` / `But`.

Scenarios AC-1 through AC-19 are authored verbatim into
`specs/apps/rhino/behavior/rhino-cli/gherkin/cursor-binding/cursor-binding.feature` during
delivery, in its own dedicated topic directory (sibling to `harness/`, never nested inside it —
see [`tech-docs.md`](./tech-docs.md) DD-15). That path sits inside the three-repo byte-identity
boundary, so the identical nineteen scenarios ship in `ose-public`, `ose-primer`, and `ose-infra` —
see [`tech-docs.md`](./tech-docs.md) for the propagation obligation.

**Roster-agnostic by construction.** Every scenario below runs against a temp-directory fixture and
speaks about "the repository under test", never about a named agent or a real roster size. That is
deliberate: the same feature file must pass in a 90-agent tree, a 64-agent tree, and a 53-agent tree
without amendment.

**AC numbering is stable, not positional.** AC-1 to AC-15 keep the numbers they were first given;
AC-16 to AC-19 were appended later and are numbered by addition order, not by position in this
document. AC-16 and AC-17 therefore appear at the end of the Emission group and AC-18 and AC-19 at
the end of the Validation group. `delivery.md` binds each scenario by its **title**, so the numbering
is a cross-reference aid only:

| ID    | Scenario title                                                               | Group      |
| ----- | ---------------------------------------------------------------------------- | ---------- |
| AC-1  | Generating emits one Cursor agent file per Claude agent                      | Emission   |
| AC-2  | A thinking-grade agent pins Composer 2.5 with fast disabled                  | Emission   |
| AC-3  | An execution-grade agent pins Composer 2.5 with fast disabled                | Emission   |
| AC-4  | An agent that omits the model field pins Composer 2.5 with fast disabled     | Emission   |
| AC-5  | A fast-grade agent pins Composer 2.5 with fast disabled                      | Emission   |
| AC-6  | The Claude color field is dropped from the Cursor frontmatter                | Emission   |
| AC-7  | The Claude name field is preserved in the Cursor frontmatter                 | Emission   |
| AC-8  | The agent body is copied unchanged below the frontmatter                     | Emission   |
| AC-9  | Generating twice is byte-identical                                           | Emission   |
| AC-10 | A Cursor mirror matching the generator passes validation                     | Validation |
| AC-11 | A hand-edited Cursor agent file fails validation                             | Validation |
| AC-12 | A Cursor agent file with no Claude counterpart fails validation              | Validation |
| AC-13 | A missing Cursor agent file fails validation                                 | Validation |
| AC-14 | A present Cursor directory absent from the catalog fails validation          | Validation |
| AC-15 | The cursor registry entry declares the generated tier and its mirror source  | Registry   |
| AC-16 | The Claude agents README is not mirrored into the Cursor binding             | Emission   |
| AC-17 | The emitter mirrors whatever roster the repository holds                     | Emission   |
| AC-18 | The naming validator reports mirror drift for a deleted Cursor agent file    | Validation |
| AC-19 | The naming validator reports mirror drift for an unsourced Cursor agent file | Validation |

### Emission

```gherkin
Scenario: Generating emits one Cursor agent file per Claude agent
  Given a repository whose .claude/agents/ directory holds three agent definitions and a README
  When the developer runs harness bindings generate
  Then the command exits successfully
  And .cursor/agents/ holds exactly three agent files
  And each emitted filename matches its Claude source filename
```

```gherkin
Scenario: A thinking-grade agent pins Composer 2.5 with fast disabled
  Given a Claude agent whose frontmatter declares the thinking-grade model alias
  When the developer runs harness bindings generate
  Then the emitted Cursor agent frontmatter declares the non-fast Composer 2.5 model identifier
  And the emitted frontmatter carries no other model field
```

```gherkin
Scenario: An execution-grade agent pins Composer 2.5 with fast disabled
  Given a Claude agent whose frontmatter declares the execution-grade model alias
  When the developer runs harness bindings generate
  Then the emitted Cursor agent frontmatter declares the non-fast Composer 2.5 model identifier
  And the emitted identifier is byte-identical to the thinking-grade agent's identifier
```

```gherkin
Scenario: An agent that omits the model field pins Composer 2.5 with fast disabled
  Given a Claude agent whose frontmatter carries no model field
  When the developer runs harness bindings generate
  Then the emitted Cursor agent frontmatter declares the non-fast Composer 2.5 model identifier
  And no conversion warning is emitted for the absent model field
```

```gherkin
Scenario: A fast-grade agent pins Composer 2.5 with fast disabled
  Given a Claude agent whose frontmatter declares the fast-grade model alias
  When the developer runs harness bindings generate
  Then the emitted Cursor agent frontmatter declares the non-fast Composer 2.5 model identifier
  And the emitted identifier is byte-identical to the thinking-grade agent's identifier
```

```gherkin
Scenario: The Claude color field is dropped from the Cursor frontmatter
  Given a Claude agent whose frontmatter declares a named color
  When the developer runs harness bindings generate
  Then the emitted Cursor agent frontmatter contains no color field
  And a conversion warning records that color has no Cursor equivalent
```

```gherkin
Scenario: The Claude name field is preserved in the Cursor frontmatter
  Given a Claude agent whose frontmatter declares a name
  When the developer runs harness bindings generate
  Then the emitted Cursor agent frontmatter declares the same name value
  And the emitted frontmatter declares the same description value
```

```gherkin
Scenario: The agent body is copied unchanged below the frontmatter
  Given a Claude agent whose body holds markdown headings and fenced code
  When the developer runs harness bindings generate
  Then the emitted Cursor agent body is byte-identical to the Claude agent body
  And the emitted file separates frontmatter from body with a single delimiter line
```

```gherkin
Scenario: Generating twice is byte-identical
  Given a repository whose Cursor mirror was already generated once
  When the developer runs harness bindings generate a second time
  Then the command exits successfully
  And every emitted Cursor agent file is byte-for-byte identical to the first emission
```

```gherkin
Scenario: The Claude agents README is not mirrored into the Cursor binding
  Given a repository whose .claude/agents/ directory holds a README alongside its agent definitions
  When the developer runs harness bindings generate
  Then .cursor/agents/ holds no README file
  And every other Claude agent filename has a Cursor counterpart
```

```gherkin
Scenario: The emitter mirrors whatever roster the repository holds
  Given a repository whose .claude/agents/ directory holds a different number of agents than another repository
  When the developer runs harness bindings generate in that repository
  Then .cursor/agents/ holds exactly as many agent files as that repository's .claude/agents/ directory
  And no roster size is hard-coded in the emitter
```

### Validation

```gherkin
Scenario: A Cursor mirror matching the generator passes validation
  Given a repository whose Cursor mirror matches the generated content
  When the developer runs harness bindings validate
  Then the command exits successfully
  And the output reports the Cursor mirror checks as passing
```

```gherkin
Scenario: A hand-edited Cursor agent file fails validation
  Given a repository where one Cursor agent file has been hand-edited away from the generated content
  When the developer runs harness bindings validate
  Then the command exits with a failure code
  And the output names the drifted Cursor agent file
  And the output advises re-running the binding generator
```

```gherkin
Scenario: A Cursor agent file with no Claude counterpart fails validation
  Given a repository whose Cursor mirror holds an agent file that no longer exists under .claude/agents/
  When the developer runs harness bindings validate
  Then the command exits with a failure code
  And the output names the stale Cursor agent file
```

```gherkin
Scenario: A missing Cursor agent file fails validation
  Given a repository whose Cursor mirror is missing one agent file present under .claude/agents/
  When the developer runs harness bindings validate
  Then the command exits with a failure code
  And the output names the missing Cursor agent file
```

```gherkin
Scenario: A present Cursor directory absent from the catalog fails validation
  Given a repository with a generated Cursor mirror and a platform-bindings catalog that omits it
  When the developer runs harness bindings validate
  Then the command exits with a failure code
  And the output identifies the Cursor directory as missing a catalog row
```

```gherkin
Scenario: The naming validator reports mirror drift for a deleted Cursor agent file
  Given a repository whose registry declares the cursor entry as a generated tier mirroring .claude/agents
  When the developer deletes one Cursor agent file and runs harness naming validate
  Then the command reports a mirror-drift violation
  And the violation names the deleted agent as present in the source but absent from the Cursor mirror
```

```gherkin
Scenario: The naming validator reports mirror drift for an unsourced Cursor agent file
  Given a repository whose registry declares the cursor entry as a generated tier mirroring .claude/agents
  When the developer adds a Cursor agent file with no Claude counterpart and runs harness naming validate
  Then the command reports a mirror-drift violation
  And the violation names the added agent as present in the Cursor mirror but absent from the source
```

### Registry

```gherkin
Scenario: The cursor registry entry declares the generated tier and its mirror source
  Given the harness registry section of repo-config.yml
  When the cursor entry is read
  Then the entry declares the generated tier
  And the entry declares .cursor/agents as its agent directory
  And the entry declares .claude/agents as the source it mirrors
```

## Product Scope

### In-scope features

| Feature                                                            | Delivered by                                              | Scope         |
| ------------------------------------------------------------------ | --------------------------------------------------------- | ------------- |
| Cursor model-tier mapping function                                 | New `convert_cursor_model` in the agents converter        | Shared code   |
| Cursor agent-file emitter (frontmatter rewrite + body passthrough) | New Cursor converter module                               | Shared code   |
| Wiring into `harness bindings generate`                            | `--harness cursor` branch plus the default path           | Shared code   |
| Cursor mirror content-parity guard                                 | New checks inside `harness bindings validate`             | Shared code   |
| Cursor mirror name-parity guard                                    | Free from the registry flip via `harness naming validate` | Shared code   |
| Companion Gherkin + cucumber-rs step definitions                   | New `.feature` file and step-definition file              | Shared specs  |
| `repo-config.yml` registry flip to the generated tier              | Same three-field entry change, applied in each repo       | Per repo (×3) |
| Generated `.cursor/agents/` output                                 | That repo's own roster: 90 / 64 / 53 files                | Per repo (×3) |
| Platform-bindings catalog row                                      | That repo's own catalog, in that repo's own table shape   | Per repo (×3) |
| Tier reclassification in `multi-harness-binding.md`                | That repo's own section headings                          | Per repo (×3) |
| `CLAUDE.md` / `AGENTS.md` binding-set updates                      | Only where that repo's file actually states the rule      | Per repo (×3) |
| Out-of-reach onboarding note                                       | New section in that repo's platform-bindings catalog      | Per repo (×3) |
| `.prettierignore` entry                                            | Only if that repo's Prettier check demands it             | Per repo (×3) |
| `repo-harness-compatibility-checker` model-pin drift axis          | Agent-definition edit in each repo that carries the agent | Per repo (×3) |
| Empirical live-subagent verification with committed evidence       | Phase 5 of the delivery checklist, run once               | Once          |

### Out-of-scope features

- Any `.cursor/rules/`, `.cursor/mcp.json`, `.cursor/skills/`, or `.cursor/cli.json` surface, in any
  repository.
- `readonly` and `is_background` frontmatter derivation (Cursor documents them; this emitter omits
  them and lets Cursor default).
- Enforcement over interactive sessions, the CLI default model, or Auto mode.
- Enterprise Model Access Control configuration.
- Any change to the OpenCode or Amazon Q bindings beyond what shared-code refactoring requires.
- Cost telemetry or spend measurement.
- Converging the three repositories' governance documents into a single shape; each is amended in
  place, in its own shape.
- Repairing `ose-infra`'s pre-existing `.opencode/agents/ci-monitor-subagent.md` orphan.

## Product Risks

| Risk                                                                          | Impact                                                                                | Handling                                                                                          |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| Cursor ignores `model:` under some conditions (staff-confirmed)               | The pin silently does nothing                                                         | AC coverage stops at the emitted file; the live check is Phase 5, evidence committed              |
| The canonical model slug is not `composer-2.5`                                | Every emitted file carries a wrong identifier                                         | Phase 1 spike U1 gates the emitter; fallback stated in `tech-docs.md`                             |
| Bracket parameters are rejected inside a frontmatter file                     | The pin degrades to plain Composer 2.5 (still non-fast in name, unverified in effect) | Phase 1 spike U2; fallback is the bare slug plus a recorded residual exposure                     |
| A contributor hand-edits `.cursor/agents/` and pushes                         | Divergence between binding and source                                                 | AC-11 makes it a pre-push failure; AC-18 and AC-19 add a second, registry-driven line of defence  |
| The catalog row is forgotten when the directory first appears                 | The directory ships undocumented in that repo                                         | AC-14 covers the fixture case; the coarse substring guard is discussed honestly in `tech-docs.md` |
| Cursor retires or renames the non-fast Composer 2.5 slug                      | Every emitted file carries a wrong identifier                                         | One `const`, one governance table; the compat checker gains this drift axis                       |
| Emitting a third mirror slows the pre-commit `harness bindings generate` step | Slower commits                                                                        | The OpenCode mirror already emits the same roster; the added cost is one more pass over it        |
| The emitter is propagated to a sibling repo without its governance record     | That repo silently grows an undocumented generated directory on its next commit       | Each repo's landing is a single PR carrying emitter, generated output, and governance together    |
| A per-repo governance step assumes `ose-public`'s document structure          | The step is unexecutable in the repo whose document differs                           | Three separate verdict tables in `tech-docs.md`; no shared governance step across repos           |
| A repo-specific roster count leaks into a scenario                            | The shared, byte-identical feature file fails in the other two repos                  | AC-17 makes roster-agnosticism itself a tested property                                           |

## Related

- [`brd.md`](./brd.md) — the business rationale and the honest scope of the "always" claim
- [`tech-docs.md`](./tech-docs.md) — architecture, design decisions, unknowns, per-file verdicts
- [`delivery.md`](./delivery.md) — the phase that binds each scenario above to a TDD cycle
- [Acceptance Criteria Convention](../../../repo-governance/development/infra/acceptance-criteria.md)
- [Specs Directory Structure Convention](../../../repo-governance/conventions/structure/specs-directory-structure.md)
