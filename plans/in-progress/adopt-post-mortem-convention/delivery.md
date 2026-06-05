# Delivery Checklist — Adopt Post-Mortem Convention

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> This plan is governance/documentation only and contains **no `[HUMAN]` steps** — every step is
> mechanically performable by an agent.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase is
> not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/adopt-post-mortem-convention/`

Provision before execution (run from repo root):

```bash
claude --worktree adopt-post-mortem-convention
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
- [ ] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift.
- [ ] [AI] Provision the worktree: `claude --worktree adopt-post-mortem-convention`
      — acceptance: `worktrees/adopt-post-mortem-convention/` exists; `git worktree list` shows it.
- [ ] [AI] Confirm the new docs directory does not yet exist:
      `test -d docs/explanation/post-mortems && echo EXISTS || echo ABSENT`
      — acceptance: prints `ABSENT` (this plan creates it).
- [ ] [AI] Record markdown-lint baseline: `npm run lint:md`
      — acceptance: pass/fail recorded; any preexisting failures documented for resolution.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] `npm run lint:md` baseline recorded; every preexisting failure documented and resolved
      (zero unresolved).

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no plan files
> changed. Safe to stop indefinitely. To resume: re-run `npm run lint:md` and confirm it is still clean.

## Phase 1: Author the Authoritative Convention

- [ ] [AI] Create `repo-governance/conventions/structure/post-mortems.md` (_New file_) adapting the
      ose-infra original at `/Users/wkf/ose-projects/ose-infra/repo-governance/conventions/structure/post-mortems.md`,
      reframed for software incidents per `tech-docs.md` §Adaptation Map. The file MUST contain, in
      this order: frontmatter (sibling shape: `title`, `description`, `category: explanation`,
      `subcategory: conventions`, `tags:`, `created:`), H1, intro stating it is the authoritative
      rule and pointing to `docs/explanation/post-mortems/README.md` as the working surface with
      "when the two disagree, the convention wins", Principles Implemented/Respected (linking
      `documentation-first.md`, `root-cause-orientation.md`, `deliberate-problem-solving.md`,
      `explicit-over-implicit.md`), Purpose, Scope, Standards (Location and Naming with pattern
      `YYYY-MM-DD-<system>-<short-failure>.md`, Blameless Principle with "second story", Timing,
      Mandatory Sections list in order, Optional Sections, authoritative Severity Scale Sev-1..Sev-4,
      No Secrets Rule referencing `repo-governance/conventions/security/no-secrets-in-git.md`,
      Diagrams citing the six WCAG AA hex codes), Examples (software-flavored PASS/FAIL filenames and
      action-item tables), Validation checklist, References (in-repo links + the four industry
      sources: Google SRE, Allspaw, PagerDuty, Atlassian).
      — acceptance: `test -f repo-governance/conventions/structure/post-mortems.md` succeeds; all
      mandatory sections present; no infra terms (Proxmox/Tailscale/dual-WAN/Proxmox) appear:
      `grep -Ei 'proxmox|tailscale|dual-wan|on-premise|pve-ose' repo-governance/conventions/structure/post-mortems.md`
      returns nothing; `grep -c 'no-secrets-in-git.md' repo-governance/conventions/structure/post-mortems.md`
      is ≥ 1 and `grep -c 'no-secrets-in-committed-files' repo-governance/conventions/structure/post-mortems.md`
      is 0.
  - _Suggested executor: repo-rules-maker_
- [ ] [AI] Verify all cross-links in the new convention resolve:
      `npm run lint:md` (link rules) — acceptance: no broken-link errors for
      `repo-governance/conventions/structure/post-mortems.md`.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `test -f repo-governance/conventions/structure/post-mortems.md` succeeds.
- [ ] [AI] `grep -Ei 'proxmox|tailscale|dual-wan|on-premise|pve-ose' repo-governance/conventions/structure/post-mortems.md`
      returns nothing (software-only framing).
- [ ] [AI] `grep -q 'no-secrets-in-git.md' repo-governance/conventions/structure/post-mortems.md` and
      NOT `no-secrets-in-committed-files` — correct ose-public no-secrets reference used.
- [ ] [AI] `npm run lint:md` passes with zero new violations.

> **Pause Safety**: one new self-contained convention file exists; nothing references it yet
> (indexes still untouched) so the tree is coherent. Safe to stop. To resume: re-run the Phase 1
> Gate greps and `npm run lint:md`.

## Phase 2: Author the Template, Index, and Worked Example

- [ ] [AI] Create directory + `docs/explanation/post-mortems/README.md` (_New file_) as the
      writer-facing template + index, adapting the ose-infra original at
      `/Users/wkf/ose-projects/ose-infra/docs/explanation/post-mortems/README.md`. It MUST: use
      sibling docs frontmatter shape (`title`, `description`, `category: explanation`, `tags:`,
      `created:`); explain post-mortems are Diátaxis explanation-tier; link the authoritative
      convention `../../../repo-governance/conventions/structure/post-mortems.md` and state the
      convention wins on conflict; provide a copy-paste template skeleton with all mandatory sections
      in order; provide filing conventions (naming, layout, timing, `doc_status`, no-secrets via
      `no-secrets-in-git.md`, blameless tone); provide an Index section listing the worked example.
      — acceptance: `test -f docs/explanation/post-mortems/README.md` succeeds; it links the
      convention with a path that resolves; `grep -q 'no-secrets-in-git.md docs/...'` —
      `grep -c 'no-secrets-in-git' docs/explanation/post-mortems/README.md` ≥ 1.
  - _Suggested executor: docs-maker_
- [ ] [AI] Create the worked example
      `docs/explanation/post-mortems/<incident-date>-amazonq-prettier-parity-guard-break.md`
      (_New file_; choose a realistic incident date, lowercase kebab-case filename matching
      `YYYY-MM-DD-<system>-<short-failure>.md`) using the incident summary in `tech-docs.md`
      §Worked-Example Incident Summary. It MUST contain: post-mortem frontmatter including
      `doc_status` and `created:`; metadata table immediately after H1 with a Sev-N tier; all
      mandatory sections in order; Root Cause distinct from Trigger; an Action Items table with at
      least one owned (`Maintainer`), prioritized (P0/P1/P2) item with a `Ticket` (`—` or a
      `plans/` ref); a Timeline using absolute WIB UTC+7 timestamps; at least one accessible Mermaid
      diagram using only `#0173B2 #DE8F05 #029E73 #CC78BC #CA9161 #808080`; the real fix
      (add `.amazonq/**` to `.prettierignore`); no secret values.
      — acceptance: file exists at the kebab-case path; `grep -Eo '#0173B2|#DE8F05|#029E73|#CC78BC|#CA9161|#808080'`
      finds only approved hex codes and `grep -E '#[0-9A-Fa-f]{6}'` finds no others; the words
      `Root Cause` and `Trigger` both appear as headings; `WIB` and `UTC+7` appear in the Timeline.
  - _Suggested executor: docs-maker_
- [ ] [AI] Verify the worked example contains no leaked secrets:
      `grep -Ei 'password|secret|token|api[_-]?key|BEGIN [A-Z ]*PRIVATE KEY' docs/explanation/post-mortems/*.md`
      — acceptance: any match is a placeholder or a reference to the no-secrets rule, never a real value.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `test -f docs/explanation/post-mortems/README.md` and the worked-example file both succeed.
- [ ] [AI] Worked-example filename matches `^[0-9]{4}-[0-9]{2}-[0-9]{2}-[a-z0-9-]+\.md$`
      (verify: `ls docs/explanation/post-mortems/ | grep -E '^[0-9]{4}-[0-9]{2}-[0-9]{2}-[a-z0-9-]+\.md$'`).
- [ ] [AI] Mermaid diagram uses only the six approved WCAG AA hex codes (no other 6-digit hex).
- [ ] [AI] `npm run lint:md` passes for the new docs files with zero violations.

> **Pause Safety**: the convention, template, and worked example all exist and pass markdown lint;
> indexes are not yet updated, so the new docs are reachable only by direct path — coherent state.
> Safe to stop. To resume: re-run the Phase 2 Gate checks.

## Phase 3: Update Index Files

- [ ] [AI] Edit `repo-governance/conventions/structure/README.md`: add a Post-Mortem Convention
      bullet under Documents — a markdown link whose text is `Post-Mortem Convention` and whose
      target is the sibling file `post-mortems.md`, placed sensibly among sibling entries.
      — acceptance: `grep -q 'post-mortems.md' repo-governance/conventions/structure/README.md` succeeds.
  - _Suggested executor: repo-rules-maker_
- [ ] [AI] Edit `repo-governance/conventions/README.md`: add the Post-Mortem Convention entry to its
      structure-conventions enumeration (the list that currently includes Diataxis Framework, File
      Naming, etc.).
      — acceptance: `grep -q 'post-mortems.md' repo-governance/conventions/README.md` succeeds.
  - _Suggested executor: repo-rules-maker_
- [ ] [AI] Edit `docs/explanation/README.md`: add a Post-Mortems entry to the Documentation Index
      linking `./post-mortems/README.md` (new subsection or under an appropriate heading).
      — acceptance: `grep -q 'post-mortems/README.md' docs/explanation/README.md` succeeds.
  - _Suggested executor: docs-maker_
- [ ] [AI] Verify no dynamic-count hardcoding was introduced (per
      [dynamic-collection-references](../../../repo-governance/conventions/writing/dynamic-collection-references.md)):
      manual read of the three diffs — acceptance: no convention/doc counts were hardcoded.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] All three greps above succeed (entry present in each index).
- [ ] [AI] `npm run lint:md` passes (link rules resolve every new index link).

> **Pause Safety**: the new surfaces are now fully discoverable from every index; the tree is
> coherent and self-consistent but not yet validated by the governance gate or pushed. Safe to stop.
> To resume: re-run the Phase 3 Gate greps and `npm run lint:md`.

## Phase 4: Validation, Quality Gates, and Push

### Repo Rules Quality Gate

- [ ] [AI] Run the `repo-rules-quality-gate` workflow at **strict** mode until double-zero
      (two consecutive checks with zero CRITICAL/HIGH/MEDIUM findings), per
      [repo-rules-quality-gate.md](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md).
      Invoke `repo-rules-checker` then `repo-rules-fixer` iteratively; fix every reported finding
      affecting the new convention, docs, and indexes.
      — acceptance: workflow reports double-zero at strict mode; no CRITICAL/HIGH/MEDIUM findings remain.

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck/lint/test/spec: `npx nx affected -t typecheck lint test:quick spec-coverage`
      — acceptance: exits 0 (docs-only change is expected to affect nothing or pass trivially).
- [ ] [AI] Run markdown lint: `npm run lint:md`
      — acceptance: exits 0, zero violations.
- [ ] [AI] Run markdown format check: `npm run format:md:check`
      — acceptance: exits 0 (or run `npm run format:md` then re-check).
- [ ] [AI] Fix ALL failures found — including preexisting issues not caused by this change — then
      re-run the failing checks to confirm resolution.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root-cause orientation principle — proactively fix preexisting errors encountered
> during work. Commit preexisting fixes separately with appropriate conventional commit messages.

### Commit Guidelines

- [ ] [AI] Commit thematically with Conventional Commits, splitting by concern:
  - `docs(governance): add blameless post-mortem convention` (the new convention file)
  - `docs(post-mortems): add writer template, index, and worked example` (docs surface)
  - `docs(governance): index the post-mortem convention` (the three index updates)
  - Any preexisting fix in its own separate commit.
    — acceptance: `git log --oneline` shows cohesive, single-concern commits; no unrelated bundling.

### Post-Push CI Verification

- [ ] [AI] Push changes to `origin main` (direct, trunk-based, no PR): `git push origin main`
      — acceptance: push succeeds.
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 3 minutes via
      `gh run list` / `gh run view --json status,conclusion`; do NOT use `gh run watch`)
      — acceptance: every triggered workflow concludes `success`.
- [ ] [AI] If any CI check fails, investigate root cause, fix, push a follow-up commit, and repeat
      until all GitHub Actions pass — acceptance: zero failing checks; do not proceed until green.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `repo-rules-quality-gate` reached double-zero at strict mode.
- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` and `npm run lint:md` both exit 0.
- [ ] [AI] `git push origin main` succeeded and all triggered GitHub Actions concluded `success`.

> **Pause Safety**: all changes are committed, pushed to `main`, and CI-green — the convention is
> live and validated. Safe to stop. To resume: re-run `gh run list` to confirm CI is still green,
> then proceed to archival.

## Phase 5: Plan Archival

- [ ] [AI] Verify ALL delivery checklist items above are ticked and all gates passed.
- [ ] [AI] Move the plan to done with today's completion date:
      `git mv plans/in-progress/adopt-post-mortem-convention plans/done/$(date +%Y-%m-%d)__adopt-post-mortem-convention`
      — acceptance: folder now under `plans/done/` with a `YYYY-MM-DD__` prefix.
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry (return Active Plans to
      `_(none)_` if it was the only one) — acceptance: no reference to this plan remains.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date — acceptance:
      entry present.
- [ ] [AI] Commit the archival: `chore(plans): move adopt-post-mortem-convention to done`
      — acceptance: commit created.
- [ ] [AI] Push archival and verify CI: `git push origin main`; monitor GitHub Actions to green
      — acceptance: push succeeds; all workflows conclude `success`.

### Phase 5 Gate

> Final gate — plan complete when all pass.

- [ ] [AI] Plan folder lives under `plans/done/YYYY-MM-DD__adopt-post-mortem-convention/`.
- [ ] [AI] Both `plans/in-progress/README.md` and `plans/done/README.md` are consistent.
- [ ] [AI] Archival commit pushed; CI green.

> **Pause Safety**: the plan is fully delivered, archived, pushed, and CI-green. This is the terminal
> safe state — nothing left to resume.
