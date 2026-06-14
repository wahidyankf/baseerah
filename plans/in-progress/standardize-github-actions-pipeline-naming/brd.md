# Business Requirements Document

## Problem statement

The `.github/` CI surface has three incompatible filename styles and carries stale wiring left behind
by the `-www` restructure. A developer cannot tell from a filename which app a workflow serves, what it
tests, or where it deploys; and several www workflows silently reference projects (`ose-web`) and
branches (`prod-ose-web`) that no longer exist. This blocks the Vercel production cutover, which needs a
coherent, correctly-wired workflow set to build on.

## Goals

- **One legible naming convention** — `{domain}-{action-chain}.yml` — so any workflow's purpose is
  readable from its filename, and cross-cutting workflows carry a clear `commons-*` / `markdown-*` /
  `{cli}-*` domain prefix.
- **Two explicit deploy tiers** — a direct www tier and a gated app tier — encoded in the filenames and
  factored into reusable workflows so adding a site is a thin caller.
- **Correct wiring** — every www caller points at its renamed `-www` project and new `prod-*-www`
  branch; every app workflow points at the new `stag-*-app-web` branches and renamed Environments.
- **Unblock the Vercel cutover** — leave `wire-vercel-www-app-cutover` with only dashboard/DNS/branch
  work.

## Non-goals

- Production continuous delivery for the app tier (separate plan).
- Standing up Vercel projects, DNS, GitHub Environments, or branches (wire-vercel).
- Backend k8s rollout (ose-infra).
- Changing any test's logic or coverage thresholds.

## Stakeholders

| Stakeholder            | Interest                                                                 |
| ---------------------- | ------------------------------------------------------------------------ |
| Repo maintainer        | Legible, correctly-wired CI; unblocked Vercel cutover                    |
| Future contributors    | Predictable place + name for each app's pipeline                         |
| `wire-vercel` executor | A workflow set that only needs branches/projects created, not rewritten  |
| ose-infra owner        | Clear boundary: be images published here, k8s rollout stays in ose-infra |

## Impact

- **Developer experience**: filename → purpose is now deterministic; the broken www callers start
  working again once their branches exist.
- **Risk surface**: one coordinated rename window; the only sharp edge is the PR-quality-gate required
  status check (tracked as Open Decision 1).
- **Downstream**: `wire-vercel-www-app-cutover` shrinks materially; a future app-tier CD plan inherits a
  clean `*-test-stag-deploy-prod` seam to extend.

## Risks

| Risk                                                            | Likelihood | Impact | Mitigation                                                                                     |
| --------------------------------------------------------------- | ---------- | ------ | ---------------------------------------------------------------------------------------------- |
| Renaming `pr-quality-gate.yml` breaks the required status check | Medium     | High   | Open Decision 1 — keep job/check names or coordinate a branch-protection update (`[HUMAN]`)    |
| New workflows push to branches that do not exist yet            | High       | Low    | Expected until wire-vercel creates them; failure is loud + non-destructive (failed `git push`) |
| `organiclever-www` has no local test stack                      | Certain    | Medium | Create `infra/dev/organiclever-www` compose in this plan                                       |
| Broken in-repo links to renamed workflow files                  | Medium     | Low    | `links:validation` gate + explicit doc-sweep phase                                             |
| Scope overlap/conflict with `wire-vercel`                       | Medium     | Medium | This plan explicitly takes workflow ownership and edits wire-vercel to match                   |

## Success criteria

- All workflow files follow `{domain}-{action-chain}.yml`; the convention doc describes the grammar.
- `actionlint` passes on every workflow; `npx nx run rhino-cli:links:validation` is green (no broken
  references to old filenames).
- `wire-vercel-www-app-cutover` no longer claims any `.github/workflows` editing; its README/tech-docs
  point at this plan.
- A manually dispatched run of each renamed pipeline reaches its deploy step (or the documented stop
  point) without a wiring error, modulo branches that wire-vercel has not yet created.
