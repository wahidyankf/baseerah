# OrganicLever — Behavior

Audience: Engineers, Technical Product/Project Managers

Behavior specifications for OrganicLever — Gherkin scenarios that exercise the product
through both the backend HTTP surface and the frontend UI surface. Sliced by surface so
each project can wire its step implementations against the right glob.

## Children

- `organiclever-be/` — Backend Gherkin scenarios (HTTP semantic).
- `organiclever-app-web/` — Frontend Gherkin scenarios (UI semantic).

## Surfaces

One row per product-surface. Each surface dir named `<product>-<perspective>` per the
flat product-surface convention.

| Surface                | Perspective                             | Background                 | Consumed by                              |
| ---------------------- | --------------------------------------- | -------------------------- | ---------------------------------------- |
| `organiclever-be`      | HTTP-semantic (GET, POST, status codes) | `Given the API is running` | `apps/organiclever-be` (Rust/Axum)       |
| `organiclever-app-web` | UI-semantic (clicks, types, sees)       | `Given the app is running` | `apps/organiclever-app-web` (Next.js 16) |

## Gherkin coverage

### `organiclever-be/gherkin/`

| Domain | Feature                       | Scenarios |
| ------ | ----------------------------- | --------- |
| health | `health/health-check.feature` | 2         |

### `organiclever-app-web/gherkin/`

Organized by bounded context (one folder per context, matching the
[DDD registry](../ddd/bounded-contexts.yaml)).

| Bounded Context | Features                                       | Count  |
| --------------- | ---------------------------------------------- | ------ |
| app-shell       | `accessibility`, `entry-loggers`, `navigation` | 3      |
| health          | `system-status-be`                             | 1      |
| journal         | `home-screen`, `journal-mechanism`             | 2      |
| landing         | `landing`                                      | 1      |
| routine         | `routine-management`                           | 1      |
| routing         | `app-routes`, `disabled-routes`                | 2      |
| settings        | `dark-mode`, `language`, `settings-screen`     | 3      |
| stats           | `history-screen`, `progress-screen`            | 2      |
| workout-session | `workout-session`                              | 1      |
| **Total**       |                                                | **16** |

## Related

- [`../components/`](../components/README.md) — C4 L3 components that the scenarios exercise
- [`../containers/contracts/`](../containers/contracts/README.md) — OpenAPI contract the
  backend scenarios assert against (moved from legacy `contracts/` in Phase 2A.7)
