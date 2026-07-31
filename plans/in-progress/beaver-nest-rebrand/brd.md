# Business Requirements Document — BeaverNest Rebrand

## Business Goal

Replace the repository's working product name, **Baseerah**, with the maintainer's chosen permanent
name, **BeaverNest**, across every git-tracked surface in this repository, so the identity the
repository presents to itself (docs, agents, CI, code) and to the outside world (README, GitHub repo
name, container images) is consistent and no longer carries a placeholder name.

## Business Impact

**Pain point**: The repository was scaffolded quickly under a working name chosen to unblock the
walking-skeleton build (see [Baseerah Vision](../../../repo-governance/vision/baseerah.md)). The
maintainer has since settled on a permanent name and wants the rename done now, before the surface
area grows further. `[Judgment call]`: every week this rename is deferred, more files, agents, CI
workflows, and specs accrue references to the old name, and the eventual rename gets strictly more
expensive — this is a monotonic cost function, not a measured one, but the direction of the
inequality is not in doubt for a monorepo that adds files roughly every session.

**Expected benefit**: A single, deliberate rename event now costs one plan's worth of mechanical
work (`[Repo-grounded]`: 246 git-tracked files outside `plans/done/` currently reference `baseerah`
case-insensitively, verified via `git grep -liE "baseerah" -- . ':!plans/done' ':!generated-reports'`
on 2026-08-01) rather than a slow drip of partial renames that leave the repository in a permanently
inconsistent state (some files say Baseerah, some say BeaverNest, nobody is sure which is current).

## Affected Roles

Solo-maintainer repository — no sign-off or stakeholder ceremony applies. The roles affected are:

- **The maintainer** (wahidyankf), wearing the hat of repo owner: decides the final name, executes
  the two irreducibly human acts (GitHub repo rename, local checkout folder rename + remote
  re-point), and is the sole approver of this plan.
- **AI coding agents operating in this repo** (Claude Code, OpenCode, and the generated bindings for
  Amazon Q / Cursor / Codex / etc.): consume `AGENTS.md`, the agent fleet under `.claude/agents/`,
  and the skills under `.claude/skills/` — all of which currently embed the `baseerah` vocabulary in
  agent names, file paths, and prose, and must be updated in lockstep so agent-selection heuristics
  and generated bindings keep resolving to real files.
- **`rhino-cli`** (the repo's own tooling, running as both a build dependency and a CI gate): reads
  `repo-config.yml`'s coverage/env-contract registries by project name, and hardcodes a small number
  of repo-identity strings (the `.amazonq/cli-agents/baseerah-default.json` path) as Rust constants —
  these must be updated in lockstep with the file renames they describe, or `rhino-cli`'s own tests
  and generated bindings break.

## Business-Level Success Metrics

- **Observable fact**: after this plan's Repo-Wide Residual Sweep phase, `git grep -liE "baseerah"`
  scoped to the whole repository except `plans/done/2026-07-31__baseerah-repo-reset/**` and the
  explicit historical-citation allowlist (see [tech-docs.md §Decision Log](./tech-docs.md#decision-log)
  Decision 6) returns zero files.
- **Observable fact**: `npx nx run-many -t typecheck,lint,test:quick --all` exits 0 after every
  renamed project (`beaver-nest-be`, `beaver-nest-be-e2e`, `beaver-nest-fe`, `beaver-nest-fe-e2e`,
  `beaver-nest-contracts`, `rhino-cli`) is renamed, matching (or improving on) the Phase 0 baseline.
- **Qualitative reasoning**: a fresh clone of the repository, read cold, presents one coherent
  product identity (BeaverNest) with no unexplained "Baseerah" residue a new contributor or agent
  would need to reconcile.
- `[Judgment call]`: there is no meaningful numeric KPI for "how rebranded" a repository is beyond
  the zero-residual grep check above; this document does not fabricate one.

## Business-Scope Non-Goals

- **Cross-repo propagation is explicitly out of scope** (Q8). `ose-public`, `ose-primer`, and
  `ose-private` are not touched by this plan. If any of those repos is later found to reference
  `baseerah` (unlikely — `[Repo-grounded]`: `AGENTS.md` states this repo "scaffolded from that
  ecosystem but does not participate in cross-repo parity syncs"), that is a separate, later plan.
- **No new deploy target is provisioned.** The existing dormant-deployer posture (deployer agents
  and CI callers wired but no `prod-*`/`stag-*` target live) is preserved as-is under the new names;
  provisioning the first real target remains the separately-tracked
  [baseerah-first-deploy idea](../../ideas/baseerah-first-deploy.md) (renamed to
  `beaver-nest-first-deploy.md` by this plan).
- **No dual-publish bridge for the GHCR image name.** Per Q9, this is a hard cutover: the old
  `ghcr.io/wahidyankf/baseerah-be` image name is abandoned with no compatibility alias, because
  nothing in production depends on it (no live deploy target consumes it yet).
- **The GitHub repository rename and the local checkout folder rename are explicitly scoped as the
  final two human acts** (Q3, Q4) — this plan's content phases do not attempt either, and do not
  block on either happening before the content phases merge.

## Business Risks and Mitigations

| Risk                                                                                                                                                                                | Mitigation                                                                                                                                                                                                                                                                            |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A missed reference leaves a stray "Baseerah" string somewhere a future reader trips over it                                                                                         | The Repo-Wide Residual Sweep phase (Phase 16) runs a repo-wide case-insensitive grep with an explicit, enumerated allowlist of the only permissible residual matches (the historical plan folder and its citations) — any other hit is a defect fixed before the phase gate passes    |
| Renaming F# namespaces or Rust constants breaks a build or a hardcoded path `rhino-cli` depends on                                                                                  | Phase 15 is dedicated to `rhino-cli`'s own functional couplings (the `.amazonq/cli-agents/baseerah-default.json` constant and its test assertions), verified by `nx run rhino-cli:test:quick` passing after the rename, not just a text grep                                          |
| The GHCR hard cutover (Q9) breaks something that silently depended on the old image name                                                                                            | `[Repo-grounded]`: `gh api repos/wahidyankf/baseerah/environments` returns zero configured GitHub Environments, and no `stag-*`/`prod-*` branch exists in `git branch -r` today (verified 2026-08-01) — nothing live consumes the old image name, so the cutover has no rollback cost |
| The GitHub repo rename (Q3, `[HUMAN]`) happens before the content phases finish merging, leaving `origin` pointing at a URL that no longer matches the local remote config mid-plan | The delivery checklist explicitly orders the GitHub rename as the second-to-last phase, after every content phase has merged to `origin main` under the OLD repo URL; the plan does not proceed to that phase until every prior phase's gate is green                                 |
