# AGENTS.md

> Canonical instruction file for any AI coding agent or human contributor working in this repo.
> Aligned with the [AGENTS.md standard](https://agents.md/) (Agentic AI Foundation / Linux Foundation).

## Repository Overview

**BeaverNest** — a personal operating layer covering an AI assistant, a content builder, a posting
helper, and a personal workflow engine,
built as an Nx monorepo. BeaverNest is a **product within the Open Sharia Enterprise (OSE) ecosystem**,
not a replacement for it — see [BeaverNest Vision](./repo-governance/vision/beaver-nest.md) and its parent
[Open Sharia Enterprise Vision](./repo-governance/vision/open-sharia-enterprise.md).

**Status**: Foundation runtime (`beaver-nest-fe` Vite CSR served from the `beaver-nest-be` ASP.NET image)
**License**: MIT
**Main Branch**: `main` (Trunk Based Development)

### Tech Stack

- **Node.js**: 24.13.1 (LTS, managed by Volta)
- **npm**: 11.10.1
- **Monorepo**: Nx workspace
- **App naming tiers**: `[domain]-www` = public website at the domain root; `[domain]-app-web` = product
  web client at `app.*`; `[domain]-be` = generic HTTP backend for a product domain; `[domain]-fe` =
  the product web client when the domain has no separate marketing site.
- **Current Apps**: Next.js sites, F# backends, Rust and F# CLIs, a contract spec, and paired E2E
  suites — names and ports in the Web Sites table below and in
  [monorepo structure](./docs/reference/monorepo-structure.md).

Polyglot demo apps extracted 2026-04-18 to [`ose-primer`](https://github.com/wahidyankf/ose-primer)
(now authoritative for the polyglot showcase).

## Project Structure

`apps/` (Nx apps), `libs/` (`rust-commons`, `web-ui`, `web-ui-token`), `docs/` (Diátaxis:
tutorials/how-to/reference/explanation), `repo-governance/`
(conventions/development/principles/workflows/vision), `plans/` (backlog/in-progress/done), `.claude/`
(primary binding: agents + skills), `.opencode/` (auto-synced from `.claude/`).

**See**: [monorepo-structure.md](./docs/reference/monorepo-structure.md)

## Build, Test, Lint Commands

```bash
npm install                           # Install deps (runs doctor)
nx build [project]                    # Build
nx run [project]:test:quick           # Pre-push quality gate
nx run [project]:test:unit            # Unit (cacheable)
nx run [project]:test:integration     # Integration (NOT cacheable)
nx run [project]:test:e2e             # E2E (NOT cacheable)
nx affected -t build,test:quick,lint  # Affected projects only
nx graph                              # Dependency graph
npm run doctor -- --fix               # Install missing tools
npm run lint:md:fix                   # Fix markdown violations
```

**Worktree setup**: After `git worktree add`, run `npm install` AND `npm run doctor -- --fix`. See
[Worktree Toolchain Initialization](./repo-governance/development/workflow/worktree-setup.md).

**See**: [nx-targets.md](./repo-governance/development/infra/nx-targets.md)
for canonical target names, coverage thresholds, caching rules, and the three-level testing standard.

## Markdown Quality

All markdown auto-linted via Prettier (pre-commit), markdownlint-cli2 (pre-push), and rhino-cli's
`md mermaid validate`, `md links validate`, and `md heading-hierarchy validate` subcommands (wired
into pre-commit/pre-push hooks and CI as raw `cargo run` invocations — not Nx targets). Quick fix:
`npm run lint:md:fix`.

**See**: [markdown.md](./repo-governance/development/quality/markdown.md),
[repository-validation.md](./repo-governance/development/quality/repository-validation.md)

## Cross-Language Lint Gates

Shell scripts, Dockerfiles, GitHub Actions, and F# gated at **warning-and-above** (CI + Husky hooks).
Linters: shellcheck (`--severity=warning`), hadolint (`--failure-threshold warning`), actionlint,
F# strict (`TreatWarningsAsErrors` + G-Research.FSharp.Analyzers + `fantomas --check`).
All installed by `npm run doctor -- --fix`.

**Instruction-file size budget** (`nx run rhino-cli:instruction-size:validation`): per-surface byte
thresholds on auto-loaded instruction files; sole remediation is progressive disclosure.
See [Instruction-File Size Budget Convention](./repo-governance/conventions/structure/instruction-file-size-budget.md).

**See**: [cross-language-lint-strictness.md](./repo-governance/development/quality/cross-language-lint-strictness.md)

## Monorepo Architecture

`apps/` — deployable, naming `[domain]-[type]`, import libs but never export, never import other apps.
`libs/` — flat, naming `ts-[name]`/`rust-[name]`/`fsharp-[name]`, import via
`@open-sharia-enterprise/ts-[lib-name]`, no circular deps.

**See**: [monorepo-structure.md](./docs/reference/monorepo-structure.md),
[add-new-app.md](./docs/how-to/add-new-app.md),
[nx-targets.md](./repo-governance/development/infra/nx-targets.md)

## Git Workflow

**Trunk Based Development** — `main` is the single integration target. Every `prod-*` and `stag-*`
ref is a deploy target — **never commit directly**. `git branch -r` is authoritative and includes
lib/backend targets (`prod-web-ui`, `stag-ose-be`) absent from the Web Sites table below.
**Commit format**: Conventional Commits `<type>(<scope>): <description>` — imperative mood, no
period. Split by domain/concern.

**See**: [commit-messages.md](./repo-governance/development/workflow/commit-messages.md)

### Worktree Path

Worktrees land at **`worktrees/<name>/`** in the repo root (gitignored). Routing handled by a
repo-local `WorktreeCreate` hook.

**See**: [worktree-path.md](./repo-governance/conventions/structure/worktree-path.md)

### Delivery Mode

Every plan declares one of four Delivery Modes — `worktree-to-pr` (**the default**),
`worktree-to-origin-main`, `main-to-origin-main`, `main-to-pr` — naming its work location (worktree
or primary checkout) and integration target (draft PR or direct push). `*-to-pr` modes run the
**PR-Review Maker→Fixer Cycle** (default 3 sequential CI-gated cycles) before the merge. **`[AI]` merges by default** in every mode; a
`[HUMAN]` merge gate applies only where a plan's own step says so explicitly, with identical
preconditions — only the actor differs.

**The PR is the independent merge point** — N parallel units become N PRs reviewed, gated, and
merged independently, which is why `worktree-to-pr` is the default; each change-producing DAG leaf
gets its own worktree and PR (strict 1-PR ↔ 1-worktree), dependent nodes staying one PR.
A PR merges only when **all five hardened preconditions** (a)-(e) hold — see the PR Merge Protocol.
**PRs open at delivery boundaries, not every phase** — a PR covers a **delivery unit**, the
contiguous phases ending where work becomes independently shippable, so a plan opens one once at the
end or several times through; folding independent nodes together to cut PR count stays forbidden.
**Phase 0 opens none under any mode** — the earliest PR is Phase 1, and Phase 0's evidence rides it.

**See**: [PR Merge Protocol](./repo-governance/development/workflow/pr-merge-protocol.md),
[Plans Organization Convention §Delivery Mode](./repo-governance/conventions/structure/plans.md#delivery-mode)
and [§Phase 0 Opens No PR](./repo-governance/conventions/structure/plans.md#phase-0-opens-no-pr--the-earliest-pr-is-phase-1-hard-rule)
and [§PRs Open at Delivery Boundaries](./repo-governance/conventions/structure/plans.md#prs-open-at-delivery-boundaries-not-every-phase-hard-rule),
[PR Review Quality Gate workflow](./repo-governance/workflows/pr/pr-review-quality-gate.md)

## Git Hooks (Automated Quality)

Pre-commit: format (Prettier/gofmt/rustfmt), validate markdown links + markdownlint, lint
shell/Dockerfiles/workflows, auto-sync platform bindings, auto-stage. Commit-msg: Commitlint.
Pre-push: `typecheck`, `lint`, `test:quick`, `specs:coverage` for affected projects (parallelism:
cores-1); markdown linting. All four Nx targets cacheable — warm cache before push if timeout occurs.

**See**: [code.md](./repo-governance/development/quality/code.md)

## Documentation Organization

**Diátaxis Framework**: `docs/tutorials/` (learning), `docs/how-to/` (problem-solving),
`docs/reference/` (specs), `docs/explanation/` (concepts). File naming: lowercase kebab-case;
exception: `README.md`.

**See**: [file-naming.md](./repo-governance/conventions/structure/file-naming.md),
[diataxis-framework.md](./repo-governance/conventions/structure/diataxis-framework.md)

## Conventions

Core principles (see [Principles Index](./repo-governance/principles/README.md) for full list):

- **Deliberate Problem-Solving**: Understand before acting; prefer reversible decisions
- **Simplicity Over Complexity**: Minimum viable abstraction
- **Root Cause Orientation**: Fix root causes, not symptoms; proactively fix preexisting errors
  encountered during work (do not mention and defer)
- **Accessibility First**: WCAG AA compliance, color-blind friendly
- **No Time Estimates**: Never give time estimates; focus on outcomes

### File Naming

Lowercase kebab-case (`[a-z0-9-]+`). Exception: `README.md`, `docs/metadata/` files.

**See**: [file-naming.md](./repo-governance/conventions/structure/file-naming.md)

### Linking

GitHub-compatible markdown with `.md` extension.

**See**: [linking.md](./repo-governance/conventions/formatting/linking.md)

### Indentation

Markdown nested bullets: 2 spaces. YAML frontmatter: 2 spaces. Code: language-specific.

**See**: [indentation.md](./repo-governance/conventions/formatting/indentation.md)

### Emoji Usage

Allowed: `docs/`, README, `plans/`, `repo-governance/`, `AGENTS.md`, `CLAUDE.md`, agent definition
files, Agent Skill files. Forbidden: config files (`*.json`, `*.yaml`, `*.toml`), source code.

**See**: [emoji.md](./repo-governance/conventions/formatting/emoji.md)

### Diagrams

Mermaid diagrams with color-blind friendly palette, proper accessibility.

**See**: [diagrams.md](./repo-governance/conventions/formatting/diagrams.md)

### Content Quality

Active voice, single H1, proper heading nesting, alt text for images, WCAG AA color contrast.

**See**: [quality.md](./repo-governance/conventions/writing/quality.md)

### Dynamic Collection References

Never hardcode counts of dynamic collections (agents, skills, conventions, practices, principles,
workflows) in docs. Reference collection by name and link.

**See**: [dynamic-collection-references.md](./repo-governance/conventions/writing/dynamic-collection-references.md)

## Development Practices

### Functional Programming

Prefer immutability, pure functions, functional core/imperative shell.

**See**: [functional-programming.md](./repo-governance/development/pattern/functional-programming.md)

### Implementation Workflow

Make it work → Make it right → Make it fast.

**See**: [implementation.md](./repo-governance/development/workflow/implementation.md)

### Test-Driven Development

Red → Green → Refactor. Required for all code changes. Every code delivery step uses the explicit
three-substep template (RED/GREEN/REFACTOR), each naming a file path, verbatim command, and acceptance
criterion.

**See**: [test-driven-development.md](./repo-governance/development/workflow/test-driven-development.md)

### Specs & Gherkin Completeness (Both Paths)

Code under `apps/`/`libs/` never lands without companion `specs/` Gherkin — **both** for direct changes
(same commit/PR; enforced by `specs:coverage` + `swe-code-checker`) and planned changes (plan carries
Gherkin steps; `plan-maker` emits them, `plan-checker` flags absence). Pure refactors and docs-only
changes are exempt.

**See**: [feature-change-completeness.md](./repo-governance/development/quality/feature-change-completeness.md)

### Regression Test Mandate (Every Bug Fix)

Every bug fix lands with a reproducing test (failing before fix, passing after) in the same commit/PR —
blocking, no exemptions. Enforced by `swe-code-checker` (Step 6.7) and `plan-checker` (Step 16b).

**See**: [regression-test-mandate.md](./repo-governance/development/quality/regression-test-mandate.md)

### Knowledge Capture

Every plan ends with a Knowledge Capture phase: `learnings.md` triaged to a home or discarded.

**See**: [knowledge-capture.md](./repo-governance/development/quality/knowledge-capture.md)

### Reproducible Environments

Volta for Node.js/npm pinning, package-lock.json, .env.example. **Hard iron rule — no secrets in
committed files**: Never commit system secrets to any git-tracked file — history is permanent. Real
values in uncommitted `.env*` (except `.env.example`). **Guardrail**: Agents must not
read/write/edit/commit real `.env*` files — only `.env.example` is permitted; scripts under
`apps/`/`libs/`/`scripts/` are exempt, as are non-dotfile course fixtures (`kata.env`, `app.env`)
under an app's published `apps/<app>/content/**` tree. **Git Identity Guardrail**: No AI agent sets or modifies git
identity at **any** scope — `git config user.*` bare/`--local`/`--global`/`--system`, or direct
`.git/config [user]` edits. Identity comes from the developer's `~/.gitconfig` (`includeIf` for
per-tree overrides). CI service-account identity in workflow YAML is exempt.

**See**: [reproducible-environments.md](./repo-governance/development/workflow/reproducible-environments.md),
[Secrets and Env Standards](./repo-governance/conventions/security/secrets-and-env-standards.md)

### Dependency Bump Stability & Safety Policy

Three-path tree: A (LTS latest patch), B (60-day soak + CVE-clean), C (security-override waiver).
Exact pins only, CVE-clean across NVD, GitHub Advisories, Snyk, vendor pages, CISA KEV. CISA-KEV
fast-track and EPSS ≥ 0.5 escalate to Path C.

**See**: [dependency-bump-policy.md](./repo-governance/development/workflow/dependency-bump-policy.md)

### Agent Workflow Orchestration

Plan mode for non-trivial tasks (3+ steps or architecture decisions). **Parallel-by-default**: the
**N+1 model** — `1 main thread + N background agents`, **default N=3** — bounds fan-out; raise/lower N
per-plan by capacity and budget headroom, never self-promote beyond it. Poll subagent mtime every 3
min; stale 30 min triggers `TaskStop` and relaunch.
**Same-machine assumption**: other agents/engineers/processes run concurrently on the same shared
disk, git object store, worktrees, and CI runners, so every orchestration and git action must be
concurrency-safe.
**File-touch ledger**: these repos are edited constantly by agents and humans — in worktrees, on
branches, and on local `main` — so keep a deliberate, append-only record of every file you touch,
**reproduce it in full through every compaction, summary, and handoff**, and reconcile it against
`git status` before staging. `git status` is the union of everyone's work, never a report of yours.
Anything not on your ledger is another actor's in-flight work: leave it untouched, and without a
ledger assume **nothing** is yours.
**Harness mirrors are generated, not hand-written**: `.claude/` is the only hand-authored surface;
`.opencode/`, `.cursor/`, and `.amazonq/` are emitted by `rhino-cli harness bindings generate`
(`npm run generate:bindings`, also run and auto-staged by pre-commit). Those mirrors are files you
touched — they go on your ledger and into the **same commit** as their source, never a follow-up
sync commit. Verify with `npm run validate:sync`; never hand-edit a mirror.
**DAG-first**: every task list/delivery checklist declares a dependency DAG (`blocks`/`blockedBy`);
independent nodes fan out up to N, dependent nodes serialize, cleanup is the terminal node.
**Background-slot preference**: fill background slots up to N, keeping the main thread the vacant
orchestrator, never splitting dependent work to fill a slot. Report every 5 min generic, 3 min CI;
maintain a live task list, marking in-progress/completed and adding discovered tasks immediately.

**See**: [agent-workflow-orchestration.md](./repo-governance/development/agents/agent-workflow-orchestration.md),
[Subagent Orchestration Convention](./repo-governance/development/agents/subagent-orchestration.md),
[Parallel-by-Default Practice](./repo-governance/development/practice/parallel-by-default.md),
[Task List Discipline](./repo-governance/development/practice/task-list-discipline.md),
[File-Touch Discipline](./repo-governance/development/practice/file-touch-discipline.md),
[No Destructive Git Operations](./repo-governance/development/workflow/no-destructive-git-operations.md),
[Worktree and Artifact Cleanup](./repo-governance/development/workflow/worktree-and-artifact-cleanup.md)

### Manual Verification & CI Blockers

- **Verify behavior**: Follow [manual verification](./repo-governance/development/quality/manual-behavioral-verification.md):
  use a real browser for UI (Chrome/DevTools, then Playwright); static-only is invalid when live checks
  are feasible.
- **User-facing delivery hardening**: Sixteen rules; near-end EWT/UWT/DWT retest for UI plans, AET
  for API plans. See [user-facing-delivery-hardening.md](./repo-governance/development/quality/user-facing-delivery-hardening.md)
- **CI blockers**: Investigate root cause, fix properly, never bypass.
  See [ci-blocker-resolution.md](./repo-governance/development/quality/ci-blocker-resolution.md)
- **CI post-push verification**: After pushing app or lib code, trigger CI and verify it passes.
  See [ci-post-push-verification.md](./repo-governance/development/workflow/ci-post-push-verification.md)
- **CI monitoring**: Poll every **2 minutes** — one `gh run view --json status,conclusion` per wakeup.
  Never tight-loop, never `gh run watch`. Rate-limited (403): wait ~35 min.
  See [ci-monitoring.md](./repo-governance/development/workflow/ci-monitoring.md)

## AI Agents

The **[agent catalog](./.claude/agents/README.md) is authoritative** — every agent is listed there by
role. Do not maintain a second roster here. Names follow `<domain>-<role>`:

- **maker / checker / fixer** — the three-stage pattern (criticality CRITICAL/HIGH/MEDIUM/LOW;
  confidence HIGH/MEDIUM/FALSE_POSITIVE), spanning docs, readme, specs, ci, `swe-{code,ui}`,
  `repo-{rules,workflow,harness-compatibility}`, and pdf-to-md.
- **`swe-*-dev`** — language implementers. **Meta** — agent-maker, repo-{rules,workflow}-maker.
  **Research** — web-researcher.
- **Operations** — `apps-beaver-nest-fe-{content-maker,content-checker,content-fixer,deployer}` and
  `apps-beaver-nest-be-deployer`. The deployer agents document their intended workflow honestly: no
  production or staging deploy target is provisioned yet (see each agent's own file).
- **Planning** — `plan-{maker,checker,execution-checker,fixer}`, repo-setup-manager. plan-maker
  grills the user before/after with multiple-choice options per the
  [Grilling-With-Options Convention](./repo-governance/development/workflow/grilling-with-options.md);
  Phase 0 first, `[AI]`/`[HUMAN]` tags, gated phases. See the
  [plan-execution](./repo-governance/workflows/plan/plan-execution.md) and
  [plan-planning](./repo-governance/workflows/plan/plan-planning.md) workflows.
- **PR Review Cycle** — eight discipline `pr-review-*-maker` specialists fan out to
  `pr-review-synthesis-maker` (coordinator) to `pr-review-fixer`, for `*-to-pr` Delivery Mode plans.
  See [Delivery Mode](./repo-governance/conventions/structure/plans.md#delivery-mode),
  [PR Review Quality Gate](./repo-governance/workflows/pr/pr-review-quality-gate.md),
  [PR Reviewer-Discipline Convention](./repo-governance/development/quality/pr-review-disciplines.md).
- **Testing** — `web-{exploratory,usability,design}-tester` (spec-aware / spec-blind / design-aware)
  and api-exploratory-tester. All non-destructive; output modes `plan` (default), `delivery`
  (rule-15 retest), `local-temp`.

**Web Research Default**: `web-researcher` is the default primitive for public-web information gathering.
See [Web Research Delegation Convention](./repo-governance/conventions/writing/web-research-delegation.md).

**agent skills infrastructure**: Two modes — **Inline** (default: inject into current conversation) and
**Fork** (`context: fork`: delegated isolated context, return summarized results). Agent definition files
at `.claude/agents/<name>.md`; skill files at `.claude/skills/<name>/SKILL.md`. Agent skills serve agents
(service relationship, not governance).

**See**: [ai-agents.md](./repo-governance/development/agents/ai-agents.md),
[maker-checker-fixer.md](./repo-governance/development/pattern/maker-checker-fixer.md),
[Agent Naming Convention](./repo-governance/conventions/structure/agent-naming.md),
[Workflow Naming Convention](./repo-governance/conventions/structure/workflow-naming.md)

## Repository Architecture

Six-layer governance hierarchy: Layer 0 (Vision — WHY we exist: democratize Shariah-compliant
enterprise), Layer 1 (Principles — WHY we value approaches), Layer 2 (Conventions — WHAT documentation
rules), Layer 3 (Development — HOW we develop), Layer 4 (AI Agents — WHO enforces rules), Layer 5
(Workflows — WHEN we compose agents/procedures). **agent skills**: delivery infrastructure (inline + fork
modes) serving agents — not a governance layer.

**See**: [repository-governance-architecture.md](./repo-governance/repository-governance-architecture.md)

## Web Sites

| App            | Domain      | Port                                        | Prod Branch |
| -------------- | ----------- | ------------------------------------------- | ----------- |
| beaver-nest-fe | same origin | 19310 (local dev)                           | TBD         |
| beaver-nest-be | same origin | 19320 (local dev); 19300 (combined runtime) | TBD         |

Each app README at `apps/[app-name]/README.md` covers framework, deployment, E2E tests, and content
details.

## Temporary Files for AI Agents

- **`generated-reports/`**: Validation/audit reports. Pattern:
  `{agent-family}__{uuid-chain}__{YYYY-MM-DD--HH-MM}__audit.md`. Checkers MUST write progressive reports.
- **`local-temp/`**: Misc temporary files.

**See**: [temporary-files.md](./repo-governance/development/infra/temporary-files.md)

### Build-Artifact Sweeper

An ambient scheduled sweeper on the host machine deletes gitignored build output (`target/`, `dist/`,
`.next/`), tool caches (`.nx/cache`), and the shared cargo `target/` at any time, mid-session and
mid-plan. A missing artifact is expected — regenerate (`nx build` / `npm install` / `npm run doctor --
--fix`) and continue. Never file a finding, commit build output, edit `.gitignore` to protect it, or
blame a concurrent agent. It never touches tracked files, `.env*`, `generated-reports/`, `local-temp/`,
worktrees, or git refs — anything else missing is not the sweeper.

**See**: [build-artifact-sweeper.md](./repo-governance/development/infra/build-artifact-sweeper.md)

## Plans

`plans/` folder: `ideas/` (two-pager briefs), `backlog/` (future; `[id]/`),
`in-progress/` (active; `[id]/`), `done/` (completed; `YYYY-MM-DD__[id]/`).

**See**: [plans.md](./repo-governance/conventions/structure/plans.md)

## Important Notes

- **Never commit secrets** (hard iron rule): No system secret goes into any git-tracked file; real values
  belong in uncommitted `.env*` (except `.env.example`). See [Secrets and Env Standards](./repo-governance/conventions/security/secrets-and-env-standards.md).
- **Do NOT stage or commit** unless explicitly instructed. Per-request commits one-time only.
- **License**: MIT. See [LICENSING-NOTICE.md](./LICENSING-NOTICE.md)
- **Agent invocation**: Use natural language to invoke agents/workflows
- **Token budget**: Don't worry about token limits — reliable compaction available
- **No time estimates**: Never give time estimates. Focus on what needs doing, not how long.

## Related Documentation

- **Conventions Index**: [README.md](./repo-governance/conventions/README.md) — Documentation writing and org standards
- **Development Index**: [README.md](./repo-governance/development/README.md) — Software dev practices and workflows
- **Principles Index**: [README.md](./repo-governance/principles/README.md) — Foundational values governing all layers
- **Primary Binding Agents Index**: [agent catalog](./.claude/agents/README.md) — Specialized agents organized by role
- **Workflows Index**: [README.md](./repo-governance/workflows/README.md) — Orchestrated processes
- **Repository Architecture**: [repository-governance-architecture.md](./repo-governance/repository-governance-architecture.md) — Six-layer governance hierarchy

## Related Repositories

Four sibling repos cross-referencing each other directly — no parent container repo. **"All of the
OSE repos" means exactly these four**; `beaver-nest` is always included, despite standing **outside**
the three-repo parity loop below:
[`ose-public`](https://github.com/wahidyankf/ose-public) (MIT — upstream source of truth for
scaffolding), [`ose-primer`](https://github.com/wahidyankf/ose-primer) (MIT — downstream public
template: governance, agents, skills, conventions, CI harness, polyglot demo apps),
[`ose-private`](https://github.com/wahidyankf/ose-private) (proprietary — GitHub Actions runner
stack, `coralpolyp`; not public), and this repo (MIT — a personal operating layer built as a product
within the OSE ecosystem).

Content parity between `ose-public` and `ose-primer` maintained via
[plan-multi-repo-parity-planning](./repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)
workflow. `ose-private` and `beaver-nest` do not participate in the parity loop.

`apps/rhino-cli` must be byte-identical (zero carve-outs) across those three parity repos, including
its Gherkin behavior tree at `specs/apps/rhino/behavior/rhino-cli/gherkin/**`, per the
[SDLC Gate Standard](./docs/reference/sdlc-gate-standard.md#rhino-cli-byte-identity-boundary). This
repo's `apps/rhino-cli` is a **fork** of that shared tool and is explicitly **not** bound by the
byte-identity rule (tech-docs Decision 14).

See: [Related Repositories reference](./docs/reference/related-repositories.md).

## Models

See [Model Selection](./repo-governance/development/agents/model-selection.md).

## General Guidelines for Working with Nx

- For navigating/exploring the workspace, invoke the `nx-workspace` skill first — it has patterns for
  querying projects, targets, and dependencies
- When running tasks (build, lint, test, e2e, etc.), prefer running through `nx` (`nx run`,
  `nx run-many`, `nx affected`) instead of underlying tooling directly
- Prefix nx commands with the workspace package manager (e.g., `pnpm nx build`, `npm exec nx test`)
- You have access to the Nx MCP server and its tools; use them
- For Nx plugin best practices, check `node_modules/@nx/<plugin>/PLUGIN.md`. Not all plugins have this
  file — proceed without it if unavailable.
- NEVER guess CLI flags — check nx_docs or `--help` first when unsure

## Scaffolding & Generators

For scaffolding tasks (creating apps, libs, project structure, setup), ALWAYS invoke the `nx-generate`
skill FIRST before exploring or calling MCP tools.

## When to use nx_docs

- USE for: advanced config options, unfamiliar flags, migration guides, plugin config, edge cases
- DON'T USE for: basic generator syntax (`nx g @nx/react:app`), standard commands, things you already
  know
- The `nx-generate` skill handles generator discovery internally — don't call nx_docs just to look up
  generator syntax

## Platform Binding Examples

The content under this heading is intentionally vendor-specific. Per the
[Governance Vendor-Independence Convention](./repo-governance/conventions/structure/governance-vendor-independence.md),
the vendor-audit scanner skips every line under a "Platform Binding Examples"
heading until the next same-level heading or end of file.

### Platform Bindings Catalog

Concrete tool integrations live **outside** `repo-governance/` in platform-binding directories:

Tier-1 harnesses — OpenAI Codex CLI, GitHub Copilot, Cursor, Windsurf, Junie, Antigravity CLI, Pi,
and Kiro CLI — read `AGENTS.md` natively and get **no** per-tool instruction file (no-shadowing).
The exceptions:

- **Claude Code** → `.claude/`, with `CLAUDE.md` as the discoverable shim importing this file
- **OpenCode** → `.opencode/agents/`; reads `AGENTS.md` and `.claude/skills/<name>/SKILL.md` natively
- **Cursor** → additionally emits `.cursor/agents/`
- **Amazon Q Developer** (sunsetting — IDE plugins EOS 2027-04-30, succeeded by Kiro CLI) → does not
  read `AGENTS.md`; gets a generated `.amazonq/` bridge (`rules/00-agents-md.md` + agent config)
- **Aider** → reads `CONVENTIONS.md` per [its own docs](https://aider.chat/docs/usage/conventions.html)

Every generated directory above is emitted by `rhino-cli harness bindings generate` — never hand-edited.

See [platform-bindings.md](./docs/reference/platform-bindings.md) for the full catalog
of binding directories, root instruction files, and mechanical translation artifacts. The two-tier
binding model and no-shadowing rule are defined in
[multi-harness-binding.md](./repo-governance/conventions/structure/multi-harness-binding.md).

### Concrete Vendor Model IDs

Concrete vendor model IDs live in each platform binding's agent definition files (e.g.,
`.claude/agents/<name>.md` frontmatter for the primary platform binding).
