# Specs

Gherkin acceptance specifications for OSE Platform applications.

## What This Is

This directory holds executable specifications written in Gherkin — the shared language between
business stakeholders, developers, and QA engineers. These specs describe _what_ each app does,
not _how_ it is implemented.

## Why Specs Live Here

Acceptance specs belong at the monorepo root rather than inside app directories because:

- **Stakeholder access** — business owners and QA engineers read specs without navigating app internals
- **Shared ownership** — Three Amigos (business + development + QA) collectively own these files
- **Clear separation** — specs define behavior; implementation tests live inside the apps

## Testing Layers

| Layer                      | Location           | Purpose                               | When it runs            |
| -------------------------- | ------------------ | ------------------------------------- | ----------------------- |
| Acceptance specs (Gherkin) | `specs/`           | Define behavior from user perspective | CI full suite           |
| Unit / integration tests   | `apps/*/src/test/` | Verify internal implementation        | Pre-push (`test:quick`) |
| E2E tests                  | `apps/*-e2e/`      | Verify flows against running system   | CI E2E suite            |

## App Specs

- **[ayokoding](./apps/ayokoding/README.md)** — AyoKoding educational website specifications (Next.js 16, multilingual programming, AI, and security tutorials)
- **[crane](./apps/crane/README.md)** — crane-cli specifications (Content Retrieval And Normalization Engine CLI, Python/pytest-bdd)
- **[organiclever](./apps/organiclever/README.md)** — OrganicLever fullstack specifications (F#/Giraffe backend REST API + Next.js 16 frontend)
- **[ose-app](./apps/ose-app/README.md)** — OSE Application specifications (GRC platform, F#/Giraffe backend + Next.js 16 frontend)
- **[ose-platform](./apps/ose-platform/README.md)** — OSE Platform web specifications (Next.js 16 + tRPC marketing and updates site)
- **[rhino](./apps/rhino/README.md)** — rhino-cli specifications (Repository Hygiene and INtegration Orchestrator CLI, Rust)
- **[wahidyankf](./apps/wahidyankf/README.md)** — wahidyankf-web specifications (personal portfolio site, Next.js 16, static)

## Experimental App Specs

- **[apps-labs/](./apps-labs/README.md)** — Specs for framework evaluations, POCs, and tech stack
  comparisons; graduates to `apps/` when the implementation is promoted

## Library Specs

- **[golang-commons](./libs/golang-commons/)** — Shared Go utility specifications
- **[golang-link-commons](./libs/golang-link-commons/)** — Go link-checking utility specifications
- **[web-ui](./libs/web-ui/)** — Shared web UI component specifications

## Standard Folder Pattern

Each application domain follows this canonical five-folder layout under `specs/apps/{domain}/`:

```
specs/apps/{domain}/
├── README.md               # Describes app, BDD framework, and feature organization
├── product/                # Product framing above C4 (vision, personas, scope)
├── system-context/         # C4 L1 — actors and external systems
├── containers/             # C4 L2 — deployable units
│   └── contracts/          # OpenAPI 3.1 contract spec (bundled + source files)
├── components/             # C4 L3 — per-container or per-perspective internals
└── behavior/               # Gherkin acceptance scenarios
    └── <surface>/
        └── gherkin/
            └── <domain>/   # Domain subdirectory (required — no flat feature files)
                └── <feature>.feature
```

The `<surface>` segment identifies the execution context for the scenarios. Valid values are:

- `be` — HTTP-semantic backend surface (REST API, GraphQL, etc.)
- `web` — UI-semantic browser surface (Next.js, Flutter web, etc.)
- `cli` — command-line tool surface (Go, Rust, etc.)
- `api` — tRPC or other in-process API perspective (used when no separate backend container exists)
- `build-tools` — build-time tooling or index-generation scripts

Domain subdirectories are required under `behavior/<surface>/gherkin/`. Feature files must
not sit directly under the `gherkin/` directory.

**Contracts** live at `specs/apps/{domain}/containers/contracts/` and are the source of truth
for API contracts shared between frontend and backend. The `{domain}-contracts` Nx project
lints and bundles the spec; downstream apps consume it via their `codegen` target.

**C4 diagrams** live in `system-context/`, `containers/`, and `components/` and describe the
system architecture at the context (L1), container (L2), and component (L3) levels.

## Standards

All feature files follow the OSE Platform BDD standards:

- [BDD Standards](../docs/explanation/software-engineering/development/behavior-driven-development-bdd/README.md) —
  framework requirements, Three Amigos process, coverage rules
- [Gherkin Standards](../docs/explanation/software-engineering/development/behavior-driven-development-bdd/gherkin-standards.md) —
  feature file structure, naming, ubiquitous language
- [Scenario Standards](../docs/explanation/software-engineering/development/behavior-driven-development-bdd/scenario-standards.md) —
  scenario independence, naming, assertions
- [Spec-to-Test Mapping](../repo-governance/development/infra/bdd-spec-test-mapping.md) —
  mandatory 1:1 mapping between CLI commands and feature file `@tags`

## Adding Specs

1. Choose the appropriate subdirectory: `specs/apps/` for production-bound applications,
   `specs/apps-labs/` for experimental/POC applications, `specs/libs/` for libraries
2. Create a folder matching the project name: `specs/apps/[app-name]/` or `specs/libs/[lib-name]/`
3. Add a `README.md` describing the project, BDD framework, and feature file organization
4. Organize `.feature` files by bounded context or user journey (kebab-case names)
5. Update this README with a link to the new folder
