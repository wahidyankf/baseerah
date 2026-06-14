# Ubiquitous Language — be-journal

**Bounded context**: `be-journal`
**Maintainer**: organiclever-be team
**Last reviewed**: 2026-06-14

## Responsibility

Server-side journal persistence for `organiclever-be`. The F# / EF Core mirror of
the PGlite client journal context: it stores typed-payload journal entries in
PostgreSQL and exposes full CRUD over `/api/v1/journal/entries`. The endpoints
ship unconsumed — the web client remains PGlite-backed — but the contract is in
place for a future server-of-record migration.

## Term index

| Term                  | Code identifier(s)                   | Used in features     |
| --------------------- | ------------------------------------ | -------------------- |
| backend journal entry | `JournalEntry`, `JournalEntryEntity` | journal-crud.feature |
| new-entry input       | `NewEntryInput`                      | journal-crud.feature |
| update-entry input    | `UpdateEntryInput`                   | journal-crud.feature |
| entry-name validation | `validateName`, `validateNewEntry`   | journal-crud.feature |
| journal storage port  | `JournalRepository`, `efRepository`  | journal-crud.feature |
| create entry use case | `create`                             | journal-crud.feature |
| list entries use case | `list`                               | journal-crud.feature |
| find entry use case   | `findById`                           | journal-crud.feature |
| update entry use case | `update`                             | journal-crud.feature |
| delete entry use case | `delete`                             | journal-crud.feature |
| journal routes        | `routes`                             | journal-crud.feature |

## Out of scope

- Client-side optimistic journaling and the PGlite store (owned by the web
  `journal` context)
- Stats projections and workout-session orchestration (downstream web contexts)

## Forbidden synonyms

- "JournalEvent" — web `journal` context term; the backend persists a JournalEntry
- "Append" — the backend create use case is `create`, not append
- "Bump" — the backend update use case is `update`, not bump
