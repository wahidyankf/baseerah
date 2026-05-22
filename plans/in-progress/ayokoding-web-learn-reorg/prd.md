# PRD — Learn-Tree Reorganization

## Functional Requirements

### FR-1: Canonical Leaf-Topic Shape

Every leaf topic under `apps/ayokoding-web/content/en/learn/<domain>/<area>/<topic>/` MUST have:

- `_index.md` — Hugo-style index (or its Next.js equivalent) with frontmatter and table of contents
- `overview.md` — short orientation page (target ~150-400 words)
- Zero or more of: `by-concept/`, `by-example/`, `in-the-field/`

A leaf topic MUST NOT contain any other track-style folder. Specifically: `concepts/`, `explanation/`, `foundations/`, `cases/`, `tools/` (when nested inside a topic), and any other ad-hoc folder are forbidden at the leaf level.

### FR-2: Allowed Track Vocabulary

The canonical three tracks correspond to the existing maker skills:

| Folder          | Skill                                   | Purpose                                  |
| --------------- | --------------------------------------- | ---------------------------------------- |
| `by-concept/`   | `apps-ayokoding-web-by-concept-maker`   | Mental models and narrative explanations |
| `by-example/`   | `apps-ayokoding-web-by-example-maker`   | Annotated code-first walk-throughs       |
| `in-the-field/` | `apps-ayokoding-web-in-the-field-maker` | Production-grade implementation guides   |

No other track folder is permitted at any depth inside `learn/`. Content currently filed under non-canonical track folders is folded into the appropriate canonical track or, if it is genuinely cross-track narrative, lifted into `overview.md`.

### FR-3: Folder-Name Hygiene

- No prefix-based fake hierarchy: `platform-linux/` → `platforms/linux/`, `platform-web/` → `platforms/web/`, `platform-mobile/` → `platforms/mobile/`.
- Grammatically consistent plurals where current form is mixed: `algorithm-and-data-structures/` → `algorithms-and-data-structures/`.
- Every directory name is kebab-case `[a-z0-9-]+` per the [File Naming Convention](../../../repo-governance/conventions/structure/file-naming.md). Existing names already comply; the rule is restated to lock it for new directories created during the reorg.

### FR-4: Area-Level Consolidation Decisions

The plan makes two consolidation decisions and one rename decision:

- **`software-architecture/` ↔ `system-design/`**: kept as siblings (status quo) but their `overview.md` files must cross-link and explicitly state the split. Working split: `software-architecture/` = code-shape patterns (DDD, hexagonal, FSM, C4); `system-design/` = whiteboard scaling and case studies. The `cases/` folder in each is folded into that domain's `by-example/cases/` track.
- **`information-security/foundations/` and `information-security/concepts/`**: folded into `information-security/by-concept/` for cross-cutting foundational material and `information-security/<area>/by-concept/` for area-specific. The `explanation/` folder (Diátaxis-style label) is collapsed into `by-concept/`.
- **`human/` → `personal-development/`**: rename. Domain content scope (CliftonStrengths and adjacent self-development) is clearer under the new name.

### FR-5: Redirect Coverage

Every URL renamed by this plan MUST resolve to its new location via HTTP 301 in production. Redirects live in the Next.js config (`next.config.ts`) or a dedicated redirect-map file imported by it. The plan ships with a complete `old-url, new-url` table (see [`tech-docs.md`](./tech-docs.md) §Redirect Map).

### FR-6: Validation Gates

The reorg is not considered complete until all of the following pass against the worktree branch:

- `ayokoding-cli links check --content apps/ayokoding-web/content` reports zero broken links
- `node --import tsx apps/ayokoding-web/src/scripts/generate-indexes.ts --validate` reports zero stale indexes
- `nx run ayokoding-web:test:quick` passes (line coverage ≥ 82%)
- `nx affected -t typecheck lint test:quick spec-coverage` (the pre-push hook surface) passes
- Spot-check: `curl -I https://ayokoding.com/<one-randomly-chosen-renamed-url>` returns 301 with the new URL in `Location`

### FR-7: Worktree Discipline

Per the [Subrepo Worktree Workflow Convention](../../../../repo-governance/conventions/structure/subrepo-worktrees.md) Standard 11 and the [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) (ose-public override), all work for this plan runs in `worktrees/ayokoding-web-learn-reorg/` at the repo root. Direct edits to the main checkout are forbidden during execution.

## Non-Functional Requirements

### NFR-1: Reversibility

Each phase's commits are independently revertable. A bad rename in phase 3 must not require unwinding phases 1-2. This implies: one phase per commit-or-commit-group, no cross-phase file moves.

### NFR-2: Preserve Git History

All file moves use `git mv` (or equivalent that preserves rename detection). Reviewers running `git log --follow <new-path>` must reach the file's original creation commit. Verified by sampling 5 randomly chosen renamed files at the end of each phase.

### NFR-3: SEO Continuity

Redirects deploy in the same Vercel build as the renames. The window where an inbound link goes to a 404 is bounded by Vercel build time (~2-5 minutes), not by a separate redirect-rollout step.

### NFR-4: Documentation Co-Movement

Any `repo-governance/` or `docs/explanation/` page that references a renamed path MUST be updated in the same phase that does the rename. The plan does not allow `governance docs say X, content tree says Y` as an interim state.

## Acceptance Criteria (Gherkin)

```gherkin
Feature: Learn-tree leaf-topic shape

  Scenario: Every leaf topic exposes only canonical tracks
    Given the working tree at apps/ayokoding-web/content/en/learn
    When I list every leaf directory containing at least one .md file other than _index.md and overview.md
    Then each such directory contains only sub-directories named "by-concept", "by-example", or "in-the-field"
    And no leaf directory contains a sub-directory named "concepts", "explanation", "foundations", "cases", or "tools"

  Scenario: Platforms hierarchy is real, not prefix-namespaced
    Given the working tree at apps/ayokoding-web/content/en/learn/software-engineering
    When I list directories matching pattern "platform-*"
    Then the list is empty
    And the directory "platforms" exists with sub-directories "linux", "web", and "mobile"

  Scenario: Human domain renamed to personal-development
    Given the working tree at apps/ayokoding-web/content/en/learn
    Then the directory "human" does not exist
    And the directory "personal-development" exists and contains the content previously under "human"

  Scenario: Information-security tracks normalized
    Given the working tree at apps/ayokoding-web/content/en/learn/information-security
    When I search recursively for directories named "concepts", "explanation", or "foundations"
    Then no results are returned

  Scenario: Internal links resolve
    Given the working tree at apps/ayokoding-web
    When I run "../../apps/ayokoding-cli/dist/ayokoding-cli links check --content content"
    Then the exit code is 0
    And the report says "Broken:   0 link(s)"

  Scenario: Generated indexes are current
    Given the working tree at apps/ayokoding-web
    When I run "node --import tsx src/scripts/generate-indexes.ts --validate"
    Then the exit code is 0

  Scenario: Pre-push gate passes
    Given the working tree on branch worktree-ayokoding-web-learn-reorg
    When I run "nx affected -t typecheck lint test:quick spec-coverage"
    Then the exit code is 0

  Scenario: Renamed URLs serve 301 with new Location
    Given the Vercel build for branch prod-ayokoding-web is deployed
    When I run "curl -I https://ayokoding.com/en/learn/software-engineering/platform-web"
    Then the response status is 301
    And the Location header contains "/en/learn/software-engineering/platforms/web"

  Scenario: Renamed paths preserve git history
    Given the working tree on branch worktree-ayokoding-web-learn-reorg after all phases complete
    When I pick five randomly chosen files under their new paths
    And for each I run "git log --follow --format=%H -- <new-path> | tail -1"
    Then each result is the same SHA as "git log --format=%H -- <old-path> | tail -1" before the reorg

  Scenario: No governance doc references a removed path
    Given the working tree on branch worktree-ayokoding-web-learn-reorg after all phases complete
    When I grep -r "platform-linux\|platform-web\|platform-mobile\|/concepts/explanation\|/foundations/by-example\|/en/learn/human" repo-governance docs apps/ayokoding-web
    Then no occurrence remains except inside the redirect map and this plan folder

  Scenario: Coverage threshold maintained
    Given the test run for ayokoding-web:test:quick
    Then line coverage is at least 82%
```

## Out of Scope (Restated)

- Content authoring inside any track folder.
- Indonesian content tree (`content/id/`).
- Changes to the three-track maker/checker/fixer agent skills.
- Changes to top-level domain count.
- `ose-web`, `wahidyankf-web`, `organiclever-web` content trees.
