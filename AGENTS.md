# AGENTS.md

> Canonical instruction file for any AI coding agent or human contributor working in this repo.
> Aligned with the [AGENTS.md standard](https://agents.md/) (Agentic AI Foundation / Linux Foundation).

## Repository Overview

**open-sharia-enterprise** — Enterprise platform for Sharia-compliant business systems, Nx monorepo.

**Status**: Phase 1 (OrganicLever — Productivity Tracker)
**License**: MIT
**Main Branch**: `main` (Trunk Based Development)

### Tech Stack

- **Node.js**: 24.13.1 (LTS, managed by Volta)
- **npm**: 11.10.1
- **Monorepo**: Nx workspace
- **App naming tiers**: `[domain]-www` = public website at the domain root; `[domain]-app-web` = product
  web client at `app.*`; `[domain]-be` = generic HTTP backend for a product domain.
- **Current Apps**:
  - `ose-www` — Next.js 16 public website for OSE Platform (TypeScript, tRPC, port 3100)
  - `ose-www-be-e2e` — Playwright BE E2E tests for ose-www tRPC API
  - `ose-www-fe-e2e` — Playwright FE E2E tests for ose-www UI
  - `ose-be` — F# / Giraffe / ASP.NET 10 REST API backend for OSE Application platform (api.oseplatform.com, port 8302)
  - `ose-be-e2e` — Playwright BE E2E tests for ose-be
  - `ose-app-web` — Next.js 16 OSE Application frontend (app.oseplatform.com, port 3300)
  - `ose-app-web-e2e` — Playwright FE E2E tests for ose-app-web
  - `ayokoding-www` — Next.js 16 fullstack educational content platform (TypeScript, tRPC, port 3101)
  - `ayokoding-www-be-e2e` — Playwright BE E2E tests for ayokoding-www tRPC API
  - `ayokoding-www-fe-e2e` — Playwright FE E2E tests for ayokoding-www UI
  - `ayokoding-cli` — Rust CLI tool for content link validation
  - `rhino-cli` — Rust CLI tool for repository management (Repository Hygiene & INtegration Orchestrator). Ported from Go 2026-05-23.
  - `ose-cli` — Rust CLI tool for OSE Platform site maintenance (link validation)
  - `crane-cli` — F# CLI tool for PDF-to-Markdown conversion pipeline (Content Retrieval And Normalization Engine). Hexagonal ports-and-adapters architecture.
  - `organiclever-www` — Next.js 16 OrganicLever marketing website (port 3200)
  - `organiclever-www-be-e2e` — Playwright BE E2E slot for organiclever-www (placeholder — no backend API)
  - `organiclever-www-fe-e2e` — Playwright FE E2E tests for organiclever-www
  - `organiclever-app-web` — Next.js 16 OrganicLever app frontend (port 3202)
  - `organiclever-app-web-e2e` — Playwright FE E2E tests for organiclever-app-web
  - `organiclever-be` — F# / Giraffe / ASP.NET 10 REST API backend for OrganicLever (port 8202)
  - `organiclever-be-e2e` — Playwright BE E2E tests for organiclever-be
  - `organiclever-contracts` — OpenAPI 3.1 API contract spec (in `specs/apps/organiclever/containers/contracts/`); generates types + encoders/decoders for organiclever apps via `codegen` Nx target
  - `wahidyankf-www` — Next.js 16 personal portfolio site (www.wahidyankf.com, port 3201)
  - `wahidyankf-www-fe-e2e` — Playwright-BDD E2E tests for wahidyankf-www UI

Polyglot demo apps (11 backend implementations + 3 frontends + 1 fullstack) were extracted 2026-04-18 to the downstream [`ose-primer`](https://github.com/wahidyankf/ose-primer) template, which is now authoritative for the polyglot showcase.

## Project Structure

```
ose-public/
├── apps/                     # Deployable applications (Nx) — full list in "Current Apps" above
├── apps-labs/                # Experimental apps (NOT in Nx)
├── libs/                     # Reusable libraries (Nx, flat structure)
│   ├── rust-commons/         # Shared Rust utilities (link-checking, HTTP)
│   ├── fsharp-crane-core/    # Shared F# PDF-to-Markdown core (PdfPig + Tesseract)
│   └── web-ui/               # Shared React component library (shadcn/ui, Radix UI, Tailwind CSS)
├── docs/                     # Documentation (Diátaxis framework)
│   ├── tutorials/            # Learning-oriented
│   ├── how-to/               # Problem-solving
│   ├── reference/            # Technical reference
│   └── explanation/          # Conceptual understanding
├── repo-governance/               # Governance documentation (vendor-neutral)
│   ├── conventions/          # Documentation standards
│   ├── development/          # Development practices
│   ├── principles/           # Core principles
│   ├── workflows/            # Multi-step processes
│   └── vision/               # Project vision
├── plans/                    # Project planning
│   ├── in-progress/          # Active plans
│   ├── backlog/              # Future plans
│   └── done/                 # Completed plans
├── .claude/                  # Claude Code platform binding
│   ├── agents/               # Agent definitions (Claude Code format)
│   └── skills/               # Agent Skill packages
├── .opencode/                # OpenCode platform binding (auto-synced from .claude/)
│   └── agents/               # Agent definitions (OpenCode format)
├── .husky/                   # Git hooks
├── nx.json                   # Nx workspace config
└── package.json              # Volta pinning + npm workspaces
```

## Build, Test, Lint Commands

```bash
# Install dependencies (automatically runs doctor to verify tool versions)
npm install

# Build/test/lint all projects
npm run build
npm run lint

# Specific project operations
nx build [project-name]
nx run [project-name]:test:quick
nx lint [project-name]
nx dev [project-name]

# Affected projects only (canonical target names)
nx affected -t build
nx affected -t test:quick
nx affected -t lint

# Three-level test targets
nx run [project-name]:test:unit          # Mocked dependencies, no Docker, cacheable
nx run [project-name]:test:integration   # Real PostgreSQL via docker-compose or MSW/Godog. NOT cacheable
nx run [project-name]:test:e2e           # Real HTTP via Playwright. NOT cacheable

# Contract codegen (generates types from OpenAPI spec into generated-contracts/)
nx run organiclever-contracts:lint   # Lint + bundle the OpenAPI spec
nx run organiclever-contracts:docs   # Generate browsable API documentation
nx run [project-name]:codegen        # Generate types for a specific app

# Dependency graph
nx graph

# Markdown linting and formatting
npm run lint:md          # Lint all markdown files
npm run lint:md:fix      # Auto-fix markdown violations
npm run format:md        # Format markdown with Prettier
npm run format:md:check  # Check markdown formatting

# Verify local development environment
npm run doctor                    # Check all required tools
npm run doctor -- --fix           # Auto-install missing tools
npm run doctor -- --fix --dry-run # Preview what would be installed
npm run doctor -- --scope minimal # Check only core tools (git, volta, node, npm, go, docker, jq)
```

**Worktree setup**: After `git worktree add`, run both `npm install` AND `npm run doctor -- --fix` explicitly. See [Worktree Toolchain Initialization](./repo-governance/development/workflow/worktree-setup.md).

**See**: [repo-governance/development/infra/nx-targets.md](./repo-governance/development/infra/nx-targets.md) for canonical target names, coverage thresholds, caching rules, and the three-level testing standard (unit/integration/e2e).

## Markdown Quality

All markdown files auto-linted and formatted through a three-gate system:

- **Prettier** (v3.6.2): Formatting (runs on pre-commit)
- **markdownlint-cli2** (v0.20.0): Linting (runs on pre-push)
- **mermaid:validation** (`npx nx run rhino-cli:mermaid:validation`): Mermaid diagram
  validation — width, label length, syntax — repo-wide scan covering `flowchart`/`graph`
  and `stateDiagram-v2`/`stateDiagram` (v1); excludes `plans/done`,
  `apps/ayokoding-www/content`, and the standard noise-skip set; runs at pre-commit on
  staged `.md` files + `markdown-validate.yml` CI; does NOT run at pre-push)
- **links:validation** (`npx nx run rhino-cli:links:validation`): Full-repo link scan
  including `#fragment` anchor validation (runs at pre-commit + CI; does NOT run at
  pre-push)
- **headings:hierarchy-validation** (`npx nx run rhino-cli:headings:hierarchy-validation`):
  Heading nesting on prose allowlist (`docs/`, `repo-governance/`, `plans/` excl.
  `done/`, `specs/`, root `*.md`, `apps/*/README.md`, `libs/*/README.md`,
  `apps/*/docs/**`, `libs/*/docs/**`) (runs at pre-commit + CI; does NOT run at
  pre-push)

**Quick Fix**: If pre-push hook blocks push due to markdown violations:

```bash
npm run lint:md:fix
```

**See**: [repo-governance/development/quality/markdown.md](./repo-governance/development/quality/markdown.md),
[repo-governance/development/quality/repository-validation.md](./repo-governance/development/quality/repository-validation.md)
(Markdown Quality Gates section)

## Cross-Language Lint Gates

Beyond markdown, the repo gates shell scripts, Dockerfiles, GitHub Actions
workflows, and F# at a uniform **warning-and-above** threshold, enforced in both
CI (`.github/workflows/commons-quality-gate.yml`) and the local Husky hooks:

- **shellcheck** (`--severity=warning`, root `.shellcheckrc`) — all tracked `.sh` files (CI `shell` job)
- **hadolint** (`--failure-threshold warning`, root `.hadolint.yaml`) — all Dockerfiles (CI `dockerfile` job)
- **actionlint** — all `.github/workflows/*.yml` (CI `actions` job)
- **F# strict** — `TreatWarningsAsErrors` on every `.fsproj` + G-Research.FSharp.Analyzers + `fantomas --check`, riding the `dotnet` job's Nx `lint`/`typecheck` targets

All three new linters are installed by `npm run doctor -- --fix`.

**See**: [repo-governance/development/quality/cross-language-lint-strictness.md](./repo-governance/development/quality/cross-language-lint-strictness.md)

## Monorepo Architecture

Uses **Nx** to manage apps and libs:

- **`apps/`** — Deployable apps (naming: `[domain]-[type]`)
  - Apps import libs but never export
  - Each app independently deployable
  - Apps never import other apps
- **`libs/`** — Reusable libraries (naming: `ts-[name]`, `rust-[name]`, `fsharp-[name]`)
  - Flat structure, no nesting
  - Import via `@open-sharia-enterprise/ts-[lib-name]`
  - Libs can import other libs (no circular deps)
- **`apps-labs/`** — Experimental apps outside Nx (framework evaluation, POCs)

**Nx Commands**:

```bash
nx dev [app-name]            # Start development server
nx build [app-name]          # Build specific project
nx affected -t build         # Build only affected projects
nx affected -t test:quick    # Run pre-push quality gate for affected projects
nx graph                     # Visualize dependencies
```

**See**: [docs/reference/monorepo-structure.md](./docs/reference/monorepo-structure.md), [docs/how-to/add-new-app.md](./docs/how-to/add-new-app.md), [repo-governance/development/infra/nx-targets.md](./repo-governance/development/infra/nx-targets.md)

## Git Workflow

**Trunk Based Development** — All development on `main`:

- **Default branch**: `main`
- **Environment branches** (Vercel deployment only — never commit directly):
  - `prod-ayokoding-www` → [ayokoding.com](https://ayokoding.com)
  - `prod-ose-www` → [oseplatform.com](https://oseplatform.com)
  - `prod-organiclever-www` → [www.organiclever.com](https://www.organiclever.com/)
  - `prod-wahidyankf-www` → [www.wahidyankf.com](https://www.wahidyankf.com/)
  - `prod-organiclever-app-web` → app.organiclever.com (app tier; staging `stag-organiclever-app-web`)
  - `prod-ose-app-web` → app.oseplatform.com (app tier; staging `stag-ose-app-web`)
- **Commit format**: Conventional Commits `<type>(<scope>): <description>`
  - Types: feat, fix, docs, style, refactor, perf, test, chore, ci, revert
  - Scope optional but recommended
  - Imperative mood (e.g., "add" not "added")
  - No period at end
- **Split commits by domain**: Different types/domains/concerns = separate commits

**See**: [repo-governance/development/workflow/commit-messages.md](./repo-governance/development/workflow/commit-messages.md)

### Worktree Path

Worktrees in this repo land at **`worktrees/<name>/`** in the repo root, overriding the upstream coding-agent default that would otherwise place them under the platform binding directory. Routing is handled by a repo-local `WorktreeCreate` hook. Both paths are gitignored.

**See**: [repo-governance/conventions/structure/worktree-path.md](./repo-governance/conventions/structure/worktree-path.md)

## Git Hooks (Automated Quality)

Husky + lint-staged enforce quality:

- **Pre-commit**:
  - Validates agent definition files and auto-syncs platform bindings when changed in staged files
  - Formats staged files with Prettier (JS/TS/JSON/YAML/CSS/MD), gofmt (Go), and rustfmt (Rust)
  - Validates markdown links in staged files
  - Validates all markdown files (markdownlint)
  - Lints staged shell scripts (shellcheck), Dockerfiles (hadolint), and workflow files (actionlint) at the warning threshold
  - Auto-stages changes
- **Commit-msg**: Validates Conventional Commits format (Commitlint)
- **Pre-push**: Runs `typecheck`, `lint`, `test:quick`, and `specs:coverage` for affected projects (parallelism: cores-1)
  - Runs markdown linting
  - All four Nx targets cacheable — if pre-push times out, run `npx nx affected -t typecheck lint test:quick specs:coverage` first to warm cache, then push again

**See**: [repo-governance/development/quality/code.md](./repo-governance/development/quality/code.md)

## Documentation Organization

**Diátaxis Framework** — Four categories:

- **Tutorials** (`docs/tutorials/`) — Learning-oriented
- **How-to** (`docs/how-to/`) — Problem-solving
- **Reference** (`docs/reference/`) — Technical specs
- **Explanation** (`docs/explanation/`) — Conceptual understanding

**File Naming**: Lowercase kebab-case. Exception: `README.md` for index files.

**See**: [repo-governance/conventions/structure/file-naming.md](./repo-governance/conventions/structure/file-naming.md), [repo-governance/conventions/structure/diataxis-framework.md](./repo-governance/conventions/structure/diataxis-framework.md)

## Conventions

All work follows foundational principles from `repo-governance/principles/` (key ones below — see [Principles Index](./repo-governance/principles/README.md) for complete list):

- **Deliberate Problem-Solving**: Understand before acting; prefer reversible decisions
- **Simplicity Over Complexity**: Minimum viable abstraction
- **Root Cause Orientation**: Fix root causes, not symptoms; minimal impact; senior engineer standard; proactively fix preexisting errors encountered during work (do not mention and defer)
- **Accessibility First**: WCAG AA compliance, color-blind friendly
- **Documentation First**: Documentation mandatory, not optional
- **No Time Estimates**: Never give time estimates; focus on outcomes
- **Progressive Disclosure**: Layer complexity; start simple
- **Automation Over Manual**: Automate repetitive tasks
- **Explicit Over Implicit**: Explicit config over magic
- **Immutability Over Mutability**: Prefer immutable data structures
- **Pure Functions Over Side Effects**: Functional core, imperative shell
- **Reproducibility First**: Deterministic builds and environments

### File Naming

Lowercase kebab-case (`[a-z0-9-]+`) with standard extension; rule anchored on standard markdown and GitHub compatibility.
Exception: `README.md` for index files, `docs/metadata/` files.

**See**: [repo-governance/conventions/structure/file-naming.md](./repo-governance/conventions/structure/file-naming.md)

### Linking

GitHub-compatible markdown: `Text` with `.md` extension.
Next.js sites (ayokoding-www, ose-www) use standard GitHub-compatible markdown links with `.md` extension.

**See**: [repo-governance/conventions/formatting/linking.md](./repo-governance/conventions/formatting/linking.md)

### Indentation

Markdown nested bullets: 2 spaces per level. YAML frontmatter: 2 spaces. Code: language-specific.

**See**: [repo-governance/conventions/formatting/indentation.md](./repo-governance/conventions/formatting/indentation.md)

### Emoji Usage

Allowed: `docs/`, README files, `plans/`, `repo-governance/`, `AGENTS.md`, `CLAUDE.md`, agent definition files, Agent Skill files.
Forbidden: config files (`*.json`, `*.yaml`, `*.toml`), source code.

**See**: [repo-governance/conventions/formatting/emoji.md](./repo-governance/conventions/formatting/emoji.md)

### Diagrams

Mermaid diagrams with color-blind friendly palette, proper accessibility.

**See**: [repo-governance/conventions/formatting/diagrams.md](./repo-governance/conventions/formatting/diagrams.md)

### Content Quality

Active voice, single H1, proper heading nesting, alt text for images, WCAG AA color contrast.

**See**: [repo-governance/conventions/writing/quality.md](./repo-governance/conventions/writing/quality.md)

### Dynamic Collection References

Never hardcode counts of dynamic collections (agents, skills, conventions, practices, principles, workflows) in docs. Reference collection by name and link.

**See**: [repo-governance/conventions/writing/dynamic-collection-references.md](./repo-governance/conventions/writing/dynamic-collection-references.md)

## Development Practices

### Functional Programming

Prefer immutability, pure functions, functional core/imperative shell.

**See**: [repo-governance/development/pattern/functional-programming.md](./repo-governance/development/pattern/functional-programming.md)

### Implementation Workflow

Make it work → Make it right → Make it fast.

**See**: [repo-governance/development/workflow/implementation.md](./repo-governance/development/workflow/implementation.md)

### Test-Driven Development

Write the failing test first, then make it pass, then refactor — Red → Green → Refactor. Required for all code changes. Mini-TDD passes encouraged: split a feature into several small Red→Green→Refactor cycles. Plan delivery checklists must express code items as TDD-shaped steps; Gherkin acceptance criteria in `prd.md` are the natural source of first failing tests. Every code delivery step uses the explicit three-substep template (RED/GREEN/REFACTOR), each naming a file path, verbatim command, and acceptance criterion.

**See**: [repo-governance/development/workflow/test-driven-development.md](./repo-governance/development/workflow/test-driven-development.md)

### Specs & Gherkin Completeness (Both Paths)

Code under `apps/`/`libs/` never lands without its companion `specs/` Gherkin — **both** when behavior is changed directly and when a plan mediates it:

- **Direct change (no plan)**: edit app/lib code and add/update the matching `specs/apps/**` or `specs/libs/**` Gherkin in the **same commit/PR**. Enforced by `specs:coverage` + `swe-code-checker` (Step 6.6).
- **Planned change**: any plan touching `apps/`/`libs/`/`specs/` MUST carry delivery steps that add/update the companion Gherkin and run `specs:coverage`. `plan-maker` emits them; `plan-checker` (Step 5j) flags absence.

Pure refactors, no-behavior-change bumps, and docs/governance-only changes are exempt.

**See**: [repo-governance/development/quality/feature-change-completeness.md](./repo-governance/development/quality/feature-change-completeness.md)

### Reproducible Environments

Volta for Node.js/npm pinning, package-lock.json, .env.example.

**Hard iron rule — no secrets in committed files**: Never commit system secrets (keys, passwords, tokens, privileged usernames, certs, connection strings) to ANY git-tracked file — history is permanent. Real values live in uncommitted `.env*` (except `.env.example`) or gitignored files; committed files use placeholders/env-var references only. Binds agents and humans. See [Secrets and Env Standards](./repo-governance/conventions/security/secrets-and-env-standards.md).

**Guardrail**: Agents must not read/write/edit/commit real `.env*` files — only `.env.example` is permitted; scripts under `apps/`/`libs/`/`scripts/` are exempt. See [guard-env-file-access policy](./repo-governance/conventions/security/secrets-and-env-standards.md#9-guard-env-file-access-policy).

**See**: [repo-governance/development/workflow/reproducible-environments.md](./repo-governance/development/workflow/reproducible-environments.md)

### Dependency Bump Stability & Safety Policy

Every dependency bump follows a three-path tree — A (LTS latest patch), B (60-day soak + CVE-clean), C (security-override waiver) — exact pins only, CVE-clean across five sources (NVD, GitHub Advisories, Snyk, vendor pages, CISA KEV); CISA-KEV fast-track and EPSS ≥ 0.5 escalate to Path C. Full rules, cutoff recording, and waiver locations in the linked policy.

**See**: [repo-governance/development/workflow/dependency-bump-policy.md](./repo-governance/development/workflow/dependency-bump-policy.md)

### Agent Workflow Orchestration

Plan mode for non-trivial tasks (3+ steps or architecture decisions), delegated agents for focused subtasks, verify before done, autonomous bug fixing, self-improvement loop after corrections.

**Subagent concurrency**: When spawning background subagents via the Agent tool, cap at **3 concurrent** at any time (user may override for a specific batch). Poll output file mtime every **3 minutes**; if mtime unchanged for 30 minutes, call `TaskStop` and relaunch.

**See**: [repo-governance/development/agents/agent-workflow-orchestration.md](./repo-governance/development/agents/agent-workflow-orchestration.md), [Subagent Orchestration Convention](./repo-governance/development/agents/subagent-orchestration.md)

### Manual Verification & CI Blockers

- **Verify behavior**: Playwright MCP for UI, curl for API ([manual-behavioral-verification.md](./repo-governance/development/quality/manual-behavioral-verification.md))
- **CI blockers**: Investigate root cause, fix properly, never bypass ([ci-blocker-resolution.md](./repo-governance/development/quality/ci-blocker-resolution.md))
- **CI post-push verification**: After pushing app or lib code to `origin main`, trigger relevant GitHub CI workflows and verify they pass before declaring work done — pre-push hook alone is not sufficient ([ci-post-push-verification.md](./repo-governance/development/workflow/ci-post-push-verification.md))
- **CI monitoring**: Default poll interval is **3 minutes** — schedule a wake-up every 3 min and run one `gh run view --json status,conclusion` per wakeup. Never tight-loop poll. **Do not use `gh run watch`** (stream-watching is prohibited for CI monitoring). If rate-limited (HTTP 403): wait ~35 min before retrying ([ci-monitoring.md](./repo-governance/development/workflow/ci-monitoring.md))

## AI Agents

**Content Creation**: docs-maker, docs-tutorial-maker, readme-maker, specs-maker, apps-ayokoding-www-general-maker, apps-ayokoding-www-by-example-maker, apps-ayokoding-www-in-the-field-maker, apps-ose-www-content-maker, swe-ui-maker

**Validation**: docs-checker, docs-tutorial-checker, docs-link-checker, docs-software-engineering-separation-checker, readme-checker, specs-checker, apps-ayokoding-www-general-checker, apps-ayokoding-www-by-example-checker, apps-ayokoding-www-in-the-field-checker, apps-ayokoding-www-facts-checker, apps-ayokoding-www-link-checker, apps-ose-www-content-checker, swe-code-checker, swe-ui-checker, ci-checker, web-research-maker, repo-rules-checker, repo-workflow-checker, repo-harness-compatibility-checker

**Fixing**: docs-fixer, docs-tutorial-fixer, docs-software-engineering-separation-fixer, readme-fixer, specs-fixer, apps-ayokoding-www-general-fixer, apps-ayokoding-www-by-example-fixer, apps-ayokoding-www-in-the-field-fixer, apps-ayokoding-www-facts-fixer, apps-ayokoding-www-link-fixer, apps-ose-www-content-fixer, docs-file-manager, swe-ui-fixer, ci-fixer, repo-rules-fixer, repo-workflow-fixer, repo-harness-compatibility-fixer

**Planning**: plan-maker (grills user before and after plan creation using multiple-choice
options per [Grilling-With-Options Convention](./repo-governance/development/workflow/grilling-with-options.md);
delivery checklists: Phase 0 first, tag steps `[AI]`/`[HUMAN]`, gate each phase),
plan-checker, plan-execution-checker, plan-fixer,
repo-setup-manager (Phase 0 setup/baseline in every plan) (see
[plan-execution workflow](./repo-governance/workflows/plan/plan-execution.md) and
[plan-establishment workflow](./repo-governance/workflows/plan/plan-establishment-execution.md))

**Development**: swe-golang-dev, swe-typescript-dev, swe-e2e-dev, swe-csharp-dev, swe-fsharp-dev, swe-rust-dev

**Operations**: apps-ayokoding-www-deployer, apps-ose-www-deployer, apps-organiclever-www-deployer, apps-organiclever-app-web-deployer, apps-ose-app-web-deployer, apps-wahidyankf-www-deployer

**Content**: pdf-to-md-maker, pdf-to-md-checker, pdf-to-md-fixer

**Meta**: agent-maker, repo-rules-maker, repo-workflow-maker, repo-ose-primer-adoption-maker, repo-ose-primer-propagation-maker, social-linkedin-post-maker

**Maker-Checker-Fixer Pattern**: Three-stage workflow with criticality levels (CRITICAL/HIGH/MEDIUM/LOW), confidence assessment (HIGH/MEDIUM/FALSE_POSITIVE).

**Web Research Default**: `web-research-maker` is the default primitive for public-web information gathering. See [Web Research Delegation Convention](./repo-governance/conventions/writing/web-research-delegation.md) for delegation threshold and exceptions.

**Agent skills infrastructure**: Agents leverage agent skills providing two modes:

- **Inline skills** (default) — Inject knowledge into current conversation
- **Fork skills** (`context: fork`) — Trigger delegated agent spawning, delegate tasks to isolated agent contexts, return summarized results

Agent skills serve agents with knowledge and execution services but don't govern them (service relationship, not governance).

**Agent definition files** live in platform-binding directories. Agent skill files live in the per-binding skill search path and are read natively by the supported coding agent.

```binding-example
# Primary platform binding (Claude Code) layout
.claude/agents/<name>.md            # Agent definitions
.claude/skills/<name>/SKILL.md      # Agent skill files
```

**See**: [repo-governance/development/agents/ai-agents.md](./repo-governance/development/agents/ai-agents.md), [repo-governance/development/pattern/maker-checker-fixer.md](./repo-governance/development/pattern/maker-checker-fixer.md), [Agent Naming Convention](./repo-governance/conventions/structure/agent-naming.md), [Workflow Naming Convention](./repo-governance/conventions/structure/workflow-naming.md)

## Repository Architecture

Six-layer governance hierarchy:

- **Layer 0: Vision** — WHY we exist (democratize Shariah-compliant enterprise)
- **Layer 1: Principles** — WHY we value approaches
- **Layer 2: Conventions** — WHAT documentation rules
- **Layer 3: Development** — HOW we develop
- **Layer 4: AI Agents** — WHO enforces rules
- **Layer 5: Workflows** — WHEN we compose agents, procedures, and/or other workflows (multi-step orchestrated processes)

**Agent skills**: Delivery infrastructure (inline and fork modes) serving agents — not a governance layer. See AI Agents section above.

**See**: [repo-governance/repository-governance-architecture.md](./repo-governance/repository-governance-architecture.md)

## Web Sites

### ose-www

- **URL**: <https://oseplatform.com>
- **Production branch**: `prod-ose-www` → oseplatform.com
- **Framework**: Next.js 16 (App Router, TypeScript, tRPC)
- **Deployment**: Vercel
- **Content**: Public marketing website for the OSE Platform
- **Dev port**: 3100
- **E2E tests**: `ose-www-be-e2e`, `ose-www-fe-e2e`

**See**: [apps/ose-www/README.md](./apps/ose-www/README.md)

### ayokoding-www

- **URL**: <https://ayokoding.com>
- **Production branch**: `prod-ayokoding-www` → ayokoding.com
- **Framework**: Next.js 16 (App Router, TypeScript, tRPC)
- **Languages**: English (primary), Indonesian
- **Deployment**: Vercel
- **Content**: Educational platform (programming, AI, security)
- **Dev port**: 3101
- **E2E tests**: `ayokoding-www-be-e2e`, `ayokoding-www-fe-e2e`

**See**: [apps/ayokoding-www/README.md](./apps/ayokoding-www/README.md)

### organiclever-www

- **URL**: <https://www.organiclever.com/>
- **Production branch**: `prod-organiclever-www` → www.organiclever.com
- **Framework**: Next.js 16 (App Router)
- **Deployment**: Vercel
- **Content**: OrganicLever marketing website
- **Dev port**: 3200
- **E2E tests**: `organiclever-www-be-e2e`, `organiclever-www-fe-e2e`

**See**: [apps/organiclever-www/README.md](./apps/organiclever-www/README.md)

### organiclever-app-web

- **URL**: (app subdomain — TBD)
- **Framework**: Next.js 16 (App Router)
- **Deployment**: Vercel (TBD)
- **Content**: OrganicLever productivity tracker app frontend
- **Dev port**: 3202
- **E2E tests**: `organiclever-app-web-e2e`

**See**: [apps/organiclever-app-web/README.md](./apps/organiclever-app-web/README.md)

### wahidyankf-www

- **URL**: <https://www.wahidyankf.com/>
- **Production branch**: `prod-wahidyankf-www` → www.wahidyankf.com
- **Framework**: Next.js 16 (App Router)
- **Deployment**: Vercel
- **Content**: Personal portfolio (Home, CV, Personal Projects)
- **Dev port**: 3201
- **E2E tests**: `wahidyankf-www-fe-e2e`

**See**: [apps/wahidyankf-www/README.md](./apps/wahidyankf-www/README.md)

### ose-app-web

- **URL**: <https://app.oseplatform.com> (TBD — no Vercel project yet)
- **Production branch**: `prod-ose-app-web` (TBD)
- **Framework**: Next.js 16 (App Router)
- **Deployment**: Vercel (TBD)
- **Content**: OSE Application platform frontend — regulatory document upload, gap analysis, policy management
- **Backend**: `ose-be` at <https://api.oseplatform.com> (TBD)
- **Future**: `ose-app-mobile` (iOS/Android) will join this `ose-app-*` family
- **Dev port**: 3300
- **E2E tests**: `ose-app-web-e2e`

**See**: [apps/ose-app-web/README.md](./apps/ose-app-web/README.md)

### organiclever-be

- **Framework**: F# / Giraffe / ASP.NET 10 REST API
- **Deployment**: Kubernetes (staging/production)
- **Content**: Backend API for OrganicLever productivity tracker
- **Dev port**: 8202
- **E2E tests**: `organiclever-be-e2e`
- **Contract**: OpenAPI 3.1 spec at `specs/apps/organiclever/containers/contracts/`

## Temporary Files for AI Agents

AI agents use designated directories:

- **`generated-reports/`**: Validation/audit reports (Write + Bash tools required)
  - Pattern: `{agent-family}__{uuid-chain}__{YYYY-MM-DD--HH-MM}__audit.md`
  - Checkers MUST write progressive reports during execution
- **`local-temp/`**: Misc temporary files

**See**: [repo-governance/development/infra/temporary-files.md](./repo-governance/development/infra/temporary-files.md)

## Plans

Project planning in `plans/` folder:

- **ideas.md**: 1-3 liner ideas
- **backlog/**: Future plans
- **in-progress/**: Active work
- **done/**: Completed plans

**Folder naming** (stage-aware):

- `backlog/` — `YYYY-MM-DD__[project-identifier]/` (creation date prefix)
- `in-progress/` — `[project-identifier]/` (no date prefix; strip it when moving from backlog)
- `done/` — `YYYY-MM-DD__[project-identifier]/` (completion date prefix; add it when archiving)

**See**: [repo-governance/conventions/structure/plans.md](./repo-governance/conventions/structure/plans.md)

## Important Notes

- **Never commit secrets** (hard iron rule): No system secret goes into any git-tracked file; real values belong in uncommitted `.env*` files (except `.env.example`) or other gitignored files. See [Secrets and Env Standards](./repo-governance/conventions/security/secrets-and-env-standards.md).
- **Do NOT stage or commit** unless explicitly instructed. Per-request commits one-time only.
- **License**: MIT. See [LICENSING-NOTICE.md](./LICENSING-NOTICE.md)
- **Agent invocation**: Use natural language to invoke agents/workflows
- **Token budget**: Don't worry about token limits — reliable compaction available
- **No time estimates**: Never give time estimates. Focus on what needs doing, not how long.

## Related Documentation

- **Conventions Index**: [repo-governance/conventions/README.md](./repo-governance/conventions/README.md) — Documentation writing and org standards
- **Development Index**: [repo-governance/development/README.md](./repo-governance/development/README.md) — Software dev practices and workflows
- **Principles Index**: [repo-governance/principles/README.md](./repo-governance/principles/README.md) — Foundational values governing all layers
- **Primary Binding Agents Index**: [agent catalog](./.claude/agents/README.md) — Specialized agents organized by role
- **Workflows Index**: [repo-governance/workflows/README.md](./repo-governance/workflows/README.md) — Orchestrated processes
- **Repository Architecture**: [repo-governance/repository-governance-architecture.md](./repo-governance/repository-governance-architecture.md) — Six-layer governance hierarchy

## Related Repositories

The `open-sharia-enterprise` ecosystem consists of three independent sibling repositories — no parent coordination repo exists:

- [`ose-public`](https://github.com/wahidyankf/ose-public) — this repository. Open-source enterprise platform. MIT licensed.
- [`ose-primer`](https://github.com/wahidyankf/ose-primer) — downstream public template packaging the scaffolding layer (governance, AI agents, skills, conventions, CI harness, polyglot demo apps) for teams building their own Sharia-compliant enterprise products. MIT licensed.
- [`ose-infra`](https://github.com/wahidyankf/ose-infra) — private infrastructure repository. Hosts the self-hosted GitHub Actions runner stack, `coralpolyp` app, and infrastructure-only governance. Proprietary; not publicly accessible.

`ose-public` is the **upstream source of truth** for scaffolding. Content flows bidirectionally between `ose-public` and `ose-primer` via `repo-ose-primer-propagation-maker` (upstream → template, via draft PR or direct push to main — caller's choice per run, neither default) and `repo-ose-primer-adoption-maker` (downstream → upstream, direct commits to `main`). `ose-infra` does not participate in the sync loop.

See: [Related Repositories reference](./docs/reference/related-repositories.md), [ose-primer sync convention](./repo-governance/conventions/structure/ose-primer-sync.md).

## Models

This repo describes model selection by capability tier, not by vendor product name:

- **Planning-grade**: highest capability, used for complex multi-step planning tasks
- **Execution-grade**: strong capability, used for standard coding and review tasks
- **Fast**: lower latency, used for simple/fast tasks

Concrete vendor model IDs resolve in each platform binding's agent definition files (see the Platform Binding Examples section near the end of this file for the canonical layout).

See [repo-governance/development/agents/model-selection.md](./repo-governance/development/agents/model-selection.md) for the capability tier definitions and how they map to agent roles.

## General Guidelines for Working with Nx

- For navigating/exploring the workspace, invoke the `nx-workspace` skill first — it has patterns for querying projects, targets, and dependencies
- When running tasks (build, lint, test, e2e, etc.), prefer running through `nx` (`nx run`, `nx run-many`, `nx affected`) instead of underlying tooling directly
- Prefix nx commands with the workspace package manager (e.g., `pnpm nx build`, `npm exec nx test`) — avoids using globally installed CLI
- You have access to the Nx MCP server and its tools; use them
- For Nx plugin best practices, check `node_modules/@nx/<plugin>/PLUGIN.md`. Not all plugins have this file — proceed without it if unavailable.
- NEVER guess CLI flags — check nx_docs or `--help` first when unsure

## Scaffolding & Generators

- For scaffolding tasks (creating apps, libs, project structure, setup), ALWAYS invoke the `nx-generate` skill FIRST before exploring or calling MCP tools

## When to use nx_docs

- USE for: advanced config options, unfamiliar flags, migration guides, plugin config, edge cases
- DON'T USE for: basic generator syntax (`nx g @nx/react:app`), standard commands, things you already know
- The `nx-generate` skill handles generator discovery internally — don't call nx_docs just to look up generator syntax

## Platform Binding Examples

The content under this heading is intentionally vendor-specific. Per the
[Governance Vendor-Independence Convention](./repo-governance/conventions/structure/governance-vendor-independence.md),
the vendor-audit scanner skips every line under a "Platform Binding Examples"
heading until the next same-level heading or end of file.

### Platform Bindings Catalog

Concrete tool integrations live **outside** `repo-governance/` in platform-binding directories:

- **Claude Code** → `.claude/`, with `CLAUDE.md` as the Claude-Code-discoverable shim importing this file
- **OpenCode** → `.opencode/agents/` (auto-synced from `.claude/`); reads this file (`AGENTS.md`) natively; reads agent skill files at `.claude/skills/<name>/SKILL.md` natively
- **OpenAI Codex CLI** → reads `AGENTS.md` natively (`.codex/config.toml` present)
- **GitHub Copilot, Cursor, Windsurf, JetBrains Junie, Google Antigravity CLI, Pi** → read root `AGENTS.md` natively (Tier-1); no per-tool instruction file shipped by default (see no-shadowing rule)
- **Amazon Q Developer** → does not read `AGENTS.md` natively; receives a generated bridge under `.amazonq/` (`rules/00-agents-md.md` + a default agent config), emitted by `rhino-cli agents emit-bindings`
- **Aider** → reads `CONVENTIONS.md` natively per Aider's own docs (<https://aider.chat/docs/usage/conventions.html>); the agents.md standard site lists Aider as a supported tool but Aider's own documentation does not document AGENTS.md specifically
- **Future**: `CONVENTIONS.md` (Aider)

See [docs/reference/platform-bindings.md](./docs/reference/platform-bindings.md) for the full catalog of binding directories, root instruction files, and mechanical translation artifacts. The two-tier binding model and no-shadowing rule are defined in [repo-governance/conventions/structure/multi-harness-binding.md](./repo-governance/conventions/structure/multi-harness-binding.md).

### Concrete Vendor Model IDs

Concrete vendor model IDs live in each platform binding's agent definition files (e.g., `.claude/agents/<name>.md` frontmatter for the primary platform binding).
