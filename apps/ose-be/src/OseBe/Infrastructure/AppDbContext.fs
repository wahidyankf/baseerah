module OseBe.Infrastructure.AppDbContext

open System
open System.ComponentModel.DataAnnotations
open System.ComponentModel.DataAnnotations.Schema
open Microsoft.EntityFrameworkCore

/// EF entity for a regulator-published rule document ingested by the
/// regulatory-source bounded context, carrying provenance metadata.
[<CLIMutable>]
[<Table("regulatory_documents")>]
type RegulatoryDocumentEntity =
    { [<Key>]
      [<Column("id")>]
      Id: Guid
      [<Column("title")>]
      Title: string
      [<Column("issuer")>]
      Issuer: string
      [<Column("jurisdiction")>]
      Jurisdiction: string
      [<Column("document_type")>]
      DocumentType: string
      [<Column("created_at")>]
      CreatedAt: DateTime }

/// EF entity for a company-internal document ingested by the internal-policy
/// bounded context, carrying version and scope metadata.
[<CLIMutable>]
[<Table("internal_policy_documents")>]
type InternalPolicyDocumentEntity =
    { [<Key>]
      [<Column("id")>]
      Id: Guid
      [<Column("title")>]
      Title: string
      [<Column("version")>]
      Version: string
      [<Column("scope")>]
      Scope: string
      [<Column("created_at")>]
      CreatedAt: DateTime }

/// EF Core context for ose-be. snake_case column mapping is configured at
/// registration via UseSnakeCaseNamingConvention.
type AppDbContext(options: DbContextOptions<AppDbContext>) =
    inherit DbContext(options)

    [<DefaultValue(false)>]
    val mutable regulatoryDocuments: DbSet<RegulatoryDocumentEntity>

    member this.RegulatoryDocuments
        with get () = this.regulatoryDocuments
        and set v = this.regulatoryDocuments <- v

    [<DefaultValue(false)>]
    val mutable internalPolicyDocuments: DbSet<InternalPolicyDocumentEntity>

    member this.InternalPolicyDocuments
        with get () = this.internalPolicyDocuments
        and set v = this.internalPolicyDocuments <- v

    override this.OnModelCreating(modelBuilder: ModelBuilder) = base.OnModelCreating(modelBuilder)
