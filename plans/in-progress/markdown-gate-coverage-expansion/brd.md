# Business Requirements Document — Markdown Gate Coverage Expansion

## Business Goal

Make the repository's **structural markdown validation trustworthy and consistent** across three
validators — Mermaid diagrams, relative links (with anchors), and heading hierarchy — so that no
markdown file with a malformed diagram, a broken link or anchor, or (for generic prose) a broken
heading structure can reach `main` undetected, whether committed locally or through CI.

## Business Rationale (WHY)

The repository runs three structural markdown validators, all built into `rhino-cli`, but they are
wired inconsistently and one is entirely orphaned. [Repo-grounded]

- **Mermaid** enforces accessible, render-correct diagrams — but only at pre-push, with a narrow
  trigger. [Repo-grounded — `.husky/pre-push:24`]
- **Relative-link checker** verifies links resolve to real files — but scans only three trees,
  never validates `#fragment` anchors, and has no way to exclude content trees that own their own
  link validation. [Repo-grounded — `links.rs:169,374`]
- **Heading-hierarchy** is fully implemented and unit-tested but runs **nowhere** — no hook, no Nx
  target, no CI. [Repo-grounded — orphaned in `cli.rs`] A validator that runs nowhere is dead code
  that gives zero protection. [Judgment call]

Unifying enforcement turns three half-wired checks into one coherent gate. The biggest correctness
win is **anchor validation**: a link like `[X](target#section-that-was-renamed)` resolves today
because only the file is checked, never the heading — so renamed-section links rot silently and
mislead readers. [Repo-grounded — `resolve_link` strips the fragment]

### The heading-hierarchy non-breaking constraint (governance-critical)

`markdownlint` disables MD025 (multiple H1) and MD001 (heading increment) globally because agent
and skill prompt artifacts legitimately use `#` as section markers, not titles. [Repo-grounded —
`.markdownlint-cli2.jsonc`] Empirically the repo has many such files under `.claude/agents/`,
`.claude/skills/`, and `.opencode/agents/`. Re-enabling heading rules naively would break all of
them.

This plan re-enables heading rules **scoped to generic prose** via rhino-cli (which can path-scope
a rule, unlike markdownlint). The narrative the maintainer values: prose docs (`docs/`,
`repo-governance/`, `plans/`, root `*.md`) regain single-H1 and non-skipping enforcement, while
prompt/skill artifacts stay exempt by a hard default-deny allowlist. This is the central business
constraint of the plan — breaking an agent or skill file is unacceptable. [Judgment call]

## Business Impact

### Pain points addressed

- **Silent diagram defects** in unscanned trees (mermaid) reach `main`. [Judgment call]
- **Rotting anchors** — section-renamed links resolve falsely because anchors are unchecked.
  [Repo-grounded]
- **Inconsistent link scope** — content trees with their own specialized link CLIs
  (`ayokoding-cli`, `ose-cli`) cannot be excluded, so the generic checker either misfires on them
  or is left repo-narrow. [Repo-grounded — no `--exclude` flag exists]
- **Dead heading validator** — fully built, tested, and registered, yet protecting nothing.
  [Repo-grounded]
- **Inconsistent enforcement layers** — mermaid at pre-push, links at pre-commit + PR-only CI,
  headings nowhere; a direct trunk push can slip the mermaid gate. [Repo-grounded]

### Expected benefits

- **One coherent markdown gate** — three validators, three consistent enforcement layers
  (pre-commit staged-only, PR CI, push CI), one consolidated CI workflow.
- **Anchor-safe links** — `#fragment` targets are validated against the destination file's actual
  headings, so renamed sections surface as `broken-anchor` findings.
- **Excludable content trees** — `--exclude` lets the generic link/mermaid gates skip trees with
  specialized validation (`plans/done/`, the two content trees).
- **Prose heading guarantees without collateral damage** — single-H1 and non-skipping nesting are
  enforced for prose, while agent/skill artifacts are provably exempt.
- **Unskippable CI** — PR and push-to-`main` CI both run all three gates; `--no-verify` only skips
  the local pre-commit layer.

## Affected Roles

Solo-maintainer repository; the maintainer wears several hats and several agents consume the
surfaces. No sign-off ceremonies apply.

- **Maintainer (tooling hat)** — owns `rhino-cli`, the Nx targets, the hooks, the CI workflow.
- **Maintainer (governance hat)** — owns `diagrams.md`, `quality.md`, `linking.md`, check-inventory
  docs.
- **Maintainer (content hat)** — authors diagrams, links, and prose headings across `docs/`,
  `plans/`, governance.
- **Consuming agents** — `swe-rust-dev` (validators/CI/hooks), `repo-rules-maker` / `docs-maker`
  (convention docs), `repo-setup-manager` (Phase 0 baseline), and any agent that authors markdown
  now covered by the gates. [Repo-grounded — agents confirmed present]

## Business-Level Success Metrics

- **Coverage completeness** (observable): the link and mermaid gates scan the whole repo minus the
  three named exclusions + noise dirs; heading-hierarchy scans exactly the prose allowlist —
  verifiable by running each validator and inspecting the scanned file set.
- **Anchor enforcement** (observable): a link to a non-existent `#fragment` produces a
  `broken-anchor` finding — verifiable by a unit test.
- **Heading non-breakage** (observable): staging a `.claude/agents/*.md` or `SKILL.md` file
  produces ZERO heading findings even at pre-commit — verifiable by a unit test asserting the
  allowlist filter excludes those paths. [Judgment call: this is the guarantee the maintainer
  values most]
- **Unskippability** (observable): a single `validate-markdown.yml` workflow runs all three gates
  on `pull_request` AND `push` to `main` — verifiable by inspecting `.github/workflows/`.
- **Zero blocking findings** (observable): all three gates report zero findings repo-wide (within
  their scopes) at plan completion — verifiable by running them.
- **Pre-push no longer runs mermaid** (observable): `.husky/pre-push` contains no mermaid
  trigger — verifiable by inspection.

## Business-Scope Non-Goals

- Not building a markdown renderer, link-liveness checker, or external-URL validator.
- Not extending heading-hierarchy to app READMEs in this plan (deferred).
- Not changing the markdownlint global config (MD025/MD001 stay disabled there; rhino-cli does the
  path-scoped prose enforcement instead).
- Not redesigning the mermaid palette or the linking convention syntax — only adding enforcement
  and correcting docs.

## Business Risks and Mitigations

| Risk                                                           | Likelihood | Mitigation                                                                                                             |
| -------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------- |
| Wider link scan surfaces a large broken-link/anchor backlog    | High       | Phase the cleanup one gate per tree; re-measure per tree at execution; fix or correct each before its gate             |
| Heading-hierarchy accidentally fires on an agent/skill file    | Medium     | Allowlist filter lives INSIDE the validator file selection + a unit test asserts `.claude/**`/`SKILL.md` are excluded  |
| Prose heading backlog larger than expected                     | Medium     | Per-tree gates isolate scope; re-measure at execution; fix or restructure each finding                                 |
| Anchor slug algorithm diverges from GitHub rendering           | Medium     | Implement the GFM slug algorithm with collision suffixes; unit-test against fixture headings                           |
| Moving mermaid to pre-commit slows commits                     | Low        | Staged-only scope keeps it light; per-file checks have no cross-file dependency                                        |
| Consolidating the existing link workflow breaks PR link checks | Low        | Migrate `pr-validate-links.yml` into `validate-markdown.yml`; verify the link job still runs on PRs                    |
| `--exclude` paths drift from the content CLIs' real coverage   | Low        | Pass the three named exclusions explicitly at call sites (visible, testable); document the rationale in `tech-docs.md` |

See [prd.md](./prd.md) for the testable scenarios that verify each mitigation.
