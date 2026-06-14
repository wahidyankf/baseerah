module OseBe.Infrastructure.Database

open System

/// Reads DATABASE_URL or fails fast. ose-be is a server backend and
/// requires PostgreSQL — there is no SQLite dev fallback (see tech-docs
/// Deviations / Decisions). Migration execution lives in the db bounded context
/// (Contexts/Db/Infrastructure/DbMigrations.fs).
let requireDatabaseUrl () : string =
    match Environment.GetEnvironmentVariable("DATABASE_URL") with
    | null
    | "" -> failwith "DATABASE_URL is required (PostgreSQL connection string)"
    | value -> value
