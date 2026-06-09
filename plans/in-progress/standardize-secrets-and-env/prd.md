# PRD — Standardize Secrets and Environment-Variable Storage

## Product Overview

A repository-wide standard for how secrets and environment variables are **named, located,
documented, validated, and kept in sync** in ose-public — delivered as one hub convention plus the
concrete code, config, and tooling changes that make the standard real and self-enforcing.

The product has four user-facing surfaces:

1. **The hub convention** — the single document a contributor reads to learn the rules.
2. **The naming + layout standard** — applied across two Rust backends and five Next.js webs so the
   repo demonstrates its own rules.
3. **Startup validators** — backends fail fast at startup; webs fail fast at build time, instead of
   silently defaulting.
4. **The `rhino-cli env validate` guard** — a command (and pre-push/CI gate) that proves code and
   config agree.

## Personas

Hats the maintainer wears and agents that consume the outputs (not external stakeholders):

- **Contributor (human or agent)** adding or changing an env var — needs one place to learn the rule
  and immediate feedback when they get it wrong.
- **Reviewer** (the maintainer at PR time, plus `ci-checker`) — needs the guard to catch drift
  automatically rather than by manual inspection.
- **Operator** running an app locally — needs a clear, named error when a required value is missing,
  not a silent fallback to a wrong default.

## User Stories

1. **As a contributor**, I want one authoritative document for secrets/env rules, so that I do not
   have to reassemble the policy from three separate files.
2. **As a contributor**, I want a clear naming standard with documented framework exemptions, so that
   I know whether a new variable should be prefixed.
3. **As an operator**, I want a backend to abort at startup with a named-variable error when a
   required value is missing, so that I never debug a silent wrong-default.
4. **As an operator**, I want a Next.js web build to fail when a required validated env var is missing,
   so that a misconfigured deploy never ships.
5. **As a reviewer**, I want code↔config drift to fail the pre-push hook and CI, so that a key
   mismatch can never merge.
6. **As a contributor**, I want each app's env template to live in exactly one place under
   `apps/<app>/`, so that I never reconcile a duplicated `infra/dev/<group>/.env.example`.
7. **As a contributor**, I want each `.env.example` variable annotated with its required/optional
   status, type, and format, so that I can populate `.env.local` correctly without reading code.
8. **As an operator**, I want `rhino-cli env backup`/`restore` to support `--dry-run`, so that I can
   preview exactly which files would be touched before committing to the operation.
9. **As an operator**, I want one `rhino-cli env backup` to capture every secret kind — `.env*`, the
   `.secrets/` directory, and `secrets.json` — so that recovering my machine does not silently leave
   host-fact notes or a JSON secret bundle behind.
10. **As a maintainer**, I want the IaC validator and backup patterns staged but inactive, so that
    adding Terraform/Ansible later is a one-line activation rather than a redesign.

## Acceptance Criteria (Gherkin)

Every scenario uses exactly one primary `Given`/`When`/`Then`, with extras chained via `And`.

### AC-01 — Single hub convention exists and the prior docs redirect

```gherkin
Scenario: Contributor finds one authoritative secrets/env document
  Given the repository governance under repo-governance/conventions/security/
  When a contributor opens secrets-and-env-standards.md
  Then it documents naming, layout, annotation, validation, and the storage-tier ladder
  And no-secrets-in-git.md, env-file-access.md, and reproducible-environments.md each contain a stub pointing to it
  And security/README.md links the hub as the authoritative source
  And npm run lint:md reports zero broken links
```

### AC-02 — Per-app naming standard with framework exemptions

```gherkin
Scenario: App-defined variable carries the per-app prefix
  Given the naming standard in the hub convention
  When the ose-app-be backend declares its port, CORS, and OpenRouter variables
  Then they are named OSE_APP_BE_PORT, OSE_APP_BE_CORS_ORIGINS, and OSE_APP_BE_OPENROUTER_API_KEY
  And the framework-reserved Next.js PORT keeps its framework name
  And the shared DATABASE_URL remains unprefixed
```

### AC-03 — Backend port is configurable after the rename

```gherkin
Scenario: Backend honors the configured port after the rename
  Given organiclever-be reading ORGANICLEVER_BE_PORT via its config loader
  When the operator sets ORGANICLEVER_BE_PORT to a non-default value and starts the server
  Then the server binds the configured port
  And grep -rn "env::var(\"PORT\")" apps/organiclever-be apps/ose-app-be returns zero hits
```

### AC-04 — Backend startup validation fails fast on missing required value

```gherkin
Scenario: Backend aborts when a required variable is unset
  Given organiclever-be loading config through dotenvy + envy
  When the server starts with DATABASE_URL unset and no default permitted
  Then the process exits non-zero
  And the error names the missing variable
```

### AC-05 — Web env validated at build time

```gherkin
Scenario: Web build fails when a required validated variable is missing
  Given ose-web validating env through @t3-oss/env-nextjs and zod in src/env.ts
  When the build runs with a required validated variable absent and no default
  Then the build step fails
  And the failure names the missing variable
```

### AC-06 — Web with no env reads validates an empty schema without breaking

```gherkin
Scenario: A static web app builds with a minimal env schema
  Given wahidyankf-web which reads no application environment variable
  When its src/env.ts defines an empty (or framework-only) createEnv schema and the build runs
  Then the build exits 0
  And no spurious required-variable error is raised
```

### AC-07 — Env template lives in exactly one place

```gherkin
Scenario: Duplicated infra/dev env template is consolidated
  Given the layout standard in the hub convention
  When a contributor looks for the ose-app-be env template
  Then apps/ose-app-be/.env.example is the single source of truth
  And infra/dev/ose-app/.env.example no longer duplicates its keys
```

### AC-08 — Drift guard catches code↔config mismatch

```gherkin
Scenario: rhino-cli env validate fails on a deliberate mismatch
  Given rhino-cli env validate comparing declared keys against code env reads
  When a contributor renames a key in code but not in .env.example
  Then rhino-cli env validate exits non-zero and names the divergent key
  And the pre-push hook and the CI workflow both invoke the same command
```

### AC-09 — Annotated env example

```gherkin
Scenario: Each variable documents its contract
  Given the annotation format in the hub convention
  When a contributor reads apps/ose-app-be/.env.example
  Then each variable has a comment stating required-or-optional, type, and format
  And the placeholder values are obviously dev-only
```

### AC-10 — Backup/restore dry-run previews without writing

```gherkin
Scenario: Dry-run reports the file set without side effects
  Given the rhino-cli env backup and env restore commands
  When the operator runs either with --dry-run
  Then the command prints exactly which files would be backed up or restored
  And no file is written, copied, or overwritten on disk
```

### AC-11 — Backup captures every secret kind, not just `.env*`

```gherkin
Scenario: One backup run includes .secrets/ and secrets.json
  Given gitignored secret files of multiple kinds (.env.local, a .secrets/note.md, a secrets.json)
  When the operator runs rhino-cli env backup
  Then the backup archive contains the .env file, the .secrets/ note, and the secrets.json
  And no secret-bearing file kind is silently skipped
```

### AC-12 — New dependencies follow the bump policy

```gherkin
Scenario: Each new dependency is pinned and cleared
  Given the new dependencies dotenvy, envy, @t3-oss/env-nextjs, and zod
  When they are added to Cargo.toml and the web package.json files
  Then each version is an exact pin with no caret or tilde
  And each is classified Path B and CVE-cleared in the tech-docs Security Clearance table
```

### AC-13 — Backup-first, non-destructive layout consolidation

```gherkin
Scenario: Real env files survive the layout consolidation
  Given a developer's gitignored .env file alongside a duplicated infra/dev/<group>/.env.example
  When the layout consolidation removes the duplicated example
  Then rhino-cli env backup captures the real .env file before any change
  And no env or secret file is removed without a backup copy existing
```

### AC-14 — IaC validator ships as inactive forward-scaffold

```gherkin
Scenario: The Terraform/Ansible validator branch is present but inactive
  Given ose-public has no Terraform or Ansible surfaces today
  When a maintainer reads the env validate implementation and the hub doc
  Then the Terraform/Ansible validator branch and the *.tfvars backup patterns are present but commented
  And each is marked "activate when IaC is added"
```

## Product Scope

### In Scope (Product)

- Hub convention document, the three stub redirects, and the `security/README.md` repoint.
- Per-app naming standard applied across both Rust backends and the five Next.js webs (rename), and
  documented for all apps.
- Layout consolidation: remove the duplicated `infra/dev/<group>/.env.example` files.
- Full-secret-floor backup: `rhino-cli env backup`/`restore` widened from `.env*`-only to a secret
  allowlist (`.env*`, `.secrets/**`, `secrets.json`), plus `--dry-run`. `*.tfvars`/inventory patterns
  shipped commented.
- Startup validation in both backends (`dotenvy`+`envy`) and every web (`@t3-oss/env-nextjs`+`zod`),
  new deps under the Dependency Bump Policy.
- Annotated `.env.example` files.
- `rhino-cli env validate` app validator + pre-push + CI wiring; Terraform/Ansible validator branches
  commented as forward-scaffold.
- Parity-decisions rationale doc under `docs/explanation/`.

### Out of Scope (Product)

- Encrypted-at-rest secret storage adoption (documented as a future tier only).
- Re-implementing env backup/restore or the `guard-env-file-access` guard.
- Any IaC (Terraform/Ansible) — only commented scaffold ships.
- Any UI changes; this is governance + CLI + config code.

## Product-Level Risks

- **Validation friction**: overly strict startup/build validation could block legitimate local dev.
  Mitigation: permit documented defaults for non-secret dev values; only secrets and structural values
  are required-no-default.
- **Guard false positives**: the drift guard's code-read detection could miss a dynamic env read.
  Mitigation: the guard parses a declared allowlist per app and reports unknown/missing keys
  explicitly; dynamic reads are documented as unsupported and flagged for manual allowlisting.
- **Empty-schema webs**: a web that reads no app env var still needs an `env.ts`. Mitigation: a minimal
  (framework-only or empty) schema is valid and tested by AC-06.
