# Product Requirements — Multi-Harness Compatibility

## Product Overview

Deliver the artifacts that make `ose-public` harness-agnostic in governance and compatible with nine named
coding-agent harnesses: a hardened vendor-audit, a two-tier set of platform bindings, a research-backed
compatibility-audit workflow, supporting `rhino-cli` changes, updated specs, and propagated governance rules.

## Personas

- **Maya, the maintainer.** Runs governance audits and owns the binding catalog. Wants mechanical
  enforcement and a repeatable way to catch upstream config drift without manually reading nine docs sites.
- **Devi, the multi-tool contributor.** Switches between Cursor at work and Codex CLI at home. Wants the
  same project instructions to apply automatically in both, with no per-tool copy-paste.
- **Tariq, the template adopter.** Forks `ose-primer` (downstream of this repo) and uses Amazon Q. Wants the
  scaffolding to already account for a harness that does not read `AGENTS.md`.

## User Stories

1. As Maya, I want governance prose to fail the audit if it names any of the nine vendors outside an
   allowlisted region, so the governance layer stays neutral.
2. As Maya, I want a workflow that re-verifies each harness's current config conventions against our catalog,
   so I learn about drift as a tracked finding instead of a contributor bug report.
3. As Devi, I want every harness I use to pick up the repo's canonical instructions automatically, so I do
   not maintain per-tool instruction copies.
4. As Tariq, I want Amazon Q (which does not read `AGENTS.md`) to still receive the repo's instructions via a
   committed bridge file, so my harness is not a second-class citizen.
5. As Maya, I want every new `rhino-cli` behavior covered by a Gherkin spec under `specs/apps/rhino/`, so the
   spec-coverage gate stays green and behavior is documented.
6. As Maya, I want a deterministic pre-push check (a `rhino-cli` command, not an agent) that fails when a
   committed binding file drifts from `AGENTS.md` or the catalog, so parity is enforced on every push without
   relying on a slow or non-deterministic agent.
7. As Maya, I want every related Markdown file (catalog, agent roster, workflow/convention indexes, `AGENTS.md`
   binding lists) updated in one sweep, so no doc references a stale binding/vendor/agent set.

## Product Scope

**In:**

- Catalog and binding files for all nine harnesses + OpenCode.
- Vendor-audit vocabulary extension with false-positive-safe patterns.
- `repo-harness-compatibility-quality-gate` workflow + its checker/fixer agents.
- `rhino-cli` vendor-audit and binding-emitter changes + `specs/apps/rhino/` features.
- A deterministic, agent-free `rhino-cli` binding-parity guard wired into `.husky/pre-push`.
- A closing sweep updating all related Markdown files (catalog, agent roster, indexes, `AGENTS.md` lists).
- Governance-rule propagation via `repo-rules-maker` and validation via `repo-rules-quality-gate`.

**Out:**

- Rewriting canonical `AGENTS.md` content.
- Global/per-user harness config.
- Harness feature parity (skills/subagents) beyond instruction compatibility.

## Acceptance Criteria (Gherkin)

### AC1 — Governance prose stays vendor-neutral for all nine harnesses

```gherkin
Given the vendor-audit vocabulary has been extended for the nine harnesses
When a contributor writes "Junie" or "Antigravity" or "Amazon Q" in load-bearing prose under repo-governance/
And runs "rhino-cli repo-governance vendor-audit repo-governance/"
Then the audit exits non-zero
And the finding names the offending file, line, and a vendor-neutral replacement suggestion
```

### AC2 — Allowlisted regions and ambiguous tokens do not produce false positives

```gherkin
Given the vendor-audit covers the nine harnesses
When governance prose contains the mathematical constant "pi" in ordinary text
Or contains a vendor name inside a "Platform Binding Examples" heading section or a binding-example fence
Then the audit does not report those occurrences as violations
```

### AC3 — Every harness has a verified catalog entry

```gherkin
Given the platform-bindings catalog has been updated
When I open docs/reference/platform-bindings.md
Then each of the nine named harnesses plus OpenCode has a row
And each row records its root instruction file, whether it reads AGENTS.md natively, and its binding status
```

### AC4 — Amazon Q receives instructions via a committed bridge

```gherkin
Given Amazon Q Developer does not read AGENTS.md natively
When the Amazon Q binding is generated
Then a committed file under .amazonq/ references the repository's canonical instructions
And the binding does not duplicate AGENTS.md content verbatim in a way that can drift
```

### AC5 — No tool-specific file silently shadows AGENTS.md

```gherkin
Given some harnesses rank a tool-specific file above AGENTS.md (GEMINI.md, .junie/AGENTS.md, AGENTS.override.md)
When the bindings are created
Then the repository does not introduce any such higher-precedence file that overrides AGENTS.md with different content
And the no-shadowing rule is documented in the multi-harness binding convention
```

### AC6 — Compatibility-audit workflow detects drift using web research

```gherkin
Given the repo-harness-compatibility-quality-gate workflow exists
When the workflow runs
Then it delegates to web-researcher to fetch each supported harness's current config conventions
And it diffs those findings against docs/reference/platform-bindings.md and the committed binding files
And it writes a drift audit report to generated-reports/ citing the web sources used
```

### AC7 — rhino-cli changes are spec-covered

```gherkin
Given rhino-cli vendor-audit and binding-emitter behavior changed
When the spec-coverage gate runs for rhino-cli
Then every new or changed behavior has a corresponding Gherkin feature under specs/apps/rhino/
And "nx run rhino-cli:test:quick" and the spec-coverage target pass
```

### AC8 — Governance rules propagated and validated

```gherkin
Given the governance rules for multi-harness binding have been authored
When repo-rules-maker has propagated them and repo-rules-quality-gate has run in strict mode
Then the quality gate reaches two consecutive zero-finding validations on the changed governance files
```

### AC9 — Deterministic pre-push parity guard catches internal binding drift

```gherkin
Given the generated binding files are committed and a deterministic rhino-cli parity guard is wired into pre-push
When a committed binding file is edited so it no longer matches a regenerate from AGENTS.md
Or a binding directory exists on disk without a row in the platform-bindings catalog
And the pre-push hook runs
Then the deterministic guard exits non-zero and blocks the push
And the guard uses no AI agent and makes no network calls
```

### AC10 — All related Markdown files are updated

```gherkin
Given the binding catalog, vendor vocabulary, agent roster, and workflow index changed
When the closing documentation sweep runs
Then a grep for the old binding/vendor/agent state across tracked .md files returns no stale references
And "npm run lint:md" exits 0
```

## Product Risks

- **Spec-coverage friction.** New `rhino-cli` behavior without matching specs would fail the gate; mitigated
  by pairing every behavior change with a feature file in the same delivery phase.
- **Workflow naming conformance.** The new workflow must match the `-(quality-gate|execution|setup)` suffix
  rule; mitigated by naming it `repo-harness-compatibility-quality-gate`. [Repo-grounded —
  `repo-governance/conventions/structure/workflow-naming.md`]
- **Agent proliferation.** Two new agents are added; mitigated by reusing the established maker-checker-fixer
  pattern and keeping them narrowly scoped to compatibility auditing.
