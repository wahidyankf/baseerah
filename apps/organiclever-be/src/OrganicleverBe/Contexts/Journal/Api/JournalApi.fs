namespace OrganicleverBe.Contexts.Journal

open System.IO
open System.Text.Json
open Microsoft.AspNetCore.Http
open Giraffe
open OrganicleverBe.Contexts.Journal.Domain
open OrganicleverBe.Contexts.Journal.Infrastructure
open OrganicleverBe.Contexts.Journal.Application

/// HTTP API for the journal bounded context: full CRUD over journal entries,
/// rooted at /api/v1/journal/entries. Handlers are parameterised over the
/// storage port so the routing surface is unit-testable against an in-memory
/// repository.
module Api =

    /// Reads the raw JSON request body.
    let private readBody (ctx: HttpContext) =
        task {
            use reader = new StreamReader(ctx.Request.Body)
            return! reader.ReadToEndAsync()
        }

    /// Extracts a string property from a JSON element, or None when absent/null.
    let private tryString (el: JsonElement) (name: string) : string option =
        match el.TryGetProperty name with
        | true, v when v.ValueKind = JsonValueKind.String -> Some(v.GetString())
        | _ -> None

    /// Extracts a JSON sub-object/array property as its raw JSON text.
    let private tryRawJson (el: JsonElement) (name: string) : string option =
        match el.TryGetProperty name with
        | true, v when v.ValueKind <> JsonValueKind.Null -> Some(v.GetRawText())
        | _ -> None

    /// Extracts a string-array property as a list, defaulting to empty.
    let private stringList (el: JsonElement) (name: string) : string list =
        match el.TryGetProperty name with
        | true, v when v.ValueKind = JsonValueKind.Array ->
            [ for item in v.EnumerateArray() do
                  if item.ValueKind = JsonValueKind.String then
                      yield item.GetString() ]
        | _ -> []

    /// Parses a NewEntryInput from a request body JSON string.
    let private parseNewEntry (body: string) : Result<NewEntryInput, string> =
        try
            use doc = JsonDocument.Parse body
            let root = doc.RootElement

            Ok
                { Name = defaultArg (tryString root "name") ""
                  Payload = defaultArg (tryRawJson root "payload") "{}"
                  StartedAt = defaultArg (tryString root "startedAt") ""
                  FinishedAt = defaultArg (tryString root "finishedAt") ""
                  Labels = stringList root "labels" }
        with ex ->
            Error(sprintf "invalid JSON body: %s" ex.Message)

    /// Parses an UpdateEntryInput from a request body JSON string.
    let private parseUpdateEntry (body: string) : Result<UpdateEntryInput, string> =
        try
            use doc = JsonDocument.Parse body
            let root = doc.RootElement

            Ok
                { Name = tryString root "name"
                  Payload = tryRawJson root "payload" }
        with ex ->
            Error(sprintf "invalid JSON body: %s" ex.Message)

    /// Serialises a journal entry to the wire shape, parsing the stored payload
    /// string back into a JSON object so clients receive structured JSON.
    let private parseJsonElement (s: string) : JsonElement =
        use doc = JsonDocument.Parse s
        doc.RootElement.Clone()

    let private entryToWire (entry: JournalEntry) =
        let payload =
            try
                parseJsonElement entry.Payload
            with _ ->
                parseJsonElement "{}"

        {| id = entry.Id
           name = entry.Name
           payload = payload
           startedAt = entry.StartedAt
           finishedAt = entry.FinishedAt
           labels = entry.Labels
           createdAt = entry.CreatedAt
           updatedAt = entry.UpdatedAt |}

    /// POST /api/v1/journal/entries → 201 with the created entry, 400 on invalid input.
    let createHandler (repo: JournalRepository) : HttpHandler =
        fun next ctx ->
            task {
                let! body = readBody ctx

                match parseNewEntry body with
                | Error msg -> return! RequestErrors.BAD_REQUEST {| error = msg |} next ctx
                | Ok input ->
                    let! result = create repo input

                    match result with
                    | Error msg -> return! RequestErrors.BAD_REQUEST {| error = msg |} next ctx
                    | Ok entry ->
                        ctx.SetStatusCode 201
                        return! json (entryToWire entry) next ctx
            }

    /// GET /api/v1/journal/entries → 200 with all entries.
    let listHandler (repo: JournalRepository) : HttpHandler =
        fun next ctx ->
            task {
                let! entries = list repo
                return! json (entries |> List.map entryToWire) next ctx
            }

    /// GET /api/v1/journal/entries/{id} → 200 with the entry, 404 when absent.
    let getByIdHandler (repo: JournalRepository) (id: string) : HttpHandler =
        fun next ctx ->
            task {
                let! found = findById repo id

                match found with
                | Some entry -> return! json (entryToWire entry) next ctx
                | None -> return! RequestErrors.NOT_FOUND {| error = "entry not found" |} next ctx
            }

    /// PUT /api/v1/journal/entries/{id} → 200 with the updated entry, 404 when
    /// absent, 400 on invalid input.
    let updateHandler (repo: JournalRepository) (id: string) : HttpHandler =
        fun next ctx ->
            task {
                let! body = readBody ctx

                match parseUpdateEntry body with
                | Error msg -> return! RequestErrors.BAD_REQUEST {| error = msg |} next ctx
                | Ok input ->
                    let! result = update repo id input

                    match result with
                    | Error msg -> return! RequestErrors.BAD_REQUEST {| error = msg |} next ctx
                    | Ok None -> return! RequestErrors.NOT_FOUND {| error = "entry not found" |} next ctx
                    | Ok(Some entry) -> return! json (entryToWire entry) next ctx
            }

    /// DELETE /api/v1/journal/entries/{id} → 204 on success, 404 when absent.
    let deleteHandler (repo: JournalRepository) (id: string) : HttpHandler =
        fun next ctx ->
            task {
                let! removed = delete repo id

                if removed then
                    ctx.SetStatusCode 204
                    return! next ctx
                else
                    return! RequestErrors.NOT_FOUND {| error = "entry not found" |} next ctx
            }

    /// Routes for the journal context, bound to a storage port.
    let routes (repo: JournalRepository) : HttpHandler =
        choose
            [ GET >=> route "/api/v1/journal/entries" >=> listHandler repo
              GET >=> routef "/api/v1/journal/entries/%s" (getByIdHandler repo)
              POST >=> route "/api/v1/journal/entries" >=> createHandler repo
              PUT >=> routef "/api/v1/journal/entries/%s" (updateHandler repo)
              DELETE >=> routef "/api/v1/journal/entries/%s" (deleteHandler repo) ]
