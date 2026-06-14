module OrganicleverBe.Tests.Unit.Steps.JournalSteps

open System.Net.Http
open System.Text
open TickSpec
open Xunit
open OrganicleverBe.Contexts.Journal.Api
open OrganicleverBe.Tests.Unit.Steps.BddState

// Step definitions for the be-journal context (journal-crud.feature). They drive
// the journal CRUD routing surface over an in-memory repository, binding each
// Gherkin step to executable code for the spec coverage validator.

let mutable private client: HttpClient option = None
let mutable private lastStatus = 0
let mutable private lastBody = ""
let mutable private createdId = ""

let private validEntryJson =
    "{\"name\":\"reading\",\"payload\":{\"title\":\"Clean Code\"},\"startedAt\":\"2026-06-14T10:00:00Z\",\"finishedAt\":\"2026-06-14T10:30:00Z\",\"labels\":[\"books\"]}"

let private jsonContent (body: string) : StringContent =
    new StringContent(body, Encoding.UTF8, "application/json")

[<Given>]
let ``the journal API is running`` () =
    let repo = inMemoryRepository ()
    client <- Some(buildClient (routes repo))

[<Given>]
let ``a journal entry has been created`` () =
    let resp =
        client.Value.PostAsync("/api/v1/journal/entries", jsonContent validEntryJson).Result

    createdId <- extractId (resp.Content.ReadAsStringAsync().Result)

[<When>]
let ``a client posts a valid journal entry`` () =
    let resp =
        client.Value.PostAsync("/api/v1/journal/entries", jsonContent validEntryJson).Result

    lastStatus <- int resp.StatusCode
    lastBody <- resp.Content.ReadAsStringAsync().Result

[<When>]
let ``a client posts a journal entry with a blank name`` () =
    let bad =
        "{\"name\":\"\",\"payload\":{},\"startedAt\":\"2026-06-14T10:00:00Z\",\"finishedAt\":\"2026-06-14T10:30:00Z\",\"labels\":[]}"

    let resp = client.Value.PostAsync("/api/v1/journal/entries", jsonContent bad).Result
    lastStatus <- int resp.StatusCode
    lastBody <- resp.Content.ReadAsStringAsync().Result

[<When>]
let ``a client lists the journal entries`` () =
    let resp = client.Value.GetAsync("/api/v1/journal/entries").Result
    lastStatus <- int resp.StatusCode
    lastBody <- resp.Content.ReadAsStringAsync().Result

[<When>]
let ``a client fetches a journal entry that does not exist`` () =
    let resp = client.Value.GetAsync("/api/v1/journal/entries/does-not-exist").Result
    lastStatus <- int resp.StatusCode
    lastBody <- resp.Content.ReadAsStringAsync().Result

[<When>]
let ``a client updates the journal entry name`` () =
    let resp =
        client.Value
            .PutAsync(sprintf "/api/v1/journal/entries/%s" createdId, jsonContent "{\"name\":\"learning\"}")
            .Result

    lastStatus <- int resp.StatusCode
    lastBody <- resp.Content.ReadAsStringAsync().Result

[<When>]
let ``a client deletes the journal entry`` () =
    let resp =
        client.Value.DeleteAsync(sprintf "/api/v1/journal/entries/%s" createdId).Result

    lastStatus <- int resp.StatusCode

[<Then>]
let ``the journal response status code should be 201`` () = Assert.Equal(201, lastStatus)

[<Then>]
let ``the journal response status code should be 400`` () = Assert.Equal(400, lastStatus)

[<Then>]
let ``the journal response status code should be 200`` () = Assert.Equal(200, lastStatus)

[<Then>]
let ``the journal response status code should be 404`` () = Assert.Equal(404, lastStatus)

[<Then>]
let ``the journal response status code should be 204`` () = Assert.Equal(204, lastStatus)

[<Then>]
let ``the journal response body should include an id`` () = Assert.Contains("\"id\"", lastBody)

[<Then>]
let ``the journal list should include the created entry`` () = Assert.Contains("reading", lastBody)

[<Then>]
let ``the updated journal entry should reflect the new name`` () = Assert.Contains("learning", lastBody)

[<Then>]
let ``fetching the deleted journal entry should return 404`` () =
    let resp =
        client.Value.GetAsync(sprintf "/api/v1/journal/entries/%s" createdId).Result

    Assert.Equal(404, int resp.StatusCode)
