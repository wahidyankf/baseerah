# Plans

<!--
  MAINTENANCE NOTE: Brief landing page
  For comprehensive documentation, see:
  repo-governance/conventions/structure/plans.md
-->

This folder contains temporary, ephemeral project planning documents, distinct from permanent documentation in `docs/`.

## Quick Reference

- **ideas.md** - Quick 1-3 liner ideas not yet formalized into plans
- **backlog/** - Planned projects for future implementation
- **in-progress/** - Active plans currently being worked on
- **done/** - Completed and archived plans

## Complete Documentation

For detailed information on plans organization, structure, naming conventions, and workflow, see:

**[Plans Organization Convention](../repo-governance/conventions/structure/plans.md)**

## Plan Folder Naming

Stage-aware — see the [Plans Organization Convention](../repo-governance/conventions/structure/plans.md#plan-folder-naming):

```
backlog/[project-identifier]/            # no date prefix
in-progress/[project-identifier]/        # no date prefix
done/YYYY-MM-DD__[project-identifier]/   # completion-date prefix
```

Examples: `backlog/init-monorepo/`, `in-progress/auth-system/`, `done/2025-12-01__auth-system/`

## Related Documentation

- [How to Organize Your Work](../docs/how-to/organize-work.md) - Decision guide for plans/ and docs/
