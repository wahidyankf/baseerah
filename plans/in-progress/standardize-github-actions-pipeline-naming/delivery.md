# Delivery Checklist

Phased, executor-tagged, TDD-shaped. Each phase ends with a gate that must pass before the next begins.
`[AI]` = an agent/automation can do it. `[HUMAN]` = requires dashboard access, secrets, or a judgment
call. Workflow YAML is "tested" by `actionlint`, the naming grep, `links:validation`, and (where
possible) a `workflow_dispatch` dry run.

Legend: **RED** = add/leave a check that currently fails → **GREEN** = make the change so it passes →
**REFACTOR** = dedupe/tidy without changing behavior.

Decisions are **resolved** (see [tech-docs Resolved decisions](./tech-docs.md#resolved-decisions)):
(D1) full `commons-quality-gate` rename + branch-protection update; (D2) app deploy force-pushes web
**and** be stag branches with a separate be-build-deploy workflow; (D3) `organiclever-www-e2e` split
into the `-be-e2e`/`-fe-e2e` pair; (D4) the tiered env/secret injection standard + value-less
`env-injection.yaml` manifest (references here, values in wire-vercel); (D5) no `test:integration`/
`test:e2e` in the fast gates, and `test-crane-cli-integration.yml` deleted (service-only scope). All
human-gated work is batched into the final **Phase 9**.

---

## Phase 0 — Setup & baseline `[AI]` (repo-setup-manager)

- [ ] 0.1 `npm install` and `npm run doctor -- --fix` (Node + .NET + Rust toolchain present).
- [ ] 0.2 Baseline green: `actionlint .github/workflows/*.yml`, `npm run lint:md`,
      `npx nx run rhino-cli:links:validation`. Record any pre-existing failures and fix them first
      (root-cause, do not defer).
- [ ] 0.3 Snapshot the current inventory: `ls .github/workflows .github/actions` saved to the plan PR
      description for before/after diffing.
- [ ] **Gate 0**: baseline commands green; inventory recorded.

## Phase 1 — Convention `[AI]`

- [ ] 1.1 **RED**: edit `repo-governance/development/infra/github-actions-workflow-naming.md` to define
      the domain-first `{domain}-{action-chain}` grammar, the cross-cutting keyword list
      (`commons`/`markdown`/`docs`/`{cli}`), the verb vocabulary (incl. `deploy-stag`/`deploy-prod` =
      branch force-push, `build-deploy-*` for backends), and the reusable/composite-action exemptions.
      Replace the stale "Complete Codebase Reference" table with the after-state filenames.
      _Acceptance_: the doc lists every target filename from tech-docs.
- [ ] 1.2 **GREEN**: align `repo-governance/development/infra/ci-conventions.md` — File Organisation
      table, Naming Conventions table, the CRON-schedule section (2.5 h staging→prod gap), the
      Invariant-A row (correct the inaccurate claim that `rhino-cli:naming:workflows-validation` enforces
      `.github/workflows` filenames; it validates `repo-governance/workflows/*.md` only), and a **new
      invariant**: `test:integration`/`test:e2e` run only in the scheduled tiered pipelines, never in
      `commons-quality-gate`, `.husky/pre-commit`, or `.husky/pre-push`.
- [ ] 1.3 **GREEN (injection standard)**: add a "Tiered injection" section to
      `repo-governance/conventions/security/secrets-and-env-standards.md` — the variable classes
      (app-runtime server/public, CI test-harness, platform-injected), the app × stage × platform
      injection matrix, the GitHub Environment ↔ key registry, the Vercel target mapping
      (`prod-*`→Production, `stag-*`→Preview), the k3s/coralpolyp contract boundary, and the
      value-less `env-injection.yaml` manifest. Extend the §7 census with the GitHub/Vercel/k3s rows.
      _Acceptance_: the doc matches [tech-docs §Tiered injection](./tech-docs.md#tiered-env--secret-injection-standard).
- [ ] **Gate 1**: `npm run lint:md` + `links:validation` green; the convention docs describe the
      after-state including the deploy-as-branch-push model and the tiered injection standard.

## Phase 2 — Reusable workflows `[AI]`

- [ ] 2.1 **RED→GREEN**: `git mv .github/workflows/_reusable-test-and-deploy.yml`
      `.github/workflows/_reusable-www-test-local-deploy.yml`; update its `name:`; keep the uniform
      `{app}-be-e2e`+`{app}-fe-e2e` runner pair (be-e2e tolerant of absence via `|| true`).
      _Command_: `actionlint <file>`. _Acceptance_: actionlint clean; `name:` derives to filename.
- [ ] 2.2 **GREEN**: create `_reusable-app-test-local-deploy-stag.yml` factoring the be+fe
      integration/e2e job graph + the **dual-branch deploy** (force-push `stag-web-branch` **and**
      `stag-be-branch`) out of the two app dev workflows. Inputs: `web-project`, `be-project`,
      `contracts-project`, `compose-dir`, `stag-web-branch`, `stag-be-branch`, `be-port`, `web-port`,
      `environment`. _Acceptance_: actionlint clean; inputs cover both groups.
- [ ] 2.3 **GREEN**: create `_reusable-app-test-stag.yml` factoring the staging-e2e job
      (`fe-e2e-project`, `environment`, web-base-url var); **no** promote job. _Acceptance_: actionlint clean.
- [ ] 2.4 **GREEN**: create `_reusable-be-build-deploy.yml` by lifting `publish-images.yml`'s per-be
      GHCR build+push job (inputs: `be-project`, `image-name`, `environment`). _Acceptance_: actionlint clean.
- [ ] **Gate 2**: `actionlint .github/workflows/*.yml` clean; four reusables present and well-formed.

## Phase 3 — www tier + e2e split `[AI]`

- [ ] 3.1 **RED**: write/extend a failing check — `git grep -n 'app-name: \(ose\|ayokoding\|wahidyankf\)-web'`
      returns the three stale callers; `nx show project organiclever-www-be-e2e` fails (not split yet).
- [ ] 3.2 **GREEN (e2e split)**: split `apps/organiclever-www-e2e` into `organiclever-www-be-e2e` +
      `organiclever-www-fe-e2e` (new `project.json`s, move specs/steps, register in Nx). _Acceptance_:
      `nx show project organiclever-www-be-e2e` and `…-fe-e2e` both resolve; `nx run
organiclever-www-fe-e2e:test:e2e` is wired.
- [ ] 3.3 **GREEN**: `git mv` + rewrite the three stale callers to the new filename,
      `app-name: {site}-www`, `prod-branch: prod-{site}-www`, calling `_reusable-www-test-local-deploy.yml`:
      `ose-www-test-local-deploy-prod.yml`, `ayokoding-www-test-local-deploy-prod.yml`,
      `wahidyankf-www-test-local-deploy-prod.yml`.
- [ ] 3.4 **GREEN**: create `organiclever-www-test-local-deploy-prod.yml` (→ `prod-organiclever-www`)
      **and** `infra/dev/organiclever-www/{docker-compose.yml,docker-compose.ci.yml,.env.example}`.
      _Acceptance_: `docker compose -f infra/dev/organiclever-www/docker-compose.yml config` valid.
- [ ] 3.5 **REFACTOR**: confirm all four callers are thin (~15 lines) and identical in shape.
- [ ] 3.6 **Verify**: the RED checks from 3.1 now pass/return nothing.
- [ ] **Gate 3**: actionlint clean; four www callers exist; `organiclever-www-{be,fe}-e2e` resolve; no
      `*-web` project references remain in www workflows; compose config valid.

## Phase 4 — app tier `[AI]`

- [ ] 4.1 **GREEN**: `git mv` + rewrite `test-and-deploy-organiclever-web-development.yml` →
      `organiclever-app-test-local-deploy-stag.yml`, calling `_reusable-app-test-local-deploy-stag.yml`;
      `stag-web-branch: stag-organiclever-app-web`, `stag-be-branch: stag-organiclever-be`; env
      `organiclever-app-local` (or omit if empty — no `development` stage); CRON 03:00/15:00 WIB.
- [ ] 4.2 **GREEN**: `git mv` + rewrite `test-organiclever-web-staging.yml` →
      `organiclever-app-test-stag-deploy-prod.yml`, calling `_reusable-app-test-stag.yml`; env
      `organiclever-app-staging`; **CRON 05:30/17:30 WIB (+2.5 h after stag)**; **no prod push**.
- [ ] 4.3 **GREEN**: `git rm .github/workflows/deploy-organiclever-web-to-production.yml` (prod CD
      deferred). Note the removal + future-CD-plan pointer in tech-docs.
- [ ] 4.4 **GREEN**: repeat 4.1–4.3 for ose-app: `test-and-deploy-ose-app-web-development.yml` →
      `ose-app-test-local-deploy-stag.yml` (`stag-be-branch: stag-ose-be`, env `ose-app-local`);
      `test-ose-app-web-staging.yml` → `ose-app-test-stag-deploy-prod.yml` (+2.5 h, env
      `ose-app-staging`); `git rm deploy-ose-app-web-to-production.yml`.
- [ ] 4.5 **REFACTOR**: confirm the two `*-local-deploy-stag` and two `*-test-stag-deploy-prod` callers
      differ only by inputs.
- [ ] **Gate 4**: actionlint clean; four app callers exist; two prod-dispatch workflows gone; CRON gap is
      2.5 h; no `stag-organiclever-web` / `organiclever-web-*` env references remain.

## Phase 5 — backend build-deploy + cross-cutting renames `[AI]`

> All human-gated actions (coralpolyp coordination, `publish-images.yml` removal, branch protection)
> are **deferred to the consolidated `[HUMAN]` hand-off in Phase 9** — this phase is fully `[AI]`.

- [ ] 5.1 **GREEN**: create `organiclever-be-build-deploy-stag.yml` (on push `stag-organiclever-be`) and
      `ose-be-build-deploy-stag.yml` (on push `stag-ose-be`), each calling `_reusable-be-build-deploy.yml`
      with its `be-project` + `image-name`. **Leave `publish-images.yml` in place** — its removal is
      cross-repo-gated (coralpolyp) and happens in Phase 9. _Acceptance_: both new workflows exist;
      actionlint clean.
- [ ] 5.2 **GREEN (cross-cutting gate renames)**: `git mv pr-quality-gate.yml commons-quality-gate.yml`
      (+ `name:`); `git mv validate-env.yml commons-env-validate.yml`;
      `git mv validate-markdown.yml markdown-validate.yml`. Update each `name:`. (The required-status-check
      binding that depends on the `commons-quality-gate` rename is updated by a human in Phase 9, in the
      same window as the push.)
- [ ] 5.3 **GREEN (D5 — delete crane-cli CI)**: `git rm .github/workflows/test-crane-cli-integration.yml`
      — CLI is not a service; CLI-tool CI is out of scope this PR (revisited later). This also removes the
      only integration suite that ran on `pull_request`. _Acceptance_: the file is gone; no workflow runs
      `crane-cli:test:integration`.
- [ ] 5.4 **Verify (no heavy tests in gates)**: `commons-quality-gate.yml`, `.husky/pre-commit`, and
      `.husky/pre-push` contain **no** `test:integration` / `test:e2e` invocation —
      `git grep -nE 'test:(integration|e2e)' -- .github/workflows/commons-quality-gate.yml .husky/`
      returns nothing.
- [ ] 5.5 **Verify**: `actionlint .github/workflows/*.yml` clean; every `name:` derives to its filename.
- [ ] **Gate 5**: the two be-build-deploy workflows exist; cross-cutting gates renamed;
      `test-crane-cli-integration.yml` deleted; actionlint clean; no `test:integration`/`test:e2e` in any
      fast gate (PR gate, pre-commit, pre-push). (`publish-images.yml` still present — removed in Phase 9.)

## Phase 6 — env/secret injection manifest + validate extension `[AI]`

- [ ] 6.1 **RED**: write a failing consistency check expectation — `test -f env-injection.yaml` fails
      (manifest absent) and `git grep -nE 'WEB_BASE_URL|VERCEL_AUTOMATION_BYPASS_SECRET' -- 'apps/*/.env.example'`
      currently finds nothing **and must keep finding nothing** (CI test-harness keys stay out of app
      templates).
- [ ] 6.2 **GREEN (manifest)**: create `env-injection.yaml` at repo root — per-app injection homes
      (`runtime: {local, local-ci, staging, production}` → `env-local`/`compose`/`vercel-preview`/`vercel-production`/`k3s-coralpolyp`),
      `keys-from: apps/<app>/.env.example`, and the `ci-harness` registry (`WEB_BASE_URL`,
      `VERCEL_AUTOMATION_BYPASS_SECRET` → `{group}-app-staging`). Names only, **no values**.
      _Acceptance_: every app in `env-contract.yaml` has an `env-injection.yaml` entry.
- [ ] 6.3 **GREEN (validate extension)**: extend the **existing** `rhino-cli env validate` command (not
      a separate target) with a static, value-free pass: every app-runtime key declared in `.env.example`
      has a documented home at each stage the app runs; every `ci-harness` key is registered and absent
      from all `.env.example`. No new Nx target — `env validate` is already wired into
      `commons-env-validate.yml` and `.husky/pre-push`, so the new pass rides along. _Acceptance_:
      `npx nx run rhino-cli:env:validation` passes; a deliberately-mismatched fixture fails it (TDD).
- [ ] 6.4 **GREEN (`.env.example` normalize)**: align every `apps/<app>/.env.example` to the injection
      variable classes — annotate server vs `NEXT_PUBLIC_*` public keys, confirm no CI test-harness key
      is present, keep `env-contract.yaml` allowlists in step. _Acceptance_: `rhino-cli env validate`
      green for all surfaces.
- [ ] 6.5 **GREEN (infra/dev rename + compose env)**: `git mv infra/dev/organiclever
infra/dev/organiclever-app` (the stack serves the app group: `organiclever-be` +
      `organiclever-app-web`; gitignored `.env` rides along), and confirm the new
      `infra/dev/organiclever-www/` stack sources keys from the app `.env.example` (placeholders only) —
      no duplicate template (§3). Repoint every `compose-dir` workflow input and doc reference to the new
      paths. _Acceptance_: `docker compose -f infra/dev/organiclever-app/docker-compose.yml config` valid.
- [ ] 6.6 **Verify**: RED checks from 6.1 now pass; no CI test-harness key sits in any `.env.example`;
      no `infra/dev/organiclever/` references remain.
- [ ] **Gate 6**: `env-injection.yaml` present + value-less; `rhino-cli:env:validation` green; injection
      doc (Phase 1.3) and manifest agree.

## Phase 7 — reference sweep + wire-vercel reduction + READMEs `[AI]`

- [ ] 7.1 **RED**: `git grep -nE '(pr-quality-gate|validate-markdown|validate-env|publish-images|test-crane-cli-integration|test-and-deploy-[a-z-]+|test-[a-z-]+-web-staging|deploy-[a-z-]+-to-production)\.yml' -- ':!plans/done/**'`
      lists every doc still naming an old file (the failing set).
- [ ] 7.2 **GREEN (renamed-file sweep)**: update `.github/README.md`, `.github/workflows/README.md`,
      `.github/actions/README.md` (workflow tables), `docs/reference/system-architecture/ci-cd.md`, and
      any agent definition that names a workflow (e.g. `apps-organiclever-web-deployer`, which targets the
      renamed promotion workflow) — then `npm run generate:bindings` if any `.claude/agents/**` changed.
- [ ] 7.3 **GREEN (env-injection governance sweep)**: update every related governance `.md`/rule so the
      injection standard is consistent repo-wide —
      `repo-governance/conventions/security/{secrets-and-env-standards,env-file-access,no-secrets-in-committed-files,README}.md`,
      `repo-governance/conventions/README.md`, `env-contract.yaml` (cross-ref `env-injection.yaml`),
      `repo-governance/development/infra/ci-conventions.md`,
      `repo-governance/development/workflow/reproducible-environments.md`,
      `docs/reference/system-architecture/ci-cd.md`, and `AGENTS.md` if its env/secret notes need the
      injection cross-link. Re-sync bindings if any `.claude/**` changed.
- [ ] 7.4 **GREEN**: reduce `wire-vercel-www-app-cutover` — remove `.github/workflows` items from its
      Scope/tech-docs/delivery, add the `stag-*-be` / `prod-*-be` branches to its branch-creation list,
      add the value-population step driven by `env-injection.yaml`, and point its workflow section at this
      plan. Keep its Vercel/DNS/Environment/branch-creation steps.
- [ ] 7.5 **Verify**: the RED grep from 7.1 returns only intentional historical mentions (none active).
- [ ] **Gate 7**: `links:validation`, `headings:hierarchy-validation`, `lint:md` green;
      `generate:bindings` idempotent (no residual diff); injection standard consistent across all docs.

## Phase 8 — final verification `[AI]`

- [ ] 8.1 `[AI]` Full gate: `actionlint .github/workflows/*.yml`; `npm run lint:md`;
      `npx nx run rhino-cli:links:validation`; `npx nx run rhino-cli:headings:hierarchy-validation`;
      `npx nx run rhino-cli:env:validation`; the prd.md validation grep returns clean.
- [ ] 8.2 `[AI]` Confirm everything human-gated is staged and ready (nothing left mid-flight): the two
      be-build-deploy workflows exist, `commons-quality-gate.yml` is renamed, `publish-images.yml` is
      still present (its removal is Phase 9), and the commit set is split and ready to push.
- [ ] **Gate 8**: all automated gates green; the tree is in a clean, push-ready state; the only
      remaining work is the consolidated `[HUMAN]` hand-off in Phase 9.

## Phase 9 — consolidated `[HUMAN]` hand-off (all human steps batched here)

> **Every human-only action in this plan is gathered here**, at the very end, so all `[AI]` work
> (Phases 0–8) completes first and the human does one contiguous batch. None of these can be automated:
> each needs cross-repo confirmation, repo-admin settings, or push authorization.

- [ ] 9.1 `[HUMAN]` **Cross-repo coordination + `publish-images.yml` removal**: confirm ose-infra
      `coralpolyp` consumes the new branch-triggered GHCR images, **then** `git rm
.github/workflows/publish-images.yml`. If coralpolyp is not ready, leave `publish-images.yml` in
      place (transitional) and track its removal as a follow-up — do not remove it blind.
- [ ] 9.2 `[HUMAN]` **Branch protection**: update the `main` required-status-check binding to the renamed
      `commons-quality-gate` check, in the **same** window as the push (9.4), so `main` stays gated.
- [ ] 9.3 `[HUMAN]` **Dry run**: dispatch one www caller and one app caller via `workflow_dispatch`;
      confirm each reaches its deploy/stop step without a wiring error (a failed `git push` to a
      not-yet-created branch is the expected, acceptable outcome until wire-vercel runs).
- [ ] 9.4 `[HUMAN]` **Authorize commit + push**. Stage **explicit paths** (no `git add -A`). Split
      commits: (a) `docs(ci)` convention + plan, (b) `ci` workflow renames/restructure + e2e split,
      (c) `feat(ci)` `env-injection.yaml` manifest + `env:validation` extension, (d) `docs` reference +
      env-injection governance sweep + wire-vercel reduction, (e) `chore` `generate:bindings` output if any.
- [ ] 9.5 `[AI]` After push, verify `HEAD == origin/main`, tree clean, and the `commons-quality-gate`
      status check runs and passes on the push.
- [ ] **Gate 9**: origin/main updated; branch protection points at the renamed check; `publish-images.yml`
      resolved (removed, or tracked as a coralpolyp-gated follow-up); env-injection manifest + standard
      landed; wire-vercel unblocked.

## Notes

- Branches `prod-*-www`, `stag-*-app-web`, `stag-*-be` (and the deferred `prod-*-app-web` / `prod-*-be`)
  are **created by wire-vercel**, not here. Scheduled runs that push to them will fail loudly until then —
  expected and non-destructive.
- The `publish-images` → branch-triggered `*-be-build-deploy-stag` swap is **cross-repo** (ose-infra
  `coralpolyp`). Do not remove the old main-push publish until coralpolyp consumes the new source.
- Staging URLs/secrets are never committed — placeholder/secret only; Environment values are a
  wire-vercel `[HUMAN]` step.
