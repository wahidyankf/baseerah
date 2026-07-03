# rhino — behavior/rhino-cli/gherkin/harness

Gherkin scenarios for rhino-cli agent-harness management commands.

Features in this domain:

- `agents-bindings.feature` — emit and validate the Amazon Q Developer binding bridge
- `agents-detect-duplication.feature` — detect duplicate agent definitions
- `agents-sync.feature` — sync agent definitions across platform bindings
- `agents-validate-claude.feature` — validate Claude Code agent files
- `agents-validate-naming.feature` — validate agent naming conventions
- `repo-governance-agents-md-size.feature` — AGENTS.md size audit (`harness instruction-size validate`)
- `repo-governance-instruction-size-governance.feature` — instruction-size governance rule (`harness instruction-size validate`)
- `repo-governance-instruction-size-pre-push.feature` — instruction-size pre-push gate (`harness instruction-size validate`)
- `repo-governance-instruction-size.feature` — instruction-size threshold audit (`harness instruction-size validate`)

> This directory was renamed from `gherkin/agents/` to `gherkin/harness/` (matching the `harness`
> CLI command group), and the four `repo-governance-*` files above were split in from
> `gherkin/repo-governance/` (their content actually covers `harness instruction-size validate`,
> not `repo-governance`) during the Phase 1 rename/split step of the
> `enforce-identical-rhino-cli-gherkin` plan. Feature file names still say
> `agents-*`/`repo-governance-*` for historical reasons — renaming the files themselves is a
> separate, later concern.

See [Specs Directory Structure Convention](../../../../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical purpose of this folder.
