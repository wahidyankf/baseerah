module OrganicleverBe.Tests.Unit.Tests.JournalApiTests

open System.Net
open System.Net.Http
open System.Text
open Xunit
open OrganicleverBe.Contexts.Journal.Application
open OrganicleverBe.Contexts.Journal.Api
open OrganicleverBe.Tests.Unit.Steps.BddState

/// Builds an in-process journal API client over an in-memory repository so the
/// CRUD routing surface can be exercised without a PostgreSQL dependency.
let private journalClient () : HttpClient =
    let repo = inMemoryRepository ()
    buildClient (routes repo)

let private jsonContent (body: string) : StringContent =
    new StringContent(body, Encoding.UTF8, "application/json")

let private validEntryJson =
    "{\"name\":\"reading\",\"payload\":{\"title\":\"Clean Code\"},\"startedAt\":\"2026-06-14T10:00:00Z\",\"finishedAt\":\"2026-06-14T10:30:00Z\",\"labels\":[\"books\"]}"

[<Fact>]
let ``POST journal entries returns 201 with the created entry`` () =
    let client = journalClient ()

    let resp =
        client.PostAsync("/api/v1/journal/entries", jsonContent validEntryJson).Result

    Assert.Equal(HttpStatusCode.Created, resp.StatusCode)
    let body = resp.Content.ReadAsStringAsync().Result
    Assert.Contains("reading", body)
    Assert.Contains("\"id\"", body)

[<Fact>]
let ``POST journal entries with a blank name returns 400`` () =
    let client = journalClient ()

    let bad =
        "{\"name\":\"\",\"payload\":{},\"startedAt\":\"2026-06-14T10:00:00Z\",\"finishedAt\":\"2026-06-14T10:30:00Z\",\"labels\":[]}"

    let resp = client.PostAsync("/api/v1/journal/entries", jsonContent bad).Result
    Assert.Equal(HttpStatusCode.BadRequest, resp.StatusCode)

[<Fact>]
let ``GET journal entries lists created entries`` () =
    let client = journalClient ()

    client.PostAsync("/api/v1/journal/entries", jsonContent validEntryJson).Result
    |> ignore

    let resp = client.GetAsync("/api/v1/journal/entries").Result
    Assert.Equal(HttpStatusCode.OK, resp.StatusCode)
    let body = resp.Content.ReadAsStringAsync().Result
    Assert.Contains("reading", body)

[<Fact>]
let ``GET a missing journal entry returns 404`` () =
    let client = journalClient ()
    let resp = client.GetAsync("/api/v1/journal/entries/does-not-exist").Result
    Assert.Equal(HttpStatusCode.NotFound, resp.StatusCode)

[<Fact>]
let ``GET a created journal entry by id returns 200`` () =
    let client = journalClient ()

    let created =
        client.PostAsync("/api/v1/journal/entries", jsonContent validEntryJson).Result

    let createdBody = created.Content.ReadAsStringAsync().Result
    let id = extractId createdBody
    let resp = client.GetAsync(sprintf "/api/v1/journal/entries/%s" id).Result
    Assert.Equal(HttpStatusCode.OK, resp.StatusCode)

[<Fact>]
let ``PUT a created journal entry updates it and returns 200`` () =
    let client = journalClient ()

    let created =
        client.PostAsync("/api/v1/journal/entries", jsonContent validEntryJson).Result

    let id = extractId (created.Content.ReadAsStringAsync().Result)
    let update = "{\"name\":\"learning\"}"

    let resp =
        client.PutAsync(sprintf "/api/v1/journal/entries/%s" id, jsonContent update).Result

    Assert.Equal(HttpStatusCode.OK, resp.StatusCode)
    let body = resp.Content.ReadAsStringAsync().Result
    Assert.Contains("learning", body)

[<Fact>]
let ``PUT a missing journal entry returns 404`` () =
    let client = journalClient ()
    let update = "{\"name\":\"learning\"}"

    let resp =
        client.PutAsync("/api/v1/journal/entries/does-not-exist", jsonContent update).Result

    Assert.Equal(HttpStatusCode.NotFound, resp.StatusCode)

[<Fact>]
let ``DELETE a created journal entry returns 204 and removes it`` () =
    let client = journalClient ()

    let created =
        client.PostAsync("/api/v1/journal/entries", jsonContent validEntryJson).Result

    let id = extractId (created.Content.ReadAsStringAsync().Result)
    let resp = client.DeleteAsync(sprintf "/api/v1/journal/entries/%s" id).Result
    Assert.Equal(HttpStatusCode.NoContent, resp.StatusCode)
    let after = client.GetAsync(sprintf "/api/v1/journal/entries/%s" id).Result
    Assert.Equal(HttpStatusCode.NotFound, after.StatusCode)

[<Fact>]
let ``DELETE a missing journal entry returns 404`` () =
    let client = journalClient ()
    let resp = client.DeleteAsync("/api/v1/journal/entries/does-not-exist").Result
    Assert.Equal(HttpStatusCode.NotFound, resp.StatusCode)
