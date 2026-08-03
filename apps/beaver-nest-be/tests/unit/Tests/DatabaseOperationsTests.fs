module BeaverNestBe.Tests.Unit.Tests.DatabaseOperationsTests

open System
open System.IO
open Microsoft.Data.Sqlite
open Xunit
open BeaverNestBe.Domain.DatabaseConfiguration
open BeaverNestBe.Infrastructure.Migrations
open BeaverNestBe.Infrastructure.Sqlite.Connection
open BeaverNestBe.Infrastructure.Sqlite.Errors
open BeaverNestBe.Operations.Database

[<Theory>]
[<InlineData("../escape.sqlite3")>]
[<InlineData("not-a-database")>]
[<InlineData("nested/backup.sqlite3")>]
let ``backup names cannot escape their fixed directory`` (name: string) =
    Assert.True(validateBackupName name |> Result.isError)

[<Fact>]
let ``valid backup names remain beneath the fixed backup directory`` () =
    Assert.Equal(Ok "/var/backups/beaver-nest/snapshot.sqlite3", validateBackupName "snapshot.sqlite3")

[<Fact>]
let ``backup and restore report safe errors when a valid backup is unavailable`` () =
    let directory =
        Path.Combine(Path.GetTempPath(), "beaver-nest-operation-" + Guid.NewGuid().ToString("N"))

    let configuration = create directory 100 |> Result.defaultWith failwith
    Assert.Equal(Error "backup failed", backup configuration "snapshot.sqlite3")
    Assert.Equal(Error "backup does not exist", restore configuration "snapshot.sqlite3")

[<Fact>]
let ``disposable SQLite backup and restore preserve a recoverable live database`` () =
    let root =
        Path.Combine(Path.GetTempPath(), "beaver-nest-backup-" + Guid.NewGuid().ToString("N"))

    let dataDirectoryPath = Path.Combine(root, "data")
    let backupDirectoryPath = Path.Combine(root, "backups")
    let configuration = create dataDirectoryPath 100 |> Result.defaultWith failwith

    use source = openConfigured configuration
    use setup = source.CreateCommand()
    setup.CommandText <- "CREATE TABLE BackupProof (Value TEXT NOT NULL); INSERT INTO BackupProof VALUES ('kept');"
    setup.ExecuteNonQuery() |> ignore
    setup.CommandText <- "PRAGMA wal_checkpoint(TRUNCATE);"
    setup.ExecuteNonQuery() |> ignore
    source.Close()

    let backupPath =
        backupAt backupDirectoryPath configuration "snapshot.sqlite3"
        |> Result.defaultWith failwith

    Assert.True(File.Exists backupPath)

    use changed = openConfigured configuration
    use change = changed.CreateCommand()
    change.CommandText <- "CREATE TABLE LaterChange (Value TEXT NOT NULL);"
    change.ExecuteNonQuery() |> ignore
    changed.Close()
    File.WriteAllText(databasePath configuration + "-wal", "stale")
    File.WriteAllText(databasePath configuration + "-shm", "stale")

    Assert.Equal(Ok(), restoreAt backupDirectoryPath configuration "snapshot.sqlite3")
    Assert.True(Directory.GetFiles(dataDirectoryPath, "beaver-nest.sqlite3.replaced-*").Length = 1)
    Assert.False(File.Exists(databasePath configuration + "-wal"))
    Assert.False(File.Exists(databasePath configuration + "-shm"))

    use restored = openConfigured configuration
    use verify = restored.CreateCommand()
    verify.CommandText <- "SELECT Value FROM BackupProof;"
    Assert.Equal("kept", string (verify.ExecuteScalar()))

[<Fact>]
let ``SQLite provider error classification is closed and safe`` () =
    let cases =
        [ SqliteException("busy", 5) :> exn, Busy, "database is busy"
          SqliteException("locked", 6) :> exn, Busy, "database is busy"
          SqliteException("corrupt", 11) :> exn, InvalidDatabase, "database operation failed"
          Exception("untrusted detail"), FailedMigration, "database migration failed" ]

    cases
    |> List.iter (fun (exceptionValue, expectedError, expectedMessage) ->
        let error = classify exceptionValue
        Assert.Equal(expectedError, error)
        Assert.Equal(expectedMessage, safeMessage error))

[<Fact>]
let ``configured SQLite connection applies required safeguards and DbUp journal is idempotent`` () =
    let directory =
        Path.Combine(Path.GetTempPath(), "beaver-nest-sqlite-" + Guid.NewGuid().ToString("N"))

    let configuration = create directory 250 |> Result.defaultWith failwith

    match initialize configuration with
    | Ok() -> ()
    | Error error -> failwithf "initial migration failed: %A" error

    match initialize configuration with
    | Ok() -> ()
    | Error error -> failwithf "restart migration failed: %A" error

    use connection = openConfigured configuration
    use settings = connection.CreateCommand()
    settings.CommandText <- "PRAGMA foreign_keys;"
    Assert.Equal(1L, unbox<int64> (settings.ExecuteScalar()))
    settings.CommandText <- "PRAGMA busy_timeout;"
    Assert.Equal(250L, unbox<int64> (settings.ExecuteScalar()))
    settings.CommandText <- "PRAGMA journal_mode;"
    Assert.Equal("wal", string (settings.ExecuteScalar()))
    Assert.Equal("current", journalState [ initializationScriptName ] [ initializationScriptName ])

[<Fact>]
let ``DbUp migration failures are reduced to a safe provider-independent error`` () =
    let directory =
        Path.Combine(Path.GetTempPath(), "beaver-nest-broken-" + Guid.NewGuid().ToString("N"))

    let configuration = create directory 100 |> Result.defaultWith failwith
    Assert.Equal(Error FailedMigration, apply configuration [ "broken.sql", "not valid SQL" ])
