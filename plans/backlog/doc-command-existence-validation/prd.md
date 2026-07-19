# Product Requirements — Doc Command Existence Validation

## Product overview

`md commands validate` is a `rhino-cli` subcommand that scans tracked markdown files, extracts
command citations belonging to three recognized families, and verifies each cited command exists
according to an authoritative in-repo oracle. It reports findings with file, line, the cited
command, and the reason it could not be resolved.

**UI-design-funnel exemption**: this plan is CLI/text-output only. It adds no user-facing screens
or components under `apps/` (web) or `libs/`. The UI-bearing design funnel does not apply; the
rationale is restated in [tech-docs.md](./tech-docs.md).

## Personas

| Persona                     | Description                                                       | Primary need                                                    |
| --------------------------- | ----------------------------------------------------------------- | --------------------------------------------------------------- |
| **Instruction-file author** | Maintainer editing `AGENTS.md`, `CLAUDE.md`, `repo-governance/**` | Immediate feedback when a cited command name is wrong           |
| **Plan author**             | `plan-maker`, or the maintainer writing `delivery.md`             | Gate acceptance criteria that are runnable as written           |
| **Plan executor**           | Agent executing a plan's delivery checklist                       | Never encounter an unrunnable command mid-gate                  |
| **Toolchain maintainer**    | Maintainer renaming or removing a command                         | Mechanical discovery of every stale citation across three repos |

## User stories

- **US-1** — As an instruction-file author, I want a pre-push gate that rejects citations of
  nonexistent Nx targets, so that `AGENTS.md` never again ships a command that cannot run.
- **US-2** — As a plan executor, I want `delivery.md` gate commands verified before I reach them,
  so that I never stall on a fabricated command.
- **US-3** — As a toolchain maintainer, I want renaming a `rhino-cli` subcommand to surface every
  stale citation, so that renames are safe rather than hopeful.
- **US-4** — As a governance maintainer, I want to annotate a genuinely aspirational command as
  exempt with a written reason, so that documenting planned tooling does not force me to either
  fabricate it or disable the validator.
- **US-5** — As a maintainer, I want illustrative and templated commands (`<project>`, `$VAR`)
  ignored by default, so that the validator does not punish me for writing examples.
- **US-6** — As a maintainer, I want an opt-in `--strict` mode, so that I can run a wider,
  noisier sweep deliberately when auditing rather than on every push.

## Acceptance criteria

### Nx target detection

```gherkin
Scenario: A markdown file citing a nonexistent Nx target fails validation
  Given a tracked markdown file containing the command "npx nx run rhino-cli:links:validation"
  And the resolved Nx project graph contains no target "links:validation" on project "rhino-cli"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the output names the file, the line number, and the cited target
  And the output states that the target does not exist on the project
```

```gherkin
Scenario: A markdown file citing an existing Nx target passes validation
  Given a tracked markdown file containing the command "npx nx run rhino-cli:test:quick"
  And the resolved Nx project graph contains target "test:quick" on project "rhino-cli"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for that file
```

```gherkin
Scenario: An inferred Nx target that is absent from project.json still passes validation
  Given a tracked markdown file citing an Nx target present only via plugin inference
  And the target is absent from the project's literal "project.json" targets map
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for that target
```

```gherkin
Scenario: A citation naming a nonexistent Nx project is reported distinctly
  Given a tracked markdown file containing the command "npx nx run ghost-app:build"
  And the resolved Nx project graph contains no project named "ghost-app"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding states that the project does not exist, distinct from a missing-target finding
```

```gherkin
Scenario: A run-many target citation is validated against the union of all project targets
  Given a tracked markdown file containing the command "npx nx run-many -t phantom-target"
  And no project in the resolved graph defines a target named "phantom-target"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding identifies the cited run-many target
```

### npm script detection

```gherkin
Scenario: A citation of a nonexistent npm script fails validation
  Given a tracked markdown file containing the command "npm run ghost:script"
  And the repository root "package.json" declares no script named "ghost:script"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding names the cited script and the package.json consulted
```

```gherkin
Scenario: A citation of an existing npm script passes validation
  Given a tracked markdown file containing the command "npm run lint:md:fix"
  And the repository root "package.json" declares a script named "lint:md:fix"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for that citation
```

### rhino-cli subcommand detection

```gherkin
Scenario: A cargo-run citation of a nonexistent rhino-cli subcommand chain fails validation
  Given a tracked markdown file citing "cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md ghost validate"
  And the rhino-cli clap command tree contains no subcommand "ghost" under "md"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding names the unresolved segment of the subcommand chain
```

```gherkin
Scenario: A cargo-run citation of an existing rhino-cli subcommand chain passes validation
  Given a tracked markdown file citing "cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate"
  And the rhino-cli clap command tree resolves the chain "md links validate"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for that citation
```

```gherkin
Scenario: The subcommand oracle is derived from the live clap tree rather than a hardcoded list
  Given a new subcommand is added to the rhino-cli clap command tree
  And no list of valid subcommands is edited anywhere in the validator source
  When the developer runs "rhino-cli md commands validate" against a file citing the new subcommand
  Then the command exits with status zero
  And no finding is reported for the new subcommand
```

### False-positive suppression (conservative default)

```gherkin
Scenario: A templated command containing an angle-bracket placeholder is ignored by default
  Given a tracked markdown file containing the command "nx run <project>:test:quick"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the templated citation
```

```gherkin
Scenario: A command containing a shell variable is ignored by default
  Given a tracked markdown file containing the command "npx nx run $PROJECT:build"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the variable-bearing citation
```

```gherkin
Scenario: Prose mentions of a command outside a fenced block are ignored by default
  Given a tracked markdown file mentioning "nx run some-app:ghost-target" in a prose sentence
  And the mention is not inside a fenced code block
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the prose mention
```

```gherkin
Scenario: Strict mode reports prose mentions that the default mode suppresses
  Given a tracked markdown file mentioning "nx run some-app:ghost-target" in a prose sentence
  And the target does not exist in the resolved Nx project graph
  When the developer runs "rhino-cli md commands validate --strict"
  Then the command exits with a nonzero status
  And the finding identifies the prose mention
```

```gherkin
Scenario: A multi-line continuation command is reassembled before validation
  Given a tracked markdown file containing a fenced command split across lines with trailing backslashes
  And the reassembled command cites an existing Nx target
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the continued command
```

### Exemption mechanism

```gherkin
Scenario: An inline exemption annotation with a reason suppresses a finding
  Given a tracked markdown file citing a nonexistent Nx target
  And the citation is preceded by the annotation "<!-- doc-command-exempt: planned in ROADMAP -->"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the exempted citation
```

```gherkin
Scenario: An inline exemption annotation without a reason is itself a finding
  Given a tracked markdown file citing a nonexistent Nx target
  And the citation is preceded by the bare annotation "<!-- doc-command-exempt -->"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding states that an exemption annotation requires a written reason
```

```gherkin
Scenario: An exemption annotation applies only to the citation that immediately follows it
  Given a tracked markdown file with an annotated exempt citation followed by a second unannotated nonexistent citation
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And exactly one finding is reported, naming the second citation
```

```gherkin
Scenario: A path in the configured exclusion allowlist is not scanned
  Given a markdown file under "plans/done/" citing a nonexistent Nx target
  And "plans/done" is listed in the validator's configured exclusions
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no finding is reported for the excluded path
```

### Regression guard for the motivating incident

```gherkin
Scenario: Reintroducing an originally-cited nonexistent target is rejected
  Given a tracked markdown file containing the command "npx nx run rhino-cli:headings:hierarchy-validation"
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with a nonzero status
  And the finding names the target "headings:hierarchy-validation"
```

```gherkin
Scenario: The repository corpus is clean after remediation
  Given the remediation phase has corrected every known citation of a nonexistent command
  When the developer runs "rhino-cli md commands validate"
  Then the command exits with status zero
  And no findings are reported
```

### Aggregation and reporting

```gherkin
Scenario: The validator participates in the aggregate md audit
  Given a tracked markdown file citing a nonexistent Nx target
  When the developer runs "rhino-cli md audit"
  Then the command exits with a nonzero status
  And the aggregated output includes the command-existence finding
```

```gherkin
Scenario: Findings are emitted as machine-readable JSON on request
  Given a tracked markdown file citing a nonexistent Nx target
  When the developer runs "rhino-cli md commands validate --format json"
  Then the output parses as valid JSON
  And each finding object carries a file path, a line number, the cited command, and a reason
```

## Product scope

### In scope

- Three detector families: Nx targets, npm scripts, rhino-cli subcommands.
- Conservative default detection; opt-in `--strict`.
- Two-tier exemption: inline per-occurrence annotation (reason required) and configured path
  exclusions.
- `--exclude <path>` flag matching the established idiom of `md links validate`.
- Text and JSON output, consistent with sibling validators.
- Participation in `md audit`.

### Out of scope

- Shell script and `make` target citations.
- Flag and argument validation.
- External tool citations (`git`, `docker`, `jq`, `curl`).
- Cross-repository command citations.
- Auto-fix.

## Product risks

| Risk                                                  | Mitigation                                                                                                                                 |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Regex-based extraction misfires on unusual formatting | Conservative default scopes to fenced blocks; every extraction edge case gets a Gherkin scenario and a unit test before implementation     |
| Nx graph resolution is slow or fails in a worktree    | Snapshot resolved once per run; a resolution failure is a hard error with a clear message, never a silent pass                             |
| Exemption annotations accumulate unaudited            | Reason is mandatory; annotations are greppable; the remediation phase establishes the precedent of honest labelling over blanket exclusion |
| Strict mode is too noisy to ever use                  | Strict mode is opt-in and ungated; it is an audit tool, not a gate                                                                         |
