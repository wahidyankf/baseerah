# 🔭 BeaverNest

✨ A personal operating layer — an AI assistant, a content builder, a posting helper, and a personal
workflow engine under one roof, for a single maintainer.

**BeaverNest** is a personal operating layer covering an AI assistant, a content builder, a posting
helper, and a personal workflow engine under one roof. See the [BeaverNest
Vision](./repo-governance/vision/beaver-nest.md) for the full "why".

🌳 **Ecosystem**: BeaverNest is a product **within** the [Open Sharia Enterprise
(OSE)](./repo-governance/vision/open-sharia-enterprise.md) ecosystem, not a replacement for it — it
inherits OSE's governance hierarchy, development practices, and AI agent fleet, while its own app
roster, agents, and identity are specific to this product. See the [Vision
Index](./repo-governance/vision/README.md) for the parent/child relationship.

## 🚧 Project Status

> ⚠️ **Walking skeleton** — a stateless F#/Giraffe backend (`beaver-nest-be`) and a Next.js frontend
> (`beaver-nest-fe`) proving the engineering harness end-to-end. No assistant, content-building, or
> posting capability exists yet; those are the deferred roadmap, not built claims.

See **[ROADMAP.md](./ROADMAP.md)** for complete development phases and strategy.

## 🚀 Getting Started

### 📋 Prerequisites

- **Node.js** 24.13.1 LTS & **npm** 11.10.1 (managed via [Volta](https://docs.volta.sh/guide/getting-started))

### 📥 Installation

```bash
npm install
```

## 🛠️ Tech Stack

**Guiding Principle**: Technologies that keep you free - open formats, portable data, no vendor lock-in.

- Node.js & npm (via Volta) — tooling and development infrastructure
- Golang/Rust — CLI tools ([rhino-cli](./apps/rhino-cli/))
- F# / Giraffe / ASP.NET 10 — `beaver-nest-be` (planned, Phase 6)
- Next.js 16 + TypeScript — `beaver-nest-fe` (planned, Phase 8)

See **[ROADMAP.md](./ROADMAP.md)** for complete tech stack evolution across all phases.

## 📂 Project Structure

This project uses **Nx** to manage applications and libraries:

```
beaver-nest/
├── apps/                  # Deployable applications (Nx monorepo)
├── libs/                  # Reusable libraries (Nx monorepo, flat structure)
├── docs/                  # Project documentation (Diataxis framework)
│   ├── tutorials/         # Learning-oriented guides
│   ├── how-to/            # Problem-oriented guides
│   ├── reference/         # Technical reference
│   └── explanation/       # Conceptual documentation
├── plans/                 # Project planning documents
│   ├── in-progress/       # Active project plans
│   ├── backlog/           # Planned projects for future
│   └── done/              # Completed and archived plans
├── nx.json                # Nx workspace configuration
├── tsconfig.base.json     # Base TypeScript configuration
├── package.json           # Project manifest with npm workspaces
└── README.md              # This file
```

**Applications** (`apps/`):

- **CLI tools**: [`rhino-cli`](./apps/rhino-cli/) — a fork of the OSE ecosystem tool, not bound by
  the upstream byte-identity rule
- **`beaver-nest-be`, `beaver-nest-fe`**: planned walking-skeleton apps, not yet scaffolded (see
  [ROADMAP.md](./ROADMAP.md))
- **Polyglot demo apps**: extracted 2026-04-18 to the downstream [`ose-primer`](https://github.com/wahidyankf/ose-primer) template repository (Go, Java, Elixir, F#, Python, Rust, Kotlin, TypeScript, C#, Clojure backends + Next.js, TanStack Start, Flutter Web frontends).

**Libraries** (`libs/`): [`rust-commons`](./libs/rust-commons/), [`web-ui`](./libs/web-ui/),
[`web-ui-token`](./libs/web-ui-token/)

**Learn More**: [Monorepo Structure Reference](./docs/reference/monorepo-structure.md) | [How to Add New App](./docs/how-to/add-new-app.md) | [How to Add New Library](./docs/how-to/add-new-lib.md) | [How to Run Nx Commands](./docs/how-to/run-nx-commands.md)

## 💻 Development

**Code Quality**: Automated checks run on every commit (Prettier formatting, Commitlint validation, markdown linting).

**Common Commands**:

```bash
npm run build                    # Build all projects
npm run test                     # Run tests
npm run lint                     # Lint code
nx dev [app-name]                # Start development server
nx build [app-name]              # Build specific project
nx affected -t build             # Build only affected projects
nx affected -t test:quick        # Run fast quality gate for affected projects
nx graph                         # Visualize dependencies
```

See [Code Quality](./repo-governance/development/quality/code.md) and [Commit Messages](./repo-governance/development/workflow/commit-messages.md) for details.

## 📊 CI & Test Coverage

All projects enforce ≥90% test coverage as part of `test:quick`.

**Quality gates**: pre-commit hooks (formatting, linting), pre-push hooks (`typecheck`, `lint`, `test:quick` for affected projects), and [PR quality gate](./.github/workflows/pr-quality-gate.yml).

- [`rhino-cli`](./apps/rhino-cli/)

For polyglot demo app CI badges, see the [`ose-primer`](https://github.com/wahidyankf/ose-primer) repository.

## 📚 Documentation

Organized using the [Diátaxis framework](https://diataxis.fr/): [Tutorials](./docs/tutorials/) (learning), [How-To](./docs/how-to/) (problem-solving), [Reference](./docs/reference/) (lookup), [Explanation](./docs/explanation/) (understanding).

See [`docs/README.md`](./docs/README.md) for details.

## 🔗 Related Repositories

`beaver-nest` is one of four sibling repositories in the Open Sharia Enterprise (OSE) family. The
four repositories cross-reference each other directly — no parent container repository exists.

- **[`ose-public`](https://github.com/wahidyankf/ose-public)** — MIT, public. Main OSE platform
  monorepo and upstream source of truth for governance, conventions, and scaffolding. The original
  upstream of the scaffolding `beaver-nest` was built from.
- **[`ose-primer`](https://github.com/wahidyankf/ose-primer)** — MIT, public. Downstream public
  template (governance, AI agents, skills, conventions, CI harness). Authoritative home of the
  polyglot demo apps extracted from this lineage on 2026-04-18.
- **[`ose-private`](https://github.com/wahidyankf/ose-private)** — Proprietary, private. The
  unexposed surface of OSE: self-hosted CI runner stack and the `coralpolyp` app. Not publicly
  accessible.
- **[`beaver-nest`](https://github.com/wahidyankf/beaver-nest)** — MIT, public. This repository.

BeaverNest is a full member of the family and a **fourth repository standing outside the three-repo
OSE parity loop** (`ose-public`, `ose-primer`, `ose-private`). It scaffolded from that ecosystem but
does not participate in cross-repo parity syncs, and its `apps/rhino-cli` is a fork not bound by the
loop's byte-identity rule. See
[Related Repositories reference](./docs/reference/related-repositories.md) for the full catalogue.

## 🎯 Motivation

Personal productivity and content work today is scattered across disconnected tools — a chat
assistant here, a note-taking app there, a separate posting workflow for each platform, no shared
memory or workflow engine tying any of it together. BeaverNest exists to give one person a coherent,
self-owned operating layer for assistant work, content building, and posting, instead of stitching
together someone else's SaaS tools.

**What We Believe:**

- 🔓 **Transparency builds trust** — open source code, self-owned over rented SaaS
- 🤖 **AI-assisted development** — systematic use of AI tools to enhance productivity and code quality
- 🛡️ **Security and governance from day one** — architectural foundations, not afterthoughts
- 🏗️ **Long-term foundation over quick wins** — building solid foundations for a life-long project

For complete principles, see [repo-governance/principles/](./repo-governance/principles/README.md).

## 🤝 Contributing

🔒 **Contributions are currently closed** while the architecture and patterns stabilize.

## 📜 License

This repository is licensed under the **[MIT License](./LICENSE)**. All code, documentation,
governance materials, specifications, and AI agent configuration are MIT-licensed — free to use,
fork, modify, and distribute for any purpose.

See [LICENSING-NOTICE.md](./LICENSING-NOTICE.md) for full details |
[LICENSE](./LICENSE) for the root license text |
[Licensing Convention](./repo-governance/conventions/structure/licensing.md) for internal rules.
