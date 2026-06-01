---
title: "No Secrets in Git Convention"
description: A hard iron rule that no system secret (SSH/private keys, passwords, API tokens, privileged usernames, certificates, connection strings, and similar) may ever be placed in any git-tracked file in this repository. Real secret values belong in uncommitted .env* files (except .env.example) or other gitignored files.
category: explanation
subcategory: conventions
tags:
  - security
  - secrets
  - git
  - data-protection
created: 2026-06-01
---

# No Secrets in Git Convention

**This is a hard iron rule.** No system secret may ever be written into any file that is committed
to git in this repository — not into plans, not into docs, not into governance files, not into source
code, configuration, agent definitions, READMEs, or anything else under version control.

A secret committed to git is compromised the moment it lands in history. Git history is permanent and
distributed: once a secret is pushed, it lives in every clone, every fork, every mirror, and every
backup, and removing it requires a destructive history rewrite that never fully guarantees the secret
was not already harvested. The only safe posture is to never let a secret enter version control in the
first place. When a real secret value is genuinely needed, it goes in an uncommitted `.env*` file
(except `.env.example`) or another gitignored file.

This convention is the broad governing principle. The
[Environment File Access Convention](./env-file-access.md) (`guard-env-file-access`) is one concrete
enforcement mechanism that implements part of it by blocking agents from reading, writing, or
committing real `.env*` files.

## Principles Implemented/Respected

- **[Reproducibility First](../../principles/software-engineering/reproducibility.md)**: Keeping
  secrets out of version control is a precondition for reproducible, auditable builds. Configuration
  is declared by shape (`.env.example`, documented variable names) while real values stay external,
  so a checkout is reproducible without embedding machine- or environment-specific credentials.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: The
  rule is an unambiguous deny, not a heuristic. Committed files use explicit placeholders and
  environment-variable references; real values live in explicitly external, gitignored locations.
- **[Documentation First](../../principles/content/documentation-first.md)**: The rule is codified
  here as governance so it is discoverable and binding regardless of which agent platform or human
  contributor performs the work.
- **[Root Cause Orientation](../../principles/general/root-cause-orientation.md)**: The robust fix is
  to prevent secrets from ever entering history (root cause), not to scrub them afterward (symptom).
  History rewrites are a last-resort remediation, never the primary control.

## Purpose

This convention protects the repository and the wider ecosystem from secret-disclosure events. It
exists to make a single rule unmistakable to every contributor — human or AI agent: **never commit a
secret**. It covers what counts as a secret, where real secret values are allowed to live, how to
represent secrets safely inside committed files, and what to do when a secret leaks anyway.

## Scope

### What This Convention Covers

- The hard rule that no secret enters any git-tracked file in this repository, across all paths.
- A working definition of what counts as a "system secret".
- The allowed homes for real secret values (uncommitted `.env*` files, gitignored files).
- How to represent secrets safely inside committed files (placeholders, env-var references).
- Remediation steps when a secret is leaked into a tracked file or history.
- The relationship between this convention and `guard-env-file-access`.

### What This Convention Does NOT Cover

- The mechanics of agent `.env*` file access — see the
  [Environment File Access Convention](./env-file-access.md).
- Server-side or CI-based secret scanning and leak detection (a complementary, separate concern).
- The internal syntax of platform-binding configuration files (see the respective binding
  directories, per the [Governance Vendor-Independence Convention](../structure/governance-vendor-independence.md)).
- How to structure `.env.example` content (out of scope here).

## Standards

### The Rule

Never place a real system secret in any file committed to git in this repository. This applies to
every tracked path without exception, including but not limited to:

- `plans/` (all plan documents — BRD, PRD, tech-docs, delivery, READMEs, notes)
- `docs/` (tutorials, how-to, reference, explanation)
- `repo-governance/` (principles, conventions, development, workflows, vision)
- Source code, tests, and fixtures under `apps/`, `libs/`, `apps-labs/`
- Configuration files (`*.json`, `*.yaml`, `*.toml`, `*.ini`, and similar)
- AI agent definitions, agent skills, and platform-binding files (`.claude/`, `.opencode/`, `.amazonq/`)
- `README.md`, `AGENTS.md`, `CLAUDE.md`, and any other root or nested markdown

The rule binds all actors equally — AI agents and human contributors alike.

### What Counts as a System Secret

Treat any of the following as a secret that must never be committed:

- **Keys and certificates**: SSH private keys, GPG/PGP private keys, TLS/SSL private keys, code-signing
  keys, any `BEGIN ... PRIVATE KEY` material, `.pem`/`.p12`/`.pfx` private key files.
- **Passwords and passphrases**: account passwords, database passwords, key passphrases, root/admin
  credentials.
- **Tokens and API credentials**: API keys, access tokens, refresh tokens, session tokens, bearer
  tokens, OAuth client secrets, personal access tokens (PATs), webhook signing secrets, CI/CD secrets.
- **Connection strings with embedded credentials**: e.g. `postgres://user:password@host/db`.
- **Privileged or identifying usernames and account identifiers** that, alone or combined, grant or
  narrow access — real admin usernames, service-account names, internal account IDs, tenant IDs tied
  to access. (Generic example names such as `alice`/`bob`/`user@example.com` are fine.)
- **Anything functionally equivalent** to the above: cloud provider credentials, SMTP credentials,
  encryption keys/seeds, JWT signing secrets, license keys that gate access.

If unsure whether a value is a secret, treat it as one.

### Where Real Secret Values Belong

When a real secret value is genuinely needed for local development or runtime, put it in an
**uncommitted** location:

- A `.env*` file **except** `.env.example` (e.g. `.env`, `.env.local`, `.env.production`). These are
  gitignored and managed manually by the human maintainer per the
  [Environment File Access Convention](./env-file-access.md).
- Another gitignored, uncommitted file (for example, scratch files under `local-temp/`).

`.env.example` is committed and therefore must contain **only placeholders**, never real values.

### Representing Secrets Safely in Committed Files

When a committed file must mention or demonstrate a secret, never use a real value. Instead:

- Reference an environment variable by name: `process.env.DATABASE_URL`, `${API_TOKEN}`.
- Use an obvious placeholder: `<YOUR_API_KEY>`, `xxxxxxxx`, `REDACTED`, `changeme`.
- Use clearly fake example values: `alice@example.com`, `password: "hunter2"` only in contexts where
  it is unmistakably illustrative and grants access to nothing real.
- In `.env.example`, document the variable name with a placeholder value and a comment.

### Remediation When a Secret Leaks

If a real secret reaches a tracked file or git history:

1. **Rotate first.** Treat the secret as compromised and rotate/revoke it immediately. Rotation is the
   only reliable remediation — scrubbing history does not un-leak an already-pushed secret.
2. **Remove it from the working tree** and replace with a placeholder or an env-var reference.
3. **Scrub history if not yet widely distributed** (e.g. `git filter-repo`), understanding this is a
   destructive rewrite and a containment step, not a substitute for rotation.
4. **Record the incident** so the leak path can be closed (e.g. add the file pattern to `.gitignore`,
   strengthen a guard).

## Examples

### Good Examples

```bash
# .env.example (committed) — placeholders only
DATABASE_URL="postgres://user:password@localhost:5432/dbname"
API_TOKEN="<your-api-token>"
```

```ts
// committed source — reference the variable, never the value
const token = process.env.API_TOKEN;
```

```markdown
<!-- committed plan/doc — placeholder, not a real value -->

Set `DEPLOY_SSH_KEY` in your local `.env` (gitignored). Never paste the key here.
```

### Bad Examples

```markdown
<!-- FAIL: real token pasted into a committed plan/doc -->

Use API token `sk-live-9f2c7a1e4b8d...` to call the endpoint.
```

```yaml
# FAIL: real password in a committed config file
db_password: "S3cr3t-Prod-Pass!"
```

```bash
# FAIL: real private key committed
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXk... (real key material)
-----END OPENSSH PRIVATE KEY-----
```

## Tools and Automation

- **`scripts/check-no-env-staged.sh`** (invoked from `.husky/pre-commit`) — rejects any commit that
  stages a real `.env*` file (anything except `.env.example`), catching `git add -f` force-adds.
- **Root `.gitignore`** — excludes real `.env*` files while force-unignoring `.env.example`.
- **Platform-binding `.env` guards** — see the
  [Environment File Access Convention](./env-file-access.md) for the per-platform enforcement paths.
- **`repo-rules-checker` / `repo-rules-fixer`** — validate that this convention is linked from the
  relevant indices and applied consistently across governance files.

## References

**Related Conventions:**

- [Environment File Access Convention](./env-file-access.md) — `guard-env-file-access`, the concrete
  mechanism that blocks agent access to and commits of real `.env*` files; one enforcement arm of
  this broader rule.
- [Governance Vendor-Independence Convention](../structure/governance-vendor-independence.md) — Why
  platform-binding enforcement syntax is referenced by path rather than embedded here.

**Repository Guidance:**

- [AGENTS.md](../../../AGENTS.md) — Project-wide guidance for AI agents and contributors.
- [Security Conventions Index](./README.md) — All security conventions.
- [Conventions Index](../README.md) — All conventions organized by category.

**Agents:**

- `repo-rules-checker` — Validates that this convention is linked from indices and applied across
  governance files.
- `repo-rules-fixer` — Applies corrections when `repo-rules-checker` identifies drift.
