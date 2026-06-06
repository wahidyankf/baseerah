# Business Requirements Document — Mermaid Gate Coverage Expansion

## Business Goal

Make the Mermaid diagram quality gate **trustworthy across the entire repository** so that no
markdown file with a malformed or unrenderable flowchart can reach `main` undetected, whether
committed locally or through CI.

## Business Rationale (WHY)

The repository invests heavily in accessible, render-correct diagrams — the
[Diagrams Convention](../../../repo-governance/conventions/formatting/diagrams.md) mandates
color-blind-friendly palettes, width limits, and ≤30-character node labels so diagrams render
without clipping. [Repo-grounded] A gate that only inspects two of seven markdown trees gives a
**false sense of safety**: contributors and agents trust the green pre-push, yet diagrams in
`plans/`, `docs/`, `apps/`, `libs/`, and root files are never checked.

The concrete failure that motivated this plan: a flowchart with 7 over-long labels landed in
`plans/in-progress/gherkin-step-keyword-cardinality/README.md` [Repo-grounded] and passed every
gate. Over-long labels clip on render, producing diagrams that silently mislead readers — a
correctness defect, not a style nit. [Judgment call: render-clipping severity]

A second, accessibility-grade gap motivates the color requirement folded into this plan. The
convention mandates a canonical color-blind-friendly palette meeting WCAG AA contrast in both
light and dark modes [Repo-grounded — `diagrams.md` Accessible Color Palette, lines 574-584], yet
the validator never checks color. Off-palette hex (e.g. `#0072B2`/`#D55E00`/`#009E73` substituted
for the palette's `#0173B2`/`#DE8F05`/`#029E73`) defeats the very accessibility guarantee the
convention exists to enforce: roughly 8% of male readers and 0.5% of female readers have some form
of color blindness [Repo-grounded — `diagrams.md` line 562], and an unenforced palette means those
readers can silently lose information a diagram encodes by color. Making the palette **machine-
enforced and blocking** turns the convention from aspiration into guarantee. Mermaid's documented
styling mechanisms determine which diagram types can carry an off-palette hex in source at all
[Web-cited — mermaid.js.org/syntax/\*, accessed 2026-06-06: "classDef className
fill:#f9f,stroke:#333,stroke-width:4px;" — flowchart/graph, classDiagram, stateDiagram/v2,
requirementDiagram, and quadrantChart support per-element hex via `classDef`/`style fill:#hex`;
sequenceDiagram/gantt/pie/gitGraph/journey/timeline/mindmap use theme-level color only]; the
gate enforces the palette wherever such a hex can appear.

## Business Impact

### Pain points addressed

- **Silent diagram defects** — malformed flowcharts in unscanned trees ship to `main` and to the
  public sites (`docs/` content feeds the Next.js docs surfaces). [Judgment call]
- **Skippable gate** — the gate runs only in local pre-push, bypassable with
  `git push --no-verify`. [Repo-grounded — no CI workflow runs `validate:mermaid`]
- **Documentation inaccuracy** — `diagrams.md` claims `docs/` coverage that does not exist,
  eroding trust in governance docs. [Repo-grounded]
- **No escape hatch for legitimately complex diagrams** — once the complexity warnings become
  blocking, authors need a documented, reviewable way to exempt an inherently complex diagram
  rather than being forced to over-simplify it. The new directive supplies that.
- **Unenforced accessibility palette** — diagrams using off-palette colors defeat the WCAG AA
  color-blind-friendly guarantee the convention promises, and nothing today catches them.
  [Repo-grounded — no color check exists in `mermaid.rs`]
- **Six convention rules documented but unenforced** — `diagrams.md` mandates no literal `\n` in
  labels, no `"` inside `[...]` node labels, edge labels ≤20 chars, `flowchart LR` by default,
  exactly one palette comment per diagram, and matched separator widths, yet the validator checks
  none of these. [Repo-grounded — diagrams.md lines 402/681/1144/1147/1219/1644] A literal `\n`
  renders as garbage text and a `"` inside `[...]` breaks the Mermaid parser entirely — both are
  always-bugs, not style nits. [Judgment call: render/parse-break severity]
- **Inconsistent default scan dirs** — the command's `collect_md_default_dirs()` omits `apps/` and
  `libs/`, and the Nx target overrides it with an even narrower path list, so the two surfaces
  disagree about what "scan everything" means. [Repo-grounded —
  `apps/rhino-cli/src/commands/docs_validate_mermaid.rs:186` + `project.json:170`]

### Expected benefits

- **Full-repo confidence** — a green gate means every flowchart in every markdown tree is valid.
- **Unskippable enforcement** — the CI backstop guarantees the gate runs even when local hooks
  are bypassed.
- **Auditable exemptions** — every complexity exemption carries a mandatory written rationale, so
  exemptions are documented rather than silent.
- **Enforced accessibility palette** — every diagram across the five eligible types provably uses
  only color-blind-friendly WCAG AA palette colors; off-palette hex blocks and can never be
  exempted, so the accessibility guarantee cannot be silently bypassed.
- **Six more convention rules now machine-enforced** — `literal_backslash_n`,
  `quotes_in_brackets`, `edge_label_too_long`, `non_lr_direction`, `palette_comment_count`, and
  `separator_width_mismatch` move from documented-but-ignored to gated, turning render/parse
  correctness and accessibility-layout rules into guarantees rather than aspirations.
- **Consistent scan surface** — the default-dir list and the Nx target agree and both cover the
  full repo, so the gate behaves identically whether invoked bare or through the target.
- **Accurate governance docs** — `diagrams.md` reflects reality (coverage, directive syntax,
  per-type validation scope, warnings-now-blocking, the blocking non-exemptable color check, and
  the six new checks with their blocking/exemptable status).

## Affected Roles

This is a solo-maintainer repository; the maintainer wears several hats, and several agents
consume the affected surfaces. No sign-off ceremonies apply.

- **Maintainer (tooling hat)** — owns `rhino-cli`, the Nx target, the pre-push hook, the CI
  workflow.
- **Maintainer (governance hat)** — owns `diagrams.md` and the convention surface.
- **Maintainer (content hat)** — authors diagrams in `docs/`, `plans/`, app READMEs.
- **Consuming agents** — `swe-rust-dev` (validator/CI/hook), `repo-rules-maker` / `docs-maker`
  (convention doc), `repo-setup-manager` (Phase 0 baseline), and any agent that authors markdown
  containing flowcharts (now gated repo-wide). [Repo-grounded — agents confirmed present]

## Business-Level Success Metrics

- **Coverage completeness** (observable): the expanded `validate:mermaid` scans
  `repo-governance/`, `.claude/`, `plans/`, `docs/`, `apps/`, `libs/`, and root instruction files
  — verifiable by running the target and inspecting `files_scanned`.
- **Unskippability** (observable): a CI job runs `validate:mermaid` on every relevant push/PR —
  verifiable by inspecting `.github/workflows/`.
- **Zero blocking findings** (observable): the full-repo gate reports zero blocking findings at
  plan completion — verifiable by running the target.
- **Rationale enforcement** (observable): an `allow-*` directive without an adjacent `reason:`
  line is itself a hard error — verifiable by a unit test. [Judgment call: this is the
  documentation-integrity guarantee the maintainer values most]
- **Palette enforcement** (observable): a diagram of any of the five eligible types containing an
  off-palette `fill:`/`stroke:`/`color:` hex produces a blocking `off_palette_color` finding, and
  no directive can suppress it — verifiable by unit tests and by a clean full-repo run at
  completion.
- **Six new checks enforced** (observable): a flowchart containing a literal `\n`, a `"` inside a
  `[...]` label, an edge label >20 chars, a non-`LR` direction, zero-or-duplicate palette comments,
  or a mismatched separator width produces the corresponding blocking finding — verifiable by unit
  tests; the never-exemptable ones (`literal_backslash_n`, `quotes_in_brackets`,
  `palette_comment_count`) reject their `allow-*` directive as a hard error.
- **Default-dir consistency** (observable): `collect_md_default_dirs()` includes `apps` and `libs`,
  and the Nx target's positional paths match the default-dir intent — verifiable by a unit test and
  by inspecting `project.json`.

## Business-Scope Non-Goals

- Not building a Mermaid renderer or visual-diff tool.
- Not extending **structural** checks to non-flowchart diagram types (deferred); only the **color**
  check extends to the four additional eligible types.
- Not validating color on `erDiagram` or C4 (PARTIAL — deferred to a future plan), nor on
  theme-only types (sequenceDiagram, gantt, pie, gitGraph, journey, timeline, mindmap — no source
  hex to check).
- Not moving the gate into pre-commit (kept in pre-push + CI).
- Not **redesigning** the palette in `diagrams.md` — the canonical palette is the source of truth
  and is enforced as-is; this plan only corrects coverage and documents the new enforcement.

## Business Risks and Mitigations

| Risk                                                                        | Likelihood | Mitigation                                                                                                                 |
| --------------------------------------------------------------------------- | ---------- | -------------------------------------------------------------------------------------------------------------------------- |
| Widened scan surfaces a large fix-all backlog (`apps/`+`libs/` ≈ 529 lines) | High       | Phase the cleanup one gate per tree; re-measure per tree at execution; use exemptions where genuine                        |
| Promoting warnings to blocking breaks existing green diagrams               | Medium     | Re-measure including warnings at Phase 0; cleanup phases fix or exempt each finding before its gate                        |
| Widened scan starts erroring on non-flowchart diagrams in `apps/`/`docs/`   | Medium     | Preserve + explicitly test the structural flowchart-only invariant; non-flowchart eligible types get color-only checks     |
| Color check produces a large off-palette fix-all backlog                    | Medium     | Re-measure off-palette findings per tree at execution; fold color fixes into each per-tree gate                            |
| `non_lr_direction` flags MANY existing `flowchart TD/TB` diagrams           | High       | Expected the largest single fix-all contributor; convert to LR or add justified `allow-direction`; re-measure per tree     |
| New `\n`/quotes checks miss non-flowchart bracket-label diagram types       | Low        | Flowchart-only by design (consistent with the structural invariant); extending to other bracket-label types is future work |
| Color allowlist drifts from `diagrams.md`                                   | Low        | Extract the allowed hex set FROM `diagrams.md` (source of truth), not from memory; document the link                       |
| Exemptions become a silent bypass                                           | Low        | `allow-*` without `reason:` is a hard error; only structure kinds are exemptable; labels AND colors never are              |
| CI job slows the pipeline                                                   | Low        | The target is cacheable; the job mirrors existing lightweight rhino-cli validator jobs                                     |

See [prd.md](./prd.md) for the testable scenarios that verify each of these mitigations.
