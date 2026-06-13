module OrganicleverBe.Infrastructure.Database

open System
open System.Reflection
open DbUp

/// Runs the embedded DbUp migrations against the given PostgreSQL connection
/// string. The migration scripts live as embedded resources in the
/// OrganicleverBe assembly (db/migrations/*.sql). Fails fast if any script
/// cannot be applied.
let runMigrations (connStr: string) : unit =
    let result =
        DeployChanges.To
            .PostgresqlDatabase(connStr)
            .WithScriptsEmbeddedInAssembly(Assembly.GetExecutingAssembly())
            .LogToConsole()
            .Build()
            .PerformUpgrade()

    if not result.Successful then
        failwith (sprintf "Database migration failed: %s" result.Error.Message)

/// Reads DATABASE_URL or fails fast. organiclever-be is a server backend and
/// requires PostgreSQL — there is no SQLite dev fallback (see tech-docs
/// Deviations / Decisions).
let requireDatabaseUrl () : string =
    match Environment.GetEnvironmentVariable("DATABASE_URL") with
    | null
    | "" -> failwith "DATABASE_URL is required (PostgreSQL connection string)"
    | value -> value
