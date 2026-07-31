# BRD — Baseerah Repo Reset

Business Requirements Document for the [Baseerah Repo Reset](./README.md) plan.

## Business Goal

Convert a clone of `ose-public` into a purpose-built home for **Baseerah**, a personal-assistant
product, while keeping the engineering harness that made `ose-public` productive — the agent fleet,
the six-layer governance hierarchy, the CI gates, the plan lifecycle, and `rhino-cli`.

The goal is explicitly **not** "start a fresh repo." A fresh repo would mean rebuilding ~200
governance files, ~59 generic agents, 27 skills, and a working polyglot CI harness from scratch. The
goal is to keep that harness and swap the product beneath it.

## Business Rationale

### The problem, with data points

The repo currently contains a product it will never ship:

| Surface                      | Count belonging to the old product                                                              |
| ---------------------------- | ----------------------------------------------------------------------------------------------- |
| Nx apps                      | 22 of 23 (only `rhino-cli` is wanted)                                                           |
| `.github/workflows/`         | 11 per-app workflows + 1 reusable template, of ~21 files                                        |
| `infra/`                     | 21 files — 100% of the tree                                                                     |
| `.claude/agents/`            | 29 of 90 app-scoped, plus 2 agents premised on the OSE↔AyoKoding split                          |
| `.claude/skills/`            | 3 of 31 app-scoped, plus 1 doctrine-moot                                                        |
| `repo-config.yml`            | 25 of 26 `coverage.projects` entries; all 8 `env-contract.surfaces`; all 8 `env-injection.apps` |
| `repo-governance/workflows/` | `ayokoding-web/` — 6 files, ~113 KB                                                             |
| `plans/done/`                | 174 archived plan folders for apps that will not exist                                          |
| `specs/apps/`                | 5 of 6 area trees                                                                               |
| `open-sharia-enterprise.sln` | 100% `crane-cli` — the file contains nothing else                                               |

Beyond disk, this is an **instruction-surface** problem. `AGENTS.md` (29.3 KB) is auto-loaded into
every agent session and currently opens with "**open-sharia-enterprise** — Enterprise platform for
Sharia-compliant business systems", carries a Web Sites table naming eight domains, and enumerates
an agent roster of which a third will be deleted. Every agent in this repo begins work with a
description of a codebase that does not match the codebase.

### Why now

Nothing has been built on Baseerah yet. The cost of the reset only grows: every Baseerah commit
landed before the purge is a commit that must coexist with dead OSE app code, dead CI jobs, and
agents that reference deleted paths. Doing the reset first means the first real Baseerah feature
lands into a repo that describes itself accurately.

There is also a correctness deadline. `AGENTS.md` states that `apps/rhino-cli` must be
**byte-identical (zero carve-outs)** across `ose-public`, `ose-primer`, and `ose-private`. This repo
is a fourth clone carrying that rule, and Phase 3 makes real source edits to `rhino-cli`: emptying
the `WEBSITE_APP_PREFIXES` frontmatter-audit **exemption** list — whose four entries all name apps
this plan deletes — plus test-fixture renames in `specs_validate_counts.rs`, where the retired app
names appear only inside a unit test and the production default already reads from
`repo-config.yml`. Until `AGENTS.md` states that `baseerah` is outside the parity loop, the repo's
own instructions forbid the edit the repo needs.

## Business Impact

**Pain points removed**

- Agents no longer plan against, search, or validate a product that isn't here.
- CI stops running — and stops being maintained for — 12 workflow files with no target.
- `repo-config.yml`, the file three separate CI gates validate, stops describing 25 absent projects.
- Grep for any product term stops returning hundreds of hits from `plans/done/`.

**Benefits gained**

- A repo whose instruction surface, CI, and code agree with each other.
- A running end-to-end stack (`baseerah-fe` → `baseerah-be`) that any subsequent feature plan can
  extend rather than bootstrap.
- Baseerah-scoped maker/checker/fixer agents from day one, so content and code quality gates exist
  before there is much to gate.

## Affected Roles

Solo-maintainer repo; these are hats and agent consumers, not sign-off parties.

| Role / consumer                                       | How this plan affects it                                                 |
| ----------------------------------------------------- | ------------------------------------------------------------------------ |
| Maintainer (product hat)                              | Gains a named product with a vision doc under the OSE ecosystem          |
| Maintainer (platform hat)                             | Keeps the harness; owns a much smaller surface                           |
| `plan-maker` / `plan-checker` / `plan-fixer`          | Read `plans/**`; the archive shrinks from 174 folders to this one        |
| `repo-rules-checker`, `ci-checker`                    | Validate against `repo-config.yml` and `.github/workflows/`; both shrink |
| `swe-fsharp-dev`, `swe-typescript-dev`, `swe-e2e-dev` | Become the primary implementers for the four new apps                    |
| `rhino-cli` itself                                    | Receives source edits; its Gherkin tree gains scenarios                  |
| `repo-setup-manager`                                  | Executes Phase 0 against a much smaller project graph                    |

## Success Metrics

1. **Observable fact** — `npx nx show projects` lists exactly: `rhino-cli`, `rust-commons`,
   `web-ui`, `web-ui-token`, `baseerah-contracts`, `baseerah-be`, `baseerah-be-e2e`, `baseerah-fe`,
   `baseerah-fe-e2e`, plus `fsharp-crane-core` if the Phase 2 audit keeps it (see
   [tech-docs.md § Dependencies](./tech-docs.md#dependencies)). Nine or ten projects total. Nothing
   else.
2. **Observable fact** — `rg -l 'ayokoding|organiclever|wahidyankf|crane-cli|ose-www|ose-app-web|ose-be|ose-cli'`
   returns zero hits outside `plans/` (this plan's own docs) and git history.
3. **Observable fact** — `main-ci.yml` passes on `origin/main` after every phase push, with no job
   removed to make it pass (phase gates assert per-job status via
   `gh run view --json jobs`, not just the overall conclusion).
4. **Observable fact** — `npx nx run baseerah-fe-e2e:test:e2e` passes against the
   `infra/dev/baseerah-app/docker-compose.yml` stack, proving `baseerah-fe` reaches `baseerah-be`
   over HTTP.
5. **Observable fact** — `npm run validate:sync` reports zero drift after `npm run generate:bindings`,
   with `.opencode/agents/`, `.cursor/agents/`, and `.amazonq/` regenerated rather than hand-edited.
6. **Observable fact** — `nx run rhino-cli:instruction-size:validation` passes, i.e. the rewritten
   `AGENTS.md` stays inside its byte budget.
7. _Judgment call:_ the maintainer expects a materially lower rate of agents proposing work against
   deleted apps. **No baseline measured** — this is a structural claim, not a metric.

## Business-Scope Non-Goals

- **Not a product launch.** No domain, no deploy, no users. Deploy branches and Vercel/GHCR projects
  are deliberately deferred.
- **Not a governance redesign.** The six-layer hierarchy, the maker-checker-fixer pattern, the
  Delivery Mode system, and the plan lifecycle survive unchanged. Only their _product_ references move.
- **Not an OSE fork severance.** Baseerah is positioned as a product _within_ the OSE ecosystem. The
  Layer 0 OSE vision document stays.
- **Not a dependency upgrade.** Toolchain versions are whatever `npm run doctor` converges on.

## Business Risks and Mitigations

| Risk                                                                                                      | Severity | Mitigation                                                                                                                                                                                                                                                                                                                    |
| --------------------------------------------------------------------------------------------------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Deleting `plans/done/` destroys 174 plans' worth of hard-won decisions                                    | High     | `ose-public` remains the authoritative archive and is untouched. Phase 3 records the upstream commit SHA in `evidence/` before deleting.                                                                                                                                                                                      |
| An empty `repo-config.yml` section (`coverage.projects`, `env-contract.surfaces`) fails schema validation | High     | Phase 2 reads the Rust schema in `apps/rhino-cli/src/**/repo_config` **before** emptying, and keeps `rhino-cli`'s own entry so no list is ever empty                                                                                                                                                                          |
| `rhino-cli` source edits break the binding generator that all other cleanup depends on                    | High     | Phase 3 edits `rhino-cli` under strict TDD with companion Gherkin, and its gate re-runs `generate:bindings` + `validate:sync` end to end                                                                                                                                                                                      |
| `main-to-origin-main` removes the PR review gate, so a bad push is immediately live on `origin/main`      | High     | Every phase gate runs the exact pre-push and CI-equivalent commands locally _before_ the push step, and the push is the last item in each phase. Recovery is a forward `git revert`, never a force-push — see [No Destructive Git Operations](../../../repo-governance/development/workflow/no-destructive-git-operations.md) |
| `libs/fsharp-crane-core` turns out to be `crane-cli`-specific and useless to `baseerah-be`                | Medium   | Phase 2 audits it explicitly and records the verdict; if it is crane-only, it is deleted there and `baseerah-be` scaffolds without it                                                                                                                                                                                         |
| Governance prose still describing deleted apps survives the sweep and silently misleads agents            | Medium   | Phase 3's gate is a `rg` sweep with an explicit zero-hit acceptance criterion, not a manual read-through                                                                                                                                                                                                                      |
| Keeping the `@open-sharia-enterprise/*` npm scope reads as leftover cruft to a future reader              | Low      | `tech-docs.md` Decision 3 records the rationale in-repo, and `AGENTS.md` states the scope is the ecosystem marker                                                                                                                                                                                                             |
