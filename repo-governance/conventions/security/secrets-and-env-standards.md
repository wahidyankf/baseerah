---
title: "Secrets and Environment-Variable Standards"
description: The authoritative hub for how this repository handles secrets and environment variables — naming convention, layout, annotation format, startup validation, tooling (rhino-cli env family), storage tiers, and the env-contract drift guard.
category: explanation
subcategory: conventions
tags:
  - security
  - secrets
  - env-files
  - guard-env-file-access
  - naming
  - reproducibility
created: 2026-06-10
---

# Secrets and Environment-Variable Standards

This document is the single authoritative reference for how this repository handles secrets and
environment variables. The three prior docs that covered overlapping ground now redirect here:

- [`no-secrets-in-committed-files.md`](./no-secrets-in-committed-files.md) — hard iron rule stub
- [`env-file-access.md`](./env-file-access.md) — `guard-env-file-access` policy stub
- [`reproducible-environments.md`](../../../repo-governance/development/workflow/reproducible-environments.md) — `.env.example` pattern stub

## Principles Implemented/Respected

- **[Reproducibility First](../../principles/software-engineering/reproducibility.md)**: Env templates
  (`*.env.example`) are committed; real values stay in gitignored files. A checkout is reproducible
  by design — no credential is bundled.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Every
  env var is declared by name, class, and type in `.env.example`; startup validators fail fast when a
  required var is absent.
- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: The
  `rhino-cli env` toolchain (backup, restore, init, validate) and `env-contract.yaml` eliminate
  manual cross-checking between templates and code.
- **[Root Cause Orientation](../../principles/general/root-cause-orientation.md)**: The drift guard
  (`env validate`) catches mismatches at the source, not in production. The hard no-secrets rule
  prevents exposure at the origin — not just after-the-fact scrubbing.
- **[Documentation First](../../principles/content/documentation-first.md)**: Every rule is codified
  here so it is discoverable and binding regardless of which agent platform or human performs the work.

## 1. Hard Iron Rule — No Secrets in Committed Files

**No system secret may enter any git-tracked file in this repository.**

System secrets include: SSH/private keys, passwords, API tokens, privileged usernames, certificates,
connection strings, and any value that grants access to a system or service.

Git history is permanent and distributed. A secret committed once lives in every clone, fork, mirror,
and backup, and removing it requires a destructive history rewrite that never fully guarantees the
secret was not already harvested. The only safe posture is prevention.

Real secret values go in:

- Uncommitted `.env*` files (e.g. `.env.local`, `.env`) — gitignored globally
- Files under `.secrets/` — gitignored globally
- `secrets.json` at repo root — gitignored globally

See also: [`no-secrets-in-committed-files.md`](./no-secrets-in-committed-files.md)

### Cross-repo doc canonicalization

The cross-repo canonical name for this rule is `no-secrets-in-committed-files.md` (aligned with the
ose-infra sibling). This repository previously used `no-secrets-in-git.md`; the file was renamed by
the `standardize-secrets-and-env` plan to match the canonical name.

## 2. Environment Variable Naming Standard

### Variable classes

| Class                      | Rule                                        | Example                                           |
| -------------------------- | ------------------------------------------- | ------------------------------------------------- |
| App-defined value          | `SCREAMING_SNAKE`, per-app prefix           | `ORGANICLEVER_BE_PORT`, `OSE_BE_OPENROUTER_MODEL` |
| Framework-reserved         | Keep the framework's required name          | `NEXT_PUBLIC_*`, Next.js `PORT`                   |
| Shared service connection  | Unprefixed, conventional name               | `DATABASE_URL`                                    |
| Environment tier in a name | **Forbidden** (keys identical across tiers) | not `PROD_DATABASE_URL`                           |

The **per-app prefix** is the app's Nx project name upcased with `_` separators: `ose-be` →
`OSE_BE_`, `ose-www` → `OSE_WWW_`.

### Framework-reserved exempt names

| Name            | Why exempt                                                                    |
| --------------- | ----------------------------------------------------------------------------- |
| `NEXT_PUBLIC_*` | Framework-required (Next.js browser-exposure prefix)                          |
| `PORT`          | Platform convention (host/PaaS injects it) — **webs only**                    |
| `NODE_ENV`      | Node reserved                                                                 |
| `DATABASE_URL`  | Cross-ecosystem convention; prefixing breaks every tool that reads it by name |
| `HOSTNAME`      | Platform convention for Next.js dev server                                    |

**Critical asymmetry**: The **Next.js dev server** reads `PORT` natively — renaming it to
`OSE_WWW_PORT` would break `nx dev ose-www`. Rust **backend** ports are app-defined code, so they
**do** take the prefix (`ORGANICLEVER_BE_PORT`, `OSE_BE_PORT`). This is the single most
error-prone point of the naming standard.

## 3. Layout Standard — One Template per App

Each app's env template lives in exactly one place: `apps/<app>/.env.example`.

- **Rust backends**: template lives at `apps/<app>/.env.example` (where `Cargo.toml` lives).
- **Next.js webs**: template lives at `apps/<app>/.env.example` (where `next.config.*` lives). Next.js
  auto-loads `.env.local` from this directory; the `.env.example` is a documentation file only —
  never auto-loaded by Next.js or Nx.
- **Duplication is forbidden**: no second template for the same app under `infra/dev/` or elsewhere.

Relocating real gitignored `.env*` files (`.env.local` etc.) is a **[HUMAN]** task — the
`guard-env-file-access` policy forbids agents from touching them directly.

## 4. `.env.example` Annotation Format

Every env var line is preceded by a comment block:

```
# REQUIRED | <type> | <description>
# Format: <format note>
KEY=obviously-dev-placeholder

# OPTIONAL | <type> | <description> (default: <value>)
# OPTIONAL_KEY=
```

Rules:

- `REQUIRED` or `OPTIONAL` (no other values).
- Type is the runtime type: `string`, `u16`, `boolean`, `url`.
- Description is one short phrase; format notes go on a second `# Format:` line.
- **Required vars**: active line with an obviously-dev placeholder value (never a real secret).
- **Optional vars**: commented-out line (`# KEY=`), so the template is parseable without forcing
  developers to set non-required vars.
- Placeholders must be obviously fake: `postgres://postgres:postgres@localhost:5432/appname` is
  obviously local; `your-api-key-here` is obviously a placeholder.

## 5. Startup Validation

### Rust backends — `dotenvy` + `envy`

```rust
#[derive(serde::Deserialize)]
pub struct Config {
    pub database_url: String,               // required; no default
    #[serde(default = "default_port")]
    pub organiclever_be_port: u16,          // optional; typed default
}

impl Config {
    pub fn load() -> Result<Self, envy::Error> {
        dotenvy::dotenv().ok();             // no-op in CI; loads .env.local locally
        envy::from_env::<Config>()
    }
}
```

- `envy` maps struct field `organiclever_be_port` ↔ env var `ORGANICLEVER_BE_PORT` automatically.
- Required fields are non-`Option`, no `#[serde(default)]` — a missing value is a typed error at
  startup naming the field.
- Deps: `dotenvy = "0.15.7"` (exact pin, successor to the unmaintained `dotenv` RUSTSEC-2021-0141),
  `envy = "0.4.2"` (exact pin; last release Jan 2021; advisory-clean; narrow scope).

### TypeScript webs — `@t3-oss/env-nextjs` + `zod`

```typescript
// apps/<app>/src/env.ts
import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";
export const env = createEnv({
  server: {
    OSE_WEB_CONTENT_DIR: z.string().optional(),
    OSE_WEB_SHOW_DRAFTS: z.string().optional(),
  },
  experimental__runtimeEnv: {},
});
```

```typescript
// apps/<app>/next.config.ts — import triggers build-time validation
import "./src/env";
```

- `t3-env` validates at **build time** — a missing required var fails `nx build`, not at runtime.
- `NEXT_PUBLIC_*` client vars are enforced by t3-env's TypeScript types — a client var without the
  prefix is a compile error.
- Deps: `@t3-oss/env-nextjs` (exact pin, `0.12.0`), `zod` (exact pin, `4.0.5`).

## 6. `rhino-cli env` Toolchain

The full `rhino-cli env` family manages the local secrets lifecycle:

| Command                             | What it does                                                         |
| ----------------------------------- | -------------------------------------------------------------------- |
| `rhino-cli env backup [--dry-run]`  | Copies all secret files to `~/<repo-name>-env-backup/`               |
| `rhino-cli env restore [--dry-run]` | Restores from the backup directory                                   |
| `rhino-cli env init`                | Scaffolds `.env.local` from every `apps/<app>/.env.example` template |
| `rhino-cli env validate`            | Checks each surface in `env-contract.yaml` for code↔template drift   |

### Backup scope — hybrid floor + registry

Backup coverage = hardcoded floor ∪ `backup_globs` from `env-contract.yaml`:

| Pattern                      | Status                                                  |
| ---------------------------- | ------------------------------------------------------- |
| `.env`, `.env.*`             | active                                                  |
| Everything under `.secrets/` | active                                                  |
| `secrets.json` at repo root  | active                                                  |
| `*.tfvars`, `*.tfvars.json`  | commented forward-scaffold — activate when IaC is added |
| Generated inventories        | commented forward-scaffold — activate when IaC is added |

The default backup target is `~/<repo-root-basename>-env-backup/` (e.g. `~/ose-public-env-backup/`).
This is the canonical per-repo backup directory aligned across the ose-public/ose-primer/ose-infra
sibling repos.

### `env-contract.yaml` and drift validation

`env-contract.yaml` at repo root declares each surface to validate:

```yaml
surfaces:
  - root: apps/organiclever-be
    kind: app
    lang: rust
    allowlist: []
  - root: apps/ose-www
    kind: app
    lang: typescript
    allowlist: [PORT, HOSTNAME]
  # Terraform/Ansible: forward-scaffold — activate when IaC is added
```

`rhino-cli env validate` compares declared keys in `.env.example` against read keys in source code,
reporting `declared-but-unread` (stale template entry) and `read-but-undeclared` (undocumented read)
drift findings. Invoked by `.husky/pre-push` and `.github/workflows/validate-env.yml`.

## 7. Secret-Surface Census

| Surface             | Path                          | Backing tool      | Backed up          | Validated            |
| ------------------- | ----------------------------- | ----------------- | ------------------ | -------------------- |
| App env file        | `apps/<app>/.env.local`       | dotenvy / Next.js | Yes (floor)        | Yes (`env validate`) |
| Blessed secrets dir | `.secrets/`                   | manual            | Yes (floor)        | No                   |
| Root secrets blob   | `secrets.json`                | manual            | Yes (floor)        | No                   |
| Terraform vars      | `infra/terraform/**/*.tfvars` | Terraform         | Commented scaffold | Commented scaffold   |
| Ansible inventory   | `infra/ansible/**/inventory`  | Ansible           | Commented scaffold | Commented scaffold   |

Template files (`*.env.example`) are tracked in git — they are not secrets. Real gitignored files are
the backup target.

## 8. `guard-env-file-access` Policy

AI agents must not directly read, write, edit, or commit any `.env*` file except `.env.example`. The
canonical identifier for this policy is **`guard-env-file-access`**.

Exceptions: project scripts under `apps/`, `libs/`, and `scripts/` are exempt (they are part of the
app's own startup/setup logic, not AI-agent operations).

See also: [`env-file-access.md`](./env-file-access.md)

## 9. IaC Forward Scaffold

Terraform and Ansible surfaces are documented in `env-contract.yaml` as **commented forward-scaffold**
entries — syntactically present but inactive. Uncomment and fill in `root` when IaC surfaces are added
to the repository. This prevents the drift guard from producing false findings before IaC exists while
ensuring the pattern is immediately available when it does.

## Related Documents

- [`no-secrets-in-committed-files.md`](./no-secrets-in-committed-files.md) — hard iron rule (stub)
- [`env-file-access.md`](./env-file-access.md) — `guard-env-file-access` agent policy (stub)
- [`reproducible-environments.md`](../../../repo-governance/development/workflow/reproducible-environments.md) — environment setup (stub for env section)
- [`docs/explanation/standardize-secrets-and-env-parity-decisions.md`](../../../docs/explanation/standardize-secrets-and-env-parity-decisions.md) — cross-repo parity decisions
- [`env-contract.yaml`](../../../env-contract.yaml) — surface registry
