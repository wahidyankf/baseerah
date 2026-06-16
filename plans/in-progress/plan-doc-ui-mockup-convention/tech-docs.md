# Tech Docs — Plan-Doc UI Mockup Convention

This document holds the full research that drives the convention: the rendering behaviour of each
candidate format across VSCode and GitHub, the comparison matrix, the ruled-out options with reasons,
copy-paste examples, and citations.

## Key Facts (resolved)

### GitHub strips inline CSS

GitHub's Markdown HTML sanitizer removes the `style=`, `class`, and `id` attributes and the
`<style>` and `<script>` elements entirely. It is an allowlist, not a partial filter — only legacy
presentation attributes survive: `align`, `border`, `cellpadding`, `cellspacing`, `color`, `height`,
`width`, `valign`, `colspan`, `rowspan`, plus `href`/`src`/`alt`/`title`.

Consequence: a `<div style="background:#f0f0f0;border-radius:8px;padding:12px">` card mockup renders
in VSCode but becomes a bare, unstyled `<div>` on GitHub. **Inline-CSS mockups are not viable for
GitHub.**

Allowed elements include `table`, `thead`, `tbody`, `tr`, `td`, `th`, `details`, `summary`, `img`,
`kbd`, `sub`, `sup`, `hr`, `blockquote` — useful for structure, but none carry layout styling.

### VSCode built-in preview is permissive

VSCode's built-in Markdown preview uses **markdown-it** with raw-HTML passthrough enabled. Its
webview CSP blocks `<script>` execution and external HTTP resources, but does **not** strip
`style=`. So inline HTML+CSS renders fully in VSCode. This is the asymmetry that makes inline-HTML
mockups misleading: they look right locally and break on GitHub.

### Mermaid has no wireframe type

Mermaid renders natively on both GitHub and VSCode, but it has **no UI/wireframe diagram type**
(requested 2020 in mermaid-js/mermaid#1184, still "contributor needed"). Repurposing flowchart nodes
produces a flow diagram, not a UI. The repo's own mermaid validator
(`rhino-cli md validate mermaid`) further caps node width and label length, making any UI layout
impossible. **Not viable for wireframes.**

### Excalidraw: use `.excalidraw.png`, not `.excalidraw.svg`, for GitHub

`.excalidraw.svg` and `.excalidraw.png` are real images carrying the Excalidraw scene JSON embedded
in metadata — both re-open as an editable canvas in the Excalidraw VSCode extension or on
excalidraw.com. Both render on GitHub via `![](./file)`. **But** Excalidraw's custom hand-drawn
fonts (Virgil, Cascadia) load from a CDN that GitHub's CSP blocks for SVG, so `.excalidraw.svg` text
labels fall back to a generic font on GitHub (excalidraw/excalidraw#4855). `.excalidraw.png`
rasterises the fonts and renders faithfully → **use PNG for any GitHub-visible mockup.**

Inline `<svg>` pasted directly into Markdown does **not** render on GitHub (sanitizer strips it) —
SVG only renders when referenced as a separate file via `![](path)` or `<img src="path">`.

## Comparison Matrix

| Approach                         | VSCode built-in | VSCode + extension      | GitHub.com              | Diffable      | Lint-safe       |
| -------------------------------- | --------------- | ----------------------- | ----------------------- | ------------- | --------------- |
| **ASCII wireframe (code block)** | Renders         | —                       | Renders                 | Excellent     | Yes             |
| **`.excalidraw.png` + `![]()`**  | Renders (image) | Edit: pomdtr Excalidraw | Renders                 | No (binary)   | Yes             |
| **Plain `.png` screenshot**      | Renders         | —                       | Renders                 | No (binary)   | Yes             |
| `.excalidraw.svg` + `![]()`      | Renders (image) | Edit: pomdtr Excalidraw | Renders (font fallback) | Partial (XML) | Yes             |
| Inline HTML + CSS                | Renders fully   | —                       | **Style stripped**      | Yes           | Yes (MD033 off) |
| Mermaid                          | Renders         | —                       | Renders                 | Yes           | Yes             |
| PlantUML Salt                    | No (built-in)   | jebbs PlantUML          | **No**                  | Yes           | Yes             |
| MDX (`.mdx`)                     | No              | —                       | **No**                  | Yes           | n/a             |
| Inline `<svg>` in `.md`          | Renders         | —                       | **Stripped**            | Yes           | Yes (MD033 off) |

Repo note: markdownlint MD033 (inline HTML) is **disabled** in this repo
(`.markdownlint-cli2.jsonc`), and Prettier uses `proseWrap: preserve`, so inline HTML is not a lint
problem — it is purely a GitHub-rendering problem.

## Worked example assets

The full funnel is demonstrated for one screen (Salary Savings Calculator, compare-all) under
[`assets/`](./assets/README.md):

- Stage 1 diverge (low-fi): [`example-low-fi-wireframe.md`](./assets/example-low-fi-wireframe.md) —
  three named alternatives (Option A / B / C).
- Stage 2 narrow (hi-fi finalists):
  [`example-hi-fi-option-a-ranked-table.png`](./assets/example-hi-fi-option-a-ranked-table.png) and
  [`example-hi-fi-option-c-split.png`](./assets/example-hi-fi-option-c-split.png) (each rasterised
  from a diffable `.svg`).
- Stages 3–4 select + justify: the named selection (Option A) and the rationale table live in
  [`assets/README.md`](./assets/README.md).

## Ground mockups in the existing design system (before drawing)

A mockup invented from scratch drifts from what the app can actually render and creates rework. So
**before** drafting either tier, survey the existing UI of the related app(s) and lib(s) and build
the mockup from what is already there:

- **Shared kit — `libs/web-ui`**: the canonical component inventory (shadcn/ui + Radix + Tailwind),
  its design tokens, and its Storybook. Reuse these components (tabs, inputs, toggles, radio groups,
  combobox, badges, alerts, cards, table) and token-driven spacing/color instead of inventing visual
  language.
- **Target app**: the app's existing pages, layout shell, theme, and locale/i18n structure (e.g.
  `apps/ayokoding-www` for the salary-savings plan) — so the new screen matches the surrounding site.
- **Sibling screens**: any existing tool/page the new screen should visually rhyme with.
- **Skill reference**: `swe-developing-frontend-ui` documents token usage, component patterns, and
  the brand context to honour.

Output of the survey: the mockup reuses real components and tokens, and any **net-new** component is
named explicitly (the salary-savings plan does this for the `Table` primitive it adds to
`libs/web-ui`) so the build gap is visible up front. The hi-fi example under [`assets/`](./assets/README.md)
deliberately uses the web-ui palette (teal primary, slate neutrals) to model this.

## Design funnel (diverge → narrow → select → justify)

The two tiers below are the **artefacts**; the funnel is the **process** that uses them. Low-fi is
cheap, so divergence happens there; hi-fi is more expensive, so only the shortlist gets that
treatment. The funnel keeps the design space wide early and the commitment explicit late.

| Stage        | Fidelity | Count             | What lands in the plan                                              |
| ------------ | -------- | ----------------- | ------------------------------------------------------------------- |
| 0. Prior art | —        | cited survey      | `web-research-maker` findings: how comparable tools solve this (R7) |
| 1. Diverge   | Low-fi   | ≥ 2 (aim 3)       | Named ASCII alternatives (Option A / B / C), genuinely different    |
| 2. Narrow    | Hi-fi    | 2 finalists       | `.excalidraw.png` mockups of the strongest; one-line drop reasons   |
| 3. Select    | —        | 1+ (named)        | The chosen design, **named** ("Selected: Option B — Ranked Table")  |
| 4. Justify   | —        | 1 decision record | Rationale: why the winner won, why each runner-up lost              |

Stage 0 pairs with the internal grounding rule (R5): **R5 surveys what the repo already has**
(`libs/web-ui`, the target app); **R7 surveys prior art in the wild** via `web-research-maker` so the
alternatives are informed by how comparable products solve the same problem rather than invented from
a blank page. Both feed the divergent alternatives and the rationale.

Why this shape: cheap, diffable ASCII makes it painless to float three real layout ideas before
anyone invests in pixels; promoting only two to hi-fi forces an early cut; naming the selection makes
the downstream build unambiguous; and the rationale preserves _why_ so a later reader (or reviewer)
does not relitigate a settled trade-off. The worked example under [`assets/`](./assets/README.md)
walks the full funnel for the compare-all screen.

### Enforcement — who checks the funnel

The funnel is enforced by the existing plan maker → checker → fixer chain, mirroring the repo's
**Specs & Gherkin completeness (both paths)** binding:

| Surface                       | Responsibility                                                               |
| ----------------------------- | ---------------------------------------------------------------------------- |
| `plan-creating-project-plans` | Documents the rule; grilling gates ask the design-funnel questions           |
| `plan-maker`                  | Requires funnel artefacts on UI-bearing plans; emits delivery steps for them |
| `plan-checker` (new step)     | FLAGS (HIGH) any missing funnel artefact on a UI-bearing plan; exempts no-UI |
| `plan-fixer`                  | Scaffolds the missing funnel sections for the author to fill                 |
| `plan-quality-gate` workflow  | Lists the new checker step in its validation scope; gate fails if skipped    |

"UI-bearing" = the plan adds/changes user-facing screens or components under `apps/` or `libs/`. Pure
refactors and non-UI plans are exempt, exactly as with the specs/Gherkin binding.

## The two required tiers

A UI-bearing plan documents each screen at **both** fidelities, in separate labelled subsections.
They are complementary, not alternatives: the low-fi tier is the diffable structural source of truth;
the hi-fi tier shows what it actually looks like. Plain `.png` is the hi-fi fallback once the design
is final and no longer iterating. Within the funnel, low-fi hosts the divergent alternatives and
hi-fi hosts the shortlist plus the named selection.

### Tier 1 (low-fi, required) — ASCII / Unicode wireframe in a fenced code block

Zero dependencies, renders identically in GitHub, VSCode, and terminals, perfectly diffable, stays
inline in the `.md`, and matches the repo's existing ASCII-tree convention. Captures layout, control
placement, and flow — the thing reviewers comment on line-by-line. Generators:
[BareMinimum](https://bareminimum.design/), [Mockdown](https://www.mockdown.design/).

Example (paste straight into a plan `.md`):

````markdown
### Low-Fidelity Wireframe — Compare-All Mode

```
┌──────────────────────────────────────────────────────┐
│  Salary Savings Calculator                           │
├──────────────────────────────────────────────────────┤
│  [ Compare All ]  ( Single City )    ← tab toggle    │
├──────────────────────────────────────────────────────┤
│  Salary (USD/mo): [________________]                 │
│  Household:       [ Single        ▼]                 │
│  Area:            ( ) Center   (•) Rural             │
├──────────────────────────────────────────────────────┤
│  City            Savings/mo    % of Salary           │
│  ──────────────  ───────────   ───────────           │
│  Singapore       $1,200        30%                   │
│  Jakarta         $2,100        52%                   │
│  Kuala Lumpur    $1,800        45%                   │
└──────────────────────────────────────────────────────┘
```
````

### Tier 2 (hi-fi, required) — Excalidraw `.excalidraw.png` referenced via `![]()`

Real spacing, grouping, color, typography, and visual hierarchy, while staying editable (embedded
scene). Lives beside the plan, e.g. `plans/in-progress/<name>/ui-compare-all.excalidraw.png`. View
needs no extension; edit needs `pomdtr.excalidraw-editor`. Cost: binary diff (acceptable — the
diffable structural record lives in the Tier-1 wireframe).

```markdown
### High-Fidelity Mockup — Compare-All Mode

![Compare-All mode — high-fidelity mockup](./ui-compare-all.excalidraw.png)

_High-fidelity mockup. Edit with the Excalidraw VSCode extension — the PNG carries the scene._
```

Hi-fi fallback — plain `.png` screenshot: zero tooling, renders everywhere, but binary and
replace-on-every-change. Use as the Tier-2 artifact only when the design is final and no longer
iterating.

## Ruled Out (with reason)

| Option                  | Why not (for plan docs)                                                           |
| ----------------------- | --------------------------------------------------------------------------------- |
| Inline HTML + CSS       | GitHub strips `style=`/`class`/`id` → renders unstyled on GitHub; VSCode-only.    |
| MDX (`.mdx`)            | Needs a build/runtime; renders on neither GitHub nor VSCode preview as plan docs. |
| Mermaid as wireframe    | No wireframe diagram type; repo validator caps layout. Flowchart ≠ UI.            |
| `.excalidraw.svg`       | Excalidraw fonts blocked by GitHub CSP → text falls back to generic font.         |
| PlantUML Salt           | Great wireframe syntax, but renders on neither GitHub nor VSCode built-in.        |
| Inline `<svg>` in `.md` | Sanitizer strips inline SVG on GitHub; only file-referenced SVG renders.          |

## Decision: where the convention lives

Default: **extend the existing `repo-governance/conventions/formatting/diagrams.md`** with a new
"UI Mockups in Plan Docs" section rather than creating a separate convention file, to avoid
convention sprawl. The diagrams convention already governs Mermaid and ASCII art, so UI wireframes
are a natural third category there. (Revisit only if the section grows large enough to warrant its
own file.)

## Citations

- [rhysd/marked-sanitizer-github — sanitizer allowlist](https://github.com/rhysd/marked-sanitizer-github)
- [GitHub Community Discussion #22728 — inline CSS stripped](https://github.com/orgs/community/discussions/22728)
- [HTML tags usable on GitHub (seanh gist)](https://gist.github.com/seanh/13a93686bf4c2cb16e658b3cf96807f2)
- [alexwlchan — how SVGs render on GitHub (2024)](https://alexwlchan.net/notes/2024/how-to-render-svgs-on-github/)
- [Excalidraw VSCode extension (pomdtr)](https://marketplace.visualstudio.com/items?itemName=pomdtr.excalidraw-editor)
- [excalidraw/excalidraw#4855 — fonts blocked on GitHub SVG](https://github.com/excalidraw/excalidraw/issues/4855)
- [mermaid-js/mermaid#1184 — wireframe request](https://github.com/mermaid-js/mermaid/issues/1184)
- [PlantUML Salt](https://plantuml.com/salt)
- [VS Code Markdown documentation](https://code.visualstudio.com/docs/languages/markdown)
- [markdownlint MD033](https://github.com/DavidAnson/markdownlint/blob/main/doc/md033.md)
- [BareMinimum — ASCII wireframe generator](https://bareminimum.design/)
- [Mockdown — ASCII wireframe editor](https://www.mockdown.design/)
