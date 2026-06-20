# Design Funnel — Empty-State Alternatives

The design-funnel divergence for the two net-new empty-state screens. One lo-fi alternative advances to
hi-fi per screen (chosen below); the hi-fi `.excalidraw.png` finalist is produced during execution before the
code lands. Grounding: `libs/web-ui` (Card, text), the ayokoding-www shell/theme, and the sibling table
mockups in the done plan. Token-only colors.

## Savings empty state

| Alt                                              | Sketch                                                                          | Pros                                                                            | Cons                                                                |
| ------------------------------------------------ | ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| **A — Centered prompt + muted glyph** (advances) | Input row on top; centered icon + one-line instruction where the table would be | Clear, calm, matches the "calm empty state" pattern; reuses Card; no false data | Slightly more vertical space                                        |
| B — Disabled/greyed table                        | The real table rendered at low opacity with an overlay prompt                   | Shows what's coming                                                             | Greyed red negatives can still read as "bad"; defeats the trust fix |
| C — Inline helper under the input only           | Just a sentence under the salary field, no table region treatment               | Minimal                                                                         | Easy to miss; the empty table region stays ambiguous                |

**Chosen: A.** It most directly removes the "wall of red" misread (UWT-003) and reuses the Card primitive.

## Minimum-role empty state

| Alt                                                            | Sketch                                                                                                           | Pros                                                                                        | Cons                                   |
| -------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | -------------------------------------- |
| **A — Segmented control + input + centered prompt** (advances) | Baseline segmented control and target input stay visible; centered ladder glyph + instruction replaces the table | Consistent with the Savings empty state; teaches the baseline control before results appear | More vertical space                    |
| B — Pre-filled example row                                     | Show one example role as a teaser                                                                                | Demonstrates output                                                                         | Risks being mistaken for a real result |
| C — Bare sentence                                              | One line, no region treatment                                                                                    | Minimal                                                                                     | Same ambiguity as Savings-C            |

**Chosen: A**, for cross-tab consistency with the Savings empty state.

## Prior Art (R7)

_Judgment call_ — Prior-art survey delegated to `web-researcher` before hi-fi authoring (step 7.0).
The following patterns ground the divergent alternatives and the Option A selection:

- **Centered instructional prompt (calm empty state)** — The dominant pattern for filter-dependent
  tools (data dashboards, comparison tables): a centered icon or illustration + one headline + one
  supporting sentence, no false data rendered. Sources: UXPin Empty State Best Practices
  (uxpin.com/studio/blog/ux-best-practices-designing-the-overlooked-empty-states/); Pencil & Paper
  empty state survey (pencilandpaper.io/articles/empty-states); Mobbin empty-state glossary
  (mobbin.com/glossary/empty-state). Access date: 2026-06-20.
  _Key finding_: "Two parts instruction, one part delight" — a message (mandatory) with a headline
  and supporting description is the minimal viable empty state for a first-run / input-required
  context. This grounds Option A (centered prompt).
- **Greyed/disabled preview (teaser pattern)** — Used when demonstrating output value upfront
  outweighs the risk of misread. Rejected for this context: greyed red-negative cells still read
  as "bad results" and defeat the trust fix (UWT-003 root cause). Option B draws from this prior
  art and is documented in the alternatives table above.
- **Inline helper only (minimal pattern)** — Bare instructional sentence under the input, no table
  region treatment. Lowest friction but leaves the empty table region ambiguous; documented as
  Option C above. Grounding: SAP Fiori empty-state global patterns
  (sap.com/design-system/fiori-design-web/v1-96/foundations/best-practices/global-patterns/designing-for-empty-states).
  Access date: 2026-06-20.

_Selection rationale_: Option A (centered prompt + Card) is the consensus best-practice for an
input-required context where showing pre-computed results would mislead the user. It reuses the
`libs/web-ui` Card primitive (R5 grounding) and does not require new design-system components.

## Hi-fi production note

**F11 Residual**: the hi-fi `.excalidraw.png` finalists are the one artefact deferred to the human
execution step (delivery.md step 7.0). They cannot be auto-generated. The Phase 7 gate in
`delivery.md` explicitly blocks code until both PNGs are committed. See also `assets/README.md`.

During execution, produce `assets/ui-empty-states-savings-option-a.excalidraw.png` and
`assets/ui-empty-states-min-role-option-a.excalidraw.png` (mobile + desktop frames each), referenced
from `delivery.md`, before the empty-state branch is implemented. Colors: `bg-muted`,
`text-muted-foreground`, `text-foreground` — no raw hex.
