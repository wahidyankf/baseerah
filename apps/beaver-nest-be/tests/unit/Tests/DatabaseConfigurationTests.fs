module BeaverNestBe.Tests.Unit.Tests.DatabaseConfigurationTests

open System
open System.IO
open Xunit
open BeaverNestBe.Domain.DatabaseConfiguration

[<Fact>]
let ``database configuration derives the fixed SQLite filename from a data directory`` () =
    let directory =
        Path.Combine(Path.GetTempPath(), "beaver-nest-test-" + Guid.NewGuid().ToString("N"))

    let configuration = create directory 100 |> Result.defaultWith failwith
    Assert.Equal(Path.Combine(Path.GetFullPath(directory), databaseFileName), databasePath configuration)

[<Theory>]
[<InlineData("/")>]
[<InlineData("")>]
let ``database configuration refuses unsafe directories`` (directory: string) =
    Assert.True(create directory 100 |> Result.isError)
