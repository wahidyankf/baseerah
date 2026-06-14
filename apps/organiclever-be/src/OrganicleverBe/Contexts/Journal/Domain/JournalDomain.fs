namespace OrganicleverBe.Contexts.Journal

open System.Text.RegularExpressions

/// Domain types for the journal bounded context.
///
/// The journal is the append-only system of record for everything the user did
/// (workout, reading, learning, meal, focus, or a custom kind). Each entry
/// mirrors the PGlite client schema
/// (apps/organiclever-web/src/contexts/journal/domain/schema.ts): a kind slug
/// (`Name`), a JSON `Payload`, an activity window (`StartedAt`/`FinishedAt`),
/// audit timestamps, and free-form `Labels`.
module Domain =

    /// Maximum length of an entry name (kind slug), per the client EntryName schema.
    [<Literal>]
    let MaxNameLength = 64

    /// A fully materialised journal entry as returned to clients.
    type JournalEntry =
        { Id: string
          Name: string
          Payload: string
          StartedAt: string
          FinishedAt: string
          Labels: string list
          CreatedAt: string
          UpdatedAt: string }

    /// Input for creating a new journal entry (POST body shape).
    type NewEntryInput =
        { Name: string
          Payload: string
          StartedAt: string
          FinishedAt: string
          Labels: string list }

    /// Input for updating an existing journal entry (PUT body shape). Both fields
    /// are optional — only the supplied fields are mutated.
    type UpdateEntryInput =
        { Name: string option
          Payload: string option }

    /// Validation regex for an entry name (kind slug): lowercase, starting with a
    /// letter, mirroring the client EntryName brand `^[a-z][a-z0-9-]*$`.
    let private nameRegex = Regex(@"^[a-z][a-z0-9-]*$", RegexOptions.Compiled)

    /// Validates an entry name against the client EntryName invariants
    /// (1..MaxNameLength chars, lowercase slug). Returns the trimmed name on success.
    let validateName (name: string) : Result<string, string> =
        if System.String.IsNullOrWhiteSpace name then
            Error "name must not be empty"
        elif name.Length > MaxNameLength then
            Error(sprintf "name must be at most %d characters" MaxNameLength)
        elif not (nameRegex.IsMatch name) then
            Error "name must match ^[a-z][a-z0-9-]*$"
        else
            Ok name

    /// Validates a new-entry input, returning the input unchanged when its name
    /// satisfies the domain invariants.
    let validateNewEntry (input: NewEntryInput) : Result<NewEntryInput, string> =
        validateName input.Name |> Result.map (fun _ -> input)
