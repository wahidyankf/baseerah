module BeaverNestBe.Infrastructure.Migrations

open System.IO
open DbUp
open BeaverNestBe.Domain.DatabaseConfiguration
open BeaverNestBe.Infrastructure.Sqlite.Connection
open BeaverNestBe.Infrastructure.Sqlite.Errors

let initializationScriptName = "001-initialize.sql"

let apply configuration scripts =
    try
        Directory.CreateDirectory(dataDirectory configuration) |> ignore
        let builder = DeployChanges.To.SqliteDatabase(connectionString configuration)

        let configured =
            scripts
            |> List.fold
                (fun (state: DbUp.Builder.UpgradeEngineBuilder) (name: string, sql: string) ->
                    state.WithScript(name, sql))
                builder

        let result = configured.LogToNowhere().Build().PerformUpgrade()
        if result.Successful then Ok() else Error FailedMigration
    with exceptionValue ->
        Error(classify exceptionValue)

let initialize configuration =
    apply configuration [ initializationScriptName, "SELECT 1;" ]

let journalState expectedScripts actualScripts =
    if Set.ofList expectedScripts = Set.ofList actualScripts then
        "current"
    else
        "pending"
