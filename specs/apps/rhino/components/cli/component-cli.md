# rhino-cli — CLI Component

See [README.md](./README.md) for C4 L3 CLI internals.

## docs validate-links flags

| Flag                 | Type              | Description                                                                                                                                                                 |
| -------------------- | ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--staged-only`      | bool              | Only validate files staged in the Git index.                                                                                                                                |
| `--exclude <prefix>` | repeatable string | Skip any markdown file whose repo-relative path starts with `<prefix>`. May be specified multiple times (e.g. `--exclude plans/done --exclude apps/ayokoding-web/content`). |

Anchor fragments are validated with a GitHub-correct slug algorithm (verified
against the `github-slugger` v2 reference): lowercase; Unicode letters/digits,
underscores, hyphens, and spaces are KEPT; everything else is stripped; spaces
become hyphens with no collapsing; duplicate slugs get `-1`, `-2`, … suffixes in
document order.

## docs validate-mermaid flags

| Flag                   | Type              | Description                                                                                                             |
| ---------------------- | ----------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `--staged-only`        | bool              | Only validate files staged in the Git index.                                                                            |
| `--changed-only`       | bool              | Only validate files changed since upstream.                                                                             |
| `--max-label-len <n>`  | int (default 30)  | Max characters per node-label line.                                                                                     |
| `--max-width <n>`      | int (default 4)   | Max nodes sharing one rank.                                                                                             |
| `--max-depth <n>`      | int (default 0)   | Depth threshold for the both-exceeded warning (0 = unlimited).                                                          |
| `--max-subgraph-nodes` | int (default 6)   | Max direct child nodes per subgraph before a density warning.                                                           |
| `--exclude <prefix>`   | repeatable string | Skip any markdown file whose repo-relative path starts with `<prefix>`. May be specified multiple times.                |
| positional paths       | strings           | Optional explicit scan roots. With no positional paths the scan is **repo-wide** minus the standardized noise-skip set. |

The flowchart parser handles pipe-labeled edges (`A -->|text| B`) and cyclic
diagrams (back edges are removed via DFS before longest-path ranking, so a
cycle ranks as its underlying chain instead of collapsing every node to rank 0).

## docs validate-heading-hierarchy flags

| Flag                 | Type              | Description                                                                         |
| -------------------- | ----------------- | ----------------------------------------------------------------------------------- |
| `--exclude <prefix>` | repeatable string | Skip any allowlisted markdown file whose repo-relative path starts with `<prefix>`. |
| positional paths     | strings           | Optional explicit scan roots (override the default allowlisted repo scan).          |

The prose allowlist (default-deny) admits: `docs/`, `repo-governance/`,
`plans/` (minus `plans/done/`), `specs/`, root-level `*.md`,
`apps/*/README.md`, `libs/*/README.md`, `apps/*/docs/**`, and `libs/*/docs/**`.
