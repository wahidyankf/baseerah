# BRD — Standardize Secrets and Environment-Variable Storage

## Business Goal

Make environment configuration **predictable, self-validating, and drift-proof** across the
ose-public monorepo, so that adding or changing a configuration value is a mechanical,
mistake-resistant operation rather than a source of silent runtime bugs. Consolidate scattered
governance into one authoritative reference so a contributor (human or agent) has exactly one place
to learn the rules.

## Why Now

- **Soft-default config hides misconfiguration.** Both Rust backends load every variable through
  `unwrap_or_else` fallbacks that always succeed, so a missing `DATABASE_URL` or a mistyped `PORT`
  silently resolves to a wrong default instead of failing fast
  ([`apps/organiclever-be/src/config.rs:22-29`](../../../apps/organiclever-be/src/config.rs),
  [`apps/ose-app-be/src/config.rs:28-38`](../../../apps/ose-app-be/src/config.rs)) [Repo-grounded].
  No Next.js web validates its env at build time. This is the same class of latent bug a naming
  standard plus startup validation prevents.
- **Inconsistent naming invites drift.** Some web vars carry a per-app prefix
  (`ORGANICLEVER_BE_URL`, `OSE_APP_BE_URL`) while others do not (`CONTENT_DIR`, `SHOW_DRAFTS`), and
  backend vars are unprefixed (`PORT`, `CORS_ORIGINS`, `OPENROUTER_*`) [Repo-grounded]. With no
  rule, the next contributor guesses.
- **Duplicated env templates.** `infra/dev/ose-app/.env.example` duplicates `DATABASE_URL` and the
  `OPENROUTER_*` keys that already live in `apps/ose-app-be/.env.example` [Repo-grounded] — two
  sources of truth that can diverge unseen.
- **Build on the momentum.** ose-public already shipped env backup/restore and the
  `guard-env-file-access` policy. Standardizing the rest **now**, while that work is fresh, is far
  cheaper than retrofitting after more apps and secrets land.

## Business Impact

| Pain point today                                                                     | After this plan                                                                           |
| ------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| Missing/mistyped env value defaults silently at runtime (both backends)              | `dotenvy` + `envy` abort backend startup with a named-variable error                      |
| No build-time env validation in any Next.js web                                      | `@t3-oss/env-nextjs` + `zod` fail the build on a missing/invalid public or server var     |
| Code and config can disagree silently                                                | `rhino-cli env validate` fails pre-push/CI on any code↔config key mismatch                |
| No naming rule; prefixed and unprefixed vars coexist arbitrarily                     | One per-app-prefix standard with documented framework exemptions                          |
| `infra/dev/<group>/.env.example` duplicates `apps/<app>/.env.example` keys           | One source of truth per app under `apps/<app>/` (Nx-native)                               |
| `env backup` skips `.secrets/` (hidden-dir) and `secrets.json` (non-`.env` basename) | One `env backup` captures every secret kind (`.env*`, `.secrets/`, `secrets.json`)        |
| `.env.example` files give no type/required hints                                     | Every variable annotated (required/optional, type, format)                                |
| Three docs, no hub; rules must be reassembled by the reader                          | One hub convention; the three become stub redirects and `security/README.md` points to it |

## Affected Roles

This repository has one maintainer collaborating with AI agents; the "roles" below are hats the
maintainer wears and agents that consume the outputs — not sign-off gates.

- **Platform maintainer** — defines and applies the standard; benefits from the drift guard
  catching config mistakes before they ship.
- **`swe-rust-dev` / `swe-typescript-dev` agents** — consume the naming standard and validation
  patterns when authoring app config code.
- **`plan-maker` / future plan authors** — reference the hub convention when a plan introduces new
  env vars or secrets.
- **`repo-rules-checker` / `ci-checker` agents** — gain a single authoritative doc to validate
  against instead of three.

## Success Criteria

All criteria are **observable facts** verifiable on demand — no fabricated metrics.

1. **Per-app naming applied, zero residue**: every app-defined variable carries its app prefix.
   Verify: `grep -rn "env::var(\"PORT\"\|env::var(\"CORS_ORIGINS\"\|env::var(\"OPENROUTER_"
apps/organiclever-be apps/ose-app-be` shows only prefixed keys; `grep -rn
"process.env.CONTENT_DIR\|process.env.SHOW_DRAFTS\|process.env\[\"SHOW_DRAFTS\"\]" apps/ose-web
apps/ayokoding-web` shows only prefixed keys; `DATABASE_URL`, framework `PORT`, and `NEXT_PUBLIC_*`
   remain as required.
2. **Backend startup validation active**: starting either Rust backend with a required var unset
   aborts with a non-zero exit and a named-variable error (where the field is required-no-default).
   Verify: run each backend with `DATABASE_URL` unset and confirm a non-zero exit naming the field.
3. **Web build-time validation active**: a Next.js web build fails when a required validated var is
   absent. Verify: unset the var, run `nx build <web-app>`, confirm the build fails naming it; restore
   and confirm the build passes.
4. **Drift guard wired and biting**: `rhino-cli env validate` exits 0 on the clean repo; a deliberate
   key mismatch in any app's `.env.example` vs its code reads makes it exit non-zero naming the key,
   and both the pre-push hook and a CI workflow invoke the same command.
5. **Single hub doc exists**: `repo-governance/conventions/security/secrets-and-env-standards.md`
   exists; the three prior docs (`no-secrets-in-git.md`, `env-file-access.md`,
   `reproducible-environments.md`) are stubs pointing to it; `security/README.md` repoints to the hub;
   no inbound link is broken (`npm run lint:md` link check passes).
6. **Layout consolidated**: no `.env.example` remains under `infra/dev/` for an app whose template now
   lives under `apps/<app>/`; each backend's `apps/<app>/.env.example` is the single source of truth.
   Verify: `find infra/dev -name ".env.example"` returns zero hits (or only files with no
   `apps/<app>/` analogue, explicitly noted).
7. **Backup covers every secret kind; no file destroyed**: `rhino-cli env backup` captures `.env*`,
   every file under `.secrets/`, and `secrets.json`; `--dry-run` on backup and restore previews the
   file list without writing. Verify: place a throwaway `.secrets/throwaway.md` and a throwaway
   `secrets.json`, run `rhino-cli env backup --dry-run`, confirm both appear and nothing is written.
8. **Dependency policy satisfied**: every new dependency (`dotenvy`, `envy`, `@t3-oss/env-nextjs`,
   `zod`) is pinned exactly (no caret/tilde), classified Path B, and CVE-cleared per the
   [Dependency Bump Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md);
   `tech-docs.md` § 8 carries the Security Clearance table. Verify: `grep -E '"\^|"~'
apps/*-web/package.json` returns nothing for the new keys and the `Cargo.toml` deps are exact
   strings.
9. **Rationale recorded**: `docs/explanation/standardize-secrets-and-env-parity-decisions.md` exists
   and explains each cross-repo decision, especially public's deviations (no IaC, doc-name, building on
   prior work, layout consolidation).

## Non-Goals (Business Scope)

- Introducing any encrypted-at-rest secret store (SOPS/age, Vault, Infisical, Doppler) in this plan.
- Re-implementing env backup/restore or the `guard-env-file-access` policy (already shipped).
- Adding Terraform/Ansible or any IaC — the IaC validators and backup patterns ship commented,
  inactive, as forward-scaffold only.
- Changing production secret-injection mechanics for CI.
- Re-architecting any web app's config beyond the env-validation boundary.

## Business Risks and Mitigations

| Risk                                                                                  | Likelihood | Mitigation                                                                                                                                       |
| ------------------------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Env-var rename breaks a developer's local `.env` (gitignored, not migrated by agents) | Medium     | Hub doc + Phase 1 note instruct re-copying from the new `.env.example`; old names grep-verified to zero; relocations are `[HUMAN]`               |
| A developer's real gitignored secret is lost during the layout consolidation          | Low        | `env backup` taken before any move; real-file relocation is **[HUMAN]** and move-only; `--dry-run` previews first                                |
| Full per-app rename misses a code/compose reference, breaking a build                 | Medium     | Phase 1 greps every reference per app and its gate runs each affected app's `build`/`test:quick` before commit; `env validate` guards recurrence |
| Folding three docs loses content or breaks inbound links                              | Medium     | Stub-redirect approach keeps old paths resolvable; `done/` plan links untouched; link check gates each commit                                    |
| `t3-env` conflicts with an Effect-TS / tRPC config style in a web app                 | Low        | Scope `t3-env` to the env boundary only (`src/env.ts` per app); downstream code consumes the validated object                                    |
| New `rhino-cli env validate` lowers crate coverage below threshold                    | Low        | Subcommand ships with unit + integration tests; `test:quick` coverage gate enforces the rhino-cli threshold                                      |
| A new dependency ships a regression or unpatched CVE                                  | Low        | Dependency Bump Policy Path B (60-day soak + CVE-clean), exact pins, Security Clearance table verified at execution                              |
| IaC scaffold (commented Terraform/Ansible branches) is mistaken for active validation | Low        | Branches ship explicitly commented with an "activate when IaC is added" marker; hub doc documents them as forward-scaffold only                  |
| Web apps that read no env var get an empty/over-engineered `env.ts`                   | Low        | Each web's `env.ts` validates only the vars that app actually reads (per the per-app read survey in tech-docs.md § 6); minimal schema            |
