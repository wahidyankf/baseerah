module OrganicleverBe.Infrastructure.AppDbContext

open System
open System.ComponentModel.DataAnnotations
open System.ComponentModel.DataAnnotations.Schema
open Microsoft.EntityFrameworkCore

/// EF entity for a journal entry. Mirrors the PGlite client schema
/// (apps/organiclever-app-web/src/contexts/journal/...) column-for-column: a string
/// id, a kind-slug name, a JSONB payload (stored verbatim as a JSON string), the
/// activity window, audit timestamps, and a text-array of labels.
[<CLIMutable>]
[<Table("journal_entries")>]
type JournalEntryEntity =
    { [<Key>]
      [<Column("id")>]
      Id: string
      [<Column("name")>]
      Name: string
      [<Column("payload", TypeName = "jsonb")>]
      Payload: string
      [<Column("started_at")>]
      StartedAt: DateTime
      [<Column("finished_at")>]
      FinishedAt: DateTime
      [<Column("labels")>]
      Labels: string array
      [<Column("created_at")>]
      CreatedAt: DateTime
      [<Column("updated_at")>]
      UpdatedAt: DateTime }

/// EF Core context for organiclever-be. snake_case column mapping is configured
/// at registration via UseSnakeCaseNamingConvention.
type AppDbContext(options: DbContextOptions<AppDbContext>) =
    inherit DbContext(options)

    [<DefaultValue(false)>]
    val mutable journalEntries: DbSet<JournalEntryEntity>

    member this.JournalEntries
        with get () = this.journalEntries
        and set v = this.journalEntries <- v

    override this.OnModelCreating(modelBuilder: ModelBuilder) = base.OnModelCreating(modelBuilder)
