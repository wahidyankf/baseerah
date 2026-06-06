# OSE Platform Web Specs

Platform-agnostic specifications for `ose-web` — the OSE Platform marketing and updates
site. Single deployable container (`web`, Next.js 16 + tRPC v11 on Vercel) with two behaviour
perspectives (`web` UI-semantic and `api` tRPC HTTP-semantic). The `ose-cli` content
link validator has its own legacy spec layout (`cli/gherkin/`) preserved unchanged.

## Structure

```
specs/apps/ose-platform/
├── README.md              # This file
├── product/               # Product framing (above C4)
│   └── README.md
├── system-context/        # C4 L1 — actors and external systems
│   ├── README.md
│   └── context.md
├── containers/            # C4 L2 — single deployable (web)
│   ├── README.md
│   └── container.md
├── components/            # C4 L3 — per-perspective internals
│   ├── README.md
│   ├── api/               # tRPC HTTP perspective component specs
│   │   ├── README.md
│   │   └── component-api.md
│   └── web/               # UI perspective component specs
│       ├── README.md
│       └── component-web.md
├── ddd/                   # DDD artifacts (registry, glossaries, BC map)
│   ├── README.md
│   ├── bounded-contexts.yaml
│   ├── bounded-context-map.md
│   └── ubiquitous-language/
│       ├── README.md
│       └── *.md           # One glossary file per bounded context
├── behavior/              # Gherkin scenarios (web UI + api tRPC HTTP)
│   ├── README.md
│   ├── api/gherkin/       # tRPC HTTP-semantic scenarios (per BC)
│   └── web/gherkin/       # UI-semantic scenarios (per BC)
└── cli/                   # ose-cli specs (legacy layout, out of scope)
    └── gherkin/
        └── links-check.feature
```

## Containers and behaviour perspectives

`ose-web` deploys as exactly **one container** (`web`). Behaviour is documented from
**two perspectives** because the tRPC API runs **inside** the same Next.js process — there is
no separate backend deployable. Perspective slugs are `web` (UI semantic) and `api` (tRPC HTTP
semantic). See [`containers/container.md`](./containers/container.md) for the slug-vs-container
note.

| Container | Perspective  | Background                 | Scenarios                                                 | Consumed by                 |
| --------- | ------------ | -------------------------- | --------------------------------------------------------- | --------------------------- |
| `web`     | `web` (UI)   | `Given the app is running` | [behavior/web/gherkin/](./behavior/web/gherkin/README.md) | `apps/ose-web` (Next.js 16) |
| `web`     | `api` (HTTP) | `Given the API is running` | [behavior/api/gherkin/](./behavior/api/gherkin/README.md) | `apps/ose-web-be-e2e`       |

## Bounded contexts

Counts are Gherkin features per perspective. `--` means no features in that perspective today.

| Bounded Context | `web` features | `api` features | Description                                                                  |
| --------------- | -------------- | -------------- | ---------------------------------------------------------------------------- |
| app-shell       | 4              | --             | Header, footer, theme toggle, navigation, responsive, accessibility chrome   |
| landing         | 1              | --             | Marketing landing page at `/`                                                |
| content         | --             | 1              | Content retrieval (tRPC procedures + DTOs + filesystem adapters + rendering) |
| search          | --             | 1              | Search backend (tRPC + index) + UI                                           |
| rss-feed        | --             | 1              | RSS 2.0 feed generation route handler                                        |
| seo             | --             | 1              | Sitemap, robots, per-route metadata                                          |
| health          | --             | 1              | Health probe (tRPC) + system-status diagnostic page (if present)             |

## tRPC procedures

| Procedure             | BC      | Description                               |
| --------------------- | ------- | ----------------------------------------- |
| `content.getBySlug`   | content | Retrieve a page by slug (HTML + metadata) |
| `content.listUpdates` | content | List all update posts sorted by date desc |
| `search.query`        | search  | Full-text search across content           |
| `health.check`        | health  | Service liveness check                    |

Route handlers (Next.js conventions, not tRPC):

| Path           | BC       | Output                          |
| -------------- | -------- | ------------------------------- |
| `/feed.xml`    | rss-feed | RSS 2.0 XML                     |
| `/sitemap.xml` | seo      | Sitemap (Next.js MetadataRoute) |
| `/robots.txt`  | seo      | Robots (Next.js MetadataRoute)  |

## DDD registry (`bounded-contexts.yaml`)

`bounded-contexts.yaml` is the machine-readable declaration of every bounded context in
`ose-web`. Two `rhino-cli ddd` subcommands read it to enforce structural and
vocabulary invariants automatically in `nx run ose-web:test:quick`.

### Schema

Each entry under `contexts:` declares:

| Field           | Meaning                                                                                                                   |
| --------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `name`          | Identifier — must match the folder name under `apps/ose-web/src/contexts/`                                                |
| `summary`       | One-paragraph human description                                                                                           |
| `layers`        | Ordered list of DDD layers that must exist as subfolders (subset per BC: `application`, `infrastructure`, `presentation`) |
| `code`          | Filesystem path to the context's implementation root                                                                      |
| `glossary`      | Path to the context's ubiquitous-language Markdown file                                                                   |
| `gherkin`       | Path to the context's Gherkin scenario directory                                                                          |
| `relationships` | List of inter-context relationships with `to`, `kind`, and `role`                                                         |

Layer subset is per-BC. Pure-presentation BCs (`app-shell`, `landing`) declare `[presentation]` only.
BCs without a UI (`rss-feed`) declare `[application, infrastructure]`.
BCs without dedicated infra (`seo`, `health`) declare `[application, presentation]`.
Full-stack BCs (`content`, `search`) declare `[application, infrastructure, presentation]`.

### `rhino-cli ddd bc ose-platform` — structural parity

Reads the registry and verifies the **filesystem** matches exactly:

- Every declared `code:` path exists with **exactly** the declared `layers:` subfolders
- Every declared `glossary:` file exists on disk
- Every declared `gherkin:` directory exists and contains ≥1 `.feature` file
- No **orphan** directories exist under `src/contexts/` that aren't in the registry
- Relationship declarations are symmetric across both context entries

### `rhino-cli ddd ul ose-platform` — glossary parity

Reads the registry to locate every `glossary:` file, then validates each:

- Required frontmatter keys present (`Bounded context`, `Maintainer`, `Last reviewed`)
- Terms table header matches canonical columns
- Code identifiers (backtick-wrapped in the table) exist somewhere in the declared `code:` path
- Feature file references in the table resolve to real `.feature` files under the declared `gherkin:` path
- Same term in two glossaries → both must carry mutual `Forbidden-synonyms` cross-links

## Spec artifacts

- **[system-context/](./system-context/README.md)**, **[containers/](./containers/README.md)**,
  **[components/](./components/README.md)** — C4 architecture diagrams (L1/L2/L3)
- **[components/api/](./components/api/README.md)** — tRPC HTTP perspective component specs
  ([Gherkin features](./behavior/api/gherkin/README.md))
- **[components/web/](./components/web/README.md)** — UI perspective component specs
  ([Gherkin features](./behavior/web/gherkin/README.md))

## Testing

| Suite           | App            | Scope                                  |
| --------------- | -------------- | -------------------------------------- |
| Unit tests      | ose-web        | Vitest, ≥ 80% line coverage            |
| api E2E         | ose-web-be-e2e | Playwright, tRPC API + route endpoints |
| web E2E         | ose-web-fe-e2e | Playwright, browser interactions       |
| Link validation | ose-cli        | Internal content link checks           |
| `ddd bc`        | rhino-cli      | Source ↔ registry structural parity    |
| `ddd ul`        | rhino-cli      | Registry ↔ glossary identifier parity  |

## Related

- [Three-Level Testing Standard](../../../repo-governance/development/quality/three-level-testing-standard.md)
- [BDD Standards](../../../docs/explanation/software-engineering/development/behavior-driven-development-bdd/README.md)
- [apps/ose-web/](../../../apps/ose-web/README.md) — Next.js implementation
- [apps/ose-cli/](../../../apps/ose-cli/README.md) — CLI tool (content link validation)
