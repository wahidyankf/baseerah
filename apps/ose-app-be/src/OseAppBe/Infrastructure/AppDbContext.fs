module OseAppBe.Infrastructure.AppDbContext

open Microsoft.EntityFrameworkCore

/// EF Core context for ose-app-be. Entities for the bounded contexts
/// (ai-orchestration, gap-analysis, internal-policy, regulatory-source) land in
/// Phase 3; snake_case column mapping is configured at registration via
/// UseSnakeCaseNamingConvention.
type AppDbContext(options: DbContextOptions<AppDbContext>) =
    inherit DbContext(options)

    override this.OnModelCreating(modelBuilder: ModelBuilder) = base.OnModelCreating(modelBuilder)
