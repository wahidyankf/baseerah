//! Infrastructure layer — database migration runner.

/// Apply all pending `sqlx` migrations to the given pool.
///
/// Uses the `./migrations` directory relative to the crate root (embedded at
/// compile time by `sqlx::migrate!`). Called once on boot before the HTTP
/// server starts, ensuring the schema is always up-to-date.
///
/// # Errors
///
/// Returns an [`anyhow::Error`] if the migration directory cannot be read,
/// a migration file is malformed, or a SQL statement fails to execute.
pub async fn run_migrations(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| anyhow::anyhow!("database migration failed: {e}"))
}
