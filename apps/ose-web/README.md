# ose-web

Official website for the **Open Sharia Enterprise** platform — an open-source Sharia-compliant
enterprise solutions platform built in the open.

**Why This Matters**: Islamic finance is a multi-trillion dollar industry, but most
Sharia-compliant enterprise solutions are proprietary and expensive. We're building an
open-source alternative with Sharia-compliance at its core — not bolted on as an afterthought.

**What This Site Does**: Showcases the platform and shares our journey. Regular updates keep the
community informed as we build with radical transparency.

## Architecture

- **Framework**: Next.js 16 (App Router, React Server Components)
- **Language**: TypeScript (strict mode)
- **API**: tRPC for type-safe server-client communication (runs inside the Next.js process)
- **Content**: Markdown with YAML frontmatter from `content/` (~6 pages: Landing, About, updates)
- **Styling**: Tailwind CSS v4 + shadcn/ui
- **Search**: FlexSearch for full-text search
- **Diagrams**: Mermaid diagram support
- **Testing**: Vitest (unit + integration), 86% line coverage enforced via rhino-cli
- **Structure**: Hexagonal feature modules under `src/contexts/<feature>/` with three layers

The codebase follows the
[hexagonal architecture for web applications](../../repo-governance/development/pattern/hexagonal-architecture-web.md)
pattern, organized into three layers per feature module:

- **application** — use cases, business logic, and ports (interfaces to the outside world)
- **infrastructure** — adapters implementing the ports (file system, HTTP, database)
- **presentation** — UI components and tRPC route handlers that face the client

## Quick Start

```bash
# Development server (port 3100)
nx dev ose-web

# Production build
nx build ose-web

# Typecheck
nx run ose-web:typecheck

# Lint (oxlint)
nx run ose-web:lint

# Unit tests + coverage + links
nx run ose-web:test:quick

# Integration tests
nx run ose-web:test:integration

# Spec coverage (both web + api perspectives)
nx run ose-web:spec-coverage
```

## Project Structure

```
ose-web/
├── src/
│   ├── app/            # Next.js App Router routes (thin glue, imports from contexts/)
│   ├── contexts/       # Feature modules (one folder per feature)
│   │   ├── app-shell/  # Site chrome + root tRPC router
│   │   ├── landing/    # Marketing landing page
│   │   ├── content/    # Content retrieval + rendering
│   │   ├── search/     # Search backend + UI
│   │   ├── rss-feed/   # RSS feed generation
│   │   ├── seo/        # Sitemap + metadata
│   │   └── health/     # Health probe + status page
│   └── lib/            # Cross-cutting utilities (tRPC infra, cn)
├── test/               # Test files (Vitest unit + integration)
├── content/            # Markdown pages with YAML frontmatter
└── project.json        # Nx project configuration
```

## Specs

Spec tree: `specs/apps/ose/`.

| Section                                                                               | What it contains                                 |
| ------------------------------------------------------------------------------------- | ------------------------------------------------ |
| [system-context/](../../specs/apps/ose/system-context/)                               | C4 L1 — actors, external systems                 |
| [containers/](../../specs/apps/ose/containers/)                                       | C4 L2 — single `web` container                   |
| [components/web/](../../specs/apps/ose/components/platform-web/)                      | C4 L3 — UI perspective                           |
| [components/api/](../../specs/apps/ose/components/platform-be/)                       | C4 L3 — tRPC HTTP perspective                    |
| [behavior/platform-web/gherkin/](../../specs/apps/ose/behavior/platform-web/gherkin/) | UI-semantic Gherkin (web perspective)            |
| [behavior/platform-be/gherkin/](../../specs/apps/ose/behavior/platform-be/gherkin/)   | tRPC HTTP-semantic Gherkin (backend perspective) |

## Deployment

Deployed to Vercel via production branch `prod-ose-web`.

- **Production**: <https://oseplatform.com>
- **Deploy**: Push `main` to `prod-ose-web`; Vercel builds automatically

```bash
git push origin main:prod-ose-web
```

## Related

- [Specs root](../../specs/apps/ose/README.md)
- [Main Repository](https://github.com/wahidyankf/ose-public)
- [apps-ose-web-deployer](../../.claude/agents/) — AI agent for deployments
