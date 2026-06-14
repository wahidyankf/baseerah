# OSE — Container Diagrams (C4 L2)

## OSE Application (`ose-app-*`)

```mermaid
%% Color Palette: Blue #0173B2 (app), Purple #CC78BC (data), Brown #CA9161 (external)
%% All colors meet WCAG AA contrast standards and are color-blind friendly.

graph TD
    WEB["ose-app-web<br/>Next.js 16<br/>port 3300"]:::app
    BE["ose-be<br/>Rust/Axum<br/>port 8302"]:::app
    PG["PostgreSQL 17<br/>Documents + Gap Reports"]:::data
    OR["OpenRouter API<br/>LLM gateway"]:::external

    WEB -->|HTTP /api/v1/*| BE
    BE -->|sqlx| PG
    BE -->|reqwest| OR

    classDef app fill:#0173B2,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef data fill:#CC78BC,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef external fill:#CA9161,stroke:#000000,color:#FFFFFF,stroke-width:2px
```

| Container      | Technology             | Port | Purpose                                            |
| -------------- | ---------------------- | ---- | -------------------------------------------------- |
| `ose-app-web`  | Next.js 16, TypeScript | 3300 | Frontend SPA — document upload UI, gap report view |
| `ose-be`       | Rust/Axum              | 8302 | REST API — document ingestion, gap analysis engine |
| PostgreSQL 17  | Docker (dev), managed  | 5432 | Persistence for documents, policies, gap reports   |
| OpenRouter API | External HTTP API      | —    | LLM gateway for AI-assisted gap analysis           |

## OSE Platform Web (`ose-web`)

`ose-web` deploys as a **single container** named `web`. The tRPC API runs inside the same
Next.js process — there is no separate backend deployable.

```mermaid
%% Color Palette: Blue #0173B2 | Orange #DE8F05 | Teal #029E73 | Purple #CC78BC | Brown #CA9161 | Gray #808080
graph TD
    VISITOR("Visitor<br/>Desktop / Tablet / Mobile"):::actor
    AUTHOR("Content Author"):::actor_author

    subgraph SYSTEM["OSE Platform Web"]
        WEB["web container<br/>──────────────────<br/>Next.js App Router + tRPC<br/>Server Components, SSG<br/>MD pipeline, RSS, sitemap<br/>Client: search, theme, nav"]:::container

        CONTENT[("Content Directory<br/>──────────────────<br/>Markdown + YAML<br/>content/**/*.md")]:::datastore

        SEARCH["Search Index<br/>──────────────────<br/>FlexSearch (in-memory)"]:::search
    end

    CICD["CI Pipelines"]:::ci
    VERCEL["Vercel CDN<br/>Edge Network"]:::infra

    VISITOR -- browser --> WEB
    AUTHOR -- write markdown --> CONTENT
    WEB -- read markdown --> CONTENT
    WEB -- query --> SEARCH
    WEB -- build index from --> CONTENT
    WEB -- standalone deploy --> VERCEL
    CICD -- test --> WEB

    classDef actor fill:#DE8F05,stroke:#000000,color:#000000,stroke-width:2px
    classDef actor_author fill:#CA9161,stroke:#000000,color:#000000,stroke-width:2px
    classDef container fill:#0173B2,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef datastore fill:#029E73,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef search fill:#CC78BC,stroke:#000000,color:#000000,stroke-width:2px
    classDef infra fill:#808080,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef ci fill:#CC78BC,stroke:#000000,color:#000000,stroke-width:2px
```

| Container | Slug  | Tech                               | Hosting             | Behavior perspectives                          |
| --------- | ----- | ---------------------------------- | ------------------- | ---------------------------------------------- |
| Web       | `web` | Next.js 16 (App Router) + tRPC v11 | Vercel (standalone) | `platform-web` (UI), `platform-be` (tRPC HTTP) |

## Related

- **Context diagram**: [context.md](../system-context/context.md)
- **Parent**: [ose specs](../README.md)
