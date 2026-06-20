# Backlog Plans

Planned projects for future implementation.

## Planned Projects

- [Repo-Rules-Checker `docs/` Coverage Extension](./2026-05-03__repo-rules-checker-docs-coverage/) — Extend `repo-rules-checker` with a new Step 8b "Cross-Documentation Rules Governance" covering the full `docs/` tree for the universal rules-governance dimension (file naming, frontmatter, no-date-metadata, traceability, broken cross-refs to `repo-governance/`). Headline new capability: vendor-binding drift detection between `docs/reference/platform-bindings.md` and the actual `.claude/` / `.opencode/` / root-level `CLAUDE.md` / `AGENTS.md` filesystem state.
- [Web Design Tester Agent](./2026-06-20__web-design-tester-agent/) — Add a new `web-design-tester` agent (scope `web`, role `tester`, `sonnet`, green) completing the live-site advocate triad alongside `web-exploratory-tester` (correctness) and `web-usability-tester` (usability). The design-team-advocate lens judges whether the LIVE rendered page matches the design (committed mockups, runtime tokens/theme, design-system primitives, optional external Figma/mockup source) and follows good design practice — the runtime counterpart to `swe-ui-checker`'s static source check, with no overlap. Files `DWT-###` findings as a backlog plan, locale- and evidence-aware. Makes the three testers reciprocally complement each other, renames the combined web workflow to `web-ux-test-fixing-planning`, and expands User-Facing Delivery Hardening Rule 15 into a three-tester near-end round for web-UI feature-change plans (consistent across `plan-maker`/`plan-checker`/`plan-execution`/`plan-execution-checker`). Lands topic-identically in all three sibling repos (direct on `main`, no worktrees), with a `repo-rules-maker` consistency sweep per repo.

## Instructions

**Quick Idea Capture**: For 1-3 liner ideas not ready for formal planning, use `../ideas.md`.

When creating a new plan:

1. Create folder: `YYYY-MM-DD__[project-identifier]/`
2. Add standard files: README.md, brd.md, prd.md, tech-docs.md, delivery.md
3. Add the plan to this list
