# Product Requirements Document — Mermaid Gate Coverage Expansion

## Product Overview

The product is the **Mermaid validation gate**: the `rhino-cli docs validate-mermaid` command,
its `validate:mermaid` Nx target, and **triple-layer enforcement** — the `.husky/pre-push` trigger
(Layer 1), a dedicated `pr-validate-mermaid.yml` workflow on `pull_request` to `main` (Layer 2),
and the same workflow on `push` to `main` (Layer 3). This
plan upgrades the gate to scan the whole repository, adds a reviewable exemption mechanism,
promotes complexity warnings to blocking, adds a blocking off-palette color check across the five
eligible diagram types, and cleans the resulting baseline.

## Personas

Solo-maintainer repository; personas are hats the maintainer wears and agents that consume the
surfaces.

- **Diagram author** — writes flowcharts in any markdown tree; wants fast, accurate feedback
  before pushing.
- **Tooling maintainer** — owns the validator; wants the byte-for-byte port's existing tests to
  stay green and new behavior fully covered.
- **Reviewer (governance hat)** — wants every complexity exemption to carry a written rationale.
- **CI** — must block PRs and report on direct `main` pushes that introduce malformed flowcharts,
  even when local hooks are skipped with `--no-verify`.

## User Stories

- **US-1** — As a diagram author, I want the gate to scan **every** markdown tree, so a malformed
  flowchart anywhere is caught before it reaches `main`.
- **US-2** — As CI, I want a dedicated workflow that runs `validate:mermaid` on BOTH `pull_request`
  to `main` AND `push` to `main`, so the gate cannot be bypassed with `git push --no-verify` and
  also gates direct trunk-based pushes that never open a PR.
- **US-3** — As a diagram author, I want an inline directive to exempt a **genuinely** complex
  flowchart from a structural complexity check, so I am not forced to over-simplify a correct
  diagram.
- **US-4** — As a reviewer, I want every exemption to require an adjacent written reason, so no
  exemption is silent or undocumented.
- **US-5** — As a tooling maintainer, I want `complex_diagram` and `subgraph_dense` to **block**,
  so inherently complex diagrams are deliberately simplified or explicitly exempted, never
  ignored.
- **US-6** — As a tooling maintainer, I want the flowchart-only invariant preserved and tested,
  so the widened scan never starts erroring on sequence/ER/gantt diagrams.
- **US-7** — As a contributor reading governance docs, I want `diagrams.md` to accurately
  describe coverage, the exemption directive, the per-type validation scope, the blocking
  behavior, and the new non-exemptable color check.
- **US-8** — As a reader with color blindness, I want every diagram across the eligible types to
  use only the canonical WCAG AA color-blind-friendly palette, so I never lose information a
  diagram encodes by color; off-palette colors must be caught and blocked, never exempted.
- **US-9** — As a tooling maintainer, I want each diagram type validated by exactly the right
  check set (flowchart = all structural + color; classDiagram / stateDiagram-v2 /
  requirementDiagram / quadrantChart = color only; all other types = skipped), so the widened scan
  enforces accessibility without misapplying flowchart-only structural rules to other types.
- **US-10** — As a diagram author, I want a literal `\n` inside any flowchart label to block, so a
  label never ships rendering as the literal characters `\n` instead of a line break.
- **US-11** — As a diagram author, I want a `"` inside a `[...]` node label to block, so a flowchart
  never silently breaks the Mermaid parser.
- **US-12** — As a diagram author, I want flowchart edge labels longer than 20 characters to block
  (edges cannot use `<br/>`), with an `allow-edge-label` + `reason:` escape hatch for genuine cases.
- **US-13** — As a diagram author, I want a flowchart whose direction is `TD`/`TB`/`BT`/`RL` (not
  `LR`) to block, with an `allow-direction` + `reason:` escape hatch for semantically top-down
  diagrams or where LR would exceed MaxWidth=4.
- **US-14** — As a reviewer, I want every flowchart to carry **exactly one** color-palette comment
  (zero or duplicate blocks), so palette documentation is present and not cluttered; this is never
  exemptable.
- **US-15** — As a diagram author, I want a box-drawing separator whose width does not match the
  longest text line in its node to block, with an `allow-separator` + `reason:` escape hatch for the
  mechanically fuzzy edge cases.
- **US-16** — As a tooling maintainer, I want `collect_md_default_dirs()` to include `apps/` and
  `libs/` and the Nx target to agree with it, so a bare invocation and the target scan the same
  full repo.

## Acceptance Criteria (Gherkin)

### Scope expansion

```gherkin
Scenario: Gate scans all markdown trees
  Given the validate:mermaid Nx target is invoked
  When the validator collects markdown files
  Then it scans repo-governance, .claude, plans, docs, apps, and libs
  And it scans root AGENTS.md, CLAUDE.md, and README.md
  And it skips .next, node_modules, and .git directories
```

```gherkin
Scenario: Pre-push fires on any markdown change
  Given a commit changes a markdown file under plans
  When the pre-push hook evaluates its triggers
  Then it runs the validate:mermaid Nx target
```

```gherkin
Scenario: Pre-push ignores non-markdown-only changes for the mermaid gate
  Given a commit changes only a Rust source file and no markdown
  When the pre-push hook evaluates its mermaid trigger
  Then it does not run the validate:mermaid target for that trigger
```

### Triple-layer enforcement (pre-push + PR CI + push-to-main CI)

Layer 1 (pre-push) is covered by the "Pre-push fires on any markdown change" and "Pre-push ignores
non-markdown-only changes" scenarios above. The two scenarios below cover Layers 2 and 3.

```gherkin
Scenario: Layer 2 — PR CI blocks a pull request on a mermaid violation
  Given a pull request to main that introduces a malformed flowchart
  When the pr-validate-mermaid workflow runs on the pull_request event
  Then the workflow runs npx nx run rhino-cli:validate:mermaid
  And the pr-validate-mermaid check fails
  And the pull request is blocked from merging
```

```gherkin
Scenario: Layer 3 — push-to-main CI reports a mermaid violation on a direct trunk push
  Given a direct push to main that introduces a malformed flowchart
  When the pr-validate-mermaid workflow runs on the push event
  Then the workflow runs npx nx run rhino-cli:validate:mermaid
  And the pr-validate-mermaid check fails for that push
```

```gherkin
Scenario: The dedicated mermaid workflow triggers on both pull_request and push to main
  Given the pr-validate-mermaid workflow definition
  When a contributor inspects its on block
  Then it triggers on pull_request to main
  And it triggers on push to main
  And its single job invokes npx nx run rhino-cli:validate:mermaid
```

### Exemption directive (flowchart-only, structure-kinds-only, rationale-required)

```gherkin
Scenario: Width exemption with reason is honored
  Given a flowchart block exceeding the max width
  And the block contains "%% validate-mermaid: allow-width"
  And the block contains an adjacent "%% reason: intentionally wide ETL fan-out"
  When the validator checks the block
  Then the width_exceeded violation is suppressed for that block
  And the validator reports zero blocking findings for that block
```

```gherkin
Scenario: Exemption without a reason is a hard error
  Given a flowchart block containing "%% validate-mermaid: allow-width"
  And the block contains no adjacent "%% reason:" line
  When the validator checks the block
  Then the validator reports a blocking error for the missing reason
  And the validator exits non-zero
```

```gherkin
Scenario: Label length can never be exempted
  Given a flowchart block with a node label longer than 30 characters
  And the block contains "%% validate-mermaid: allow-label" with a reason
  When the validator checks the block
  Then the label_too_long violation still blocks
  And the validator exits non-zero
```

```gherkin
Scenario: Multiple diagrams per block can never be exempted
  Given a single mermaid fence containing two flowchart headers
  And the block contains an allow directive with a reason
  When the validator checks the block
  Then the multiple_diagrams violation still blocks
  And the validator exits non-zero
```

```gherkin
Scenario: Exemption directive on a non-flowchart block has no effect
  Given a sequenceDiagram block containing an allow directive
  When the validator checks the block
  Then the block yields zero findings
  And the directive is ignored because the block is not a flowchart
```

### Warnings promoted to blocking

```gherkin
Scenario: Complex diagram now blocks without an exemption
  Given a flowchart block exceeding both max width and max depth
  And the block contains no exemption directive
  When the validator checks the block
  Then a complex_diagram violation blocks
  And the validator exits non-zero
```

```gherkin
Scenario: Dense subgraph now blocks without an exemption
  Given a flowchart subgraph with more children than the configured maximum
  And the block contains no exemption directive
  When the validator checks the block
  Then a subgraph_dense violation blocks
  And the validator exits non-zero
```

### Off-palette color check (eligible-type matrix, blocking, non-exemptable)

```gherkin
Scenario: Off-palette fill in a flowchart blocks
  Given a flowchart block whose classDef uses fill:#0072B2
  And #0072B2 is not a member of the canonical palette
  When the validator checks the block
  Then an off_palette_color violation blocks
  And the validator exits non-zero
```

```gherkin
Scenario: On-palette colors in a flowchart pass
  Given a flowchart block whose classDef uses only fill:#0173B2, stroke:#000000, color:#FFFFFF
  And every hex literal is a member of the canonical palette
  When the validator checks the block
  Then no off_palette_color violation is reported
```

```gherkin
Scenario: Off-palette color in a classDiagram blocks
  Given a classDiagram block whose style uses fill:#D55E00
  And #D55E00 is not a member of the canonical palette
  When the validator checks the block
  Then an off_palette_color violation blocks
  And the validator exits non-zero
```

```gherkin
Scenario: classDiagram receives the color check only, never structural checks
  Given a classDiagram block with a node label longer than 30 characters
  And a structurally wide arrangement of classes
  When the validator checks the block
  Then no label_too_long, width_exceeded, complex_diagram, or subgraph_dense finding is reported
  And only the off_palette_color check is applied to the block
```

```gherkin
Scenario: Off-palette inline color in a quadrantChart point blocks
  Given a quadrantChart block with an inline point color color:#009E73
  And #009E73 is not a member of the canonical palette
  When the validator checks the block
  Then an off_palette_color violation blocks
  And the validator exits non-zero
```

```gherkin
Scenario: stateDiagram-v2 and requirementDiagram get the color check
  Given a stateDiagram-v2 block and a requirementDiagram block each using an off-palette fill
  When the validator checks the blocks
  Then each block reports an off_palette_color violation
  And the validator exits non-zero
```

```gherkin
Scenario: Theme-only diagram types are skipped entirely
  Given a sequenceDiagram block and a gantt block
  When the validator runs over the full repository
  Then no finding of any kind is reported for either block
  And no off_palette_color check is applied because they expose no per-element source hex
```

```gherkin
Scenario: Color can never be exempted
  Given a flowchart block with an off-palette fill
  And the block contains "%% validate-mermaid: allow-color" with a reason
  When the validator checks the block
  Then the allow-color directive is rejected as an unsupported invalid directive
  And the off_palette_color violation still blocks
  And the validator exits non-zero
```

```gherkin
Scenario: Attempting to exempt label via directive is rejected
  Given a flowchart block containing "%% validate-mermaid: allow-label" with a reason
  When the validator checks the block
  Then the allow-label directive is rejected as an unsupported invalid directive
  And the validator exits non-zero
```

### New flowchart-only checks (Amendment A)

```gherkin
Scenario: Literal backslash-n in a flowchart label blocks and can never be exempted
  Given a flowchart block whose node label contains a literal "\n" escape
  When the validator checks the block
  Then a literal_backslash_n violation blocks
  And the validator exits non-zero
  And an "allow-backslash-n" directive with a reason is rejected as an invalid directive
```

```gherkin
Scenario: A double-quote inside a bracket node label blocks and can never be exempted
  Given a flowchart block with a node label of the form [He said "hi"]
  When the validator checks the block
  Then a quotes_in_brackets violation blocks
  And the validator exits non-zero
  And an "allow-quotes" directive with a reason is rejected as an invalid directive
```

```gherkin
Scenario: An over-long edge label blocks but is exemptable
  Given a flowchart edge label exceeding 20 characters
  And the block contains no exemption directive
  When the validator checks the block
  Then an edge_label_too_long violation blocks
  And the validator exits non-zero

Scenario: An over-long edge label with allow-edge-label and reason is suppressed
  Given a flowchart edge label exceeding 20 characters
  And the block contains "%% validate-mermaid: allow-edge-label"
  And the block contains an adjacent "%% reason: contract event name is fixed"
  When the validator checks the block
  Then the edge_label_too_long violation is suppressed for that block
```

```gherkin
Scenario: A non-LR flowchart direction blocks but is exemptable
  Given a flowchart whose opening directive is "flowchart TD"
  And the block contains no exemption directive
  When the validator checks the block
  Then a non_lr_direction violation blocks
  And the validator exits non-zero

Scenario: A non-LR direction with allow-direction and reason is suppressed
  Given a flowchart whose opening directive is "flowchart TD"
  And the block contains "%% validate-mermaid: allow-direction"
  And the block contains an adjacent "%% reason: inheritance is intrinsically top-down"
  When the validator checks the block
  Then the non_lr_direction violation is suppressed for that block
```

```gherkin
Scenario: Exactly one palette comment is required and can never be exempted
  Given a flowchart block with zero color-palette comments
  When the validator checks the block
  Then a palette_comment_count violation blocks
  And the validator exits non-zero

Scenario: Duplicate palette comments block
  Given a flowchart block with two identical color-palette comments
  When the validator checks the block
  Then a palette_comment_count violation blocks
  And the validator exits non-zero

Scenario: A single palette comment passes
  Given a flowchart block with exactly one color-palette comment
  When the validator checks the block
  Then no palette_comment_count violation is reported

Scenario: A palette-comment exemption directive is rejected
  Given a flowchart block with zero palette comments
  And the block contains "%% validate-mermaid: allow-palette-comment" with a reason
  When the validator checks the block
  Then the directive is rejected as an invalid directive
  And the palette_comment_count violation still blocks
```

```gherkin
Scenario: A mismatched separator width blocks but is exemptable
  Given a flowchart node whose box-drawing separator length does not match its longest text line
  And the block contains no exemption directive
  When the validator checks the block
  Then a separator_width_mismatch violation blocks
  And the validator exits non-zero

Scenario: A mismatched separator with allow-separator and reason is suppressed
  Given a flowchart node with a mismatched separator width
  And the block contains "%% validate-mermaid: allow-separator"
  And the block contains an adjacent "%% reason: intentional fixed-width header"
  When the validator checks the block
  Then the separator_width_mismatch violation is suppressed for that block
```

```gherkin
Scenario: The six new checks apply to flowcharts only
  Given a classDiagram block and a sequenceDiagram block
  When the validator checks the blocks
  Then no literal_backslash_n, quotes_in_brackets, edge_label_too_long, non_lr_direction,
       palette_comment_count, or separator_width_mismatch finding is reported for either block
```

### Default-dir reconciliation (Amendment B)

```gherkin
Scenario: Default-dir collection includes apps and libs
  Given the validator collects markdown files with no positional path arguments
  When collect_md_default_dirs runs
  Then the returned set includes files under apps and libs
  And it includes files under docs, repo-governance, .claude, and plans
  And it skips generated directories (.next, node_modules, .git, dist, target, .opencode)
```

### Flowchart-only structural invariant (non-regression)

```gherkin
Scenario: Non-flowchart structural checks never fire
  Given the widened scan covers apps and docs
  And those trees contain sequenceDiagram, gantt, classDiagram, and stateDiagram-v2 blocks
  When the validator runs over the full repository
  Then no label_too_long, width_exceeded, complex_diagram, or subgraph_dense finding
       is reported for any non-flowchart block
  And the only check that may fire on classDiagram / stateDiagram blocks is off_palette_color
```

### Baseline cleanup (per tree)

```gherkin
Scenario: Each tree reports zero blocking findings after its cleanup phase
  Given a tree-specific cleanup phase is complete
  When the expanded validator runs against that tree
  Then it reports zero blocking findings for that tree
```

### Documentation accuracy

```gherkin
Scenario: diagrams.md is accurate
  Given the diagrams convention document
  When a reader inspects the coverage statement
  Then it correctly lists the full set of scanned trees
  And it documents the triple-layer enforcement (pre-push, PR CI, push-to-main CI)
  And it documents the allow-<kind> directive and mandatory reason syntax
  And it states the flowchart-only scope for structural checks
  And it states the five eligible types for the color check
  And it states that complex_diagram and subgraph_dense are blocking
  And it states that off_palette_color is blocking and never exemptable
```

### Dogfooding

```gherkin
Scenario: This plan passes its own gate
  Given the five plan documents under plans/in-progress/mermaid-gate-coverage-expansion
  When the expanded validator scans the plans tree
  Then every flowchart in this plan uses LR direction and exactly one palette comment
  And no flowchart contains a literal "\n", a quote inside brackets, or an edge label over 20 chars
  And every color hex used in this plan is a member of the canonical palette
  And the validator reports zero blocking findings of every kind for this plan
```

## Product Scope

### In scope (features)

- Full-repo scan path expansion (Nx target + `collect_md_default_dirs()` reconciled to include
  `apps`/`libs` and agree with the target — Amendment B).
- Any-markdown pre-push trigger (Layer 1).
- Dedicated `pr-validate-mermaid.yml` CI workflow with dual triggers — `pull_request` to `main`
  (Layer 2) and `push` to `main` (Layer 3) — running `npx nx run rhino-cli:validate:mermaid`.
- Inline exemption directive with mandatory `reason:`. Exemptable kinds:
  `allow-width`, `allow-complexity`, `allow-subgraph-density`, `allow-edge-label`,
  `allow-direction`, `allow-separator`.
- Six new flowchart-only checks (Amendment A): `literal_backslash_n` (never exemptable),
  `quotes_in_brackets` (never exemptable), `edge_label_too_long` (exemptable), `non_lr_direction`
  (exemptable), `palette_comment_count` (never exemptable), `separator_width_mismatch` (exemptable).
- Promotion of `complex_diagram` and `subgraph_dense` to blocking.
- New blocking `off_palette_color` check across the five eligible diagram types (flowchart/graph,
  classDiagram, stateDiagram/v2, requirementDiagram, quadrantChart), allowlist sourced from
  `diagrams.md`, never exemptable.
- Per-diagram-type dispatch matrix (flowchart = all structural + color; the four other eligible
  types = color only; all remaining types = skipped).
- Phased per-tree fix-all baseline cleanup (labels, structure, AND off-palette colors).
- `diagrams.md` accuracy + new-behavior documentation.

### Out of scope (features)

- **Structural** checks on non-flowchart diagram types (label/width/complex/subgraph-dense) —
  flowchart-only, unchanged.
- Color validation on `erDiagram` and C4 (PARTIAL — deferred to a future plan).
- Color validation on theme-only types (sequenceDiagram, gantt, pie, gitGraph, journey, timeline,
  mindmap — no per-element source hex).
- Exemptions for `label_too_long`, `multiple_diagrams`, `off_palette_color`, `literal_backslash_n`,
  `quotes_in_brackets`, or `palette_comment_count` — never exemptable (any `allow-*` for these is
  rejected as an invalid directive).
- Extending `literal_backslash_n` / `quotes_in_brackets` (and the other new checks) to non-flowchart
  bracket-label diagram types — flowchart-only, deferred to a future plan.
- Mermaid rendering / visual diffing.
- Moving the gate to pre-commit.

## Product Risks

| Risk                                                         | Mitigation                                                                                                              |
| ------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------- |
| Directive syntax ambiguity                                   | Single documented syntax; unit tests cover present/absent/malformed cases                                               |
| Author confusion about which kinds are exemptable            | `diagrams.md` lists exemptable vs never-exemptable kinds explicitly                                                     |
| Re-measured warning count larger than expected               | Per-tree gates isolate scope; executor re-measures with blocking semantics on                                           |
| Existing byte-for-byte port tests regress                    | All existing in-file tests must stay green; gate each Rust phase on them                                                |
| Color allowlist hardcoded from memory drifts from convention | Extract the allowed hex set FROM `diagrams.md` at implementation time; unit-test the set against the documented palette |
| Hex case / shorthand mismatch (`#FFF` vs `#FFFFFF`)          | Normalize case and expand 3-digit shorthand before comparison; unit-test both forms                                     |
| Color check misapplied as structural to non-flowchart types  | Dispatch matrix is explicit and unit-tested: non-flowchart eligible types get color-only                                |
| `non_lr_direction` flags a large existing TD/TB backlog      | Expected largest fix-all contributor; convert to LR or add justified `allow-direction`; re-measure per tree             |
| `separator_width_mismatch` is mechanically fuzzy             | Exemptable via `allow-separator` + reason for genuine edge cases; unit-test the matched vs mismatched cases             |
| Six new checks fire on non-flowchart blocks                  | Flowchart-only by design; unit-test that classDiagram/sequenceDiagram yield none of the six                             |
