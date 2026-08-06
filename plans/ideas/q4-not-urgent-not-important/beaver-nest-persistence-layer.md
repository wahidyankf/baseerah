# BeaverNest product persistence slice

One-line summary: introduce the first concrete BeaverNest feature that durably stores and retrieves
product data on the explicit SQLite foundation planned in
[`beaver-nest-app-setup`](../../in-progress/beaver-nest-app-setup/README.md).

> Idea, added 2026-07-31 from `baseerah-repo-reset`; narrowed 2026-08-02 when the infrastructure-only
> SQLite foundation became its own active plan.

## Problem / context

The app-setup plan deliberately creates SQLite configuration, migrations, readiness, durability,
and recovery without a domain table. That avoids inventing a generic note, capture, or settings
model before a product behavior needs one, but it also means BeaverNest still cannot remember real
product data after the foundation lands.

## Why now

Not yet. Promote this brief together with the first assistant/content capability whose value depends
on durable state. The feature must drive aggregate shape, queries, audit actor, retention, and
soft-delete behavior.

## Prior art / precedents

- [`beaver-nest-app-setup`](../../in-progress/beaver-nest-app-setup/README.md) — fixes SQLite, DbUp,
  single-host, no-ORM, backup, and real-database test boundaries.
- [BeaverNest Vision](../../../repo-governance/vision/beaver-nest.md) — names assistant, content,
  posting, and workflow capabilities that may supply the first stateful slice.
- [Functional Programming](../../../repo-governance/development/pattern/functional-programming.md) —
  requires pure domain logic and an explicit imperative persistence edge.
- [Database Audit Trail](../../../repo-governance/development/pattern/database-audit-trail.md) — applies
  to every future domain table.

## Proposed direction (sketch)

Choose one minimal stateful behavior, design its table and repository port with the behavior, use
explicit parameterized SQL, and add a query builder only if measured query composition makes direct
SQL materially worse. Apply a forward-only DbUp migration and test against real disposable SQLite
files at integration/E2E levels.

## Rough scope & non-goals

In scope: one concrete product aggregate with durable create/read behavior, audit fields, migration,
repository boundary, and restart/backup coverage.

Out of scope: generic persistence abstractions, an ORM, selecting a query builder without need,
multi-tenant ownership, PostgreSQL, or a catch-all key/value table.

## Risks & open questions

- Which real capability supplies the first aggregate?
- What actor value is honest while the app has one VPN-trusted shared workspace and no identities?
- Does the behavior require update/soft-delete now, or only create/read?
- Are direct parameterized queries sufficient, or does concrete dynamic composition justify a query
  builder?

## What success looks like + promotion signal

Success means BeaverNest durably stores and retrieves real product data through one useful user
behavior, not merely a migration journal or diagnostic setting. Promote only when that behavior has
been selected and its lifecycle can be specified in Gherkin.
