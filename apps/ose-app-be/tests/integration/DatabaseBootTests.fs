module OseAppBe.IntegrationTests.DatabaseBootTests

open System
open Microsoft.EntityFrameworkCore
open Npgsql
open Xunit
open OseAppBe.Infrastructure.AppDbContext
open OseAppBe.Infrastructure.Database

let private connectionString () =
    match Environment.GetEnvironmentVariable("DATABASE_URL") with
    | null
    | "" -> failwith "DATABASE_URL must be set for integration tests"
    | value -> value

let private schemaVersionsExists (connStr: string) : bool =
    use conn = new NpgsqlConnection(connStr)
    conn.Open()

    use cmd =
        new NpgsqlCommand(
            "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'schemaversions')",
            conn
        )

    cmd.ExecuteScalar() :?> bool

[<Fact>]
let ``DbUp creates the SchemaVersions tracking table on boot`` () =
    let connStr = connectionString ()
    runMigrations connStr
    Assert.True(schemaVersionsExists connStr, "DbUp SchemaVersions table should exist after boot")

[<Fact>]
let ``EF context boots against PostgreSQL after migration`` () =
    let connStr = connectionString ()
    runMigrations connStr

    let options =
        DbContextOptionsBuilder<AppDbContext>().UseNpgsql(connStr).UseSnakeCaseNamingConvention().Options

    use ctx = new AppDbContext(options)
    Assert.True(ctx.Database.CanConnect(), "EF context should connect to PostgreSQL")
