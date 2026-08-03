module BeaverNestBe.Operations.Database

open System
open System.IO
open Microsoft.Data.Sqlite
open BeaverNestBe.Domain.DatabaseConfiguration
open BeaverNestBe.Infrastructure.Sqlite.Connection

let private backupDirectory = "/var/backups/beaver-nest"

let private validName name =
    not (String.IsNullOrWhiteSpace name)
    && name = Path.GetFileName name
    && name.EndsWith(".sqlite3", StringComparison.Ordinal)

let private backupPath name =
    if validName name then
        Ok(Path.Combine(backupDirectory, name))
    else
        Error "backup name is invalid"

let validateBackupName = backupPath

let private verify (path: string) =
    use connection = new SqliteConnection($"Data Source={path};Mode=ReadOnly")
    connection.Open()
    use integrity = connection.CreateCommand()
    integrity.CommandText <- "PRAGMA integrity_check;"
    let integrityResult = string (integrity.ExecuteScalar())
    use foreignKeys = connection.CreateCommand()
    foreignKeys.CommandText <- "PRAGMA foreign_key_check;"
    use rows = foreignKeys.ExecuteReader()
    integrityResult = "ok" && not (rows.Read())

let backup configuration name =
    match backupPath name with
    | Error error -> Error error
    | Ok destination when File.Exists destination -> Error "backup already exists"
    | Ok destination ->
        try
            Directory.CreateDirectory backupDirectory |> ignore
            use source = openConfigured configuration
            use target = new SqliteConnection($"Data Source={destination}")
            target.Open()
            source.BackupDatabase target

            if verify destination then
                Ok destination
            else
                Error "backup verification failed"
        with _ ->
            Error "backup failed"

let restore configuration name =
    match backupPath name with
    | Error error -> Error error
    | Ok source when not (File.Exists source) -> Error "backup does not exist"
    | Ok source when not (verify source) -> Error "backup verification failed"
    | Ok source ->
        try
            let live = databasePath configuration
            Directory.CreateDirectory(dataDirectory configuration) |> ignore
            let preserved = live + ".replaced-" + Guid.NewGuid().ToString("N")

            if File.Exists live then
                File.Move(live, preserved)

            File.Copy(source, live, false)

            [ live + "-wal"; live + "-shm" ]
            |> List.iter (fun stale ->
                if File.Exists stale then
                    File.Delete stale)

            Ok()
        with _ ->
            Error "restore failed"
