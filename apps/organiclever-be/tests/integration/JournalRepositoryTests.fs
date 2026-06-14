module OrganicleverBe.IntegrationTests.JournalRepositoryTests

open System
open Microsoft.EntityFrameworkCore
open Xunit
open OrganicleverBe.Infrastructure.AppDbContext
open OrganicleverBe.Contexts.Db.Infrastructure
open OrganicleverBe.Contexts.Journal.Infrastructure
open OrganicleverBe.Contexts.Journal.Domain
open OrganicleverBe.Contexts.Journal.Application

let private connectionString () =
    match Environment.GetEnvironmentVariable("DATABASE_URL") with
    | null
    | "" -> failwith "DATABASE_URL must be set for integration tests"
    | value -> value

let private newContext (connStr: string) : AppDbContext =
    let options =
        DbContextOptionsBuilder<AppDbContext>().UseNpgsql(connStr).UseSnakeCaseNamingConvention().Options

    new AppDbContext(options)

let private sampleInput () : NewEntryInput =
    { Name = "reading"
      Payload = "{\"title\":\"Clean Code\"}"
      StartedAt = "2026-06-14T10:00:00Z"
      FinishedAt = "2026-06-14T10:30:00Z"
      Labels = [ "books" ] }

[<Fact>]
let ``journal CRUD round-trips against PostgreSQL`` () =
    let connStr = connectionString ()
    runMigrations connStr

    task {
        use ctx = newContext connStr
        let repo = efRepository ctx

        // Create
        let! created = create repo (sampleInput ())

        let entry =
            match created with
            | Ok e -> e
            | Error msg -> failwith (sprintf "create failed: %s" msg)

        Assert.Equal("reading", entry.Name)
        Assert.False(String.IsNullOrWhiteSpace entry.Id)

        // Read by id
        let! found = findById repo entry.Id
        Assert.True(found.IsSome)

        // Update
        let! updated =
            update
                repo
                entry.Id
                { Name = Some "learning"
                  Payload = None }

        match updated with
        | Ok(Some e) -> Assert.Equal("learning", e.Name)
        | Ok None -> failwith "update returned None for an existing entry"
        | Error msg -> failwith (sprintf "update failed: %s" msg)

        // Delete
        let! removed = delete repo entry.Id
        Assert.True(removed)

        let! afterDelete = findById repo entry.Id
        Assert.True(afterDelete.IsNone)
    }
    |> fun t -> t.GetAwaiter().GetResult()
