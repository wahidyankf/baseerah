---
title: Database Audit Trail Pattern
description: Required 6-column audit trail that every database table in open-sharia-enterprise must include for auditability, soft-delete, compliance, and production debugging
category: explanation
subcategory: development
tags:
  - database
  - audit-trail
  - soft-delete
  - sqlx
  - migrations
created: 2026-03-09
---

# Database Audit Trail Pattern

Every database table in open-sharia-enterprise MUST include six audit trail columns. These columns record who created, last updated, and soft-deleted each row, along with when each action occurred. Rows with `deleted_at IS NULL` are active; rows with a non-null `deleted_at` are soft-deleted and invisible to normal queries.

## Principles Implemented/Respected

This pattern implements the following core principles:

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: All audit metadata is stored in dedicated, named columns with mandatory types and nullability. There is no implicit or hidden tracking; every change is visible in the schema.

- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: SQLx's `sqlx::migrate!()` macro embeds and applies migrations automatically at startup. Manual service code is only required for soft-delete columns (`deleted_at`, `deleted_by`).

- **[Reproducibility First](../../principles/software-engineering/reproducibility.md)**: SQLx embeds migration files at compile time, ensuring the schema is reproducible across PostgreSQL environments (dev/staging/prod) and Dockerised test databases without divergence.

- **[Documentation First](../../principles/content/documentation-first.md)**: This pattern documents the required columns, types, and implementation approach before any table is created, ensuring teams follow a consistent and verifiable standard.

## Conventions Implemented/Respected

This pattern respects the following conventions:

- **[Content Quality Principles](../../conventions/writing/quality.md)**: This document uses active voice, a single H1, and proper heading nesting.

- **[Acceptance Criteria Convention](../infra/acceptance-criteria.md)**: The compliance checklist at the end of this document provides testable, concrete criteria for verifying a table meets this pattern.

## Required Audit Columns

Every table MUST include all six columns in the order listed below.

```mermaid
graph TD
    T["Table<br/>(any domain entity)"] --> R[Required Audit Columns]
    T --> O[Optional Audit Columns]

    R --> C1["created_at<br/>TIMESTAMPTZ NOT NULL"]
    C1 --> C2["created_by<br/>VARCHAR NOT NULL"]
    C2 --> C3["updated_at<br/>TIMESTAMPTZ NOT NULL"]
    C3 --> C4["updated_by<br/>VARCHAR NOT NULL"]

    O --> C5["deleted_at<br/>TIMESTAMPTZ NULL"]
    C5 --> C6["deleted_by<br/>VARCHAR NULL"]

    classDef required fill:#0173B2,color:#ffffff,stroke:#0173B2
    classDef optional fill:#029E73,color:#ffffff,stroke:#029E73

    class R,C1,C2,C3,C4 required
    class O,C5,C6 optional
```

| Column       | Type           | Nullable | Default    | Description                         |
| ------------ | -------------- | -------- | ---------- | ----------------------------------- |
| `created_at` | `TIMESTAMPTZ`  | NOT NULL | `NOW()`    | When the row was created (UTC)      |
| `created_by` | `VARCHAR(255)` | NOT NULL | `'system'` | Who or what created the row         |
| `updated_at` | `TIMESTAMPTZ`  | NOT NULL | `NOW()`    | When the row was last updated (UTC) |
| `updated_by` | `VARCHAR(255)` | NOT NULL | `'system'` | Who or what last updated the row    |
| `deleted_at` | `TIMESTAMPTZ`  | NULL     | —          | When the row was soft-deleted (UTC) |
| `deleted_by` | `VARCHAR(255)` | NULL     | —          | Who or what soft-deleted the row    |

Blue columns (required) are always non-null and populated by the database default or the calling service. Green columns (optional by value) are always present in the schema but null for active rows.

## Why This Pattern Exists

**Auditability**: Every change to every row is traceable to an actor and a timestamp. Security reviews, compliance audits, and internal investigations can reconstruct the full history of any record.

**Soft-Delete**: Setting `deleted_at` and `deleted_by` hides a row from normal queries without destroying data. Hard deletes make recovery impossible and break foreign key history. Soft-delete preserves referential integrity and enables undelete workflows.

**Compliance**: Sharia-compliant financial systems require evidence that transactions and contracts were not retroactively altered. The audit columns provide an immutable creation record and a last-modified record for every entity.

**Production Debugging**: When an incident occurs, `updated_at` narrows the time window and `updated_by` identifies the service or user responsible. Without these columns, incident investigation relies on log search, which is slower and less reliable.

## Migration Tool by Language

Each backend uses the idiomatic migration tool for its language and framework ecosystem. All tools must apply the same six audit columns to every table.

| App             | Migration Tool | License |
| --------------- | -------------- | ------- |
| organiclever-be | SQLx migrate   | MIT     |
| ose-app-be      | SQLx migrate   | MIT     |

> For polyglot migration tool patterns (Liquibase, Ecto, Alembic, goose, Flyway, EF Core, Migratus, @effect/sql, SQLx, Drizzle), see the [ose-primer](https://github.com/wahidyankf/ose-primer) repository.

For licensing decisions related to Liquibase's FSL-1.1-ALv2 licence (introduced in version 5.0), see [Licensing Decisions](../../../docs/explanation/software-engineering/licensing/licensing-decisions.md).

## Schema Migration

Every backend applies the six audit columns through its migration tool. The canonical column definitions are identical regardless of tool — only the migration file format differs.

Regardless of the tool used, migrations must satisfy:

- All six audit columns present in every table, in the order listed above
- `created_at` and `updated_at` use timezone-aware timestamps (`TIMESTAMPTZ` for PostgreSQL, equivalent for other databases)
- `created_by` and `updated_by` default to `'system'` so raw migrations and background jobs produce a traceable actor
- `deleted_at` and `deleted_by` are nullable with no default — `NULL` is the active-row state
- Each migration is reversible (rollback support where the tool provides it)

### Rust / SQLx: `sqlx::migrate!`

Use plain `.sql` files under `migrations/`. SQLx embeds them at compile time via the `sqlx::migrate!()` macro and applies them in filename order at startup.

The following example shows the `members` table as the reference implementation. Apply the same pattern to every new table.

```sql
-- migrations/20240101000001_create_members.sql
CREATE TABLE members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    -- audit columns
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_by UUID NOT NULL,
    deleted_at TIMESTAMPTZ,
    deleted_by UUID
);
```

Key points:

- Migration files are named `{timestamp}_{description}.sql` so SQLx applies them in deterministic order.
- `DEFAULT now()` provides a safe fallback for raw SQL inserts (migrations, seeds, background jobs).
- `created_by` and `updated_by` are `UUID` referencing the actor; use `TEXT NOT NULL DEFAULT 'system'` when the actor is a string identifier rather than a UUID.
- `deleted_at` and `deleted_by` are nullable with no default — `NULL` is the active-row state.
- SQLx does not have a built-in rollback concept for embedded migrations; provide a separate `_down.sql` file or manage rollbacks manually if required.

## Rust Entity Implementation

### Struct Definition

Derive `sqlx::FromRow` on every domain struct that maps to an audited table. Use `time::OffsetDateTime` for timestamp columns and `uuid::Uuid` for UUID columns.

```rust
// src/domain/member.rs
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Member {
    pub id: Uuid,
    pub name: String,
    // audit columns
    pub created_at: OffsetDateTime,
    pub created_by: Uuid,
    pub updated_at: OffsetDateTime,
    pub updated_by: Uuid,
    pub deleted_at: Option<OffsetDateTime>,
    pub deleted_by: Option<Uuid>,
}
```

### Run Migrations at Startup

Call `sqlx::migrate!()` in `main` (or in the application builder) before the HTTP server starts accepting requests.

```rust
// src/main.rs (excerpt)
let pool = sqlx::PgPool::connect(&database_url).await?;
sqlx::migrate!("./migrations").run(&pool).await?;
```

### Soft-Delete in the Repository Layer

`deleted_at` and `deleted_by` are set explicitly in the repository layer. Never issue a `DELETE` statement on audited tables; always use a soft-delete `UPDATE`.

```rust
// src/repository/member_repository.rs (excerpt)
pub async fn soft_delete_member(
    pool: &PgPool,
    id: Uuid,
    actor: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE members
        SET deleted_at = now(),
            deleted_by = $1
        WHERE id = $2
          AND deleted_at IS NULL
        "#,
        actor,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
```

## Soft-Delete Query Discipline

All queries against audited tables MUST include `WHERE deleted_at IS NULL` unless the endpoint is an explicit admin or audit endpoint.

**PASS: active-row query — soft-deleted rows excluded**:

```rust
sqlx::query_as!(
    Member,
    "SELECT * FROM members WHERE deleted_at IS NULL"
)
.fetch_all(pool)
.await?
```

**FAIL: never omit the `deleted_at` filter without explicit justification**:

```rust
// Returns soft-deleted rows — only acceptable for admin/audit endpoints
sqlx::query_as!(Member, "SELECT * FROM members")
    .fetch_all(pool)
    .await?
```

When an admin or audit endpoint legitimately needs soft-deleted rows, name the function clearly (e.g., `fetch_all_including_deleted`) and restrict the route to admin roles.

## Rust Nullability Convention

Rust's type system encodes nullability directly. The audit field mapping is:

| Column       | Rust type                | Rationale                          |
| ------------ | ------------------------ | ---------------------------------- |
| `created_at` | `OffsetDateTime`         | Non-null; database `DEFAULT now()` |
| `created_by` | `Uuid`                   | Non-null; caller must supply actor |
| `updated_at` | `OffsetDateTime`         | Non-null; database `DEFAULT now()` |
| `updated_by` | `Uuid`                   | Non-null; caller must supply actor |
| `deleted_at` | `Option<OffsetDateTime>` | `None` means active row            |
| `deleted_by` | `Option<Uuid>`           | `None` means active row            |

`Option<T>` maps directly to nullable SQL columns via SQLx's `FromRow` derive.

## Compliance Checklist

Use this checklist when adding a new table or reviewing an existing one.

### Schema (All Migration Tools)

- [ ] Migration includes all six audit columns in the correct order
- [ ] `created_at` and `updated_at` are timezone-aware timestamps, NOT NULL, defaulting to the current time
- [ ] `created_by` and `updated_by` are string columns (max 255 chars), NOT NULL, defaulting to `'system'`
- [ ] `deleted_at` and `deleted_by` are nullable with no default
- [ ] Migration is reversible (rollback or down migration provided where the tool supports it)

**Rust / SQLx additional checks:**

- [ ] Migration file name follows `{timestamp}_{description}.sql` format
- [ ] `sqlx::migrate!("./migrations").run(&pool).await?` is called before the server starts accepting requests

### Struct (Rust / SQLx)

- [ ] Struct derives `sqlx::FromRow`
- [ ] `created_at` and `updated_at` fields use `time::OffsetDateTime` (non-`Option`)
- [ ] `created_by` and `updated_by` fields use `Uuid` (non-`Option`)
- [ ] `deleted_at` and `deleted_by` fields use `Option<OffsetDateTime>` and `Option<Uuid>` respectively

### Repository Layer (Rust / SQLx)

- [ ] No `DELETE` statement issued against audited tables
- [ ] Soft-delete issues an `UPDATE` setting both `deleted_at = now()` and `deleted_by = $actor`
- [ ] Soft-delete query includes `AND deleted_at IS NULL` to guard against double-deletes

### Queries

- [ ] All `SELECT` queries include `WHERE deleted_at IS NULL` unless the endpoint is explicitly an admin/audit endpoint
- [ ] Functions that intentionally return soft-deleted rows are named clearly (e.g., `fetch_all_including_deleted`) and the route is restricted to admin roles

## Related Documentation

- [Acceptance Criteria Convention](../infra/acceptance-criteria.md) - Writing testable criteria for features involving audited entities
- [Functional Programming Practices](./functional-programming.md) - Pure functions for business logic separate from audit side effects
- [Reproducible Environments Convention](../workflow/reproducible-environments.md) - Why consistent PostgreSQL environments across dev/staging/prod matter for test reliability
- [Licensing Decisions](../../../docs/explanation/software-engineering/licensing/licensing-decisions.md) - License analysis for migration tools (Liquibase FSL-1.1-ALv2 and others)

## References

**Project Plans:**

- [Auth Register/Login Tech Docs](../../../plans/done/2026-04-22__auth-register-login/tech-docs.md) - Reference implementation of the `users` table applying this pattern

**External (Rust / SQLx):**

- [SQLx `migrate!` macro](https://docs.rs/sqlx/latest/sqlx/macro.migrate.html)
- [SQLx `Migrate` trait](https://docs.rs/sqlx/latest/sqlx/migrate/trait.Migrate.html)
- [time crate `OffsetDateTime`](https://docs.rs/time/latest/time/struct.OffsetDateTime.html)
- [uuid crate](https://docs.rs/uuid/latest/uuid/)

**External (Other Active Ecosystems):**

- [goose migrations (Go)](https://github.com/pressly/goose)
- [DbUp migrations (F#/.NET)](https://dbup.readthedocs.io/)
- [EF Core Migrations (C#/.NET)](https://learn.microsoft.com/en-us/ef/core/managing-schemas/migrations/)
- [@effect/sql Migrator (TypeScript)](https://effect.website/docs/sql/sql-migrator)
- [Drizzle migrations (TypeScript)](https://orm.drizzle.team/docs/migrations)

## Migration Tooling Pitfalls

Lessons from adding migration tooling across 8 language ecosystems (2026-03-27). These apply to any
project replacing programmatic DDL (`AutoMigrate`, `create_all`, `EnsureCreated`, `SchemaUtils.create`)
with dedicated migration tools.

### Match the original schema exactly

Migration SQL must produce the **identical schema** that the previous programmatic DDL created — same
column types, same precision, same constraints. Common mismatches that break E2E tests:

- **DECIMAL precision**: `DECIMAL(19,4)` forces trailing zeros (`10.5000` vs `10.50`). If the
  original DDL used `DECIMAL` (arbitrary precision), keep it.
- **FK constraints**: ORMs like EF Core `EnsureCreated()` and Exposed `SchemaUtils.create()` often
  do NOT create FK constraints unless navigation properties are explicitly defined. Adding FKs in
  migration SQL breaks code that inserts with empty/placeholder foreign keys (e.g., `Guid.Empty`).
- **Type mismatches**: `UUID` vs `TEXT`, `TIMESTAMPTZ` vs `timestamp without time zone`. Check what
  the ORM driver actually generates.

**Rule**: Before writing migration SQL, inspect the original DDL source (the programmatic code being
replaced) and replicate its types exactly.

### Coverage tool configuration

When adding new Cargo crates (SQLx migrate), Go modules (goose), or Python packages (Alembic):

- **cargo-llvm-cov (Rust)**: Migration SQL files do not inflate coverage. Ensure `migrations/`
  is listed in Nx `inputs` so cache invalidates when migration files change.
- **Nx inputs**: For Go apps using `embed.FS`, add the migrations directory to `inputs` in
  `project.json` so Nx cache invalidates when migration files change.

### Embedded filesystem paths

Go's `embed.FS` creates a filesystem rooted at the package directory. When using
`goose.NewProvider(dialect, db, embedFS)`, goose expects migration files at the FS root. If the
embed directive is `//go:embed migrations/*.sql`, files live under `migrations/` — use
`fs.Sub(embedFS, "migrations")` to give goose the correct root.

### JVM locale affects test parsing

Cucumber JVM's `{double}` parameter type uses the JVM default locale. On locales where `.` is the
thousands separator (e.g., `id_ID`), `50.5` parses as `505.0`. Fix by adding
`-Duser.language=en -Duser.country=US` to JVM opts in test runner configuration.

### Docker environment differences

Integration tests (built inside `Dockerfile.integration`) and E2E tests (`cargo run` / `uv run`
with volume mount) may behave differently:

- **Rust migrations**: SQLx `sqlx::migrate!` embeds migration SQL at compile time. When using
  volume mounts, ensure `migrations/` is present in the container build context.
- **Go version**: Keep `Dockerfile`, `Dockerfile.integration`, AND `Dockerfile.be.dev` in sync with
  `go.mod`'s Go version requirement.
- **Python Alembic**: The `Dockerfile.integration` needs explicit `COPY alembic/ alembic/` and
  `COPY alembic.ini alembic.ini` — the standard `COPY . .` may not include them if `.dockerignore`
  is aggressive.
