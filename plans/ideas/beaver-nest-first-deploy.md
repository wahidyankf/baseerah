# BeaverNest first deploy (provision prod/stag targets)

One-line summary: provision the first real deploy targets for `beaver-nest-fe`/`beaver-nest-be` —
today the deployer agents and CI caller workflows ship wired but dormant, with nothing on the
other end.

> Idea, added 2026-07-31, filed from `baseerah-repo-reset`'s Product Scope § Out of scope.

## Problem / context

`baseerah-repo-reset` shipped `apps-beaver-nest-fe-deployer` and `apps-beaver-nest-be-deployer`, plus
the CI caller workflows (`beaver-nest-be-build-deploy-stag.yml`,
`beaver-nest-app-test-local-deploy-stag.yml`, `beaver-nest-app-test-stag.yml`, and the `beaver-nest-be` job
in `publish-images.yml`) — but deliberately did not provision anything: no Vercel project, no GHCR
repository consumer, no `prod-beaver-nest-fe`/`stag-beaver-nest-fe`/`stag-beaver-nest-be` branch exists yet.
Both deployer agents' own files say this plainly. Pushing to those branches today reaches nothing.

## Why now

Not yet — provisioning a real deploy target is an infrastructure decision (hosting account, DNS,
k3s wiring via the separate `ose-private`/`coralpolyp` repo) that belongs to its own plan once
BeaverNest has something worth deploying beyond hello world.

## Prior art / precedents

- `apps/beaver-nest-fe/README.md` and `apps/beaver-nest-be/README.md` (once authored) would document the
  intended framework/deployment per app, per AGENTS.md's Web Sites section convention.
- `.github/workflows/_reusable-be-build-deploy.yml`'s own comment already documents that the actual
  k3s rollout is orchestrated by `ose-private`'s `coralpolyp` — out of scope for this repo, and
  `coralpolyp` does not yet know about `beaver-nest-be` at all.
- `stag-beaver-nest-fe` (a scheduled Vercel **preview** deploy, not production) is the only
  deploy-like branch wired up today, driven by `beaver-nest-app-test-local-deploy-stag.yml` — that is
  a smoke-test workflow, not a real staging environment.

## Proposed direction (sketch)

- For `beaver-nest-fe`: provision a Vercel project building from `prod-beaver-nest-fe`.
- For `beaver-nest-be`: provision a running consumer of the `ghcr.io/wahidyankf/beaver-nest-be` image —
  most likely wiring `coralpolyp` (in `ose-private`) to know about `beaver-nest-be` and roll it out to
  k3s on push to `stag-beaver-nest-be`.
- Only after provisioning, re-verify both deployer agents' "Current State" sections no longer
  describe a dormant target.

## Rough scope & non-goals

In scope: the first real `prod-beaver-nest-fe` Vercel project and the first real `stag-beaver-nest-be`
k3s rollout via `coralpolyp`.

Out of scope (for now): any change to the deployer agents' push-based mechanism itself — that part
already works and is real; only the receiving end is missing.

## Risks & open questions

- Does `coralpolyp` need BeaverNest-specific changes, or just a config entry? (open — `ose-private`
  is out of scope for this repo, so this may itself need its own cross-repo coordination)
- Is a Vercel project for `beaver-nest-fe` provisioned manually (human `[HUMAN]` step) or can it be
  automated? Account/billing implications likely make this a human step.
- Does hello-world content warrant a real deploy yet, or should this wait for the first real
  feature (see [beaver-nest-persistence-layer](./beaver-nest-persistence-layer.md))?

## What success looks like + promotion signal

Success: a real, working `prod-beaver-nest-fe` URL and a real running `beaver-nest-be` staging server,
with both deployer agents' files updated to drop their "no target provisioned" caveats. Ready to
promote once a maintainer decides BeaverNest is ready for a first live deploy — until then it
correctly stays an under-specified idea.
