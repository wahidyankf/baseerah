# Delivery Checklist — Unify Web UI Kit and Deploy Storybook

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase is
> not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

**This plan executes DIRECTLY ON `main` — NO worktree, NO PR (per explicit user override).**

This overrides the plan-execution worktree default. Rationale: the user instructed "this plan will
assume we will do it directly to main branch" and "commit and push all to origin main". Work is
committed in thematic [Conventional Commits](../../../repo-governance/development/workflow/commit-messages.md)
straight to `main` and pushed with `git push origin HEAD:main` (lands via the Quality-gate bypass).
No `worktrees/` directory is provisioned for this plan.

> **Note**: This plan is DIRECT TO `main` — no worktree is provisioned. The standard provisioning
> command (`claude --worktree unify-web-ui-kit-and-deploy-storybook`) is intentionally NOT used.
> See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
> and [Plans Organization Convention §Worktree
> Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).
> For plan-execution Step 0: do NOT provision or enter a worktree for this plan. Confirm the
> working tree is on `main` and clean, then proceed with Phase 0 in the root checkout.

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Confirm the working tree is on `main` and clean: `git rev-parse --abbrev-ref HEAD`
      — acceptance: prints `main`; `git status --porcelain` is empty
- [ ] [AI] Install dependencies in the root checkout: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized
- [ ] [AI] Converge the toolchain: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift
- [ ] [AI] Record the baseline for affected projects:
      `npx nx run-many -t typecheck lint test:quick specs:coverage --projects=web-ui,web-ui-token,ose-www,ayokoding-www,organiclever-www,ose-app-web,organiclever-app-web,wahidyankf-www`
      — acceptance: baseline pass/fail recorded; every preexisting failure documented
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no unresolved preexisting failures remain

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `git rev-parse --abbrev-ref HEAD` prints `main` and `git status --porcelain` is empty
- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [ ] [AI] The run-many baseline above is recorded and every preexisting failure is resolved (zero unresolved)

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no feature work
> exists yet. Safe to stop indefinitely. To resume: re-run the run-many baseline command and confirm
> it is still clean.

## Phase 1: web-ui Primitives Layer + Exact-Pinned, CVE-Cleared Dependencies

- [ ] [AI] Re-resolve every primitive dependency version from the lockfile:
      `node -e 'const l=require("./package-lock.json").packages; ["radix-ui","@radix-ui/react-slot","@radix-ui/react-dialog","@radix-ui/react-dropdown-menu","@radix-ui/react-tabs","@radix-ui/react-tooltip","@radix-ui/react-scroll-area","@radix-ui/react-separator","class-variance-authority","clsx","tailwind-merge","lucide-react"].forEach(p=>console.log(p, l["node_modules/"+p].version))'`
      — acceptance: every version printed; recorded into `tech-docs.md` Dependency Table if it differs from the snapshot
- [ ] [AI] Record CVE clearance across the five policy sources (NVD, GitHub Advisories, Snyk DB, vendor security page, CISA KEV) for each package@version, plus the clearance cutoff date, into `tech-docs.md` §CVE Clearance Record
      — acceptance: each source has a recorded status; cutoff date written; KEV/EPSS escalation not triggered (or escalation documented)
- [ ] [AI] Edit `libs/web-ui/package.json`: convert every primitive dependency to an EXACT pin (no caret/tilde) matching the lockfile, and add the missing `@radix-ui/react-*` subpackages used by the primitives; align `radix-ui` from `^1.0.0` to `1.4.3`
      — command: `node -e 'const d=require("./libs/web-ui/package.json").dependencies; Object.entries(d).forEach(([k,v])=>{if(/^[~^]/.test(v))throw new Error("non-exact pin: "+k+" "+v)})'`
      — acceptance: the guard script exits 0 (no caret/tilde in `web-ui` dependencies)
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **RED**: For each primitive being added to `libs/web-ui/src/primitives/` (superset: `button, badge, sheet, command, dialog, dropdown-menu, tabs, card, tooltip, scroll-area, separator`), write a failing render test in `libs/web-ui/src/primitives/<name>/<name>.test.tsx` asserting the primitive mounts and renders its slot
      — command: `nx run web-ui:test:unit`
      — acceptance: new tests fail with "module not found" / "is not defined" for the not-yet-created primitives
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **GREEN**: Create `libs/web-ui/src/primitives/<name>/<name>.tsx` for each primitive (port the superset from `apps/ose-www/src/features/app-shell/presentation/ui/` and `apps/ayokoding-www/src/contexts/app-shell/presentation/ui/`), and re-export them from `libs/web-ui/src/index.ts`
      — command: `nx run web-ui:test:unit`
      — acceptance: all new primitive tests pass; no existing `web-ui` test broken
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] **REFACTOR**: Deduplicate shared helpers (e.g. `cn`) and align primitive prop names across the superset in `libs/web-ui/src/primitives/`
      — command: `nx run web-ui:test:unit`
      — acceptance: all tests still pass; no duplicated `cn` definitions inside `primitives/`
  - _Suggested executor: `swe-typescript-dev`_

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `nx run web-ui:typecheck` exits 0
- [ ] [AI] `nx run web-ui:lint` exits 0
- [ ] [AI] `nx run web-ui:test:unit` passes (all new primitive tests green)
- [ ] [AI] The no-caret/tilde guard on `libs/web-ui/package.json` dependencies exits 0
- [ ] [AI] `tech-docs.md` records the resolved pins and the five-source CVE clearance with a cutoff date

> **Pause Safety**: `web-ui` has a complete, typechecked, tested primitives layer with exact deps;
> no app consumes it yet, so no app behaviour changed. Safe to stop. To resume:
> `nx run web-ui:test:unit`.

## Phase 2: web-ui-token Brand Files (ose / ayokoding / wahidyankf)

- [ ] [AI] Read `libs/web-ui-token/src/organiclever.css` to capture the canonical token custom-property name set
      — acceptance: the full list of `--*` token names is enumerated
- [ ] [AI] Create `libs/web-ui-token/src/ose.css` defining the SAME token custom-property names as `organiclever.css`, with OSE brand values, scoped to an OSE brand class on `:root`/`html`
      — command: `node -e 'const fs=require("fs");const ol=fs.readFileSync("libs/web-ui-token/src/organiclever.css","utf8").match(/--[a-z0-9-]+/gi)||[];const o=fs.readFileSync("libs/web-ui-token/src/ose.css","utf8");const miss=[...new Set(ol)].filter(t=>!o.includes(t));if(miss.length)throw new Error("ose.css missing tokens: "+miss.join(","))'`
      — acceptance: the parity guard exits 0 (ose.css defines every token organiclever.css defines)
- [ ] [AI] Create `libs/web-ui-token/src/ayokoding.css` with the same token-name parity for the AyoKoding brand
      — command: same parity guard pattern against `ayokoding.css`
      — acceptance: parity guard exits 0
- [ ] [AI] Create `libs/web-ui-token/src/wahidyankf.css` with the same token-name parity for the wahidyankf brand
      — command: same parity guard pattern against `wahidyankf.css`
      — acceptance: parity guard exits 0

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] All three new brand files exist under `libs/web-ui-token/src/` (`ose.css`, `ayokoding.css`, `wahidyankf.css`)
- [ ] [AI] The token-name parity guard exits 0 for all three new files against `organiclever.css`
- [ ] [AI] `nx run web-ui-token:typecheck` exits 0

> **Pause Safety**: four brand token files exist with identical token surfaces; no app imports the
> three new ones yet, so no app changed. Safe to stop. To resume: re-run the three parity guards.

## Phase 3: Storybook Stories + Brand Switcher + vercel.json + Deployer Agent + CI + prod-web-ui (all AI; NOT yet live)

- [ ] [AI] Edit `libs/web-ui/.storybook/preview.ts`: import the four brand token sheets and extend `withThemeByClassName` to map labels `OSE`, `AyoKoding`, `wahidyankf`, `OrganicLever` to their brand classes on the `html` element (`parentSelector: 'html'`)
      — command: `nx run web-ui:build-storybook`
      — acceptance: build exits 0; `libs/web-ui/storybook-static/index.html` exists
  - _Suggested executor: `swe-ui-maker`_
- [ ] [AI] Author one `*.stories.tsx` per primitive in `libs/web-ui/src/primitives/<name>/<name>.stories.tsx`
      — command: `nx run web-ui:build-storybook`
      — acceptance: build exits 0; every primitive has a discovered story (no "no stories" warning for primitives)
  - _Suggested executor: `swe-ui-maker`_
- [ ] [AI] Author one `*.stories.tsx` per composite in `libs/web-ui/src/components/<name>/<name>.stories.tsx` (for composites lacking a story)
      — command: `nx run web-ui:build-storybook`
      — acceptance: build exits 0; every composite has a discovered story
  - _Suggested executor: `swe-ui-maker`_
- [ ] [AI] Create `libs/web-ui/vercel.json` with `"framework": null`, `"buildCommand": "npx nx run web-ui:build-storybook"`, `"outputDirectory": "libs/web-ui/storybook-static"`, and an SPA rewrite `[{"source":"/(.*)","destination":"/index.html"}]`
      — command: `node -e 'const v=require("./libs/web-ui/vercel.json");if(v.framework!==null)throw new Error("framework must be null");if(!v.rewrites)throw new Error("missing rewrites")'`
      — acceptance: guard exits 0
- [ ] [AI] Create `.claude/agents/apps-web-ui-storybook-deployer.md` (Fast/haiku tier), modeled on `.claude/agents/apps-ose-www-deployer.md`, force-pushing `main` → `prod-web-ui`; keep it vendor-neutral
      — command: `test -f .claude/agents/apps-web-ui-storybook-deployer.md`
      — acceptance: file exists; frontmatter `model: haiku`; no vendor-specific governance content
  - _Suggested executor: `agent-maker`_
- [ ] [AI] Resync platform bindings: `npm run generate:bindings`
      — acceptance: command exits 0; `.opencode/` and `.amazonq/` mirrors regenerated (not hand-edited)
- [ ] [AI] Create `.github/workflows/web-ui-build-deploy-prod.yml` with `name: web-ui-build-deploy-prod`,
      triggers `schedule` + `workflow_dispatch` (NOT push), a `build-storybook` smoke job
      (`npx nx run web-ui:build-storybook`, `STORYBOOK_DISABLE_TELEMETRY=1`), and a `deploy` job
      that force-pushes `HEAD:prod-web-ui`
      — command: `npx --yes actionlint .github/workflows/web-ui-build-deploy-prod.yml`
      — acceptance: actionlint exits 0; filename and `name:` mirror per the workflow-naming convention
- [ ] [AI] Update `repo-governance/development/infra/github-actions-workflow-naming.md` §Target File Set table:
      add `web-ui-build-deploy-prod.yml` under a new "Library deploy workflows" subsection, with
      domain `web-ui`, purpose "Smoke-builds Storybook and force-pushes to `prod-web-ui` branch"
      — command: `grep -q "web-ui-build-deploy-prod" repo-governance/development/infra/github-actions-workflow-naming.md`
      — acceptance: grep returns 0 (filename present in the convention doc)
- [ ] [AI] Create the `prod-web-ui` environment branch from `main` and push it: `git branch prod-web-ui main && git push origin prod-web-ui`
      — acceptance: `git ls-remote --heads origin prod-web-ui` lists the branch

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `nx run web-ui:build-storybook` exits 0 and produces `libs/web-ui/storybook-static/index.html`
- [ ] [AI] Every primitive and composite has a discovered story (build emits no "no stories" warning for them)
- [ ] [AI] `libs/web-ui/vercel.json` guard exits 0 (`framework: null` + rewrites present)
- [ ] [AI] `.claude/agents/apps-web-ui-storybook-deployer.md` exists and `npm run generate:bindings` exited 0 with mirrors regenerated
- [ ] [AI] `actionlint` passes on `web-ui-build-deploy-prod.yml`
- [ ] [AI] `grep -q "web-ui-build-deploy-prod" repo-governance/development/infra/github-actions-workflow-naming.md` exits 0
- [ ] [AI] `git ls-remote --heads origin prod-web-ui` lists the branch

> **Pause Safety**: all deploy machinery exists (vercel.json, agent, CI workflow, prod-web-ui
> branch) but the site is NOT live — no Vercel project is connected and no domain is bound. Safe to
> stop. To resume: `nx run web-ui:build-storybook`.

## Phase 4: Migrate ose-www onto web-ui (snapshot-gated, zero visual change)

- [ ] [AI] Record ose-www visual baseline: `nx run ose-www-fe-e2e:test:e2e`
      — acceptance: suite passes; result recorded as regression oracle before any import changes
  - _Suggested executor: `swe-e2e-dev`_
- [ ] [AI] **GREEN**: Repoint `ose-www` imports from `@/features/app-shell/presentation/ui/<name>` to `@open-sharia-enterprise/web-ui` across `apps/ose-www/src/**`
      — command: `nx run ose-www-fe-e2e:test:e2e`
      — acceptance: the e2e suite still passes (zero visual/behavioural change); `nx run ose-www:typecheck` exits 0
  - _Suggested executor: `apps-ose-www-content-maker`_
- [ ] [AI] **REFACTOR**: Remove now-unused local import aliases / dead re-export shims in `apps/ose-www/src/` left over from the local `ui/` dir (do NOT delete the `ui/` dir yet — Phase 6)
      — command: `nx run ose-www:typecheck && nx run ose-www-fe-e2e:test:e2e`
      — acceptance: both pass; no source file still imports from `features/app-shell/presentation/ui/`

### Manual UI Verification — ose-www (Playwright MCP)

- [ ] [AI] Start ose-www dev server: `npx nx dev ose-www`
      — acceptance: server starts on port 3100 (or configured port)
- [ ] [AI] `browser_navigate` to `http://localhost:3100` — verify homepage renders
      — acceptance: page loads without HTTP error
- [ ] [AI] `browser_snapshot` — inspect DOM for layout/component structure matching pre-migration
      baseline (compare against baseline screenshots from the BASELINE step above)
      — acceptance: no unexpected layout shifts or missing components
- [ ] [AI] `browser_console_messages` — verify zero JS errors in the browser console
      — acceptance: zero errors (warnings acceptable if pre-existing)
- [ ] [AI] `browser_navigate` through all major routes rendered by migrated primitives (nav links,
      dialogs, dropdowns, tabs, tooltips, scroll areas visible in the app shell)
      — acceptance: all routes render without console errors; components appear as expected
- [ ] [AI] `browser_take_screenshot` for visual record of the migrated state
      — acceptance: screenshot saved; confirms visual parity with pre-migration baseline

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `nx run ose-www:typecheck` exits 0
- [ ] [AI] `nx run ose-www:lint` exits 0
- [ ] [AI] `nx run ose-www-fe-e2e:test:e2e` passes (zero visual change confirmed)
- [ ] [AI] `grep -r "features/app-shell/presentation/ui" apps/ose-www/src/` returns no source-import matches

> **Pause Safety**: `ose-www` renders identically and now consumes `web-ui`; its local `ui/` dir
> still exists but is unreferenced. Safe to stop. To resume: `nx run ose-www-fe-e2e:test:e2e`.

## Phase 5: Migrate ayokoding-www onto web-ui (snapshot-gated, zero visual change)

- [ ] [AI] Record ayokoding-www visual baseline: `nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: suite passes; result recorded as regression oracle before any import changes
  - _Suggested executor: `swe-e2e-dev`_
- [ ] [AI] **GREEN**: Repoint `ayokoding-www` imports from `@/contexts/app-shell/presentation/ui/<name>` to `@open-sharia-enterprise/web-ui` across `apps/ayokoding-www/src/**`
      — command: `nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: e2e suite still passes; `nx run ayokoding-www:typecheck` exits 0
  - _Suggested executor: `apps-ayokoding-www-general-maker`_
- [ ] [AI] **REFACTOR**: Remove dead import aliases / shims in `apps/ayokoding-www/src/` (do NOT delete the `ui/` dir yet — Phase 6)
      — command: `nx run ayokoding-www:typecheck && nx run ayokoding-www-fe-e2e:test:e2e`
      — acceptance: both pass; no source file still imports from `contexts/app-shell/presentation/ui/`

### Manual UI Verification — ayokoding-www (Playwright MCP)

- [ ] [AI] Start ayokoding-www dev server: `npx nx dev ayokoding-www`
      — acceptance: server starts on port 3101 (or configured port)
- [ ] [AI] `browser_navigate` to `http://localhost:3101` — verify homepage renders
      — acceptance: page loads without HTTP error
- [ ] [AI] `browser_snapshot` — inspect DOM for layout/component structure matching pre-migration
      baseline (compare against baseline screenshots from the BASELINE step above)
      — acceptance: no unexpected layout shifts or missing components
- [ ] [AI] `browser_console_messages` — verify zero JS errors in the browser console
      — acceptance: zero errors (warnings acceptable if pre-existing)
- [ ] [AI] `browser_navigate` through all major routes rendered by migrated primitives (nav links,
      dialogs, dropdowns, tabs, tooltips, scroll areas visible in the app shell)
      — acceptance: all routes render without console errors; components appear as expected
- [ ] [AI] `browser_take_screenshot` for visual record of the migrated state
      — acceptance: screenshot saved; confirms visual parity with pre-migration baseline

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `nx run ayokoding-www:typecheck` exits 0
- [ ] [AI] `nx run ayokoding-www:lint` exits 0
- [ ] [AI] `nx run ayokoding-www-fe-e2e:test:e2e` passes (zero visual change confirmed)
- [ ] [AI] `grep -r "contexts/app-shell/presentation/ui" apps/ayokoding-www/src/` returns no source-import matches

> **Pause Safety**: both content sites render identically and consume `web-ui`; both local `ui/`
> dirs exist but are unreferenced. Safe to stop. To resume: `nx run ayokoding-www-fe-e2e:test:e2e`.

## Phase 6: Token-Parity Wiring (all 6 apps) + Delete Local ui/ Dirs + Dep Cleanup

- [ ] [AI] Wire token imports in each app's entry CSS for all six apps (base `tokens.css` + brand sheet): `ose-www`→`ose.css`, `ayokoding-www`→`ayokoding.css`, `organiclever-www`→`organiclever.css`, `ose-app-web`→`ose.css`, `organiclever-app-web`→`organiclever.css`, `wahidyankf-www`→`wahidyankf.css`
      — command: `npx nx run-many -t typecheck --projects=ose-www,ayokoding-www,organiclever-www,ose-app-web,organiclever-app-web,wahidyankf-www`
      — acceptance: run-many exits 0; each app's entry CSS imports the base sheet and its brand sheet
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Delete the local UI dirs now that nothing references them: `git rm -r apps/ose-www/src/features/app-shell/presentation/ui/ apps/ayokoding-www/src/contexts/app-shell/presentation/ui/`
      — command: `npx nx run-many -t typecheck test:quick --projects=ose-www,ayokoding-www`
      — acceptance: both dirs are removed; run-many exits 0
- [ ] [AI] Remove now-unused primitive dependencies from `apps/ose-www/package.json` and `apps/ayokoding-www/package.json` that are now provided transitively by `web-ui` (only those no longer directly imported)
      — command: `npm install && npx nx run-many -t typecheck test:quick --projects=ose-www,ayokoding-www`
      — acceptance: lockfile updates cleanly; run-many exits 0
  - _Suggested executor: `swe-typescript-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
- [ ] [AI] Run affected spec coverage: `npx nx affected -t specs:coverage`
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes
- [ ] [AI] Re-run failing checks to confirm resolution; verify zero failures before pushing

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work. Commit preexisting fixes separately with appropriate conventional commit messages.

### Commit Guidelines

- [ ] [AI] Commit changes thematically — group related changes into logically cohesive commits
- [ ] [AI] Follow Conventional Commits format: `<type>(<scope>): <description>`
- [ ] [AI] Split different domains/concerns into separate commits (web-ui, web-ui-token, each app, CI, agent)
- [ ] [AI] Do NOT bundle unrelated changes into a single commit

### Post-Push CI Verification

- [ ] [AI] Push changes to `main`: `git push origin HEAD:main`
- [ ] [AI] Monitor the following GitHub Actions workflows (poll `gh run list --limit 10` every 3 minutes;
      do NOT use `gh run watch`): `commons-quality-gate`, `markdown-validate`, `commons-env-validate`,
      and `web-ui-build-deploy-prod` (if triggered)
- [ ] [AI] Verify ALL named CI checks pass — no exceptions
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit; repeat until green

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick specs:coverage` exits 0
- [ ] [AI] Both local `ui/` directories no longer exist (`test ! -d apps/ose-www/src/features/app-shell/presentation/ui && test ! -d apps/ayokoding-www/src/contexts/app-shell/presentation/ui`)
- [ ] [AI] All six apps import `web-ui-token` base + brand sheets
- [ ] [AI] Changes pushed to `main` and ALL GitHub Actions are green

> **Pause Safety**: the unification is complete and merged to `main` — all six apps on the unified
> kit + tokens, duplicates deleted, CI green. The site is still NOT publicly live (no Vercel project
> / domain yet). Safe to stop. To resume: `git pull origin main && npx nx affected -t test:quick`.

## Phase 7: Go-Live / External Activation (ALL human Vercel + DNS steps clustered here)

> This is the ONLY phase containing `[HUMAN]` steps. Each requires real Vercel/registrar credentials
> and authority an agent must not hold. The plan-execution workflow STOPS at each `[HUMAN]` step and
> waits for the human to confirm the stated observable signal before continuing.

- [ ] [HUMAN] In the Vercel dashboard, create a new Vercel project for the Storybook site and connect it to the GitHub repo
      — handoff: human creates the project; **resume signal**: the project appears in the Vercel dashboard linked to the repo
- [ ] [HUMAN] In the new project's settings, set Framework Preset = **Other** (must NOT be Next.js), Node.js version to **20.16+** (24.x preferred), and the Root Directory + Output Directory consistent with `libs/web-ui/vercel.json` (root = monorepo root → output `libs/web-ui/storybook-static`)
      — handoff: human sets the settings; **resume signal**: project settings show Framework = Other and Node ≥ 20.16
- [ ] [HUMAN] Connect the project's Production branch to **`prod-web-ui`**
      — handoff: human sets the production branch; **resume signal**: project Git settings show production branch = `prod-web-ui`
- [ ] [AI] Trigger the activating deploy: force-push `main` → `prod-web-ui` (`git push origin main:prod-web-ui --force`) via the `apps-web-ui-storybook-deployer` agent or the `workflow_dispatch` trigger
      — command: `git push origin main:prod-web-ui --force`
      — acceptance: Vercel starts a build from `prod-web-ui`; the build runs `nx run web-ui:build-storybook` (NOT `next build`)
- [ ] [AI] Verify the Vercel preview/production build serves the static Storybook (before domain bind)
      — acceptance: the Vercel-assigned deployment URL returns HTTP 200 and renders the Storybook index
- [ ] [HUMAN] In Vercel project Domains, add the custom domain **`web-ui.oseplatform.com`** and copy the CNAME target Vercel displays
      — handoff: human adds the domain; **resume signal**: Vercel shows the domain pending with a CNAME target value
- [ ] [HUMAN] At the DNS registrar for `oseplatform.com`, create a CNAME record `web-ui` → the Vercel-provided target
      — handoff: human creates the CNAME; **resume signal**: `dig CNAME web-ui.oseplatform.com` resolves to the Vercel target
- [ ] [AI] Verify the live custom domain: `curl -s -o /dev/null -w "%{http_code}" https://web-ui.oseplatform.com`
      — acceptance: returns `200`; the deep-link SPA rewrite works (a story-deep URL also returns 200, not 404)

### Manual UI Verification — Storybook Live Site (Playwright MCP)

- [ ] [AI] `browser_navigate` to `https://web-ui.oseplatform.com` — verify the Storybook index loads
      — acceptance: page renders the Storybook sidebar with primitive and composite stories listed
- [ ] [AI] `browser_snapshot` — inspect DOM for correct Storybook UI structure (sidebar, canvas, toolbar)
      — acceptance: no missing panels or layout errors
- [ ] [AI] `browser_console_messages` — verify zero JS errors in the browser console
      — acceptance: zero errors (warnings acceptable if pre-existing in the Storybook build)
- [ ] [AI] Switch brand themes via the Storybook toolbar: cycle through OSE, AyoKoding, wahidyankf,
      OrganicLever theme options
      — acceptance: each brand switch updates component tokens visibly; no console errors on switch
- [ ] [AI] `browser_navigate` to a deep story URL (e.g. `https://web-ui.oseplatform.com/?path=/story/primitives-button--default`)
      — acceptance: returns the story canvas, NOT a 404; SPA rewrite confirmed working
- [ ] [AI] `browser_take_screenshot` for visual record of the live Storybook with each brand active
      — acceptance: screenshots saved for OSE and at least one other brand theme

### Phase 7 Gate

> All checks below must pass to declare the plan complete.

- [ ] [HUMAN] Vercel project exists, Framework = Other, production branch = `prod-web-ui`, custom domain added
- [ ] [HUMAN] DNS CNAME `web-ui` → Vercel target created at the registrar
- [ ] [AI] `curl -s -o /dev/null -w "%{http_code}" https://web-ui.oseplatform.com` returns `200`
- [ ] [AI] A deep story URL on `web-ui.oseplatform.com` returns `200` (SPA rewrite confirmed)

> **Pause Safety**: the Storybook is publicly live at `web-ui.oseplatform.com` with SSL; the
> unification is merged and CI is green. This is the terminal state. To re-verify:
> `curl -s -o /dev/null -w "%{http_code}" https://web-ui.oseplatform.com`.

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify ALL quality gates pass (local + CI)
- [ ] [AI] Verify the live-site assertion passes (`https://web-ui.oseplatform.com` returns 200)
- [ ] [AI] Move: `git mv plans/in-progress/unify-web-ui-kit-and-deploy-storybook/ plans/done/YYYY-MM-DD__unify-web-ui-kit-and-deploy-storybook/` using the completion date (NOT the creation date)
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date
- [ ] [AI] Update any other READMEs that reference this plan
- [ ] [AI] Commit the archival: `chore(plans): move unify-web-ui-kit-and-deploy-storybook to done`
