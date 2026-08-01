# BeaverNest Rebrand

## Status

In Progress

## Context

This repository was scaffolded under the working name **Baseerah** (Arabic بصيرة — insight, inner
vision). The maintainer has decided to rename the product to **BeaverNest**, a plain product name
with no invented etymology. This plan renames every git-tracked reference across the repository —
identity docs, governance prose, agent fleet, specs, applications, CI, infrastructure, and
environment-variable prefixes — from the `baseerah`/`Baseerah`/`BASEERAH` vocabulary to the
`beaver-nest`/`BeaverNest`/`BEAVER_NEST` vocabulary, then hands off the GitHub repository rename and
local checkout folder rename to the maintainer as the final two human acts before archival.

This is a **mechanical identifier rename**, not a feature change: no new capability is added, no
existing behavior is removed, and (with one deliberate exception — see
[Non-Goals](./brd.md#business-scope-non-goals) and [Decision Log](./tech-docs.md#decision-log) Decision 3) no
observable behavior changes beyond the brand name and copy text a visitor reads.

## Scope

**In scope** (git-tracked content in this repository only):

- Root identity files: `README.md`, `CONTRIBUTING.md`, `ROADMAP.md`, `AGENTS.md`, `package.json`,
  `package-lock.json`, `.gitignore`
- `repo-governance/` (vision, conventions, development, workflows) — prose and illustrative paths
- `docs/` (reference, how-to, explanation, tutorials)
- `plans/backlog/`, `plans/ideas/`, `plans/in-progress/README.md` (active plan content only)
- `repo-config.yml` (harness registry entries, coverage projects, env-contract surfaces,
  env-injection apps)
- `specs/apps/baseerah/**` → `specs/apps/beaver-nest/**`
- `libs/web-ui-token/src/baseerah.css` → `beaver-nest.css` (OKLCH values unchanged, comments reworded)
- `apps/baseerah-be`, `apps/baseerah-be-e2e`, `apps/baseerah-fe`, `apps/baseerah-fe-e2e` → their
  `beaver-nest-*` equivalents, including F# namespaces, env var prefixes, Dockerfiles, and
  `baseerah.sln` → `beaver-nest.sln`
- `infra/dev/baseerah-app/` → `infra/dev/beaver-nest-app/`
- `.github/workflows/` filenames, job/output names, GHCR image name, deploy-branch name strings,
  environment name strings
- `.claude/agents/`, `.opencode/agents/`, `.cursor/agents/`, `.claude/skills/`,
  `.amazonq/cli-agents/baseerah-default.json`
- `apps/rhino-cli` source and tests where they hardcode repo-identity strings (the
  `.amazonq/cli-agents/baseerah-default.json` path constant and its embedded template, plus
  self-contained test fixtures using `baseerah`/`baseerah-be` as example data)
- `specs/apps/rhino/behavior/rhino-cli/gherkin/harness/agents-bindings.feature` — its step text is
  matched verbatim by a `cucumber` step-binding literal in `apps/rhino-cli/tests/agents.rs` and must
  rename in lockstep with it

**Out of scope**:

- `plans/done/2026-07-31__baseerah-repo-reset/**` — an immutable historical record of work done
  under the old name; never retroactively renamed
- Any literal citation of the `baseerah-repo-reset` plan-id/path elsewhere in the repo (e.g.
  `libs/README.md`, `apps/README.md`, an `apps/rhino-cli` code comment) — the path is real and
  unrenamed, so the citation string stays; only the surrounding brand prose around it changes
- `ose-public`, `ose-primer`, `ose-private` — cross-repo scope is explicitly deferred (see
  [brd.md Non-Goals](./brd.md#business-scope-non-goals))
- GHCR dual-publish or any compatibility shim for the old image name — hard cutover, no bridge
- Provisioning any new production/staging deploy target (none exists today; out of scope per the
  existing dormant-deployer posture)

## Business Rationale (Summary)

See [brd.md](./brd.md) for the full Business Requirements Document. In short: the maintainer wants
a permanent, deliberate product identity before the walking skeleton grows further, and the earlier
this rename happens the fewer files it touches. Waiting increases the cost of every future rename
linearly with repo growth.

## Product Requirements (Summary)

See [prd.md](./prd.md) for the full Product Requirements Document, including Gherkin acceptance
criteria for the one true behavior change (the landing-page brand copy and the removal of the
Arabic/Indonesian brand-chip feature, per [Decision Log](./tech-docs.md#decision-log) Decision 3).

## Technical Approach (Summary)

See [tech-docs.md](./tech-docs.md) for the full architecture and design-decision record, including
the nine pre-write grill decisions (Q1–Q9), the canonical substitution vocabulary, the ordered
substitution rules (most-specific first), and the complete file-impact inventory by directory.

## Delivery Checklist

See [delivery.md](./delivery.md) for the full phased checklist, `## Worktree` declaration, and
`## Delivery Mode: main-to-origin-main` declaration.

## Related Documentation

- [Baseerah Vision](../../../repo-governance/vision/beaver-nest.md) — the document this plan replaces
  with `beaver-nest.md`
- [Baseerah Repo Reset](../../done/2026-07-31__baseerah-repo-reset/README.md) — the plan that
  created the walking skeleton this rename now relabels
