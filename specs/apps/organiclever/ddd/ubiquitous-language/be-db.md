# Ubiquitous Language — be-db

**Bounded context**: `be-db`
**Maintainer**: organiclever-be team
**Last reviewed**: 2026-06-12

## Responsibility

Manages the database schema lifecycle for `organiclever-be`. On each startup, applies
any pending `sqlx` migrations to the connected PostgreSQL instance before the HTTP
server begins accepting requests. This ensures the schema is always up-to-date
without manual intervention.

## Term index

| Term              | Code identifier(s) | Used in features   |
| ----------------- | ------------------ | ------------------ |
| migration         | `run_migrations`   | migrations.feature |
| migration routine | `run_migrations`   | migrations.feature |
| applied migration | `run_migrations`   | migrations.feature |

## Out of scope

- Application-level data models (belong to their respective bounded contexts)
- Connection pool configuration (belongs to infrastructure / `Config`)
- Schema rollback / downgrade — not supported; forward-only migrations only
