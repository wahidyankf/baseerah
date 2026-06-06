# wahidyankf-web — Gherkin Features

Audience: Engineers, Technical Product/Project Managers

UI-semantic Gherkin feature files for `wahidyankf-web`, organized by bounded context. Each
subfolder maps to one bounded context from the
bounded-context registry.

## Structure

```
specs/apps/wahidyankf/behavior/web/gherkin/
├── app-shell/
│   ├── accessibility.feature
│   ├── responsive.feature
│   └── theme.feature
├── cv/
│   └── cv.feature
├── home/
│   └── home.feature
├── personal-projects/
│   └── personal-projects.feature
└── search/
    └── search.feature
```

## Coverage

| Bounded Context     | Features                               | Count |
| ------------------- | -------------------------------------- | ----- |
| `app-shell`         | `accessibility`, `responsive`, `theme` | 3     |
| `cv`                | `cv`                                   | 1     |
| `home`              | `home`                                 | 1     |
| `personal-projects` | `personal-projects`                    | 1     |
| `search`            | `search`                               | 1     |
| **Total**           |                                        | **7** |

## Consumed by

| App                     | Level | Tool             |
| ----------------------- | ----- | ---------------- |
| `wahidyankf-web-fe-e2e` | E2E   | `playwright-bdd` |

## Related

- `../../../ddd/bounded-context-map.md` — context relationships
- `../../../ddd/ubiquitous-language/` — vocabulary
