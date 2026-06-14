module OrganicleverBe.Infrastructure.Database

open System

/// Reads DATABASE_URL or fails fast. organiclever-be is a server backend and
/// requires PostgreSQL — there is no SQLite dev fallback (see tech-docs
/// Deviations / Decisions). The on-boot migration routine lives in the db
/// bounded context (Contexts/Db/Infrastructure/DbMigrations.fs).
let requireDatabaseUrl () : string =
    match Environment.GetEnvironmentVariable("DATABASE_URL") with
    | null
    | "" -> failwith "DATABASE_URL is required (PostgreSQL connection string)"
    | value -> value
