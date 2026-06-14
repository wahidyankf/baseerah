module OseBe.Infrastructure.Repositories.RepositoryTypes

open System
open System.Threading.Tasks
open OseBe.Infrastructure.AppDbContext

/// Repository for regulatory-source document operations.
type RegulatoryDocumentRepository =
    { Create: RegulatoryDocumentEntity -> Task<RegulatoryDocumentEntity>
      FindById: Guid -> Task<RegulatoryDocumentEntity option>
      List: unit -> Task<RegulatoryDocumentEntity list> }

/// Repository for internal-policy document operations.
type InternalPolicyDocumentRepository =
    { Create: InternalPolicyDocumentEntity -> Task<InternalPolicyDocumentEntity>
      FindById: Guid -> Task<InternalPolicyDocumentEntity option>
      List: unit -> Task<InternalPolicyDocumentEntity list> }
