---
title: Technology Stack
description: Technology stack summary, quality tools, and future architecture considerations
category: reference
tags:
  - architecture
  - technology
  - tooling
created: 2025-11-29
---

# Technology Stack

Technology stack summary, quality tools, and future architecture considerations for the Baseerah platform.

> **2026 Baseerah repo reset**: every prior application except `rhino-cli` was deleted. The
> Frontend and Backend sections below describe the **planned** `baseerah-fe`/`baseerah-be` stack
> (not yet scaffolded); the CLI Tools section describes the **current** state. See
> [applications.md](./applications.md).

## Technology Stack Summary

### Frontend (planned — `baseerah-fe` not yet scaffolded)

**Web Application** (Next.js):

- **Next.js**: 16 (App Router)
- **React**: 19
- **Styling**: TailwindCSS + Radix UI / shadcn-ui
- **Deployment**: Vercel (planned)
- **Applications**: `baseerah-fe` (planned, port 19310)

### Backend (planned — `baseerah-be` not yet scaffolded)

**REST API**:

- **Framework**: Likely Giraffe (ASP.NET Core) — framework TBD pending backend tech-stack decision
- **Language**: Likely F# (.NET 10)
- **Build**: dotnet via Nx (if F#)
- **Applications**: `baseerah-be` (planned, port 19320)

### CLI Tools (current)

**Rust CLI Tools**:

- **Language**: Rust (edition 2024, rust-version 1.88)
- **Build**: Cargo via Nx
- **Distribution**: Local binaries
- **Applications**: `rhino-cli` (Repository Hygiene & INtegration Orchestrator) — the sole
  surviving app after the 2026 Baseerah repo reset

### Infrastructure

- **Monorepo**: Nx workspace
- **Node.js**: 24.13.1 LTS (Volta-managed)
- **Package Manager**: npm 11.10.1
- **Git Workflow**: Trunk-Based Development
- **CI**: GitHub Actions
- **CD**: Vercel (planned, for `baseerah-fe` once scaffolded — no app is deployed today)

### Quality Tools

- **Formatting**: Prettier 3.6.2
- **Markdown Linting**: markdownlint-cli2 0.21.0
- **Link Validation**: rhino-cli md links validate (Rust)
- **Commit Linting**: Commitlint + Conventional Commits
- **Git Hooks**: Husky + lint-staged
- **Testing**: Nx test orchestration

## Future Architecture Considerations

### Future Additions

- **Shared Libraries**: TypeScript, Rust, F# libs in `libs/`
- **Additional Applications**: More domain-specific enterprise apps
- **Backend Services**: Sharia-compliant business logic services
- **Authentication Service**: Centralized auth for all applications
- **Observability Stack**:
  - Metrics: Prometheus + Grafana
  - Logging: ELK/Loki stack
  - Tracing: Jaeger/Tempo

### Scalability Considerations

- **Nx Cloud**: Distributed task execution and caching
- **CDN**: Static asset delivery optimization (Vercel, once `baseerah-fe` is deployed)
- **Additional Next.js Sites**: More specialized content platforms
- **CLI Tool Suite Expansion**: More specialized automation tools
- **Shared Rust Crates**: Common functionality across Rust CLI tools

## Related Documentation

- **Monorepo Structure**: [docs/reference/monorepo-structure.md](../monorepo-structure.md)
- **Adding New Apps**: [docs/how-to/add-new-app.md](../../how-to/add-new-app.md)
- **Git Workflow**: [repo-governance/development/workflow/commit-messages.md](../../../repo-governance/development/workflow/commit-messages.md)
- **Markdown Quality**: [repo-governance/development/quality/markdown.md](../../../repo-governance/development/quality/markdown.md)
- **Trunk-Based Development**: [repo-governance/development/workflow/trunk-based-development.md](../../../repo-governance/development/workflow/trunk-based-development.md)
- **Repository Architecture**: [repo-governance/repository-governance-architecture.md](../../../repo-governance/repository-governance-architecture.md)
