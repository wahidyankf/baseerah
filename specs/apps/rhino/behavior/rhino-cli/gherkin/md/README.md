# rhino — behavior/rhino-cli/gherkin/md

Gherkin scenarios for rhino-cli documentation validation commands.

Features in this domain:

- `docs-validate-frontmatter.feature` — frontmatter validation
- `docs-validate-heading-hierarchy.feature` — heading hierarchy validation
- `docs-validate-links.feature` — link validation
- `docs-validate-mermaid.feature` — Mermaid diagram validation
- `docs-validate-naming.feature` — file naming convention validation
- `repo-governance-frontmatter-audit.feature` — frontmatter-dates audit (`md frontmatter-dates validate`)
- `repo-governance-readme-index-audit.feature` — README index audit (`md readme-index validate`)

> This directory was renamed from `gherkin/docs/` to `gherkin/md/` (matching the `md` CLI command
> group), and the two `repo-governance-*` files above were split in from `gherkin/repo-governance/`
> (their content actually covers `md` commands, not `repo-governance`) during the Phase 1
> rename/split step of the `enforce-identical-rhino-cli-gherkin` plan. Feature file names still say
> `docs-*`/`repo-governance-*` for historical reasons — renaming the files themselves is a separate,
> later concern.

See [Specs Directory Structure Convention](../../../../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical purpose of this folder.
