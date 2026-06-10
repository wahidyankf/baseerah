# Tech Docs — Standardize Secrets and Environment-Variable Storage

This document holds the design decisions, their rationale, the file-impact analysis, the full
cross-repo deviation matrix, and the mechanics. The step-by-step checklist lives in
[delivery.md](./delivery.md).

## 1. Naming Standard

### Decision

| Variable class             | Rule                                        | Example                                               |
| -------------------------- | ------------------------------------------- | ----------------------------------------------------- |
| App-defined value          | `SCREAMING_SNAKE`, per-app prefix           | `ORGANICLEVER_BE_PORT`, `OSE_APP_BE_OPENROUTER_MODEL` |
| Framework-reserved value   | Keep the framework's required name          | `NEXT_PUBLIC_*`, Next.js `PORT`                       |
| Shared service connection  | Unprefixed, conventional name               | `DATABASE_URL`                                        |
| Environment tier in a name | **Forbidden** (keys identical across tiers) | not `PROD_DATABASE_URL`                               |

The per-app prefix is the app's Nx project name upcased with `_` separators (`ose-app-be` →
`OSE_APP_BE_`, `ose-web` → `OSE_WEB_`). It prevents collisions when one process loads multiple apps'
vars and makes a variable's owner obvious at a glance.

**On 12-factor authority (precise framing).** The Twelve-Factor App is **silent on naming
structure**: it mandates config-in-environment and per-deploy values, but prescribes nothing about
prefixes or casing. So 12-factor **authorizes** a per-app prefix without **prescribing** it; the
prefix is a **practitioner-consensus** convention for shared environments where many services' vars
coexist, not a 12-factor requirement. Distinct from app-defined names is a **framework-reserved /
exempt class** that this standard never prefixes:

| Reserved/exempt name | Why exempt                                           |
| -------------------- | ---------------------------------------------------- |
| `NEXT_PUBLIC_*`      | Framework-required (Next.js browser-exposure prefix) |
| `PORT`               | Platform convention (host/PaaS injects it)           |
| `NODE_ENV`           | Node reserved                                        |
| `DATABASE_URL`       | Cross-ecosystem convention, intentionally unprefixed |

### Per-app rename map (full surface) [Repo-grounded]

Derived from the live env-read survey (§ 6). `DATABASE_URL`, framework `PORT`, and `NEXT_PUBLIC_*`
are exempt.

| App                | Current key                | New key                          | Notes                                         |
| ------------------ | -------------------------- | -------------------------------- | --------------------------------------------- |
| `organiclever-be`  | `PORT`                     | `ORGANICLEVER_BE_PORT`           | app-defined backend port (not Next.js `PORT`) |
| `organiclever-be`  | `CORS_ORIGINS`             | `ORGANICLEVER_BE_CORS_ORIGINS`   |                                               |
| `organiclever-be`  | `DATABASE_URL`             | `DATABASE_URL`                   | **exempt** — shared conventional name         |
| `ose-app-be`       | `PORT`                     | `OSE_APP_BE_PORT`                |                                               |
| `ose-app-be`       | `CORS_ORIGINS`             | `OSE_APP_BE_CORS_ORIGINS`        |                                               |
| `ose-app-be`       | `OPENROUTER_API_KEY`       | `OSE_APP_BE_OPENROUTER_API_KEY`  |                                               |
| `ose-app-be`       | `OPENROUTER_MODEL`         | `OSE_APP_BE_OPENROUTER_MODEL`    |                                               |
| `ose-app-be`       | `OPENROUTER_BASE_URL`      | `OSE_APP_BE_OPENROUTER_BASE_URL` |                                               |
| `ose-app-be`       | `DATABASE_URL`             | `DATABASE_URL`                   | **exempt**                                    |
| `organiclever-web` | `ORGANICLEVER_BE_URL`      | `ORGANICLEVER_BE_URL`            | already conforming — confirm only             |
| `ose-app-web`      | `OSE_APP_BE_URL` (compose) | `OSE_APP_BE_URL`                 | already conforming — confirm only             |
| `ose-web`          | `CONTENT_DIR`              | `OSE_WEB_CONTENT_DIR`            |                                               |
| `ose-web`          | `SHOW_DRAFTS`              | `OSE_WEB_SHOW_DRAFTS`            |                                               |
| `ose-web`          | `PORT`                     | `PORT`                           | **exempt** — Next.js framework var            |
| `ayokoding-web`    | `CONTENT_DIR`              | `AYOKODING_WEB_CONTENT_DIR`      |                                               |
| `ayokoding-web`    | `SHOW_DRAFTS`              | `AYOKODING_WEB_SHOW_DRAFTS`      |                                               |
| `ayokoding-web`    | `PORT`                     | `PORT`                           | **exempt** — Next.js framework var            |
| `wahidyankf-web`   | (none)                     | (none)                           | reads no app env var                          |

> **Verification note**: the rename map above is derived from the read survey in § 6. Phase 1
> re-greps each app to confirm no additional reads exist before renaming, and the Phase 6 guard makes
> recurrence impossible.

### Why framework-reserved names are exempt

`NEXT_PUBLIC_*` is the Next.js mechanism that decides which vars are bundled into browser JS — it is
not ours to rename. Likewise the Next.js dev server reads `PORT` natively; renaming `ose-web`'s
framework `PORT` to `OSE_WEB_PORT` would break `nx dev ose-web`. The **backend** port is different:
Axum binds whatever value our own code reads, so the backend port **is** app-defined and **does**
take the prefix (`ORGANICLEVER_BE_PORT`, `OSE_APP_BE_PORT`). This asymmetry is the single most
error-prone point of the standard and is documented explicitly in the hub doc.

### Why `DATABASE_URL` stays unprefixed

`DATABASE_URL` is the de-facto conventional name understood by Postgres tooling, `sqlx`, migration
runners, and most operators. The cost of renaming it (every tool that reads it by convention)
exceeds the marginal collision-safety benefit. It is documented as an explicitly-blessed unprefixed
shared name, not an oversight.

> Source: [The Twelve-Factor App — Config](https://12factor.net/config) (accessed 2026-06-09):
> "store config in environment variables"; keys identical across deploys, only values differ — hence
> no environment tier in the name. 12-factor is silent on prefix/casing structure; it authorizes but
> does not prescribe per-app prefixes (those are practitioner consensus, not a 12-factor mandate).
> [Web-cited]

<!-- separates adjacent blockquotes (markdownlint MD028) -->

> Source: [Next.js Environment Variables](https://nextjs.org/docs/app/guides/environment-variables)
> (accessed 2026-06-09): `NEXT_PUBLIC_` is the framework-enforced browser-exposure prefix; never put
> secrets behind it. [Web-cited]

## 2. Layout Decision — single source of truth under `apps/<app>/`

### Decision

Each app's env template lives in exactly one place: `apps/<app>/.env.example`. The two Rust backends
already have this ([`apps/organiclever-be/.env.example`](../../../apps/organiclever-be/.env.example),
[`apps/ose-app-be/.env.example`](../../../apps/ose-app-be/.env.example)) [Repo-grounded]. The
duplicated `infra/dev/<group>/.env.example` files that repeat backend keys are **removed**; the
web-only `infra/dev/<web>/.env.example` files that document framework vars are consolidated to the
relevant `apps/<web>/.env.example`.

### Current state vs target [Repo-grounded]

| Current path                           | Content today                                          | Action                                                                |
| -------------------------------------- | ------------------------------------------------------ | --------------------------------------------------------------------- |
| `apps/organiclever-be/.env.example`    | `DATABASE_URL`, `PORT`, `CORS_ORIGINS`                 | Keep (source of truth); rename keys (§ 1); annotate (Phase 5)         |
| `apps/ose-app-be/.env.example`         | `DATABASE_URL`, `PORT`, `CORS_ORIGINS`, `OPENROUTER_*` | Keep (source of truth); rename keys; annotate                         |
| `infra/dev/organiclever/.env.example`  | placeholder prose only (no real vars)                  | Remove (no app vars; future-feature note moves to compose README)     |
| `infra/dev/ose-app/.env.example`       | duplicates `DATABASE_URL` + `OPENROUTER_*`             | Remove (duplicates `apps/ose-app-be/.env.example`)                    |
| `infra/dev/ose-web/.env.example`       | commented framework vars (`PORT`, `CONTENT_DIR`)       | Consolidate into `apps/ose-web/.env.example` (new), then remove       |
| `infra/dev/ayokoding-web/.env.example` | commented framework vars                               | Consolidate into `apps/ayokoding-web/.env.example` (new), then remove |

> **HARD safety rule (move-only, never delete a real secret).** The `.env.example` files are
> **tracked** (committed) templates, so removing/`git mv`-ing them is safe and reversible from git
> history. Any **real gitignored** `.env`/`.env.local` a developer created locally is relocated
> **only by a human** ([HUMAN] step) — the `guard-env-file-access` policy forbids agents from
> touching real `.env*` files
> ([env-file-access.md](../../../repo-governance/conventions/security/env-file-access.md))
> [Repo-grounded]. A full `rhino-cli env backup` is taken before any change (Phase 3).

### `.gitignore` impact [Repo-grounded]

The root `.gitignore` already ignores `.env`, `.env.local`, `.env.*.local` and the per-environment
variants **globally** and force-unignores `.env.example`
([`.gitignore:24-31`](../../../.gitignore)). Backends already sit at `apps/<app>/.env.example`
tracked, so colocating the web templates needs **no new ignore rule**. Phase 3 verifies this with
`git check-ignore` rather than adding rules, and adds `!apps/**/.env.example` only if a check fails.

### Next.js / Nx `.env` loading — why colocation under `apps/<app>/` is correct

The layout consolidation (removing duplicated `infra/dev/<app>/.env.example` and keeping one template
per app under `apps/<app>/`) is exactly what the framework loading rules want:

- **`.env.local` belongs at the Next.js app ROOT** — the directory holding `next.config.*`
  (`apps/ose-web/.env.local`), **not** under `src/`. Next.js loads env files from the app root only;
  a `src/.env.local` is never read. (The validated `src/env.ts` boundary is a TypeScript module, not
  an env file — it is imported, not auto-loaded.)
- **`.env.example` is NEVER auto-loaded** by Next.js or Nx — it is a committed documentation/template
  file only. Only `.env` / `.env.local` (and `.env.<mode>` variants) are read at runtime/build.
- **Nx loads from the workspace root AND the project root**, with **project root taking priority**.
  Colocating the env files under `apps/<app>/` is precisely what lets both `nx dev`/`nx build` and the
  underlying `next dev`/`next build` auto-load each app's values without extra wiring — reinforcing
  the removal of the duplicate `infra/dev/<app>/.env.example` files.

> Source: [Next.js Environment Variables](https://nextjs.org/docs/app/guides/environment-variables)
> (accessed 2026-06-09): env files are loaded from the app root (where `next.config.*` lives);
> `.env.example` is a template and is not auto-loaded. [Web-cited]

### `rhino-cli env init` scaffold-path note [Repo-grounded]

`apps/rhino-cli/src/commands/env_init.rs:36` scaffolds `.env` files from `repo_root/infra/dev`
[Repo-grounded]. Because this plan **removes** duplicated `infra/dev/<group>/.env.example` files
rather than relocating live backend templates (those already live under `apps/<app>/`), Phase 3
updates `env_init` to also scan `apps/<app>/` for `.env.example` templates so it keeps finding every
template. Its tests are extended to assert it discovers `apps/ose-app-be/.env.example`.

### The `infra/dev/<group>/docker-compose*.yml` files stay put

Only the `.env.example` files move/are removed. The compose files remain under `infra/dev/<group>/`
(they are infra artifacts), but their inline `environment:` blocks are updated to the new variable
names during the Phase 1 naming rename.

## 3. Startup Validation Design

### Rust backends — `dotenvy` + `envy`

Replace the hand-rolled `Config::from_env` (which uses `unwrap_or_else` defaults and hides missing
values, [`apps/organiclever-be/src/config.rs:22-29`](../../../apps/organiclever-be/src/config.rs),
[`apps/ose-app-be/src/config.rs:28-38`](../../../apps/ose-app-be/src/config.rs)) [Repo-grounded] with
a serde-derived struct deserialized by `envy`, after `dotenvy::dotenv().ok()` loads `.env.local` for
local runs (a no-op in CI, where the compose env is injected directly).

```rust
// apps/ose-app-be/src/config.rs (target shape)
#[derive(serde::Deserialize)]
pub struct Config {
    pub database_url: String,                       // required, no default (shared name, exempt)
    #[serde(default = "default_port")]
    pub ose_app_be_port: u16,                        // optional, typed default
    #[serde(default = "default_cors")]
    pub ose_app_be_cors_origins: String,             // optional, typed default
    #[serde(default)]
    pub ose_app_be_openrouter_api_key: String,       // optional
    #[serde(default = "default_or_model")]
    pub ose_app_be_openrouter_model: String,
    #[serde(default = "default_or_base_url")]
    pub ose_app_be_openrouter_base_url: String,
}

fn default_port() -> u16 { 8302 }

impl Config {
    pub fn load() -> Result<Self, envy::Error> {
        dotenvy::dotenv().ok();
        envy::from_env::<Config>()
    }
}
```

- `envy` maps struct field `ose_app_be_port` ↔ env var `OSE_APP_BE_PORT` automatically
  (SCREAMING_SNAKE). The field names already encode the prefix, keeping the struct self-documenting.
- Required fields are non-`Option`, no `#[serde(default)]` → a missing value is a hard error naming
  the field. `database_url` is required-no-default; the others keep typed defaults (preserving
  today's soft-default behavior for non-secret values while making the structural one fail fast).
- New crate deps: `dotenvy`, `envy` (serde is already present). The unmaintained `dotenv` crate is
  **not** used. Exact pins chosen at execution per § 8.
- **`envy` staleness caveat.** `envy`'s last crates.io release is `0.4.2` (Jan 2021, ~5 years stale).
  It carries **no** RustSec/CVE advisory and is functionally complete for its narrow scope
  (deserialize env vars into a serde struct), so it stays a Path-B candidate — but the staleness is
  recorded explicitly in § 8 and each backend's `Cargo.toml` carries a comment noting a re-evaluation
  trigger: if a RustSec advisory analogous to RUSTSEC-2021-0141 (the `dotenv` unmaintained flag) is
  ever filed against `envy`, revisit the choice.
- **`dotenvy` note.** `dotenvy` is the accepted successor to the unmaintained `dotenv`
  (RUSTSEC-2021-0141). Its last release is `0.15.7` (Mar 2023; the `0.16` branch is unpublished), so
  it is **stable-but-not-recently-released** — advisory-clean, pinned `"0.15"` (§ 8).
- **Edition-2024 note**: the existing tests use `from_env_with(...)` constructors to avoid `unsafe`
  `set_var`/`remove_var` (config.rs comments confirm) [Repo-grounded]. The new `load()` is tested via
  `envy::from_iter` over an explicit key/value vector so the missing-required-var test needs no
  process-env mutation.

> Source: [envy — docs.rs](https://docs.rs/envy) (accessed 2026-06-09): field `foo_bar` ↔ `FOO_BAR`;
> non-`Option` fields fail fast when absent; `from_iter` deserializes from an explicit pair list.
> Last release `0.4.2` (Jan 2021); no RustSec advisory. [Web-cited]

<!-- separates adjacent blockquotes (markdownlint MD028) -->

> Source: [dotenvy — crates.io](https://crates.io/crates/dotenvy) (accessed 2026-06-09): maintained
> successor to the unmaintained `dotenv` crate (RUSTSEC-2021-0141). Last release `0.15.7` (Mar 2023);
> `0.16` unpublished. [Web-cited]

### Next.js webs — `@t3-oss/env-nextjs` + `zod`

Each web adds `src/env.ts` exporting a validated `env` object, imported in `next.config.ts` so
validation runs at build time. App code reads from the validated `env` object instead of
`process.env` directly. The schema per web contains **only the vars that web actually reads** (§ 6):

```ts
// apps/ose-web/src/env.ts (target shape)
import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";

export const env = createEnv({
  server: {
    OSE_WEB_CONTENT_DIR: z.string().optional(),
    OSE_WEB_SHOW_DRAFTS: z.enum(["true", "false"]).optional(),
  },
  experimental__runtimeEnv: {},
});
```

```ts
// apps/wahidyankf-web/src/env.ts (target shape — reads no app env var)
import { createEnv } from "@t3-oss/env-nextjs";

export const env = createEnv({ server: {}, experimental__runtimeEnv: {} });
```

- `t3-env` enforces the `NEXT_PUBLIC_` prefix on **client** vars at TypeScript compile time — a
  client var without the prefix is a type error, encoding the naming standard into the type system.
  ose-public's webs currently read **server-side** vars only (no `NEXT_PUBLIC_*` runtime read found,
  § 6), so the schemas use the `server` block; when a web later adds a browser-exposed var it goes in
  `client` under `NEXT_PUBLIC_*`.
- **`zod` v4 API form (HARD).** The default `zod` export is v4 (since Jul 2025; v3 now lives at
  `zod/v3`). Of the 5 Next.js webs, **two** (`ose-web`, `ayokoding-web`) are currently on
  `zod` 3.25.76 and **migrate to v4** under this plan; the remaining three (`organiclever-web`,
  `ose-app-web`, `wahidyankf-web`) have no `zod` dependency today and **receive v4 fresh**.
  In v4 the string-format helpers moved to top-level functions: `z.string().email()` / `.uuid()` /
  `.ip()` became `z.email()` / `z.uuid()` / `z.ipv4()`. The env schemas MUST use the new top-level
  form (e.g. `z.url()`, not `z.string().url()`); the `.string()` / `.enum()` / `.optional()` helpers
  used above are unchanged across the bump. The hub doc's annotation/validation section records this
  so future env schemas do not regress to the v3 form.
- **`zod` is an OPTIONAL peer of `t3-env`, not a hard requirement.** `@t3-oss/env-nextjs` (0.13.x)
  accepts any Standard-Schema-v1 validator (Valibot, ArkType, …); `zod` is needed only because we
  author zod-based schemas. The dependency on `zod` is ours, not transitively forced by t3-env —
  relevant when reasoning about the dependency surface in § 8.
- **Next.js standalone caveat.** A standalone Next.js build must list `@t3-oss/env-nextjs` and
  `@t3-oss/env-core` in `transpilePackages` (`next.config.ts`) so the validator is bundled. The webs
  here run Next.js 16, which is compatible (t3-env requires Next ≥ 13.4.4). Each of the five webs gets
  its own `src/env.ts` boundary.
- New deps: `@t3-oss/env-nextjs`, `zod` (each web's `package.json`). Exact pins per § 8 (`zod` on the
  4.x line — see § 8).
- **Effect-TS / tRPC interaction**: `t3-env` is scoped to the env boundary only. Where a web reads
  `process.env.X` today (e.g.
  [`apps/organiclever-web/src/contexts/health/infrastructure/backend-client-live.ts:5`](../../../apps/organiclever-web/src/contexts/health/infrastructure/backend-client-live.ts))
  [Repo-grounded], the read changes to `env.X` from `src/env.ts`. Downstream code consumes the
  already-validated value; no competing config system is introduced.

> Source: [T3 Env — Next.js](https://env.t3.gg/docs/nextjs) (accessed 2026-06-09): import `env.ts`
> into `next.config.ts` for build-time validation; `experimental__runtimeEnv` lists client/runtime
> vars; client vars must carry `NEXT_PUBLIC_`; `server` block validates server-only vars;
> `createEnv({ server, client, runtimeEnv })` is current (0.13.x); `zod` is an optional
> Standard-Schema-v1 peer (any Standard-Schema-v1 validator works); Next ≥ 13.4.4; standalone builds
> need `@t3-oss/env-nextjs` + `@t3-oss/env-core` in `transpilePackages`. [Web-cited]

<!-- separates adjacent blockquotes (markdownlint MD028) -->

> **Alternative considered — per-web Effect `Config` only.** Idiomatic where a web uses Effect, but it
> does not enforce the Next.js `NEXT_PUBLIC_` browser-exposure rule and runs at request time, not
> build time. `t3-env` fails the **build** on a missing validated var, the stronger guarantee for a
> statically-built frontend. Decision: `t3-env` at the boundary, app code downstream.

## 4. `rhino-cli env` Subcommand Family

The repo already ships `rhino-cli env backup`, `env restore`, and `env init` (under
[`apps/rhino-cli/src/commands/`](../../../apps/rhino-cli/src/commands/), backed by
[`internal/envbackup.rs`](../../../apps/rhino-cli/src/internal/envbackup.rs); registered in
[`apps/rhino-cli/src/cli.rs:122-131`](../../../apps/rhino-cli/src/cli.rs)) [Repo-grounded]. This plan
**extends** that family: it **widens backup/restore to the full secret floor**, adds `--dry-run`,
updates `env init`'s scan path, and adds a new `env validate` drift guard.

> **public-specific note**: public's `rhino-cli` currently has `env` **backup/restore/init only — no
> `validate`**. This plan adds `validate`. (The reference ose-infra plan also adds `validate`; primer
> authors it Go-canonical. Recorded in § 9.)

### 4.0 Full-secret-floor scope for `backup` / `restore` (NEW) [Repo-grounded]

Today `internal/envbackup.rs::discover()` misses non-`.env` secrets in **two** independent ways:

1. **Hidden-dir skip** (`envbackup.rs:289-291`): the walker calls `walker.skip_current_dir()` on any
   directory whose basename `starts_with('.')`, so it **never descends into `.secrets/`**. The repo's
   `.secrets/` directory (gitignored via [`.gitignore:104`](../../../.gitignore)) is invisible to
   backup today.
2. **Basename-prefix filter** (`envbackup.rs:299`; `restore()` mirrors it at `:580`):
   `if !base.starts_with(".env") { continue }`. That captures `.env`/`.env.local` but **silently
   misses every non-`.env`-named secret**, most importantly `secrets.json` (gitignored via
   [`.gitignore:105`](../../../.gitignore)).

This plan fixes **both** misses: it carves `.secrets/` out of the hidden-dir skip (a single
blessed-secrets-dir exception) and replaces the single-prefix filter with an explicit **secret-file
allowlist**:

| Pattern                          | Examples (real files are gitignored) | Status                 |
| -------------------------------- | ------------------------------------ | ---------------------- |
| `.env`, `.env.*` (existing)      | `apps/<app>/.env.local`              | active                 |
| any file under `.secrets/` (NEW) | `.secrets/notes.md`                  | active                 |
| `secrets.json` (NEW)             | `secrets.json` at repo root          | active                 |
| `*.tfvars`, `*.tfvars.json`      | _none today_                         | **commented scaffold** |
| generated inventories            | _none today_                         | **commented scaffold** |

Mechanics and safety:

- The `.secrets/` directory is the one **blessed-secrets-dir exception** to the hidden-dir skip: the
  walker descends into a top-level `.secrets/` (every file inside it is gitignored, so all of it is a
  real secret) while still skipping all other dot-directories (`.git`, `.next`, …). Inside
  `.secrets/`, **every** file qualifies (no basename filter).
- The `*.tfvars`/inventory allowlist entries ship **commented out** with an "activate when IaC is
  added" marker — ose-public has no IaC today, so an active entry would only invite false confidence.
- Discovery still honors the existing skip-dirs, the max-size guard, and the inside-repo backup-dir
  refusal — the allowlist only **widens which basenames qualify** (plus the one `.secrets/` exception),
  it does not loosen any safety check.
- `*.example` templates are **tracked** (committed), so they are not the backup target; the real
  gitignored files are.
- The same widened allowlist drives `restore()` (its `:580` filter is widened in lockstep) so a
  backup round-trips exactly.

**Backup source of truth — hybrid (floor + registry).** Backup coverage is the union of:

1. A **hardcoded broad floor** — `.env*`, everything under `.secrets/`, and `secrets.json`. Matched
   by pattern regardless of any config, so a known secret kind can **never** be silently
   under-declared.
2. **Registry extras** — any additional `backup_globs` declared per surface in `env-contract.yaml`
   (§ 4.3). A future exotic secret surface becomes a one-line config add, no code change.

### 4.1 `--dry-run` for `backup` and `restore` (safety preview) [Repo-grounded]

Today `EnvBackupArgs`/`EnvRestoreArgs`
([`env_backup.rs:17`](../../../apps/rhino-cli/src/commands/env_backup.rs),
[`env_restore.rs:17`](../../../apps/rhino-cli/src/commands/env_restore.rs)) expose `--dir` and the
worktree/force flags but no preview mode. Add a `--dry-run` boolean to both:

- Thread a `dry_run: bool` into `internal::envbackup::Options`.
- In `backup()`/`restore()`, when `dry_run` is set, compute the exact file set (same discovery +
  skip-dir logic) but perform **no** filesystem writes/copies/overwrites; report a "would back up /
  would restore" list.
- `--dry-run` implies no overwrite prompt (nothing is written) and is honored across all output
  formats (text/json/markdown — matching the existing `OutputFormat`).

### 4.2 `env init` scan-path update

As described in § 2, `env_init.rs` is taught to scan `apps/<app>/` (in addition to the surviving
`infra/dev/` compose templates) so it keeps finding `.env.example` templates after the duplicated
ones are removed. Shipped in the same phase as the layout change (Phase 3).

### 4.3 `env validate` drift guard (new) — app validator + commented IaC scaffold

The guard runs a **validator per surface**. For ose-public today the only active surface is the
**app** validator; the **Terraform** and **Ansible** validator branches ship as **commented
forward-scaffold** (no IaC surfaces exist). All extraction is line-oriented regex, so **no HCL/YAML
parser dependency is added** (see § 8).

#### 4.3a App validator (`apps/<app>/`) — active

1. Parses `apps/<app>/.env.example` into a set of **declared keys** (ignoring comments/blank lines).
2. Scans the app's source for **read keys** the code actually reads, language-aware:
   - Rust: literal arguments to `env::var("…")`/`std::env::var("…")`, and `envy`-derived struct field
     names (field `ose_app_be_port` → key `OSE_APP_BE_PORT`).
   - TypeScript: `process.env.KEY` and `process.env["KEY"]`, plus keys listed in a `createEnv({...})`
     block.
3. Reports two diff sets and exits non-zero if either is non-empty (subject to an allowlist):
   - **Declared-but-unread** — a key in `.env.example` no code reads (stale declaration).
   - **Read-but-undeclared** — a key the code reads that `.env.example` omits.

#### 4.3b Terraform validator — commented forward-scaffold

The Terraform validator (diff `terraform.tfvars.example` keys against `variable` blocks in `*.tf`,
casing not checked) is implemented as a **commented-out branch** with an "activate when IaC is added"
marker. The hub doc documents the design so a future IaC plan only uncomments and registers the
surface.

#### 4.3c Ansible validator — commented forward-scaffold

The Ansible validator (diff `.env.example` keys against playbook `lookup('ansible.builtin.env', 'X')`
keys) ships the same way — a commented-out branch, documented in the hub doc.

#### Regex extractor failure modes (known, deliberate)

Because the extractors are line-oriented regex with **no HCL/YAML parser dependency**, they have
known false-positive/negative modes. ose-public has **no IaC surfaces today** (the Terraform/Ansible
branches are commented forward-scaffold), but these modes are documented openly now so the active
app-surface validator and the future-activated IaC branches both inherit them — the per-surface
allowlist + the required-comment rule **surface** (not silence) any case the regex cannot handle:

- **Terraform** — heredoc values (`<<EOT` / `<<-EOT`), `#`-comment lines, multi-line object/map
  `default` blocks, dynamic/computed defaults (data-source lookups), and the literal word `variable`
  appearing inside a comment or string can all confuse the line-oriented matcher.
- **Ansible** — multi-line YAML `lookup(...)` calls, dynamic env-key interpolation
  (`lookup('ansible.builtin.env', var)` where the key is itself a variable), the short form
  `lookup('env', ...)` versus the FQCN `lookup('ansible.builtin.env', ...)`, and Jinja2 filter forms
  such as `{{ lookup('env','X') | default(...) }}`.

Mature tools (tflint, checkov) use full parsers; the regex here is a **deliberate lightweight
first-approximation**, not a general HCL/YAML analyzer. Any construct the regex cannot resolve
statically MUST be allowlisted with a comment, so the unsupported case is visible in the contract
rather than silently mis-scanned. The extractors are unit-tested to keep the approximation honest
(§ 4.4).

#### Configuration & escape hatches

- A single `env-contract.yaml` (parsed with the YAML support already used by rhino-cli's other
  commands — confirmed against the existing config pattern at implementation; no new `toml` crate)
  declares the **surfaces** to validate. Each surface entry carries its root (`apps/organiclever-be/`,
  `apps/ose-web/`, …), its kind (`app`; `terraform`/`ansible` kinds documented but commented), the
  source globs, and an **allowlist** of keys intentionally exempt (e.g. framework-injected `PORT`, a
  key read only in tests). Dynamic/computed reads are unsupported by static detection and MUST be
  allowlisted with a comment, surfacing them rather than silently passing.
- Each surface entry MAY also carry optional **`backup_globs`** — the "registry extras" half of the
  hybrid backup model (§ 4.0), unioned with the hardcoded floor.
- Framework-reserved keys (`NEXT_PUBLIC_*` declared+read normally; Next's `PORT` allowlisted as
  framework-injected) are handled via the allowlist so the guard does not false-positive on them.

#### Why a custom subcommand over `dotenv-linter --compare`

`dotenv-linter --compare` only diffs two `.env` files (declared-vs-declared); it cannot see what the
**code** reads, so it would not catch a read-but-undeclared key. The guard's whole point is the
code↔config axis, which requires source scanning. `dotenv-linter` is noted in the hub doc as a
complementary optional tool for `.env`-vs-`.env.example` key parity.

> Source: [dotenv-linter](https://github.com/dotenv-linter/dotenv-linter) (accessed 2026-06-09):
> `--compare` diffs `.env` files for missing keys; no code-read cross-check. [Web-cited]

### 4.4 Testing & coverage (whole family)

`rhino-cli` holds a coverage gate enforced by `rhino-cli test-coverage validate` in `test:quick`
[Repo-grounded]. The new/changed surface ships with:

- **Unit tests** (`cargo test --lib`): `--dry-run` performs zero writes (assert temp dir unchanged);
  the widened secret-file allowlist matches `.env*`, descends into `.secrets/` and backs up every file
  under it while still skipping `.git`/`.next`, and matches `secrets.json`; skip-dirs and oversized
  files are still skipped; `.env.example` parser; Rust read-scanner; TS read-scanner; diff logic;
  allowlist handling — using in-memory / temp-dir fixtures.
- **Integration tests** (`cargo test --tests`): a temp-dir fixture app with a seeded mismatch asserts
  `env validate` exits non-zero naming the divergent key; a matching fixture asserts exit 0; a
  `backup --dry-run` over a fixture containing a `.env`, a `.secrets/notes.md`, and a `secrets.json`
  asserts all three appear in the reported list and no file is created.

### 4.5 Wiring (`env validate`)

- **Pre-push**: add `rhino-cli env validate` to the `.husky/pre-push` sequence. One invocation
  validates every configured app surface. It is fast (file parse + grep-class scan).
- **CI**: a workflow (new `validate-env.yml` or a step in an existing quality-gate workflow — decided
  against the current `.github/workflows/` layout during execution) runs `rhino-cli env validate` on
  PRs.

## 5. Storage-Tier Ladder (documented, not adopted)

The hub doc records the pragmatic progression so a future decision has a blessed path:

| Tier | Mechanism                                    | When                                                           |
| ---- | -------------------------------------------- | -------------------------------------------------------------- |
| 0    | Gitignored plaintext `.env` / `secrets.json` | **Current.** Solo operator; values also in a PM                |
| 1    | **SOPS + age** (commit encrypted `.enc`)     | **Blessed next rung.** Trigger: lost-machine risk, or team > 1 |
| 2    | Self-hosted Infisical / Doppler SaaS         | Team 3–5; needs UI, audit log                                  |
| 3    | Vault / Sealed Secrets                       | Overkill at this scale; only with k8s + dynamic secrets        |

**Trigger condition for Tier 1** (explicit, so the upgrade is a decision not a drift): adopt SOPS +
age when either (a) any secret exists only on one machine with no recovery copy, or (b) a second
person needs repo secret access. This plan does **not** adopt Tier 1 — it only records the ladder.
Today's `rhino-cli env backup` (Tier 0) is the interim recovery copy; the dry-run added here makes it
safe to preview.

> Source: [Secure Your Environment Files with Git, SOPS, and age](https://blog.cmmx.de/2025/08/27/secure-your-environment-files-with-git-sops-and-age/)
> (accessed 2026-06-09): SOPS + age gives version-controlled, encrypted secret files with no running
> server — the right next rung for a solo/small setup. [Web-cited]

The hub doc also documents the **IaC secret patterns as forward-scaffold** (`terraform.tfvars`
gitignored + `sensitive = true`; the Ansible `.env` env-lookup pattern) so that when ose-public adds
IaC, the backup floor entries and the `env validate` branches are uncommented rather than redesigned.

## 6. File-Impact Analysis

Env-read survey (basis for the rename map and the per-web schemas) [Repo-grounded]:

| App                | Env vars the code reads today                                               |
| ------------------ | --------------------------------------------------------------------------- |
| `organiclever-be`  | `DATABASE_URL`, `PORT`, `CORS_ORIGINS`                                      |
| `ose-app-be`       | `DATABASE_URL`, `PORT`, `CORS_ORIGINS`, `OPENROUTER_API_KEY/MODEL/BASE_URL` |
| `organiclever-web` | `ORGANICLEVER_BE_URL` (server-side)                                         |
| `ose-web`          | `CONTENT_DIR`, `SHOW_DRAFTS`, `PORT` (framework)                            |
| `ayokoding-web`    | `CONTENT_DIR`, `SHOW_DRAFTS`, `PORT` (framework)                            |
| `ose-app-web`      | none in `src/` (compose injects `OSE_APP_BE_URL`)                           |
| `wahidyankf-web`   | none                                                                        |

| File / area                                                                                                  | Change                                                                                                                                                                                                  | Phase |
| ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----- |
| `apps/organiclever-be/src/config.rs`                                                                         | Rename vars; switch to `dotenvy` + `envy` fail-fast loader                                                                                                                                              | 1, 4  |
| `apps/ose-app-be/src/config.rs`                                                                              | Rename vars (incl. `OPENROUTER_*`); switch to `dotenvy` + `envy`                                                                                                                                        | 1, 4  |
| `apps/organiclever-be/Cargo.toml`, `apps/ose-app-be/Cargo.toml`                                              | Add `dotenvy`, `envy` (exact pins per § 8)                                                                                                                                                              | 4     |
| `apps/organiclever-be/.env.example`, `apps/ose-app-be/.env.example`                                          | Rename keys → annotate                                                                                                                                                                                  | 1, 5  |
| `apps/ose-web/.env.example`, `apps/ayokoding-web/.env.example` (new)                                         | Create as single source of truth; annotate                                                                                                                                                              | 3, 5  |
| `apps/ose-web/src/...`, `apps/ayokoding-web/src/...`                                                         | Rename `CONTENT_DIR`/`SHOW_DRAFTS` → prefixed; read via `env.ts`                                                                                                                                        | 1, 4  |
| `apps/*-web/src/env.ts` (new, per web)                                                                       | `t3-env` + `zod` validated env (schema scoped to that web's reads)                                                                                                                                      | 4     |
| `apps/*-web/next.config.ts`                                                                                  | `import "./src/env.ts"` for build-time validation                                                                                                                                                       | 4     |
| `apps/*-web/package.json`                                                                                    | Add `@t3-oss/env-nextjs`, `zod` (exact pins)                                                                                                                                                            | 4     |
| `infra/dev/organiclever/.env.example`, `infra/dev/ose-app/.env.example`                                      | Remove (duplicate/placeholder)                                                                                                                                                                          | 3     |
| `infra/dev/ose-web/.env.example`, `infra/dev/ayokoding-web/.env.example`                                     | Consolidate into `apps/<web>/.env.example` then remove                                                                                                                                                  | 3     |
| `infra/dev/organiclever/docker-compose*.yml`, `infra/dev/ose-app/docker-compose*.yml`                        | Rename env keys in `environment:` blocks                                                                                                                                                                | 1     |
| `apps/rhino-cli/src/commands/env_backup.rs`, `env_restore.rs`                                                | Add `--dry-run` arg                                                                                                                                                                                     | 2     |
| `apps/rhino-cli/src/internal/envbackup.rs`                                                                   | Carve `.secrets/` out of the `:289` hidden-dir skip; widen `discover()`/`restore()` filter to `.env*`/`.secrets/**`/`secrets.json` (tfvars/inventory commented); thread `dry_run`; no-write path; tests | 2     |
| `apps/rhino-cli/src/commands/env_init.rs`                                                                    | Scan `apps/<app>/` for templates (in addition to `infra/dev`)                                                                                                                                           | 3     |
| `apps/rhino-cli/` (`env validate` cmd + tests + `cli.rs` registration)                                       | New `env validate` subcommand (app validator active; Terraform/Ansible commented) + unit/integration tests                                                                                              | 6     |
| `.husky/pre-push`                                                                                            | Invoke `rhino-cli env validate`                                                                                                                                                                         | 6     |
| `.github/workflows/` (new or existing)                                                                       | Invoke `rhino-cli env validate` on PRs                                                                                                                                                                  | 6     |
| `repo-governance/conventions/security/secrets-and-env-standards.md` (new)                                    | Hub convention                                                                                                                                                                                          | 7     |
| `repo-governance/conventions/security/no-secrets-in-committed-files.md` → `no-secrets-in-committed-files.md` | **Rename** (`git mv`) to the canonical name, then reduce to stub redirect (preserve hard-iron-rule summary); rewrite all inbound links                                                                  | 7     |
| `repo-governance/conventions/security/env-file-access.md`                                                    | Reduce to stub redirect (preserve `guard-env-file-access` summary)                                                                                                                                      | 7     |
| `repo-governance/development/workflow/reproducible-environments.md`                                          | Reduce to stub redirect (preserve `.env.example` pattern summary)                                                                                                                                       | 7     |
| `repo-governance/conventions/security/README.md`                                                             | Repoint to the hub doc                                                                                                                                                                                  | 7     |
| `docs/explanation/standardize-secrets-and-env-parity-decisions.md` (new)                                     | Cross-repo parity rationale (esp. deviations)                                                                                                                                                           | 7     |
| Active inbound links (CLAUDE.md, AGENTS.md, docs/, indexes, skills, agents)                                  | Repoint to hub doc; `done/` plan links left on stubs                                                                                                                                                    | 7     |

## 7. Risks & Rollback

- **Rename misses a reference** → app reads a default silently. Mitigation: each Phase 1 app step
  greps zero residue; the `env validate` guard makes recurrence impossible.
- **Real secret lost during the move** → Mitigation: `env backup` before any change; real-file
  relocation is **[HUMAN]** and move-only; `--dry-run` preview.
- **`env init` stops finding templates** after the layout change → Mitigation: the scan-path update
  ships in the same phase and an integration test asserts it discovers the relocated templates.
- **Doc fold breaks links** → Mitigation: stub redirects keep old paths live; link check gates each
  commit. Rollback: stubs revertible to full docs from git history.
- **New dependency regression / CVE** → Mitigation: Dependency Bump Policy Path B + exact pins + CVE
  clearance (§ 8). Rollback: a dep add is an isolated commit.
- **Coverage regression on rhino-cli** → Mitigation: tests ship with every change; `test:quick` gate
  blocks a drop below the threshold.

## 8. Dependency Additions & Security Clearance (Dependency-Bump Policy)

This plan introduces four runtime dependencies, governed by the
[Dependency Bump Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md).
None has an LTS line → **all are Path B** (latest version released ≥ 60 days before the bump date AND
CVE-clean). `zod` is a canonical Path-B example.

**Cutoff computed 2026-06-10**: today − 60 days = **2026-04-11**. Eligible = released on/before
2026-04-11.

| Dependency           | Manifest(s)                                                     | Path | Pinned version | Release date | Clearance                                                                                                                                                                                                   |
| -------------------- | --------------------------------------------------------------- | ---- | -------------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `dotenvy`            | `apps/organiclever-be/Cargo.toml`, `apps/ose-app-be/Cargo.toml` | B    | `0.15.7`       | 2023-03-22   | CLEAR — no CVE/RustSec/NVD; absolute latest version; not yanked                                                                                                                                             |
| `envy`               | `apps/organiclever-be/Cargo.toml`, `apps/ose-app-be/Cargo.toml` | B    | `0.4.2`        | 2021-01-04   | CLEAR — stale (no releases since 2021) but no CVE/RustSec advisory; functionally complete; not yanked                                                                                                       |
| `@t3-oss/env-nextjs` | each `apps/*-web/package.json`                                  | B    | `0.13.11`      | 2026-03-22   | CLEAR — no CVE/NVD/Snyk; latest 0.13.x; not deprecated                                                                                                                                                      |
| `zod`                | each `apps/*-web/package.json`                                  | B    | `4.3.6`        | 2026-01-22   | HAS_CVE: CVE-2026-6991 (CVSS 6.3 Medium, EPSS 0.00008, not in CISA KEV); fix in 4.4.0 (2026-04-29, post-cutoff); EPSS < 0.5 → no Path C escalation; accepted risk; see `docs/reference/security-waivers.md` |

Per-dependency notes:

- **`zod` — 4.x line, exact pin.** Since Jul 2025 the default `zod` export is v4 (v3 now lives at
  `zod/v3`). Two webs (`ose-web`, `ayokoding-web`) are on `zod` 3.25.76 today and **migrate to v4**;
  three webs (`organiclever-web`, `ose-app-web`, `wahidyankf-web`) have no `zod` today and
  **receive v4 fresh** (see § 3 for the `z.email()`/`z.uuid()`/`z.ipv4()` top-level-helper change).
  The Dependency Bump Policy requires an **exact** pin (no caret/tilde — brd § 8):
  `"zod": "X.Y.Z"` resolved at execution to the most recent eligible 4.x.
- **`@t3-oss/env-nextjs` — 0.13.x.** `createEnv({ server, client, runtimeEnv })` is current. `zod` is
  an **optional** Standard-Schema-v1 peer (t3-env accepts Valibot/ArkType/etc.); our `zod` dependency
  is ours, not transitively forced. Standalone Next.js builds need `@t3-oss/env-nextjs` +
  `@t3-oss/env-core` in `transpilePackages`; Next.js 16 is compatible (≥ 13.4.4).
- **`dotenvy` — 0.15.7.** Pin `"0.15.7"` (Mar 2023; `0.16` unpublished). No CVE; the accepted
  successor to the unmaintained `dotenv` (RUSTSEC-2021-0141). Stable-but-not-recently-released.
- **`envy` — 0.4.2 (STALENESS CAVEAT).** Last release `0.4.2` (Jan 2021, ~5 years stale); **no**
  CVE/RustSec advisory; functionally complete for its narrow deserialize-env-into-struct scope. It
  stays Path B, but each backend's `Cargo.toml` carries a comment recording the staleness and the
  **re-evaluation trigger**: revisit if a RustSec advisory analogous to RUSTSEC-2021-0141 is ever
  filed against `envy`. Example pin + comment:

  ```toml
  # envy 0.4.2 is the latest release (Jan 2021); stale but advisory-clean and
  # functionally complete. Re-evaluate if a RustSec advisory (cf. RUSTSEC-2021-0141)
  # is ever filed against envy.
  envy = "0.4.2"
  ```

### Execution-time obligations (HARD)

Because a plan may span more than 60 days, the exact eligible version and CVE status are resolved
**at execution**, not frozen here. The four deps are all added in **Phase 4**. The rhino-cli work in
Phases 2/6 — the widened backup/restore allowlist, `--dry-run`, and the `env validate` app validator
— introduces **no** new external crates: the read-scanners are line-oriented regex over already-walked
files, reusing `clap`, `walkdir`, `serde`, and the YAML/JSON support already in
`apps/rhino-cli/Cargo.toml`. Should any rhino-cli change unavoidably require a new crate, that crate
is itself classified under this policy. When Phase 4 runs, the executor MUST:

1. **Compute the cutoff in writing**: `Today − 60 days = cutoff`; eligible = released on/before cutoff
   (Path B). Record it in this section.
2. **Select the most recent eligible version** for each dep that is not yanked / has no open
   release-blocker.
3. **Pin exactly** — no caret/tilde: `Cargo.toml` `dotenvy = "X.Y.Z"`; `package.json`
   `"zod": "X.Y.Z"`. Verify: `grep -E '"\^|"~' apps/*-web/package.json` returns nothing for these keys.
4. **CVE-clear** against NVD, GitHub Advisories, Snyk, the project page, and the CISA KEV feed; record
   EPSS for any CVE with CVSS ≥ 7.0. Fill the clearance column with `CLEAR` / `CLEAR (patch-of …)` /
   `WAIVER` / `FUNCTIONAL-HOLD`.
5. **Re-audit**: `npm audit --audit-level=moderate` (webs) and `cargo audit` (backends) post-install;
   resolve any finding at root cause.
6. **Record results** in this table; if Path C is ever required, add a Security Waivers subsection here.

No Path C waiver is anticipated — these are mature, widely-used libraries — but the obligation to
verify at execution stands regardless.

## 9. Resolved Cross-Repo Deviation Matrix (verbatim, with public's column)

The 15-decision matrix governing all three sibling plans (source: the parity workflow's resolved
matrix). public's specific column and justification follow each decision.

| #   | Dimension                | Decision (cross-repo)                                                                                                                                       | public's column / justification                                                                                                                                                                                                                              |
| --- | ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| R1  | Parity set               | Author primer + public; infra = reference (adjustable for parity links/matrix)                                                                              | **public is authored here.** Mirrors infra's rigor; encodes public deviations, not a copy.                                                                                                                                                                   |
| R2  | Delivery mode            | infra `main-to-main`; primer + public `worktree-to-main`                                                                                                    | **worktree-to-main** — direct push to `origin main` from `worktrees/standardize-secrets-and-env/`, no PR.                                                                                                                                                    |
| R3  | IaC surfaces             | Forward-looking scaffold — IaC validators + `*.tfvars`/inventory backup patterns shipped commented/gated "when IaC is added"                                | **public has no IaC (docker-compose only).** Terraform/Ansible `env validate` branches + tfvars/inventory backup patterns ship **commented**.                                                                                                                |
| R4  | Research                 | Ran (Step 4) — findings inform validator choices                                                                                                            | Adopted: `dotenvy`+`envy` (Rust), `@t3-oss/env-nextjs`+`zod` (Next build-time + `NEXT_PUBLIC_`).                                                                                                                                                             |
| R5  | primer PR override       | DEVIATION ACCEPTED: primer `worktree-to-main` despite its PR-only invariant                                                                                 | N/A to public (public has no PR-only invariant; worktree-to-main is its normal Trunk-Based mode).                                                                                                                                                            |
| R6  | primer rhino-cli tooling | primer spec-first dual-impl (Go canonical + Rust twin)                                                                                                      | N/A — **public's rhino-cli is Rust single.** Widen Rust `backup`/`restore` + add Rust `env validate`.                                                                                                                                                        |
| R7  | Startup validation       | Full adoption in every app, both repos                                                                                                                      | **Full adoption:** both Rust backends (`dotenvy`+`envy`) + all five Next.js webs (`@t3-oss/env-nextjs`+`zod`).                                                                                                                                               |
| R8  | primer polyglot reach    | All 11 primer backends — validator-per-language table                                                                                                       | N/A — public's backends are Rust only; webs are Next.js only.                                                                                                                                                                                                |
| R9  | Naming prefix            | Full per-app prefix rename across all existing vars, both repos                                                                                             | **Full rename** across backends (`PORT`/`CORS_ORIGINS`/`OPENROUTER_*`) and webs (`CONTENT_DIR`/`SHOW_DRAFTS`); `DATABASE_URL`/framework exempt.                                                                                                              |
| R10 | Hub doc                  | New `secrets-and-env-standards.md` hub; fold 3 existing docs to stubs; `security/README.md` → hub                                                           | **Same + this repo acts on doc canonicalization:** rename `no-secrets-in-committed-files.md` → `no-secrets-in-committed-files.md` (match infra canonical), fold all 3 into the hub as stubs, **rewrite every inbound link**; `security/README.md` repointed. |
| R11 | Backup allowlist         | Per-repo real floor + IaC gated scaffold: all = `.env*` + `.secrets/`; public also `secrets.json`. `*.tfvars`/inventory commented. Hybrid floor ∪ registry. | **public floor = `.env*` + `.secrets/` + `secrets.json`** ([`.gitignore:104-105`](../../../.gitignore)); tfvars/inventory commented scaffold.                                                                                                                |
| R12 | Layout                   | public consolidates `.env.example` to `apps/<app>/` (remove `infra/dev/<app>/` dup). Real gitignored-file relocations = [HUMAN]                             | **Same.** Backends already at `apps/<app>/`; remove duplicated/placeholder `infra/dev/<group>/.env.example`; real-file moves are [HUMAN].                                                                                                                    |
| R13 | Rationale doc            | `docs/explanation/standardize-secrets-and-env-parity-decisions.md` in each repo                                                                             | **Authored** (Phase 7); aligns with existing `*-parity-decisions.md` precedents in `docs/explanation/`.                                                                                                                                                      |
| R14 | Drift `APP_PORT` fix     | DROP for primer + public                                                                                                                                    | **Dropped** — public has no `APP_PORT` drift (no such read exists).                                                                                                                                                                                          |
| R15 | Backup default dir       | Canonical per-repo-derived default `~/<repo-root-basename>-env-backup` (ose-infra canonical); **all-three-align**                                           | **all-three-align, ose-infra canonical.** Adopt `~/<repo-root-basename>-env-backup` (here `~/ose-public-env-backup`), replacing the current `ose-open-env-backup` constant. Single Rust rhino-cli (no go twin) — landed once.                                |

### Recorded public-specific deviations (called out explicitly)

1. **Doc canonicalization (this repo acts)** — public's hard-iron-rule doc is currently
   `no-secrets-in-committed-files.md` [Repo-grounded]. This plan **renames** it to
   `no-secrets-in-committed-files.md` to match the ose-infra canonical name (R10), folds it with
   `env-file-access.md` and `reproducible-environments.md` into the hub, leaves the old paths as
   stubs, and **rewrites every inbound link** to the renamed/folded targets (link-check gated). The
   hub doc records the now-aligned cross-repo doc name.
2. **No IaC** — Terraform/Ansible validators and `*.tfvars`/inventory backup patterns ship commented,
   inactive, as forward-scaffold. No live IaC drift fix exists (vs the reference's `terraform.tfvars`
   fix).
3. **Builds on prior work** — env backup/restore (`2026-04-22__env-backup-restore`,
   `2026-03-31__env-enhanced-backup-restore`) and `guard-env-file-access`
   (`2026-05-24__guard-env-file-access`) are already shipped [Repo-grounded]; this plan extends, not
   re-implements.
4. **Layout already partly migrated** — backends already have `apps/<app>/.env.example`; the work is
   removing the **duplicated** `infra/dev/<group>/.env.example` files, not a full migration.
5. **No `APP_PORT` / framework-PORT drift** — public's `PORT` reads are either renamed to a per-app
   prefix (backends) or kept as the Next.js framework var (webs); there is no dead-config drift bug to
   fix.

Deviation count: **15 recorded decisions, 0 silent deviations.**

## 10. Rollback Strategy

Each phase is an independent thematic commit. Reverting any single phase's commit restores the prior
state without touching later phases, except the ordered dependencies: Phase 3 (layout) assumes Phase
1 (rename); Phase 4 (validation) assumes Phases 1 + 3; Phase 6 (guard) assumes Phases 1 + 3 + 4.
Revert in reverse phase order if unwinding multiple phases. The backup taken in Phase 3 is the
recovery path for any real env file disturbed by the consolidation.
