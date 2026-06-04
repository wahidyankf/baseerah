# Backlog Plans

Planned projects for future implementation.

## Planned Projects

- [Repo-Rules-Checker `docs/` Coverage Extension](./2026-05-03__repo-rules-checker-docs-coverage/) — Extend `repo-rules-checker` with a new Step 8b "Cross-Documentation Rules Governance" covering the full `docs/` tree for the universal rules-governance dimension (file naming, frontmatter, no-date-metadata, traceability, broken cross-refs to `repo-governance/`). Headline new capability: vendor-binding drift detection between `docs/reference/platform-bindings.md` and the actual `.claude/` / `.opencode/` / root-level `CLAUDE.md` / `AGENTS.md` filesystem state.
- [Dependency Bump — June 2026 Sweep](./2026-06-04__dependency-bump-2026-06/) — Focused, pre-approved dependency bump across four tiers per the Dependency Bump Stability & Safety Policy: migrate `rhino-cli` off the unmaintained `serde_yml` crate (RUSTSEC-2025-0068), floor tokio ≥ 1.51.0, refresh Node LTS (24.16.0) and the backend Debian runtime base (trixie-slim), migrate the `crane-cli` test stack to xunit.v3 + coverlet 8, remove deprecated `@hey-api/client-fetch`, and confirm-then-bump GitHub Actions major tags. Snapshot as of 2026-06-04; re-verify eligibility before execution.

## Instructions

**Quick Idea Capture**: For 1-3 liner ideas not ready for formal planning, use `../ideas.md`.

When creating a new plan:

1. Create folder: `YYYY-MM-DD__[project-identifier]/`
2. Add standard files: README.md, brd.md, prd.md, tech-docs.md, delivery.md
3. Add the plan to this list
