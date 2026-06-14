# Delivery Checklist

Phased, executor-tagged, TDD-shaped. Each phase ends with a gate that must pass before the next begins.
`[AI]` = an agent/automation can do it. `[HUMAN]` = requires dashboard access, secrets, or a judgment
call. Workflow YAML is "tested" by `actionlint`, the naming grep, `links:validation`, and (where
possible) a `workflow_dispatch` dry run.

Legend: **RED** = add/leave a check that currently fails → **GREEN** = make the change so it passes →
**REFACTOR** = dedupe/tidy without changing behavior.

Decisions are **resolved** (see [tech-docs Resolved decisions](./tech-docs.md#resolved-decisions)):
full `commons-quality-gate` rename + branch-protection update; app deploy force-pushes web **and** be
stag branches with a separate be-build-deploy workflow; `organiclever-www-e2e` split into the
`-be-e2e`/`-fe-e2e` pair.

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
      table, Naming Conventions table, the CRON-schedule section (2.5 h staging→prod gap), and the
      Invariant-A row (correct the inaccurate claim that `rhino-cli:naming:workflows-validation` enforces
      `.github/workflows` filenames; it validates `repo-governance/workflows/*.md` only).
- [ ] **Gate 1**: `npm run lint:md` + `links:validation` green; both convention docs describe the
      after-state including the deploy-as-branch-push model.

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

## Phase 5 — backend build-deploy + cross-cutting renames `[AI]` + `[HUMAN]`

- [ ] 5.1 **GREEN**: create `organiclever-be-build-deploy-stag.yml` (on push `stag-organiclever-be`) and
      `ose-be-build-deploy-stag.yml` (on push `stag-ose-be`), each calling `_reusable-be-build-deploy.yml`
      with its `be-project` + `image-name`. `git rm .github/workflows/publish-images.yml` once its job is
      fully absorbed.
- [ ] 5.2 `[HUMAN]` **Cross-repo coordination**: confirm ose-infra `coralpolyp` is updated to consume the
      branch-triggered GHCR images before removing the old main-push publish. If not ready, keep
      `publish-images.yml`'s `main` trigger transitionally (documented) and defer its removal.
- [ ] 5.3 `[HUMAN]` **Branch protection**: prepare to update the `main` required-status-check binding so
      it points at the renamed `commons-quality-gate` check, applied in the **same** window as 5.4.
- [ ] 5.4 **GREEN**: `git mv pr-quality-gate.yml commons-quality-gate.yml` (+ `name:`);
      `git mv validate-env.yml commons-env-validate.yml`;
      `git mv validate-markdown.yml markdown-validate.yml`;
      `git mv test-crane-cli-integration.yml crane-cli-test-local.yml`. Update each `name:`.
- [ ] 5.5 **Verify**: `actionlint .github/workflows/*.yml` clean; every `name:` derives to its filename.
- [ ] **Gate 5**: all 18 workflows follow the grammar; actionlint clean; branch-protection update staged.

## Phase 6 — reference sweep + wire-vercel reduction + READMEs `[AI]`

- [ ] 6.1 **RED**: `git grep -nE '(pr-quality-gate|validate-markdown|validate-env|publish-images|test-crane-cli-integration|test-and-deploy-[a-z-]+|test-[a-z-]+-web-staging|deploy-[a-z-]+-to-production)\.yml' -- ':!plans/done/**'`
      lists every doc still naming an old file (the failing set).
- [ ] 6.2 **GREEN**: update `.github/README.md`, `.github/workflows/README.md`, `.github/actions/README.md`
      (workflow tables), `docs/reference/system-architecture/ci-cd.md`, and any agent definition that
      names a workflow (e.g. `apps-organiclever-web-deployer`, which targets the renamed promotion
      workflow) — then `npm run generate:bindings` if any `.claude/agents/**` changed.
- [ ] 6.3 **GREEN**: reduce `wire-vercel-www-app-cutover` — remove `.github/workflows` items from its
      Scope/tech-docs/delivery, add the `stag-*-be` / `prod-*-be` branches to its branch-creation list,
      and point its workflow section at this plan. Keep its Vercel/DNS/Environment/branch-creation steps.
- [ ] 6.4 **Verify**: the RED grep from 6.1 returns only intentional historical mentions (none active).
- [ ] **Gate 6**: `links:validation`, `headings:hierarchy-validation`, `lint:md` green;
      `generate:bindings` idempotent (no residual diff).

## Phase 7 — final verification + delivery `[AI]` + `[HUMAN]`

- [ ] 7.1 `[AI]` Full gate: `actionlint .github/workflows/*.yml`; `npm run lint:md`;
      `npx nx run rhino-cli:links:validation`; `npx nx run rhino-cli:headings:hierarchy-validation`;
      the prd.md validation grep returns clean.
- [ ] 7.2 `[HUMAN]` Apply the branch-protection update (5.3) so the `commons-quality-gate` check is the
      required one, then dispatch a dry run of one www caller and one app caller via `workflow_dispatch`;
      confirm each reaches its deploy/stop step without a wiring error (a failed `git push` to a
      not-yet-created branch is the expected, acceptable outcome until wire-vercel runs).
- [ ] 7.3 `[HUMAN]` Authorize commit + push. Stage **explicit paths** (no `git add -A`). Split commits:
      (a) `docs(ci)` convention + plan, (b) `ci` workflow renames/restructure + e2e split, (c) `docs`
      reference sweep + wire-vercel reduction, (d) `chore` `generate:bindings` output if any.
- [ ] 7.4 `[AI]` After push, verify `HEAD == origin/main`, tree clean, and the `commons-quality-gate`
      status check runs and passes on the push.
- [ ] **Gate 7**: origin/main updated; all gates green; branch protection points at the renamed check;
      wire-vercel unblocked.

## Notes

- Branches `prod-*-www`, `stag-*-app-web`, `stag-*-be` (and the deferred `prod-*-app-web` / `prod-*-be`)
  are **created by wire-vercel**, not here. Scheduled runs that push to them will fail loudly until then —
  expected and non-destructive.
- The `publish-images` → branch-triggered `*-be-build-deploy-stag` swap is **cross-repo** (ose-infra
  `coralpolyp`). Do not remove the old main-push publish until coralpolyp consumes the new source.
- Staging URLs/secrets are never committed — placeholder/secret only; Environment values are a
  wire-vercel `[HUMAN]` step.
