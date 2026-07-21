# Demo apps standards recheck

One-line summary: re-verify that the polyglot demo apps still meet current repo standards, now that
they live in ose-primer.

> Idea, added 2026-07-21 (original capture undated).

## Problem / context

The polyglot demo/showcase apps were extracted from `ose-public` to
[`ose-primer`](https://github.com/wahidyankf/ose-primer) on 2026-04-18, which is now authoritative for
them. Repo standards (lint strictness, testing tiers, CI parity, Gherkin coverage) have moved on since,
and there is no recent confirmation the demo apps still conform.

## Why now

Standards have shifted materially since the extraction (toolchain-parity, lint-safety-parity,
three-tier testing). Drift, if any, is easiest to catch before it compounds.

## Prior art / precedents

- **standardize-repo-toolchain-parity plan (done)** — the standards shift the extracted demo apps may
  now lag behind. [toolchain-parity](../done/2026-06-13__standardize-repo-toolchain-parity/README.md)
- **lint-safety-parity plan (done)** — companion parity work whose lint gates the demo apps must now
  meet. [lint-safety-parity](../done/2026-06-12__lint-safety-parity/README.md)
- **Related Repositories reference** — documents ose-primer as authoritative for the extracted demo
  apps, framing the repo-ownership open question. [related-repositories](../../docs/reference/related-repositories.md)
- **ci-checker** — the conformance-check agent to run against the demo apps. [ci-checker agent](../../.claude/agents/ci-checker.md)

## Proposed direction (sketch)

- Run the current standard checks (lint, test tiers, specs coverage, CI) against the demo apps in
  ose-primer.
- File whatever fails as targeted fixes in that repo.

## Rough scope & non-goals

In scope: a conformance recheck of the demo apps against today's standards.

Out of scope (for now): adding new demo apps; changes to `ose-public` (the apps live in ose-primer).

## Risks & open questions

- This idea belongs primarily to `ose-primer` — should the two-pager live there instead of, or in
  addition to, here? (open — repo-relevance)
- Which standards actually apply to demo/showcase apps vs. production apps? (open)

## What success looks like + promotion signal

Success: the demo apps provably pass current standard checks, or their gaps are filed as fixes. Ready
to promote once it is decided which repo owns the work (likely ose-primer).
