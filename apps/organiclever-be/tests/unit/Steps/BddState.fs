module OrganicleverBe.Tests.Unit.Steps.BddState

open System
open System.Collections.Generic
open System.Text.RegularExpressions
open System.Threading.Tasks
open Microsoft.AspNetCore.TestHost
open Microsoft.AspNetCore.Hosting
open Microsoft.Extensions.DependencyInjection
open Giraffe
open OrganicleverBe.Contexts.Journal.Infrastructure

/// Build an in-process test server from a Giraffe HttpHandler.
let buildClient (handler: HttpHandler) : System.Net.Http.HttpClient =
    let builder =
        WebHostBuilder()
            .ConfigureServices(fun (s: IServiceCollection) -> s.AddGiraffe() |> ignore)
            .Configure(fun app -> app.UseGiraffe(handler))

    let server = new TestServer(builder)
    server.CreateClient()

/// An in-memory implementation of the journal repository port, used to exercise
/// the journal CRUD routing surface in unit tests without a PostgreSQL backend.
let inMemoryRepository () : JournalRepository =
    let store = Dictionary<string, JournalEntryRow>()

    { Create =
        fun row ->
            task {
                store[row.Id] <- row
                return row
            }
      FindById =
        fun id ->
            task {
                match store.TryGetValue id with
                | true, row -> return Some row
                | false, _ -> return None
            }
      List = fun () -> task { return store.Values |> Seq.sortByDescending (fun r -> r.CreatedAt) |> List.ofSeq }
      Update =
        fun row ->
            task {
                if store.ContainsKey row.Id then
                    store[row.Id] <- row
                    return Some row
                else
                    return None
            }
      Delete =
        fun id ->
            task {
                match store.Remove id with
                | true -> return true
                | false -> return false
            } }

/// Extracts the `id` field value from a JSON entry response body.
let extractId (body: string) : string =
    let m = Regex.Match(body, "\"id\"\\s*:\\s*\"([^\"]+)\"")

    if m.Success then
        m.Groups[1].Value
    else
        failwith (sprintf "no id found in response body: %s" body)
