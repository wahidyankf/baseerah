# Rhino speccoverage: multi-line `Scenario(...)` title scan

## Summary

`rhino`'s legacy `speccoverage` engine extracts vitest-cucumber scenario titles **per physical
line**: `extract_ts_scenario_titles` (`apps/rhino-cli/src/application/speccoverage/checker.rs`)
loops `for line in content.lines()` and applies `scenario_def_re()` —
`Scenario\s*\(\s*(?:"..."|'...')\s*,` — to each line in isolation. A `Scenario(` call whose title
string lands on a **different line** than the `Scenario(` token never matches, so the scenario is
reported as an uncovered "scenario gap" even though its binding exists. Make the extractor
multi-line-aware (scan across line boundaries) so title placement no longer affects coverage.

## Origin

Surfaced during `plans/done/2026-07-16__web-ui-code-block-copy-button` (the Rule-15 fix pass).
Prettier (`printWidth: 120`) wraps a long binding such as
`Scenario("A successful copy swaps to the success icon and announces via a live region", (cb) => {`
onto two lines — `Scenario(` on line N, the title string on line N+1. The per-line scanner then
could not see the title and `specs:behavior:coverage` failed with a spurious gap for a scenario
that was fully bound. The workaround applied in that plan was a `// prettier-ignore` above a
hand-collapsed single-line `Scenario("title",` call (safe only because no eslint `max-len` rule
exists). That workaround is fragile: it depends on every author remembering the annotation, and a
future prettier/eslint change could reintroduce the wrap.

## Proposed fix (to be scoped)

- Make `extract_ts_scenario_titles` scan the whole file content (not line-by-line) so
  `scenario_def_re()` matches across newlines — e.g. read `content` once and run
  `captures_iter(&content)` with a `(?s)`-tolerant pattern, or normalise whitespace before matching.
- Add regression fixtures for a `Scenario(` call whose title is on the next physical line (both
  double- and single-quoted), asserting the title is still extracted.
- Once the scanner is multi-line-safe, the `// prettier-ignore` single-line hacks in
  `libs/web-ui/src/primitives/code-block/*.steps.tsx` (and any similar sites) can be removed.

## Status

Backlog — not yet scoped into requirements/tech-docs/delivery detail. Filed per the
[Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md)'s
code-routing downstream rule (code-homed learnings are always filed as backlog, never landed inline
in the originating plan's PR).

## Related

- [Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md)
- `apps/rhino-cli/src/application/speccoverage/checker.rs` — `extract_ts_scenario_titles`,
  `scenario_def_re`
- `plans/done/2026-07-16__web-ui-code-block-copy-button/learnings.md` — full origin context
