namespace OrganicleverBe.IntegrationTests

open Xunit

// Disable cross-class test parallelisation for the whole integration assembly so
// the on-boot DbUp migration routine is never run concurrently against the
// shared PostgreSQL instance (DbUp's SchemaVersions bootstrap is not
// concurrency-safe).
[<assembly: CollectionBehavior(DisableTestParallelization = true)>]
do ()
