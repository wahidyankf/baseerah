# AyoKoding mermaid diagram remediation

One-line summary: 644 mermaid violations across 241 `apps/ayokoding-www/content` tutorial files
became visible when a validator bug was fixed; remediate them and drop the temporary CI exclude.

> Surfaced 2026-07-21 while fixing the `detect_kind` leading-comment bug in `rhino-cli md mermaid validate`.

## Problem / context

`rhino-cli md mermaid validate` silently skipped **any** mermaid block whose first line inside the
fence was a `%%` comment. `detect_kind` skipped blank lines but treated a comment as an unrecognised
diagram type, returned `DiagramKind::Other`, and `validate_one_block` then returned early — bypassing
every label-length, width, depth, and subgraph rule for that block.

**2,852 of 3,923 mermaid blocks (73%), across 637 files, opened with a `%%` line** and were therefore
never validated. The colour-palette header used repo-wide is a `%%` line, and the Diagrams Convention
itself _mandates_ a `%%` justification comment above the directive for the `TD` exception — so the
diagrams most in need of checking were exactly the ones skipped.

With the parser fixed, 665 violations appeared. 21 were fixed immediately (governance, plans, docs,
specs, ose-www). The remainder is concentrated in one tree:

| Tree                         | Findings  | Files |
| ---------------------------- | --------- | ----- |
| `apps/ayokoding-www/content` | **644**   | 241   |
| everything else              | 0 (fixed) | —     |

Breakdown of the 644: predominantly `label_too_long` (node labels over the 30-char-per-line limit),
with a minority of `width_exceeded` (chain depth over 4 in `LR`, or over 4 nodes at one rank in `TD`).

## Why now

A temporary `--exclude apps/ayokoding-www/content` is in place in `.github/workflows/main-ci.yml` and
the `package.json` lint-staged `*.md` chain so the parser fix could land green. That exclude is a
coverage hole in the **reader-facing** site — precisely where the convention's mobile-rendering
rationale matters most. It should not become permanent by default.

## Prior art / precedents

- **Diagrams Convention → Flowchart Width Constraints** — the direction-aware rule being enforced:
  in `LR` the checked horizontal axis is _depth_; in `TD` it is _span_.
  [diagrams](../../repo-governance/conventions/formatting/diagrams.md)
- **`md links validate` content excludes** — the existing precedent for exempting
  `apps/ayokoding-www/content` and `apps/ose-www/content` from a repo-wide markdown gate, already
  applied in both the pre-push hook and CI. This brief follows that shape but aims to _remove_ the
  exemption rather than institutionalise it.
- **The split-into-shallow-LR-diagrams technique** — proven on 7 diagrams during the parser fix: because
  `LR` checks only depth, splitting one deep chain into several 2-level `LR` diagrams passes while
  preserving every node and edge. This is the mechanical remedy for `width_exceeded`.
- **mermaid-state-label-render-clipping-warn** — the sibling brief on a different mermaid rule gap.
  [brief](./mermaid-state-label-render-clipping-warn.md)

## Proposed direction (sketch)

1. **Triage by violation kind.** `label_too_long` is mostly mechanical (insert `<br/>` at a sensible
   phrase boundary, or shorten wording); `width_exceeded` needs the split technique and real editorial
   judgement.
2. **Batch by tutorial family**, not by file — `by-example`, `annotated-concept`, `primer`,
   `in-the-field` have distinct diagram idioms, so a fix pattern established once applies across a
   family.
3. **Respect bilingual parity** — `apps/ayokoding-www/content` is bilingual; an `en` diagram edit needs
   its `id` counterpart edited to match, or the two drift.
4. **Drop the exclude** from `main-ci.yml` and `package.json` as the terminal step, and assert the
   repo-wide run is clean without it.

## Rough scope & non-goals

In scope: the 644 findings in `apps/ayokoding-www/content`; removing the temporary exclude from both
invocation sites; a spot-check that rendered pages still read correctly on a narrow viewport.

Out of scope (for now): relaxing the 30-char / 4-node thresholds themselves (a separate conversation —
if the thresholds are wrong for educational content, that is a convention change, not a remediation);
the `subgraph_density` warnings, which are advisory and non-blocking; the other two repos, whose
content trees differ.

## Risks & open questions

- Are the current thresholds (30 chars/line, 4 nodes) actually right for **educational** diagrams, or
  is the volume itself evidence they are mistuned for this content type? Answering this first could
  shrink the work substantially — or convert it into a convention change instead. (open)
- Bilingual parity: is there an existing checker that would catch an `en`/`id` diagram divergence
  introduced by a partial fix, or does that need adding first? (open)
- Should remediation be one plan or one-plan-per-tutorial-family, given 241 files? (open)

## What success looks like + promotion signal

Success: `md mermaid validate` runs with **no** `apps/ayokoding-www/content` exclude and reports zero
violations, in CI and in the lint-staged chain, with no diagram having lost a node or an edge.

Promotion signal: ready once the threshold question above is settled — if the thresholds stand, this is
a large but mechanical remediation plan; if they do not, it becomes a much smaller convention change
plus a re-baseline.
