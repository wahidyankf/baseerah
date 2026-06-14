module OrganicleverBe.IntegrationTests.Steps.DbMigrationSteps

open System
open Npgsql
open TickSpec
open Xunit
open OrganicleverBe.Contexts.Db.Infrastructure

// Step definitions for the be-db context (migrations.feature). They bind the
// Gherkin steps to the DbUp on-boot migration routine for the spec coverage
// validator.

let private connectionString () =
    match Environment.GetEnvironmentVariable("DATABASE_URL") with
    | null
    | "" -> failwith "DATABASE_URL must be set for integration tests"
    | value -> value

let private appliedMigrationCount (connStr: string) : int =
    use conn = new NpgsqlConnection(connStr)
    conn.Open()

    use exists =
        new NpgsqlCommand(
            "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'schemaversions')",
            conn
        )

    if not (exists.ExecuteScalar() :?> bool) then
        0
    else
        use cmd = new NpgsqlCommand("SELECT COUNT(*) FROM schemaversions", conn)
        cmd.ExecuteScalar() :?> int64 |> int

[<Given>]
let ``the database has no applied migrations`` () =
    // A fresh container starts with no schemaversions table; nothing to do.
    ()

[<When>]
let ``the backend runs its migration routine`` () = runMigrations (connectionString ())

[<Then>]
let ``the migrations table records at least one applied migration`` () =
    Assert.True(appliedMigrationCount (connectionString ()) >= 1, "at least one migration should be recorded")
