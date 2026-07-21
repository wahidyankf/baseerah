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
