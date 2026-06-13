module OrganicleverBe.Infrastructure.AppDbContext

open Microsoft.EntityFrameworkCore

/// EF Core context for organiclever-be. The journal entity + CRUD repository
/// land in Phase 4; snake_case column mapping is configured at registration via
/// UseSnakeCaseNamingConvention.
type AppDbContext(options: DbContextOptions<AppDbContext>) =
    inherit DbContext(options)

    override this.OnModelCreating(modelBuilder: ModelBuilder) = base.OnModelCreating(modelBuilder)
