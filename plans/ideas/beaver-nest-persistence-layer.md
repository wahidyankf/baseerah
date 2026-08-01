# BeaverNest persistence layer

One-line summary: give `beaver-nest-be` a real data store — currently every route is stateless and the
greeting is a hardcoded constant.

> Idea, added 2026-07-31, filed from `baseerah-repo-reset`'s Product Scope § Out of scope.

## Problem / context

`baseerah-repo-reset` deliberately built `beaver-nest-be`/`beaver-nest-fe` as a stateless hello-world
skeleton: "No persistence. No database, no in-memory store, no state of any kind. The greeting is a
constant." That was the right scope for a walking skeleton, but it means BeaverNest has zero
capability to remember anything — no notes, no captures, no user data — which is the entire point
of the product per [BeaverNest Vision](../../repo-governance/vision/beaver-nest.md).

## Why now

Not yet — this is a placeholder for the first plan that needs real product state. Building
persistence before a concrete feature needs it would be speculative infrastructure.

## Prior art / precedents

- Sibling repo backends (`ose-app`, `organiclever-be` before their removal/migration) used
  PostgreSQL via `docker-compose.integration.yml` + `db/migrations/` — the established pattern this
  repo already has tooling conventions for, per
  [monorepo-structure](../../docs/reference/monorepo-structure.md).
- `repo-governance/development/pattern/functional-programming.md` — functional core/imperative
  shell shapes how a persistence boundary should be designed (pure domain logic, impure I/O at the
  edge).

## Proposed direction (sketch)

- A database service (likely PostgreSQL, matching the sibling-repo pattern) wired via
  `docker-compose.integration.yml`, with `db/migrations/` and a test-reset hook for E2E isolation.
- Introduced alongside the first real feature that needs state, not ahead of it.

## Rough scope & non-goals

In scope: eventually, a persistence boundary for whatever the first stateful feature turns out to
be.

Out of scope (for now): choosing a schema, an ORM, or even confirming PostgreSQL — none of that can
be decided without a concrete feature driving the requirements.

## Risks & open questions

- Which feature drives the first real persistence requirement? (open — determines the whole shape)
- PostgreSQL (matching siblings) vs. something lighter for a still-small product? (open)
- Does E2E isolation need a test-reset hook from day one, or can it be added when the first
  stateful E2E scenario is written? (open)

## What success looks like + promotion signal

Success: `beaver-nest-be` can durably store and retrieve real product data. Ready to promote only when
a concrete feature (capture, notes, or similar) needs state to design against — until then it
correctly stays an under-specified idea.
