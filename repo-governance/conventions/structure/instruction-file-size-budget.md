---
title: "Instruction-File Size Budget Convention"
description: Per-surface byte thresholds for auto-loaded instruction files, enforced by rhino-cli and git hooks
category: explanation
subcategory: conventions
tags:
  - instruction-files
  - agents-md
  - size-budget
  - governance
  - rhino-cli
created: 2026-06-27
---

# Instruction-File Size Budget Convention

Every coding-agent harness auto-loads certain instruction files before the first user message.
When those files exceed harness limits, instructions are **silently truncated or ignored** —
rules disappear without warning. This convention defines per-surface byte thresholds and the
single sanctioned remediation when a file exceeds them.

## Why This Rule Exists

Instruction files that grow without limit cause three classes of failure:

1. **Silent truncation**: Codex CLI truncates resolved `AGENTS.md` trees at 32,768 bytes.
   Some coding agents warn at 40,000 chars; AI coding editors hard-cap at 12,000 chars per file. Content
   past the limit vanishes silently — the agent behaves as if the rule never existed.
2. **Context dilution**: Every byte of instruction file is a byte not available for code
   context. Bloated instruction files degrade the agent's reasoning quality even when no
   truncation occurs.
3. **Governance debt**: Files that grow unchecked accumulate prose that duplicates content
   already reachable via a link — wasting both tokens and future maintenance effort.

The thresholds below reflect the narrowest known harness limit (12,000 chars/file for AI
coding editor context files) plus a conservative safety margin for the root AGENTS.md/CLAUDE.md pair.
Harness limit claims carry `[Judgment call]` labels in the plan that authored this convention;
consult the relevant harness documentation for authoritative values.

## Monitored Surfaces

The surfaces below are auto-loaded by at least one harness. Size thresholds are configured in
the `instruction-size:` section of `repo-config.yml` at the repo root and enforced by
`rhino-cli harness instruction-size validate`.

| Surface                           | Target (✅) | Warn (⚠️)  | Fail (❌)  |
| --------------------------------- | ----------- | ---------- | ---------- |
| `AGENTS.md`                       | ≤ 24,000 B  | ≤ 27,000 B | ≤ 30,000 B |
| `**/AGENTS.md`                    | ≤ 24,000 B  | ≤ 27,000 B | ≤ 30,000 B |
| `CLAUDE.md`                       | ≤ 6,000 B   | ≤ 8,000 B  | ≤ 10,000 B |
| `.amazonq/rules/*.md`             | ≤ 4,000 B   | ≤ 8,000 B  | ≤ 12,000 B |
| `.windsurf/rules/*.md`            | ≤ 6,000 B   | ≤ 9,000 B  | ≤ 12,000 B |
| `.cursor/rules/*.mdc`             | ≤ 4,000 B   | ≤ 8,000 B  | ≤ 12,000 B |
| `.junie/guidelines.md`            | ≤ 6,000 B   | ≤ 8,000 B  | ≤ 10,000 B |
| `.github/copilot-instructions.md` | ≤ 6,000 B   | ≤ 8,000 B  | ≤ 10,000 B |
| `CONVENTIONS.md`                  | ≤ 10,000 B  | ≤ 13,000 B | ≤ 16,000 B |
| Resolved tree (root `CLAUDE.md`)  | ≤ 30,000 B  | ≤ 34,000 B | ≤ 38,000 B |

**Classification**:

- **✅ OK** (`size ≤ target`): no output; gate exits 0
- **⚠️ Warn** (`target < size ≤ fail`): warning message; gate exits 0
- **❌ Fail** (`size > fail`): error message with remediation pointer; gate exits 1

The `warn` threshold signals "approaching the ceiling — act soon"; the `fail` threshold is the
hard gate. Resolved-tree size is computed by summing `CLAUDE.md` + all `@path`-imported files
(depth ≤ 4, cycle-guarded).

## Enforcement Points

The gate runs at four enforcement points, providing overlapping coverage:

### 1. Pre-push hard gate (primary)

```bash
# .husky/pre-push — changed-path gated
if echo "$CHANGED" | grep -qE '^(AGENTS\.md$|CLAUDE\.md$|repo-config\.yml$|...)'; then
  npx nx run rhino-cli:instruction-size:validation
fi
```

Fires on pushes that touch any monitored surface. Exits 1 if any surface is in Fail zone.
See `.husky/pre-push` for the full glob pattern.

### 2. Pre-commit backstop

`rhino-cli convention audit` (which runs in CI pre-commit) includes `instruction-size` as one
of four categories. Catches violations staged for commit before they reach the push gate.

### 3. PR quality gate (CI)

The `instruction-size` CI job in `.github/workflows/pr-quality-gate.yml` runs
`npx nx run rhino-cli:instruction-size:validation` on every PR and push to `main`. A failing
job blocks merge.

### 4. Deterministic preflight (`repo-governance audit`)

`rhino-cli repo-governance audit` includes `instruction-size` as category 4 alongside
`layer-coherence`, `traceability-audit`, and `vendor-audit`. This lets `repo-rules-checker`
consume the byte findings without re-deriving them (per the
[deterministic-vs-ai-validation-split](./deterministic-vs-ai-validation-split.md) convention).

## When the Gate Fails

**The only sanctioned remediation is progressive disclosure.**

Replace inline-expanded content with a one-line summary and a `See` link that points to the
content's canonical home in `repo-governance/`, `docs/`, or per-app `README.md`. The detail
remains fully accessible — it is just no longer inlined.

### Example

❌ **Before** (inline-expanded, 2,700 bytes):

```markdown
### ose-www

- **URL**: <https://oseplatform.com>
- **Production branch**: `prod-ose-www`
- **Framework**: Next.js 16 (App Router, TypeScript, tRPC)
- **Deployment**: Vercel
- **Dev port**: 3100
- **E2E tests**: `ose-www-be-e2e`, `ose-www-fe-e2e`
```

✅ **After** (progressive disclosure, ~50 bytes added to a table):

```markdown
| `ose-www` | oseplatform.com | 3100 | `prod-ose-www` |
```

See per-app `README.md` for framework, deployment, and E2E details.

### Forbidden Anti-Fixes

The following approaches **must not** be used:

1. **Delete a rule**: Removing governance or convention content to make the file smaller defeats
   the purpose of the instruction file. Rules must remain reachable.
2. **Compress to dense prose**: Stripping line breaks and turning readable paragraphs into
   dense single lines makes the file harder for agents and humans to parse. Brevity must not
   come at the cost of clarity.
3. **Split into another auto-loaded file**: Creating a second `AGENTS.md` or adding a new
   harness-auto-loaded file merely moves bytes without reducing the resolved-tree total — and
   may exceed per-file harness limits.
4. **Point at an incomplete target**: Replacing an inline enumeration with a `See` link to a
   document, table, or section that does not in fact cover every case the inline text covered.
   This looks like progressive disclosure and is actually **rule deletion in disguise** — the cases
   missing from the target silently lose their rule.

   **Observed failure**: compressing `AGENTS.md` under its 30,000-byte limit replaced an inline
   environment-branch enumeration with a pointer to a table that was not complete. Three deploy
   targets ended up uncovered by a "never commit directly" rule — and an agent force-pushes to one
   of them.

   **Before every `See`-link replacement, diff the target against ground truth** (per the
   [Absence and Completeness Claims](../../development/quality/plan-anti-hallucination.md#absence-and-completeness-claims-hard)
   rules — text search cannot find omissions):

   ```bash
   # ground truth from its owning authority (often NOT a file on disk)
   git branch -r | sed 's#^ *origin/##' | command grep -E '^(prod|stag)-' | sort > /tmp/truth.txt
   # what the See-link target actually covers
   command grep -oE '(prod|stag)-[a-z0-9-]+' <target-doc>.md | sort -u > /tmp/covered.txt
   comm -23 /tmp/truth.txt /tmp/covered.txt   # non-empty = incomplete target, DO NOT link to it
   ```

   **When the target is incomplete, the correct fixes are** (in order of preference): make the
   target complete first, then link to it; or restate the inline rule **as a pattern rather than
   an enumeration** so completeness is structural rather than maintained — e.g. "every `prod-*`
   and `stag-*` ref is a deploy target — never commit directly; `git branch -r` is authoritative",
   which is both shorter than the enumeration it replaces and immune to new branches appearing.
   Stating a guard by **what it protects** rather than by what it enumerates is the general form of
   this fix — see
   [Anti-Pattern 10: Enumeration-Based Guards](../../development/agents/anti-patterns.md#anti-pattern-10-enumeration-based-guards-denylist-guards-that-fail-open).

**Never compress a safety guardrail to save bytes.** The secrets/`.env` rules, the Git Identity
Guardrail, and the environment-branch rule are trimmed **last and only via a complete target** —
never by dropping cases, and never by dense-prose compression.

If none of the above remediation approaches is applicable, open a plan and request a threshold
adjustment with a documented rationale and harness source citation.

## Updating Thresholds

Thresholds live in the `instruction-size:` section of `repo-config.yml` at the repo root. To change
a threshold:

1. Edit the `instruction-size:` section of `repo-config.yml` with the new values.
2. Record the rationale and harness source citation as a comment in the YAML.
3. Run `npx nx run rhino-cli:instruction-size:validation` to confirm the new threshold is
   respected.
4. Commit and push; CI validates the change.

Do not adjust thresholds to paper over a bloated file — that defeats the purpose of the gate.
Adjust only when a new harness limit is discovered or when a surface genuinely requires more
content than the current threshold allows (with documented justification).

## Vision Supported

This convention serves the [Open Sharia Enterprise Vision](../../vision/open-sharia-enterprise.md)
by ensuring that governance rules embedded in instruction files are actually loaded and acted
upon — not silently dropped because the file grew past a harness limit. Reliable instruction
delivery is a prerequisite for the multi-harness agent ecosystem that powers the OSE Platform
development workflow.

## Principles Implemented/Respected

- **[Progressive Disclosure](../../principles/content/progressive-disclosure.md)**: The
  sole sanctioned remediation for size violations. Replace inline-expanded content with
  summary + link, not with deletions or dense compression. The convention is a direct
  application of this principle to instruction-file governance.

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Budget thresholds are declared explicitly in the `instruction-size:` section of `repo-config.yml`
  rather than embedded in the validator binary. Per-surface entries make limits visible and auditable.

- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**:
  All four enforcement points are automated (pre-push hook, CI job, `convention audit`,
  preflight category). No manual size-check is required.

- **[Reproducibility First](../../principles/software-engineering/reproducibility.md)**:
  The byte count is deterministic — same file always produces the same size finding. AI
  validators consume the finding rather than re-deriving it, per the
  [deterministic-vs-ai-validation-split](./deterministic-vs-ai-validation-split.md)
  convention.

## Related Conventions

- [Deterministic vs AI Validation Split](./deterministic-vs-ai-validation-split.md) — governs
  why byte-counting is a rhino-cli responsibility, not a repo-rules-checker responsibility
- [Governance Vendor-Independence Convention](./governance-vendor-independence.md) — the
  platform-binding section of AGENTS.md is skipped by vendor-audit; instruction-size still
  counts those bytes
- [Multi-Harness Binding Convention](./multi-harness-binding.md) — defines the monitored
  surfaces and their harness associations
