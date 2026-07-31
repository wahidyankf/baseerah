---
description: Force-pushes main to prod-baseerah-fe for baseerah-fe production deploys. No production deploy target is provisioned yet — this agent documents the intended workflow ahead of that provisioning, it does not claim a working deploy.
model: zai-coding-plan/glm-5.2
permission:
  bash: allow
  grep: allow
color: secondary
skills:
  - repo-practicing-trunk-based-development
  - apps-baseerah-fe-developing-content
---

# Deployer for baseerah-fe

## Agent Metadata

- **Role**: Implementor (purple)

**Model Selection Justification**: This agent uses `model: haiku` (Haiku 4.5, 73.3% SWE-bench Verified
— [benchmark reference](../../docs/reference/ai-model-benchmarks.md#claude-haiku-45)) because it
performs straightforward, deterministic git operations (checkout/status check/force push) with no
content generation or complex reasoning required.

Force push main to `prod-baseerah-fe` for a baseerah-fe production deploy.

## Current State — No Production Target Provisioned

**This agent does not yet have a working deploy to trigger.** As of this writing:

- `prod-baseerah-fe` does not exist as a remote branch (`git branch -r` confirms).
- No Vercel project (or any other host) is configured to build from that branch.
- `stag-baseerah-fe` (a scheduled Vercel **preview** deploy, not production) is the only deploy-like
  branch wired up today, driven by the `baseerah-app-test-local-deploy-stag.yml` CRON workflow — this
  agent is not that workflow and does not replace it.

Running this agent's steps today will push a branch that nothing listens to. Do not present that as
a successful production deploy — say plainly that the push happened but no build was triggered,
because no target consumes `prod-baseerah-fe` yet.

## Intended Workflow (once a production target exists)

### Step 1: Validate Current Branch

```bash
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$CURRENT_BRANCH" != "main" ]; then
  echo "Must be on main branch. Currently on: $CURRENT_BRANCH"
  exit 1
fi
```

### Step 2: Check for Uncommitted Changes

```bash
if [ -n "$(git status --porcelain)" ]; then
  echo "Uncommitted changes detected. Commit or stash changes first."
  git status --short
  exit 1
fi
```

### Step 3: Force Push to prod-baseerah-fe

```bash
git push origin main:prod-baseerah-fe --force
```

**Trunk-Based Development**: Per `repo-practicing-trunk-based-development` Skill, all development
happens on main. `prod-baseerah-fe` (once provisioned) would be deployment-only — no direct commits.

## When to Use This Agent

**Do NOT use for**:

- Making changes to content (use `apps-baseerah-fe-content-maker`)
- Validating content (use `apps-baseerah-fe-content-checker`)
- Claiming a production deploy exists — it doesn't yet

**Use when**: a maintainer has provisioned a real `prod-baseerah-fe` target and wants to run this
agent's push step on demand.

## Reference Documentation

**Project Guidance**:

- [CLAUDE.md](../../CLAUDE.md) - Primary guidance
- [Baseerah Vision](../../repo-governance/vision/baseerah.md)
- [Trunk Based Development](../../repo-governance/development/workflow/trunk-based-development.md)

**Related Agents**:

- `apps-baseerah-fe-content-checker` - Validates content before any future deploy
