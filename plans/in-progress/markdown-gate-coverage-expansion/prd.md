# Product Requirements Document — Markdown Gate Coverage Expansion

## Product Overview

The product is the **markdown gate**: three `rhino-cli` validators (`docs validate-mermaid`,
`docs validate-links`, `docs validate-heading-hierarchy`), their Nx targets, and **three
consistent enforcement layers** — `.husky/pre-commit` staged-only (Layer 1), PR CI (Layer 2), and
push-to-`main` CI (Layer 3), with Layers 2 and 3 consolidated into a single
`.github/workflows/validate-markdown.yml`. This plan moves mermaid to pre-commit, widens the link
checker (scope + `--exclude` + anchors), un-orphans the heading-hierarchy checker under a
prose-allowlist default-deny scope, and cleans the resulting baseline.

## Personas

Solo-maintainer repository; personas are hats the maintainer wears and agents that consume the
surfaces.

- **Diagram author** — writes flowcharts; wants fast, accurate pre-commit feedback.
- **Link author** — writes `[text](target#anchor)` links; wants broken files AND broken
  anchors caught.
- **Prose author** — writes `docs/`, `repo-governance/`, `plans/` content; wants single-H1 and
  non-skipping heading nesting enforced.
- **Prompt/skill author** — writes `.claude/agents/`, `SKILL.md`, `.opencode/agents/` artifacts;
  must NEVER be tripped by heading rules.
- **Tooling maintainer** — owns the validators; wants existing tests green and new behavior covered.
- **CI** — must block PRs and report on direct `main` pushes, even when local hooks are skipped.

## User Stories

- **US-1** — As a tooling maintainer, I want the mermaid gate to run at pre-commit staged-only
  instead of pre-push, so diagram feedback arrives earlier and the pre-push hook is lighter.
- **US-2** — As a link author, I want the link checker to scan the whole repo minus excluded trees,
  so a broken link anywhere outside the specialized content CLIs is caught.
- **US-3** — As a tooling maintainer, I want a repeatable `--exclude <path>` flag on
  `docs validate-links`, so trees with their own validation (`plans/done/`, the two content trees)
  can be skipped explicitly at call sites.
- **US-4** — As a link author, I want a link to a non-existent `#fragment` to be flagged, so renamed
  sections do not leave silently-rotten anchors.
- **US-5** — As a prose author, I want heading-hierarchy enforced on `docs/`, `repo-governance/`,
  `plans/`(−`done/`), and root `*.md`, so prose docs keep one H1 and non-skipping nesting.
- **US-6** — As a prompt/skill author, I want heading rules to NEVER fire on `.claude/**`,
  `.opencode/**`, or `.amazonq/**`, so my section-marker `#` usage is never a finding even at
  pre-commit.
- **US-7** — As a tooling maintainer, I want heading-hierarchy to gain `--exclude` for parity, so
  its scope is adjustable the same way as the link checker.
- **US-8** — As CI, I want one consolidated workflow running all three gates on `pull_request` AND
  `push` to `main`, so the gate is unskippable and direct trunk pushes are covered.
- **US-9** — As a tooling maintainer, I want the anchor validator to reuse the fence-aware heading
  parser from `heading_hierarchy.rs`, so heading parsing is not duplicated across two modules.
- **US-10** — As a maintainer, I want `diagrams.md`, `quality.md`, and `linking.md` to accurately
  describe the new enforcement, so governance docs match the tooling.
- **US-11** — As a tooling maintainer, I want the rhino-cli BDD specs under `specs/apps/rhino/` to
  gain scenarios for the new validator behavior (link `--exclude`/repo-wide/anchors, heading prose
  allowlist/`--exclude`, staged pre-commit mermaid+heading steps), so the `spec-coverage` gate stays
  green and the specs remain the source of the first failing tests.
- **US-12** — As a governance maintainer, I want the related convention `.md` updates propagated via
  `repo-rules-maker` and validated by `repo-rules-quality-gate` (strict, double-zero), so the rule
  change reaches every governance surface, not just the obvious files.

## Acceptance Criteria (Gherkin)

> Each scenario obeys the repo keyword-cardinality norm: at most one `Given`, one `When`, one
> `Then`; additional steps use `And`/`But`.

### Gate A — Mermaid moves to pre-commit

```gherkin
Scenario: Pre-push no longer triggers the mermaid gate
  Given the .husky/pre-push hook
  When a contributor inspects its trigger blocks
  Then no block runs the validate:mermaid target
  And the mermaid trigger has been removed
```

```gherkin
Scenario: Pre-commit runs the mermaid gate on staged markdown
  Given a staged markdown file containing a malformed flowchart
  When the pre-commit suite runs
  Then the mermaid gate reports the violation
  And the commit is blocked
```

```gherkin
Scenario: Pre-commit mermaid gate ignores unstaged markdown
  Given an unstaged markdown file containing a malformed flowchart
  When the pre-commit suite runs
  Then the mermaid gate does not report that file
  And the commit is allowed
```

### Gate B — Link checker scope, exclude flag, anchors

```gherkin
Scenario: Link checker scans the whole repo minus exclusions
  Given a broken relative link in a file under libs
  When docs validate-links runs a full scan
  Then the broken link is reported
  And files under the noise-skip set are not scanned
```

```gherkin
Scenario: A repeated exclude flag skips named trees
  Given a broken relative link in a file under plans/done
  When docs validate-links runs with --exclude plans/done
  Then the broken link under plans/done is not reported
  And links outside the excluded tree are still validated
```

```gherkin
Scenario: The .claude/skills tree stays hard-skipped
  Given a broken relative link in a SKILL.md under .claude/skills
  When docs validate-links runs a full scan
  Then the broken link is not reported
  And the hard-skip applies without an explicit exclude flag
```

```gherkin
Scenario: A link to a missing anchor is flagged
  Given a link [X](./target.md#missing-section) whose target file exists
  And target.md contains no heading that slugifies to missing-section
  When docs validate-links validates the link
  Then a broken-anchor finding is reported
  And the validator exits non-zero
```

```gherkin
Scenario: A link to an existing anchor passes
  Given a link [X](./target.md#real-section) whose target file exists
  And target.md contains a heading "## Real Section"
  When docs validate-links validates the link
  Then no broken-anchor finding is reported for that link
```

```gherkin
Scenario: GitHub slug collisions get numeric suffixes
  Given target.md contains two headings both titled "Setup"
  When the anchor validator slugifies the headings
  Then the first heading slug is setup
  And the second heading slug is setup-1
```

```gherkin
Scenario: A same-file anchor link is validated against its own headings
  Given a file linking to [Y](#own-section)
  And the file contains no heading that slugifies to own-section
  When docs validate-links validates the link
  Then a broken-anchor finding is reported for that file
```

### Gate C — Heading-hierarchy prose allowlist, default-deny

```gherkin
Scenario: Heading-hierarchy runs on the prose allowlist
  Given a prose file under docs with two H1 headings
  When docs validate-heading-hierarchy runs a full scan
  Then a duplicate-h1 finding is reported for that file
  And the validator exits non-zero
```

```gherkin
Scenario: An agent file is exempt from heading rules
  Given a .claude/agents file with zero H1 headings
  When docs validate-heading-hierarchy runs a full scan
  Then no missing-h1 finding is reported for that file
  And the file is excluded by the default-deny allowlist
```

```gherkin
Scenario: A staged skill file never trips heading rules at pre-commit
  Given a staged SKILL.md under .claude/skills with many H1 headings
  When the pre-commit suite runs the heading gate
  Then no heading finding is reported for the skill file
  And the allowlist filter excludes it inside the validator file selection
```

```gherkin
Scenario: plans/done is excluded from heading rules
  Given a frozen plan file under plans/done with a skipped heading level
  When docs validate-heading-hierarchy runs a full scan
  Then no skipped-level finding is reported for that file
  And plans/done is outside the allowlist
```

```gherkin
Scenario: Heading-hierarchy honors a repeated exclude flag
  Given a prose file under docs with a duplicate H1
  When docs validate-heading-hierarchy runs with --exclude docs
  Then no finding is reported for the excluded docs tree
  And other allowlist trees are still validated
```

```gherkin
Scenario: An app README is not checked by the MVP heading scope
  Given an apps/example/README.md with a skipped heading level
  When docs validate-heading-hierarchy runs a full scan
  Then no skipped-level finding is reported for the app README
  And apps is outside the MVP allowlist
```

### Enforcement consolidation (three layers, one CI workflow)

```gherkin
Scenario: The consolidated workflow triggers on PR and push to main
  Given the validate-markdown workflow definition
  When a contributor inspects its on block
  Then it triggers on pull_request to main
  And it triggers on push to main
```

```gherkin
Scenario: The consolidated workflow runs all three gates
  Given the validate-markdown workflow definition
  When a contributor inspects its jobs
  Then it runs the mermaid validator
  And it runs the links validator
  And it runs the heading-hierarchy validator
```

```gherkin
Scenario: The legacy link workflow is migrated
  Given the .github/workflows directory after this plan
  When a contributor lists the workflows
  Then pr-validate-links.yml no longer exists as a standalone file
  And its link check now runs inside validate-markdown.yml
```

```gherkin
Scenario: Pre-commit blocks but --no-verify is the escape
  Given a staged markdown file with a broken link
  When the contributor commits without --no-verify
  Then the commit is blocked by the link gate
  But committing with --no-verify bypasses the local gate
```

### Per-tree cleanup and dogfooding

```gherkin
Scenario: Each tree reports zero findings after its cleanup phase
  Given a tree-specific cleanup phase is complete
  When all three gates run against that tree within scope
  Then zero blocking findings are reported for that tree
```

```gherkin
Scenario: This plan passes its own gates
  Given the five plan documents under plans/in-progress/markdown-gate-coverage-expansion
  When all three gates run against the plans tree
  Then every diagram, link, anchor, and prose heading in this plan is valid
  And zero blocking findings are reported for this plan
```

### Spec parity and governance propagation

```gherkin
Scenario: The rhino-cli BDD specs cover the new validator behavior
  Given the spec files under specs/apps/rhino/behavior/cli/gherkin
  When the spec-coverage gate maps scenarios to validator code
  Then docs-validate-links.feature has scenarios for --exclude, repo-wide scan, and broken-anchor
  And docs-validate-heading-hierarchy.feature has scenarios for the prose allowlist and --exclude
  And git-pre-commit.feature has scenarios for the staged mermaid and heading steps
```

```gherkin
Scenario: The convention change is propagated and gate-validated
  Given the related governance docs updated for the new enforcement
  When repo-rules-maker performs the governance propagation sweep
  Then every related surface (diagrams, quality, linking, check-inventory) reflects the change
  And a strict repo-rules-quality-gate run reaches double-zero
```

## Product Scope

### In scope (features)

- Move the mermaid gate from pre-push to pre-commit staged-only (remove the `.husky/pre-push`
  mermaid trigger).
- `docs validate-links`: repeatable `--exclude <path>` flag wired through `ScanOptions.skip_paths`;
  repo-wide full scan minus the three named exclusions + noise-skip set; keep `.claude/skills/`
  hard-skip; new `broken-anchor` finding category via GitHub-slugify anchor validation reusing the
  `heading_hierarchy.rs` fence-aware parser.
- `docs validate-heading-hierarchy`: prose-allowlist default-deny scope enforced inside file
  selection (`docs/`, `repo-governance/`, `plans/`−`done/`, root `*.md`); `--exclude` flag for
  parity; wired into pre-commit staged-only + CI full-scan, blocking.
- A single `.github/workflows/validate-markdown.yml` (push + PR to `main`) running all three gates;
  migrate `pr-validate-links.yml` into it.
- Per-tree fix-all of surfaced violations (mermaid, broken links, broken anchors, prose headings),
  gated.
- Doc updates: `diagrams.md`, `quality.md`, `linking.md`, check-inventory docs.
- BDD spec updates under `specs/apps/rhino/` (link/heading/git-pre-commit `.feature` files +
  `component-cli.md`) in lockstep with the code, keeping the `spec-coverage` gate green.
- Governance propagation via `repo-rules-maker` (broad sweep), `npm run generate:bindings` re-sync,
  and a strict `repo-rules-quality-gate` double-zero validation.

### Out of scope (features)

- App READMEs in the heading-hierarchy allowlist (deferred).
- Markdown rendering / link-liveness / external-URL validation.
- Cross-file link-graph analysis beyond existence + anchor presence.
- Changing markdownlint's global MD025/MD001 config.

## Product Risks

| Risk                                                       | Mitigation                                                                                                                 |
| ---------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| Anchor slug algorithm diverges from GitHub rendering       | Implement GFM slug (lowercase, strip non-alnum except hyphen, spaces→hyphens, `-N` collisions); unit-test against fixtures |
| Heading allowlist leaks an agent/skill file                | Allowlist filter inside file selection; unit-test `.claude/**` and `SKILL.md` are excluded                                 |
| Wider link scan produces a large backlog                   | Per-tree gated fix-all; re-measure each tree at execution                                                                  |
| Migrating the link workflow drops PR link coverage         | Verify the link job runs in `validate-markdown.yml` on PRs before deleting the old file                                    |
| Pre-commit mermaid staged-only misses a cross-file issue   | Mermaid checks are per-file with no cross-file dependency — staged-only loses nothing                                      |
| `--exclude` prefix matching is too broad/narrow            | Reuse existing `filter_skip_paths` prefix semantics; unit-test included/excluded paths                                     |
| Shared heading-parser refactor regresses heading-hierarchy | Factor the parser without behavior change; all existing `heading_hierarchy.rs` tests stay green                            |
