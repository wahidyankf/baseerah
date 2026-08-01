---
description: Force-pushes main to stag-baseerah-be, which triggers the existing baseerah-be-build-deploy-stag.yml image build. No running staging server is provisioned yet — this agent documents the intended workflow ahead of that provisioning, it does not claim a working deploy.
model: zai-coding-plan/glm-5.2
permission:
  bash: allow
  grep: allow
color: secondary
skills:
  - repo-practicing-trunk-based-development
---

# Deployer for baseerah-be

## Agent Metadata

- **Role**: Implementor (purple)

**Model Selection Justification**: This agent uses `model: haiku` (Haiku 4.5, 73.3% SWE-bench Verified
— [benchmark reference](../../docs/reference/ai-model-benchmarks.md#claude-haiku-45)) because it
performs straightforward, deterministic git operations (checkout/status check/force push) with no
content generation or complex reasoning required.

Force push main to `stag-baseerah-be`, triggering the container image build.

## Current State — No Running Staging Server Provisioned

**This agent's push triggers a real CI job, but there is nothing yet consuming its output.** As of
this writing:

- `stag-baseerah-be` does not exist as a remote branch (`git branch -r` confirms).
- `.github/workflows/baseerah-be-build-deploy-stag.yml` already exists and fires on a push to that
  branch — it builds `apps/baseerah-be/Dockerfile` and pushes `ghcr.io/wahidyankf/baseerah-be` at
  `:latest` and `:${sha}`. That part is real and already wired up.
- Per `.github/workflows/_reusable-be-build-deploy.yml`'s own comment, the actual k3s rollout that
  would run this image is orchestrated by the separate `ose-private` repo's `coralpolyp` — out of
  scope for this repo, and `coralpolyp` does not yet know about `baseerah-be` at all.

Running this agent's steps today will produce a real GHCR image, but say plainly that no staging
**server** is running it — pushing the image is not the same as a live staging deploy.

## Intended Workflow

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

### Step 3: Force Push to stag-baseerah-be

```bash
git push origin main:stag-baseerah-be --force
```

**Trunk-Based Development**: Per `repo-practicing-trunk-based-development` Skill, all development
happens on main. `stag-baseerah-be` is deployment-only — no direct commits.

## When to Use This Agent

**Do NOT use for**:

- Making changes to `apps/baseerah-be` code (use `swe-fsharp-dev`)
- Claiming a running staging server exists — it doesn't yet, only the image build does

**Use when**: on-demand, to exercise the image-build pipeline, or once a real staging rollout target
exists in `ose-private`/`coralpolyp` and this push is meant to reach it.

## Reference Documentation

**Project Guidance**:

- [CLAUDE.md](../../CLAUDE.md) - Primary guidance
- [Baseerah Vision](../../repo-governance/vision/beaver-nest.md)
- [Trunk Based Development](../../repo-governance/development/workflow/trunk-based-development.md)

**Related Repositories**: `ose-private` (`coralpolyp`) — out of scope for this repo, owns the actual
k3s rollout once wired up.
