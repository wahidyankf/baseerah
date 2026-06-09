# Delivery — Standardize Secrets and Environment-Variable Storage

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (here: relocating real gitignored `.env*` files, which the
> `guard-env-file-access` policy forbids agents from touching). `[AI+HUMAN]`: agent prepares,
> human performs the final guarded action.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase is
> not complete until its gate is green; do not start phase N+1 while any gate check fails.

All checkboxes are `[AI]` unless tagged otherwise. Commit + push at each phase gate (Conventional
Commits, `origin main`).

## Worktree

Worktree path: `worktrees/standardize-secrets-and-env/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree standardize-secrets-and-env
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

> **Safety rule for the whole plan**: no `.env`, `.env.local`, or other real secret file is ever
> deleted. Tracked `.env.example` templates are moved/removed via `git mv`/`git rm` (reversible);
> real gitignored files are relocated **only by a human** ([HUMAN] steps), move-only, after a backup.

---

## Phase 0 — Environment Setup + Baseline

> _Executor: repo-setup-manager_

<!-- separates adjacent blockquotes (markdownlint MD028) -->

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work.

- [ ] [AI] From the worktree root, run `npm install` — exits 0 and `node_modules/` is present.
- [ ] [AI] Converge the toolchain: `npm run doctor -- --fix` — exits 0, no unresolved drift (Rust,
      Node, cargo-llvm-cov, jq present).
- [ ] [AI] Capture the backend baseline: run
      `./node_modules/.bin/nx run organiclever-be:test:quick` and
      `./node_modules/.bin/nx run ose-app-be:test:quick` — both exit 0 (record coverage %).
- [ ] [AI] Capture the web baseline: run
      `./node_modules/.bin/nx run-many -t test:quick -p organiclever-web ose-web ayokoding-web ose-app-web wahidyankf-web`
      — all exit 0.
- [ ] [AI] Capture the rhino-cli baseline: run `./node_modules/.bin/nx run rhino-cli:test:quick` —
      exits 0 (record coverage %).
- [ ] [AI] Record the rename baseline: run
      `grep -rn "env::var(\"PORT\")\|env::var(\"CORS_ORIGINS\")\|env::var(\"OPENROUTER_" apps/organiclever-be apps/ose-app-be`
      and `grep -rn "process.env.CONTENT_DIR\|process.env.SHOW_DRAFTS\|process.env\[\"SHOW_DRAFTS\"\]" apps/ose-web apps/ayokoding-web`
      — save the hit lists; Phase 1 eliminates exactly these unprefixed reads.
- [ ] [AI] Confirm the env-file inventory: run `find apps infra -name ".env.example"` — note the two
      backend templates under `apps/`, and the four `infra/dev/<group>/.env.example` files (Phase 3
      consolidates these).
- [ ] [AI] Confirm the secret-backup gaps (no `--dry-run` exists yet — that lands in Phase 2): create
      a throwaway `.secrets/throwaway.md` and a throwaway `secrets.json` at the repo root, run
      `rhino-cli env backup --dir "$(mktemp -d)"`, and confirm **both** are **absent** from the
      archive (the hidden-dir skip at `envbackup.rs:289` and the `.env`-prefix filter at `:299`).
      Delete the throwaway files and dir after. Phase 2 makes both appear.

### Phase 0 Gate

> All checks below must pass before starting Phase 1; if any fails, fix it in Phase 0 first.

- [ ] [AI] All backend, web, and rhino-cli `test:quick` targets above exit 0 (clean baseline).
- [ ] [AI] Run `git status` — working tree clean (no changes yet).

> **Pause Safety**: Phase 0 made no code changes; the repo is at a clean, green baseline. Resume by
> re-running the `test:quick` targets to reconfirm before starting Phase 1.

---

## Phase 1 — Naming Standard: per-app prefix rename (backends + webs + compose)

> Per the rename map in `tech-docs.md § 1`. `DATABASE_URL`, framework `PORT`, and `NEXT_PUBLIC_*`
> are exempt. Do code + `.env.example` + compose for each app together so sources never disagree.

- [ ] [AI] **RED**: in `apps/organiclever-be/src/config.rs`, write a failing unit test asserting that
      `ORGANICLEVER_BE_PORT=8299` resolves to `port == 8299` (using the existing `from_env_with`-style
      seam or `envy::from_iter` over an explicit pair list — do not mutate process env). Run
      `./node_modules/.bin/nx run organiclever-be:test:unit` — acceptance: fails (still reads `PORT`).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: edit `apps/organiclever-be/src/config.rs`: rename read keys `PORT` →
      `ORGANICLEVER_BE_PORT`, `CORS_ORIGINS` → `ORGANICLEVER_BE_CORS_ORIGINS` (leave `DATABASE_URL`).
      Keep the existing loader shape for now (the `envy` switch is Phase 4). Run
      `./node_modules/.bin/nx run organiclever-be:test:unit` — acceptance: the port test passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: in `apps/ose-app-be/src/config.rs`, write a failing test asserting
      `OSE_APP_BE_PORT=8399` resolves to `port == 8399`. Run
      `./node_modules/.bin/nx run ose-app-be:test:unit` — acceptance: fails.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: edit `apps/ose-app-be/src/config.rs`: rename `PORT` → `OSE_APP_BE_PORT`,
      `CORS_ORIGINS` → `OSE_APP_BE_CORS_ORIGINS`, `OPENROUTER_API_KEY` →
      `OSE_APP_BE_OPENROUTER_API_KEY`, `OPENROUTER_MODEL` → `OSE_APP_BE_OPENROUTER_MODEL`,
      `OPENROUTER_BASE_URL` → `OSE_APP_BE_OPENROUTER_BASE_URL` (leave `DATABASE_URL`). Run
      `./node_modules/.bin/nx run ose-app-be:test:unit` — acceptance: the port test passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Edit `apps/organiclever-be/.env.example` and `apps/ose-app-be/.env.example`: rename the
      same keys; placeholders stay obviously-dev (`OSE_APP_BE_OPENROUTER_API_KEY=` blank as today).
- [ ] [AI] **RED**: in `apps/ose-web/`, write a failing test asserting the content reader reads
      `OSE_WEB_CONTENT_DIR` (not `CONTENT_DIR`). Run
      `./node_modules/.bin/nx run ose-web:test:unit` — acceptance: fails.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **GREEN**: edit `apps/ose-web/src/` reads — `CONTENT_DIR` → `OSE_WEB_CONTENT_DIR`,
      `SHOW_DRAFTS` → `OSE_WEB_SHOW_DRAFTS` (in `repository-fs.ts` and `service.ts`); leave framework
      `PORT` in `lib/trpc/client.ts` untouched. Run
      `./node_modules/.bin/nx run ose-web:test:unit` — acceptance: the rename test passes.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **RED**: in `apps/ayokoding-web/`, write a failing test asserting the reader reads
      `AYOKODING_WEB_CONTENT_DIR`. Run `./node_modules/.bin/nx run ayokoding-web:test:unit` —
      acceptance: fails.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **GREEN**: edit `apps/ayokoding-web/src/` reads — `CONTENT_DIR` →
      `AYOKODING_WEB_CONTENT_DIR`, `SHOW_DRAFTS` → `AYOKODING_WEB_SHOW_DRAFTS` (in `reader.ts` and
      `repository-fs.ts`); leave framework `PORT` untouched. Run
      `./node_modules/.bin/nx run ayokoding-web:test:unit` — acceptance: passes.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Confirm `organiclever-web` (`ORGANICLEVER_BE_URL`) and `ose-app-web`/`wahidyankf-web`
      (already prefixed / no app vars) need no rename: run
      `grep -rn "process.env" apps/organiclever-web/src apps/ose-app-web/src apps/wahidyankf-web/src`
      — only `ORGANICLEVER_BE_URL` appears; no unprefixed app var. Document this in the commit message.
- [ ] [AI] Edit the compose `environment:` blocks to the new keys: in
      `infra/dev/ose-app/docker-compose.yml` and `docker-compose.ci.yml` rename
      `OPENROUTER_*`/`PORT` → `OSE_APP_BE_OPENROUTER_*`/`OSE_APP_BE_PORT` (keep `DATABASE_URL`); in
      `infra/dev/organiclever/docker-compose.yml` confirm only `ORGANICLEVER_BE_URL` is set (already
      conforming). Run `grep -rn "OPENROUTER_API_KEY:\|\bPORT:\|CORS_ORIGINS:" infra/dev/ose-app infra/dev/organiclever`
      — acceptance: only prefixed keys appear.
- [ ] [AI] Verify zero residue: run
      `grep -rn "env::var(\"PORT\")\|env::var(\"CORS_ORIGINS\")\|env::var(\"OPENROUTER_" apps/organiclever-be apps/ose-app-be`
      and `grep -rn "process.env.CONTENT_DIR\|process.env\[\"CONTENT_DIR\"\]\|SHOW_DRAFTS" apps/ose-web apps/ayokoding-web | grep -v "OSE_WEB_\|AYOKODING_WEB_"`
      — both return zero hits.
- [ ] [AI] Run `./node_modules/.bin/nx run-many -t test:quick -p organiclever-be ose-app-be ose-web ayokoding-web`
      — all exit 0, coverage ≥ baseline.

### Phase 1 Gate

> All checks below must pass before starting Phase 2; if any fails, fix it in Phase 1 first.

- [ ] [AI] The two residue greps above return zero hits.
- [ ] [AI] `./node_modules/.bin/nx run-many -t test:quick -p organiclever-be ose-app-be ose-web ayokoding-web`
      exits 0 with coverage at or above each project's threshold.
- [ ] [AI] `npm run lint:md` exits 0.
- [ ] [AI] Commit as thematic commits (split backends vs webs vs compose) and push to `origin main`:
      `refactor(organiclever-be,ose-app-be): prefix env vars with per-app name`,
      `refactor(ose-web,ayokoding-web): prefix CONTENT_DIR/SHOW_DRAFTS with per-app name`,
      `chore(infra): rename compose env keys to per-app prefixes`; `git status` clean.

> **Pause Safety**: Phase 1 left all config sources naming the same per-app-prefixed keys, with
> framework/shared vars exempt; tests green. Resume by re-running the four apps' `test:quick`.

---

## Phase 2 — `env backup`/`restore`: full secret floor + `--dry-run`

- [ ] [AI] **RED**: write failing unit tests in `apps/rhino-cli/src/internal/envbackup.rs` (temp-dir
      fixtures) asserting: (a) `.secrets/notes.md` appears in the discovered set; (b) `.git/` is still
      skipped; (c) a root `secrets.json` appears in the discovered set; (d) a `backup` with
      `dry_run=true` creates no files. Run `./node_modules/.bin/nx run rhino-cli:test:unit` —
      acceptance: all new tests fail.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN — carve `.secrets/` out of the hidden-dir skip** (`tech-docs.md § 4.0`): in
      `apps/rhino-cli/src/internal/envbackup.rs`, the dir branch at `envbackup.rs:289-291`
      (`if base.starts_with('.') { walker.skip_current_dir(); continue; }`) currently skips **every**
      dot-directory. Add an exception so a top-level `.secrets/` is descended into (skip the hidden
      dir unless its repo-relative path is exactly `.secrets`); all other dot-dirs still skip.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN — widen the secret-file scope** (`tech-docs.md § 4.0`): replace the `discover()`
      basename filter (`if !base.starts_with(".env")`, `envbackup.rs:299`) with a secret allowlist
      matching `.env`/`.env.*`, `secrets.json`, **and** any file reached under `.secrets/`. Ship the
      `*.tfvars`/`*.tfvars.json`/inventory patterns **commented** with an
      `// activate when IaC is added` marker. Apply the same widened filter to `restore()`'s
      non-config branch (`envbackup.rs:580`). Keep all skip-dir, max-size, and inside-repo-refusal
      checks intact.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN — add `--dry-run`**: add a `dry_run: bool` field to `Options` (default false); add
      a `--dry-run` clap arg to `EnvBackupArgs` (`apps/rhino-cli/src/commands/env_backup.rs`) and
      `EnvRestoreArgs` (`env_restore.rs`); thread it into `Options.dry_run`. In `backup()`/`restore()`,
      when `dry_run` is true, run discovery but perform **no** filesystem writes; report the "would
      back up / would restore" list.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Run `./node_modules/.bin/nx run rhino-cli:test:unit` — acceptance: all RED tests pass, no
      previously passing test broken. Then run `./node_modules/.bin/nx run rhino-cli:test:quick` —
      exits 0, coverage at or above threshold.
- [ ] [AI] **REFACTOR**: extract the allowlist match into a single named predicate
      (`fn is_secret_file(rel: &str) -> bool`) used by both `discover()` and `restore()`; run
      `./node_modules/.bin/nx run rhino-cli:test:quick` — acceptance: all tests still pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Smoke-check: create a throwaway `.secrets/throwaway.md` and a `secrets.json`, run
      `rhino-cli env backup --dry-run` at the repo root — the would-back-up list now includes **both**
      (the Phase 0 gaps are closed) and creates nothing under `~/ose-open-env-backup`. Delete the
      throwaway files after.

### Phase 2 Gate

> All checks below must pass before starting Phase 3; if any fails, fix it in Phase 2 first.

- [ ] [AI] `./node_modules/.bin/nx run rhino-cli:test:quick` exits 0, coverage at or above threshold.
- [ ] [AI] `rhino-cli env backup --dry-run` and `rhino-cli env restore --dry-run` both run, print a
      file list (including `.secrets/` files and `secrets.json`), and write nothing.
- [ ] [AI] A throwaway `.secrets/` file and a `secrets.json` both appear in the `backup --dry-run`
      list (both absent at Phase 0); a backup→restore round-trip over a fixture reproduces all secret
      kinds byte-for-byte.
- [ ] [AI] `npm run lint:md` exits 0.
- [ ] [AI] Commit (`feat(rhino-cli): back up and restore all secret kinds; add --dry-run`) and push;
      `git status` clean.

> **Pause Safety**: Phase 2 left backup/restore covering every repo secret kind and able to preview
> without side effects. Resume by running `rhino-cli env backup --dry-run`.

---

## Phase 3 — Layout Consolidation: remove duplicated `infra/dev/` env templates

- [ ] [AI] **Preview**: run `rhino-cli env backup --dry-run` — confirm every repo secret file appears
      (each `.env*` including any gitignored real one, plus `.secrets/` files and any `secrets.json`).
- [ ] [AI] **Back up for real**: run `rhino-cli env backup` — exits 0; confirm the archive under
      `~/ose-open-env-backup` contains the env files and any `.secrets/`/`secrets.json` (pre-change
      safety copy).
- [ ] [AI] Consolidate web framework-var docs into new app-colocated templates: create
      `apps/ose-web/.env.example` and `apps/ayokoding-web/.env.example` carrying the (now prefixed)
      framework/content vars previously documented in `infra/dev/ose-web/.env.example` and
      `infra/dev/ayokoding-web/.env.example` (e.g. `OSE_WEB_CONTENT_DIR`, `OSE_WEB_SHOW_DRAFTS`,
      commented framework `PORT`). Placeholders only.
- [ ] [AI] Remove the duplicated/placeholder infra templates via `git rm`:
      `git rm infra/dev/organiclever/.env.example infra/dev/ose-app/.env.example infra/dev/ose-web/.env.example infra/dev/ayokoding-web/.env.example`
      (the `ose-app` one duplicated `apps/ose-app-be/.env.example`; `organiclever` was a placeholder;
      the two webs are now consolidated under `apps/<web>/`).
- [ ] [HUMAN] Relocate any **real gitignored** `.env`/`.env.local` that a developer created under
      `infra/dev/<group>/` to the matching `apps/<app>/.env.local`, move-only (never delete). The
      `guard-env-file-access` policy forbids the agent from touching real `.env*` files, so a human
      performs this. — observable signal the agent checks to resume: the human confirms
      "real env files relocated (or none existed)"; the agent then runs
      `git status` and proceeds.
- [ ] [AI] Confirm ignore status: run `git check-ignore apps/ose-web/.env.local apps/ayokoding-web/.env.local`
      — both ignored; `git check-ignore apps/ose-web/.env.example apps/ayokoding-web/.env.example` —
      **not** ignored (expect non-zero exit / no output). If a `.env.example` is unexpectedly ignored,
      add `!apps/**/.env.example` to `.gitignore`.
- [ ] [AI] Update `apps/rhino-cli/src/commands/env_init.rs`: extend the scaffold scan (currently
      `repo_root/infra/dev`, line 36) to also walk `apps/<app>/` for `.env.example` templates. Update
      its tests to assert it discovers `apps/ose-app-be/.env.example`. Run
      `./node_modules/.bin/nx run rhino-cli:test:quick` — exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Grep for stale references to the removed infra templates: run
      `grep -rn "infra/dev/organiclever/.env\|infra/dev/ose-app/.env\|infra/dev/ose-web/.env\|infra/dev/ayokoding-web/.env" . --include="*.md" --include="*.yml" --include="*.yaml" --include="*.json" --include="*.rs"`
      (excluding `node_modules`, `plans/done`) and update each hit to the `apps/<app>/.env.example`
      path — acceptance: re-run the same grep and confirm zero hits.
- [ ] [AI] Run `./node_modules/.bin/nx run-many -t build test:quick -p organiclever-be ose-app-be ose-web ayokoding-web rhino-cli`
      — all exit 0 (the consolidation broke no compose/CI/scaffold reference).

### Phase 3 Gate

> All checks below must pass before starting Phase 4; if any fails, fix it in Phase 3 first.

- [ ] [AI] `find infra/dev -name ".env.example"` returns zero hits for the four removed groups;
      `ls apps/ose-web/.env.example apps/ayokoding-web/.env.example` both exist.
- [ ] [AI] The pre-change backup archive exists and contains every pre-change env file (no content
      lost; nothing deleted without a backup copy).
- [ ] [AI] `rhino-cli env init` (or its test) discovers templates under `apps/<app>/`.
- [ ] [AI] All five projects' `build`/`test:quick` exit 0; `npm run lint:md` exits 0.
- [ ] [AI] Commit (`refactor(infra): consolidate app env templates under apps/<app>/ (backup-first)`)
      and push; `git status` clean.

> **Pause Safety**: Phase 3 left one env template per app under `apps/<app>/`, the duplicated infra
> templates removed, `env init` repointed, and any real files relocated (not deleted) with a backup
> copy retained; builds green. Resume by re-running the five projects' `build`.

---

## Phase 4 — Startup Validation (`dotenvy`+`envy` backends, `@t3-oss/env-nextjs`+`zod` webs)

- [ ] [AI] **Dependency clearance (HARD)**: per `tech-docs.md § 8`, compute the cutoff
      (`today − 60 days`) in writing, select the most recent eligible (Path B) version of `dotenvy`,
      `envy`, `@t3-oss/env-nextjs`, `zod`, confirm none is yanked / has an open release-blocker, and
      CVE-clear each against NVD / GitHub Advisories / Snyk / project page / CISA KEV. Record results
      in the `tech-docs.md § 8` clearance table.
- [ ] [AI] Add `dotenvy`, `envy` to `apps/organiclever-be/Cargo.toml` and `apps/ose-app-be/Cargo.toml`
      as **exact pins** (`dotenvy = "X.Y.Z"`, `envy = "X.Y.Z"`); run
      `cargo build -p organiclever-be -p ose-app-be` — compiles; run `cargo audit` — clean.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: write a failing unit test in `apps/organiclever-be/src/config.rs` asserting
      `Config::load()` returns an error naming the field when `DATABASE_URL` is unset (test via
      `envy::from_iter` over an explicit pair list, no process-env mutation). Run
      `./node_modules/.bin/nx run organiclever-be:test:unit` — acceptance: fails (envy not wired).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: rewrite `apps/organiclever-be/src/config.rs` to the `envy` fail-fast shape
      (`tech-docs.md § 3`): serde-derived `Config`, `database_url` required-no-default,
      `organiclever_be_port`/`organiclever_be_cors_origins` with typed `#[serde(default)]`, `load()`
      calling `dotenvy::dotenv().ok()` then `envy::from_env`; update call sites to `Config::load()`.
      Run `./node_modules/.bin/nx run organiclever-be:test:unit` — acceptance: the RED test passes and
      a fully-set env resolves correctly.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: write a failing unit test in `apps/ose-app-be/src/config.rs` asserting
      `Config::load()` returns an error naming the field when `DATABASE_URL` is unset (test via
      `envy::from_iter` over an explicit pair list, no process-env mutation). Run
      `./node_modules/.bin/nx run ose-app-be:test:unit` — acceptance: fails (envy not wired).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: rewrite `apps/ose-app-be/src/config.rs` to the `envy` fail-fast shape
      (`tech-docs.md § 3`): serde-derived `Config`, `database_url` required-no-default, five
      `OSE_APP_BE_*` fields with typed `#[serde(default)]`, `load()` calling
      `dotenvy::dotenv().ok()` then `envy::from_env`; update call sites. Run
      `./node_modules/.bin/nx run ose-app-be:test:unit` — acceptance: the RED test passes and a
      fully-set env resolves correctly.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Run `./node_modules/.bin/nx run-many -t test:quick -p organiclever-be ose-app-be` — exits
      0, coverage at or above threshold.
- [ ] [AI] Add `@t3-oss/env-nextjs` and `zod` to each web `package.json`
      (`apps/organiclever-web`, `apps/ose-web`, `apps/ayokoding-web`, `apps/ose-app-web`,
      `apps/wahidyankf-web`) as **exact pins** (no caret/tilde); run `npm install` from root; run
      `npm audit --audit-level=moderate` — clean; verify
      `grep -E '"\^|"~' apps/*-web/package.json` returns nothing for these two keys.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **RED**: write a failing test in `apps/ose-web/` asserting `createEnv` validates
      `OSE_WEB_SHOW_DRAFTS` as the documented enum (or that `env.ts` exports the validated object).
      Run `./node_modules/.bin/nx run ose-web:test:quick` — acceptance: fails (env.ts not created).
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **GREEN**: create `apps/ose-web/src/env.ts` (`tech-docs.md § 3`) validating
      `OSE_WEB_CONTENT_DIR`/`OSE_WEB_SHOW_DRAFTS` in the `server` block; create the analogous
      `apps/ayokoding-web/src/env.ts` (`AYOKODING_WEB_*`), `apps/organiclever-web/src/env.ts`
      (`ORGANICLEVER_BE_URL`), and a **minimal empty-schema** `src/env.ts` for `ose-app-web` and
      `wahidyankf-web` (they read no app env var — AC-06). Run
      `./node_modules/.bin/nx run-many -t test:quick -p ose-web ayokoding-web organiclever-web ose-app-web wahidyankf-web`
      — acceptance: all pass.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Edit each web's `apps/<web>/next.config.ts` to `import "./src/env.ts"` so validation runs
      at build time.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Repoint each web's `process.env.X` reads to `env.X` from `src/env.ts` (e.g.
      `apps/organiclever-web/src/contexts/health/infrastructure/backend-client-live.ts` and
      `apps/ose-web`/`apps/ayokoding-web` content readers); leave framework `process.env.PORT` reads as
      framework vars. Run `./node_modules/.bin/nx run-many -t typecheck -p organiclever-web ose-web ayokoding-web ose-app-web wahidyankf-web`
      — all exit 0.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Prove web build-time validation on one app: temporarily set an invalid value for a
      validated `ose-web` var and run `./node_modules/.bin/nx run ose-web:build` — acceptance: build
      fails naming the variable; restore and re-run `./node_modules/.bin/nx run ose-web:build` —
      exits 0.
- [ ] [AI] Verify a backend starts with the renamed+validated vars: start `organiclever-be` locally
      with `ORGANICLEVER_BE_PORT=8299 DATABASE_URL=<local-dev-url>` and run
      `curl -sf http://localhost:8299/health` — acceptance: returns HTTP 200 with a JSON body.

### Manual UI Verification (Playwright MCP)

- [ ] [AI] Start a web dev server: `./node_modules/.bin/nx dev ose-web` (port 3100).
- [ ] [AI] Navigate via `browser_navigate` to `http://localhost:3100`; take `browser_snapshot` —
      acceptance: the page renders; `browser_console_messages` returns zero JavaScript errors.
- [ ] [AI] Repeat the snapshot + console check for `organiclever-web` (`nx dev organiclever-web`, port 3200) — acceptance: renders, zero console errors.

### Manual API Verification (curl)

- [ ] [AI] Start `./node_modules/.bin/nx dev organiclever-be`; run
      `curl -s http://localhost:8202/health | jq .` — acceptance: HTTP 200, JSON body.
- [ ] [AI] Start `./node_modules/.bin/nx dev ose-app-be`; run
      `curl -s http://localhost:8302/health | jq .` — acceptance: HTTP 200, JSON body.

### Phase 4 Gate

> All checks below must pass before starting Phase 5; if any fails, fix it in Phase 4 first.

- [ ] [AI] `tech-docs.md § 8` clearance table filled (exact versions, Path B, CVE status); no
      caret/tilde for the new keys in any manifest; `cargo audit` and `npm audit` clean.
- [ ] [AI] `./node_modules/.bin/nx run-many -t test:quick -p organiclever-be ose-app-be` exits 0,
      coverage at or above threshold; each backend's missing-`DATABASE_URL` test asserts a named-field
      error.
- [ ] [AI] `./node_modules/.bin/nx run-many -t typecheck test:quick -p organiclever-web ose-web ayokoding-web ose-app-web wahidyankf-web`
      exits 0; the `ose-web` build fails on an invalid validated var (then restored to passing).
- [ ] [AI] `npm run lint:md` exits 0.
- [ ] [AI] Commit thematically (`feat(organiclever-be,ose-app-be): fail-fast env validation via envy`;
      `feat(web): build-time env validation via t3-env and zod`) and push; `git status` clean.

> **Pause Safety**: Phase 4 left both backends validating env at startup and every web validating at
> build time, all gates green, deps cleared. Resume by re-running the backend + web `test:quick`.

---

## Phase 5 — `.env.example` Annotation Format

- [ ] [AI] Annotate `apps/organiclever-be/.env.example` and `apps/ose-app-be/.env.example`: above each
      variable add a comment block stating required-or-optional, type, and format (per `tech-docs.md`
      and the hub doc's annotation standard). Example: `# Required. Postgres connection URL.` for
      `DATABASE_URL`; `# Optional. Integer. Backend listen port (default 8302).` for
      `OSE_APP_BE_PORT`; mark `OSE_APP_BE_OPENROUTER_API_KEY` as a secret placeholder.
- [ ] [AI] Annotate `apps/ose-web/.env.example` and `apps/ayokoding-web/.env.example`: same treatment
      for the prefixed content vars and the commented framework `PORT` (optional, integer, Next.js dev
      server).
- [ ] [AI] Verify placeholders are obviously-dev (no real-looking secret): run
      `grep -rnE "secret|token|key|pass" apps/*/.env.example` and confirm every value is a placeholder,
      not a credential.
- [ ] [AI] Run `npm run lint:md` and `npm run format:md:check` — exit 0.

### Phase 5 Gate

> All checks below must pass before starting Phase 6; if any fails, fix it in Phase 5 first.

- [ ] [AI] Every variable in the annotated `.env.example` files has a required/optional + type +
      format comment.
- [ ] [AI] `npm run lint:md` exits 0.
- [ ] [AI] Commit (`docs(env): annotate env example files with type and required status`) and push;
      `git status` clean.

> **Pause Safety**: Phase 5 left the env templates self-documenting; no code touched. Resume by
> re-reading the annotated files (no command needed).

---

## Phase 6 — `env validate` Drift Guard (app validator; IaC scaffold commented) + CI Wiring

- [ ] [AI] Inspect rhino-cli's existing subcommand + config layout (`apps/rhino-cli/src/`,
      `cli.rs:122-131`) to match the established pattern (clap subcommand module, config source).
      Decide the config surface (`env-contract.yaml` parsed with rhino-cli's existing YAML support —
      no new `toml` crate) against that pattern — acceptance: record the chosen config approach as a
      `// ENV-VALIDATE CONFIG: <choice>` comment at the top of the new
      `apps/rhino-cli/src/commands/env_validate.rs`; verify the comment exists and names the chosen
      format. The contract lists **surfaces**, each with a root, a kind (`app`; `terraform`/`ansible`
      documented but commented), globs, and an allowlist (`tech-docs.md § 4.3`).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: write failing unit tests in `apps/rhino-cli/src/` (in-memory fixtures) for the
      app validator: (a) a fixture app with a seeded declared-but-unread key causes non-zero exit
      naming the key; (b) a fixture app with a read-but-undeclared key causes non-zero exit naming the
      key; (c) a matching fixture exits 0. Run `./node_modules/.bin/nx run rhino-cli:test:unit` —
      acceptance: all new tests fail (validator not implemented).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN — implement the app validator**: register `Validate(env_validate::EnvValidateArgs)`
      in `cli.rs`'s `EnvCommands` enum and dispatch; parse `apps/<app>/.env.example` declared keys;
      scan Rust (`env::var("…")` literals + `envy` struct field names) and TS (`process.env.X` +
      `createEnv` keys) for read keys; compute declared-but-unread and read-but-undeclared sets; honor
      the allowlist; exit non-zero with named keys on any non-empty set. Ship the Terraform and Ansible
      validator branches **commented** with an `// activate when IaC is added` marker. Run
      `./node_modules/.bin/nx run rhino-cli:test:unit` — acceptance: app-validator RED tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Write integration tests (`cargo test --tests`) with temp-dir fixtures: an app with a seeded
      mismatch (non-zero + key named); a matching app (exit 0). Run
      `./node_modules/.bin/nx run rhino-cli:test:quick` — exits 0, coverage at or above threshold.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Run `rhino-cli env validate` against the real repo — exits 0 on all app surfaces (Phases
      1–5 aligned the apps; allowlist the framework-injected web `PORT`).
- [ ] [AI] Add `rhino-cli env validate` to `.husky/pre-push`. Verify by running the pre-push script
      body locally — it invokes the command and passes.
- [ ] [AI] Add a CI invocation: create `.github/workflows/validate-env.yml` (or add a step to an
      existing quality-gate workflow, matching the repo's workflow layout) that runs
      `rhino-cli env validate` on `pull_request`. Validate the YAML against the repo's workflow
      conventions.
- [ ] [AI] Prove the guard bites: temporarily rename a key in `apps/ose-app-be/.env.example` without
      updating the code read — `rhino-cli env validate` exits non-zero naming the key; revert.

### Phase 6 Gate

> All checks below must pass before starting Phase 7; if any fails, fix it in Phase 6 first.

- [ ] [AI] `./node_modules/.bin/nx run rhino-cli:test:quick` exits 0, coverage at or above threshold.
- [ ] [AI] `rhino-cli env validate` exits 0 on the clean repo and non-zero on a seeded app mismatch.
- [ ] [AI] `.husky/pre-push` and a `.github/workflows/` file both invoke the command; the
      Terraform/Ansible branches are present but commented with the activation marker.
- [ ] [AI] `npm run lint:md` exits 0.
- [ ] [AI] Commit (`feat(rhino-cli): add env validate drift guard for apps`) and push; `git status`
      clean.

> **Pause Safety**: Phase 6 left a working app drift guard enforced by pre-push and CI, with the IaC
> branches staged-but-inactive, repo passing. Resume by running `rhino-cli env validate`.

---

## Phase 7 — Hub Convention Doc + Stub Redirects + Rationale Doc + Link Repointing

- [ ] [AI] Create `repo-governance/conventions/security/secrets-and-env-standards.md` — the hub
      convention: principles, naming standard (with framework exemptions + the per-app prefix rule),
      layout standard (single template per app under `apps/<app>/`; real-file relocation is [HUMAN]),
      `.env.example` annotation format, startup-validation expectations per language
      (`dotenvy`+`envy`; `@t3-oss/env-nextjs`+`zod`), the `rhino-cli env` family
      (backup/restore/init/validate — including the full-secret-floor backup and the app validator),
      the storage-tier ladder + Tier-1 trigger, and the **IaC forward-scaffold** note (Terraform/
      Ansible patterns documented but inactive). Fold the substantive content of the three existing
      docs into it.
  - _Suggested executor: `docs-maker`_
- [ ] [AI] In the hub doc, add the canonical **secret-surface census** table: one row per secret kind
      — `apps/<app>/.env.local`, `.secrets/`, `secrets.json` (active), and `*.tfvars`/inventory
      (commented forward-scaffold) — each with its path, consuming tool, and whether it is backed up
      and/or validated. Document the **hybrid backup** source-of-truth (hardcoded floor ∪
      `env-contract.yaml` `backup_globs`) and note the cross-repo doc-name difference
      (`no-secrets-in-git.md` here vs `no-secrets-in-committed-files.md` in siblings).
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Reduce `repo-governance/conventions/security/no-secrets-in-git.md` to a stub: keep its
      title + a one-paragraph summary of the hard iron rule (so the rule stays greppable) and link to
      the hub doc as the authoritative source.
- [ ] [AI] Reduce `repo-governance/conventions/security/env-file-access.md` to a stub redirecting to
      the hub (preserve the `guard-env-file-access` policy summary so enforcement rationale stays
      discoverable).
- [ ] [AI] Reduce `repo-governance/development/workflow/reproducible-environments.md` to a stub
      redirecting to the hub (preserve the `.env.example` pattern summary).
- [ ] [AI] Repoint `repo-governance/conventions/security/README.md` to the hub doc as the primary
      secrets/env reference (update the two existing convention bullets to mention the hub).
- [ ] [AI] Write `docs/explanation/standardize-secrets-and-env-parity-decisions.md` explaining each
      cross-repo decision (the full 14-decision matrix from `tech-docs.md § 9`), emphasizing public's
      deviations: no IaC (commented scaffold), the `no-secrets-in-git.md` doc-name, building on prior
      env-backup/guard work, and the layout consolidation. Cross-link the sibling plans
      (`tech-docs.md § 9` / README "Sibling Plans"). Match the structure of the existing
      `docs/explanation/plan-domain-parity-decisions.md` precedent.
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Repoint **active** inbound links to the hub doc — update CLAUDE.md, AGENTS.md, the
      `repo-governance/conventions/README.md` + `development/*/README.md` indexes, `docs/` references,
      and any `.claude/skills/` / `.claude/agents/` references found in an inbound-link sweep. Leave
      `plans/done/**` links on the stubs (historical, must not be rewritten).
- [ ] [AI] Run `npm run generate:bindings` if any `.claude/` agent/skill text changed, to resync
      `.opencode/`.
- [ ] [AI] Run `npm run lint:md` — exits 0 (no broken links from the fold). Then run the inbound-link
      verification:
      `grep -rl "no-secrets-in-git\|env-file-access\|reproducible-environments" --include="*.md" . | grep -v node_modules | grep -v plans/done`
      — every remaining active hit either is a stub file itself or also links the hub doc.

### Phase 7 Gate

> All checks below must pass before starting Phase 8; if any fails, fix it in Phase 7 first.

- [ ] [AI] `secrets-and-env-standards.md` exists; the three prior docs are stubs linking to it;
      `security/README.md` references the hub; the parity-decisions doc exists.
- [ ] [AI] `npm run lint:md` exits 0 (link check passes; no `done/` link broken).
- [ ] [AI] If `.claude/` changed, `.opencode/` is in sync (`git status` shows matching regenerated
      files).
- [ ] [AI] Commit (`docs(governance): consolidate secrets/env rules into one hub convention`) and
      push; `git status` clean.

> **Pause Safety**: Phase 7 left one authoritative hub doc with the three prior docs redirecting,
> `security/README.md` repointed, the rationale doc written, and all links intact. Resume by
> re-running `npm run lint:md`.

---

## Phase 8 — Final Quality Gate + Commit + Push

- [ ] [AI] Run the full affected gate:
      `./node_modules/.bin/nx affected -t typecheck lint test:quick spec-coverage` across `main` —
      all exit 0.
- [ ] [AI] Run `rhino-cli env validate` — exits 0 across all app surfaces.
- [ ] [AI] Run `npm run lint:md` and `npm run format:md:check` — exit 0.
- [ ] [AI] Re-verify every BRD success criterion: per-app naming applied with zero residue; both
      backends validate startup; every web validates at build time; the drift guard is wired and
      bites; the hub doc exists with the three stubs + `security/README.md` repoint; layout
      consolidated; backup covers `.env*`/`.secrets/`/`secrets.json` with `--dry-run`; deps
      exact-pinned + cleared; the rationale doc exists.
- [ ] [AI] Confirm all per-phase commits landed on `origin main`:
      `git log --oneline origin/main -15` shows the Phase 1–7 commits; `git status` clean, nothing
      unpushed.

### Post-Push CI Verification

- [ ] [AI] Monitor the GitHub Actions workflows triggered by the pushes (including the new
      `validate-env` workflow on any PR path).
- [ ] [AI] Verify all CI checks pass — no exceptions. If any fails, fix at root cause and push a
      follow-up commit; repeat until green. Do NOT archive while CI is red.

### Phase 8 Gate

> All checks below must pass before archiving this plan; if any fails, fix it in Phase 8 first.

- [ ] [AI] `./node_modules/.bin/nx affected -t typecheck lint test:quick spec-coverage` exits 0.
- [ ] [AI] `rhino-cli env validate` exits 0; `npm run lint:md` exits 0.
- [ ] [AI] Every BRD success criterion verified true.
- [ ] [AI] Working tree clean; all phase commits pushed to `origin main`; CI green.

> **Pause Safety**: Phase 8 is terminal — the standard is live and self-enforcing. The plan is ready
> for archival.

---

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Verify ALL manual assertions pass (Playwright MCP for webs / curl for backends).
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/standardize-secrets-and-env/ plans/done/2026-MM-DD__standardize-secrets-and-env/`
      using today's completion date (NOT the creation date).
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Update any other READMEs that reference this plan.
- [ ] [AI] Commit the archival: `chore(plans): move standardize-secrets-and-env to done`.
