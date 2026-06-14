# .github

GitHub-specific configuration for the `open-sharia-enterprise` monorepo: the CI/CD
surface (GitHub Actions workflows and the composite actions they reuse).

This directory is **hand-authored CI only**. It carries no agent/skill binding
artifacts — the Nx Copilot artifacts that some tooling drops here (the `nx-*` agent
skills under `skills/`, and the Nx CI-monitor `agents/`/`prompts/` files) were removed.
The repo reads Nx skills via the `nx-mcp` plugin and monitors CI with the `gh` CLI per
its [CI-monitoring convention](../repo-governance/development/workflow/ci-monitoring.md).

## Contents

| Path         | Purpose                                                                                                                             |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------- |
| `workflows/` | GitHub Actions workflows — PR gates, scheduled test-and-deploy, image publishing. See [workflows/README.md](./workflows/README.md). |
| `actions/`   | Reusable composite actions for toolchain setup. See [actions/README.md](./actions/README.md).                                       |

## See also

- [CI Conventions](../repo-governance/development/infra/ci-conventions.md) — naming, reusable-workflow, and CRON-schedule conventions
- [CI/CD Pipeline reference](../docs/reference/system-architecture/ci-cd.md) — git hooks, workflows, and the Nx build system
- [GitHub Actions Workflow Naming](../repo-governance/development/infra/github-actions-workflow-naming.md)
- [AGENTS.md](../AGENTS.md) — canonical contributor instructions
