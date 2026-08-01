# Product Requirements Document — BeaverNest Rebrand

## Product Overview

This plan renames the product identity of the repository from **Baseerah** to **BeaverNest**
throughout every git-tracked surface. The functional product (a stateless F#/Giraffe backend and a
Next.js frontend proving the engineering harness end-to-end) does not change — its name, its
visible copy, and every identifier derived from its name do.

## Personas

- **The maintainer** (wahidyankf) — reads the renamed docs, executes the GitHub repo rename and the
  local folder rename, and is the sole consumer of the business rationale in `brd.md`.
- **A coding agent operating in this repo** (Claude Code / OpenCode / any bound harness) — resolves
  agent names, skill names, and file paths that must all point at real, renamed files after this
  plan merges.
- **A first-time visitor to the `beaver-nest-fe` landing page** — reads the rendered brand name and
  one-line description; this persona's experience is the one place this plan has genuine, testable
  observable behavior.

## User Stories

**US1** — As the maintainer, I want every prose and identifier reference to "Baseerah" replaced with
"BeaverNest" so that the repository presents one consistent identity.

**US2** — As a first-time visitor to the landing page, I want to see "BeaverNest" as the product
name and a plain-language description, so that the page still tells me what the product is without
referencing a name or etymology that no longer applies.

**US3** — As a coding agent, I want the agent fleet, skills, and `.amazonq` binding to reference
real files under their new names, so that agent selection and generated bindings do not silently
break after the rename.

**US4** — As `rhino-cli` (the repo's own tooling), I want the `.amazonq/cli-agents/beaver-nest-default.json`
path constant and its embedded template to match the renamed file, so that `harness bindings
generate` keeps producing a valid, consistent binding.

**US5** — As the maintainer, I want the GitHub repository and container image to adopt the new name
so that the public-facing identity (repo URL, GHCR image tag) matches the internal one.

## Acceptance Criteria

### AC1 — Repo-wide rename completeness

```gherkin
Scenario: A repo-wide case-insensitive search finds no unexpected "baseerah" residue
  Given the Repo-Wide Residual Sweep phase has completed all prior content phases
  When a case-insensitive search runs for "baseerah" across all git-tracked files
  Then the only matches are inside plans/done/2026-07-31__baseerah-repo-reset/
  And any match outside that folder is a file the historical-citation preservation rule recognizes as legitimately citing baseerah-repo-reset
```

### AC2 — Landing page shows the new brand name

```gherkin
Scenario: The landing page names the product and shows the backend greeting
  Given the beaver-nest-fe app is running on port 19310 against a live beaver-nest-be
  When I navigate to "/"
  Then the page shows a level-one heading containing "BeaverNest"
  And the page shows the text "Hello from BeaverNest" sourced from the backend
```

### AC3 — Homepage description no longer references the old etymology

```gherkin
Scenario: The homepage tells a first-time visitor what BeaverNest is, with no invented meaning
  Given a first-time visitor with no prior context navigates to "/"
  When the page finishes loading
  Then a one-line description of what BeaverNest does is visible without scrolling
  And the description contains no Arabic or Indonesian etymology gloss
```

### AC4 — The multilingual brand chip is removed, not relabeled

```gherkin
Scenario: The homepage no longer renders a brand-chip etymology gloss
  Given a first-time visitor viewing the rendered homepage
  When they inspect the page for a hoverable multilingual term chip
  Then no بصيرة/wawasan-style etymology chip is present
  And no automated test or Gherkin scenario asserts one exists
```

### AC5 — 404 page shows the new branding

```gherkin
Scenario: A visitor to a non-existent path can recover
  Given a visitor navigates to a non-existent path on beaver-nest-fe
  When the 404 page renders
  Then it shows BeaverNest branding
  And it offers a link back to the homepage
```

### AC6 — Agent fleet resolves under the new names

```gherkin
Scenario: The renamed agent fleet is internally consistent
  Given the .claude/agents/ directory after the rename phases complete
  When the .claude/agents/README.md catalog is checked against the directory listing
  Then every catalog entry under the apps-beaver-nest-* prefix names a file that exists
  And no catalog entry still names an apps-baseerah-* file
```

### AC7 — The `.amazonq` binding constant matches the renamed file

```gherkin
Scenario: rhino-cli's Amazon Q binding constant points at the renamed file
  Given apps/rhino-cli's AMAZONQ_AGENT_DEFINITION constant after the rhino-cli rename phase
  When nx run rhino-cli:test:integration runs
  Then the test asserting the constant's path value passes against ".amazonq/cli-agents/beaver-nest-default.json"
  And the generated file's "name" field reads "beaver-nest-default"
```

### AC8 — CI publishes the renamed image, not the old one

```gherkin
Scenario: A push to main that affects beaver-nest-be publishes the renamed GHCR image
  Given a commit lands on main touching apps/beaver-nest-be
  When the publish-images workflow runs
  Then it builds and pushes ghcr.io/wahidyankf/beaver-nest-be:latest and :<sha>
  And it does not build or push any ghcr.io/wahidyankf/baseerah-be tag
```

## Product Scope

**In scope**: every user-facing and machine-facing rename enumerated in [README.md
§Scope](./README.md#scope) — identity docs, agent fleet, specs, applications (F#, Next.js,
Playwright E2E), CI workflows, infra compose files, the `rhino-cli` functional couplings, and the
GHCR image name.

**Out of scope**: cross-repo propagation (Q8), any new deploy-target provisioning, any GHCR
dual-publish bridge (Q9), the GitHub repo rename and local folder rename content itself (both are
`[HUMAN]` acts sequenced at the end of `delivery.md`, not product requirements this PRD specifies
behavior for).

## Product-Level Risks

- **Risk**: removing the multilingual brand chip (AC4) is a bigger change than a pure text
  substitution — it deletes a Gherkin scenario, a unit test, an E2E step, and rendered JSX, not just
  renames a string. **Mitigation**: this is flagged explicitly in the post-write grill (see the
  plan's grilling record) so the maintainer confirms the interpretation before execution, rather than
  the plan silently assuming it.
- **Risk**: a first-time visitor to the renamed landing page could be confused if the removed
  etymology chip leaves an obvious visual gap. **Mitigation**: Phase 10's delivery steps replace the
  chip's layout slot with a plain one-line description (already required by AC3), not a blank gap —
  verified by the manual Playwright UI verification section.
