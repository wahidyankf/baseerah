module BeaverNestBe.Tests.Unit.Tests.DatabaseOperationsTests

open Xunit
open BeaverNestBe.Operations.Database

[<Theory>]
[<InlineData("../escape.sqlite3")>]
[<InlineData("not-a-database")>]
[<InlineData("nested/backup.sqlite3")>]
let ``backup names cannot escape their fixed directory`` (name: string) =
    Assert.True(validateBackupName name |> Result.isError)
