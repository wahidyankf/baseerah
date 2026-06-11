module CraneCore.Tests.Unit.Tests.SkiplistManagerTests

open System
open System.IO
open Xunit
open CraneCore.Logic.SkiplistManager

let private withTempPath (f: unit -> unit) =
    let tempPath =
        Path.Combine(Path.GetTempPath(), sprintf "crane-skiplist-test-%s.md" (Guid.NewGuid().ToString("N").[..11]))

    Environment.SetEnvironmentVariable("CRANE_SKIPLIST_PATH", tempPath)

    try
        f ()
    finally
        if File.Exists(tempPath) then
            File.Delete(tempPath)

        Environment.SetEnvironmentVariable("CRANE_SKIPLIST_PATH", null)

[<Fact>]
let ``stableKey produces 16-char hex string`` () =
    let key = stableKey "test.md" "category" "description"
    Assert.Equal(16, key.Length)
    Assert.Matches(@"^[0-9a-f]+$", key)

[<Fact>]
let ``stableKey is deterministic`` () =
    let key1 = stableKey "test.md" "category" "description"
    let key2 = stableKey "test.md" "category" "description"
    Assert.Equal(key1, key2)

[<Fact>]
let ``add creates new entry and returns true`` () =
    withTempPath (fun () ->
        match add "test.md" "text-completeness" "some description" with
        | Ok added -> Assert.True(added)
        | Error msg -> Assert.Fail(sprintf "add failed: %s" msg))

[<Fact>]
let ``add returns false for duplicate entry`` () =
    withTempPath (fun () ->
        add "test.md" "text-completeness" "some description" |> ignore

        match add "test.md" "text-completeness" "some description" with
        | Ok added -> Assert.False(added)
        | Error msg -> Assert.Fail(sprintf "add failed: %s" msg))

[<Fact>]
let ``check returns true for existing entry`` () =
    withTempPath (fun () ->
        add "test.md" "text-completeness" "some description" |> ignore

        match check "test.md" "text-completeness" "some description" with
        | Ok found -> Assert.True(found)
        | Error msg -> Assert.Fail(sprintf "check failed: %s" msg))

[<Fact>]
let ``check returns false for non-existing entry`` () =
    withTempPath (fun () ->
        match check "test.md" "text-completeness" "nonexistent" with
        | Ok found -> Assert.False(found)
        | Error msg -> Assert.Fail(sprintf "check failed: %s" msg))

[<Fact>]
let ``add creates file when it does not exist`` () =
    withTempPath (fun () ->
        add "test.md" "text-completeness" "new entry" |> ignore
        Assert.True(File.Exists(Environment.GetEnvironmentVariable("CRANE_SKIPLIST_PATH")))
        File.Delete(Environment.GetEnvironmentVariable("CRANE_SKIPLIST_PATH")))

[<Fact>]
let ``add appends to existing file`` () =
    withTempPath (fun () ->
        add "test.md" "text-completeness" "entry one" |> ignore
        add "test.md" "heading-depth" "entry two" |> ignore

        match list "test.md" with
        | Ok entries -> Assert.Equal(2, entries.Length)
        | Error msg -> Assert.Fail(sprintf "list failed: %s" msg)

        File.Delete(Environment.GetEnvironmentVariable("CRANE_SKIPLIST_PATH")))

[<Fact>]
let ``resolveSkiplistPath returns env var override when set`` () =
    let tempPath = Path.GetTempFileName()

    try
        Environment.SetEnvironmentVariable("CRANE_SKIPLIST_PATH", tempPath)
        let result = resolveSkiplistPath ()
        Assert.Equal(tempPath, result)
    finally
        Environment.SetEnvironmentVariable("CRANE_SKIPLIST_PATH", null)

        if File.Exists(tempPath) then
            File.Delete(tempPath)

[<Fact>]
let ``resolveSkiplistPath returns default path when env var is empty string`` () =
    let prev = Environment.GetEnvironmentVariable("CRANE_SKIPLIST_PATH")

    try
        Environment.SetEnvironmentVariable("CRANE_SKIPLIST_PATH", "")
        let result = resolveSkiplistPath ()
        Assert.NotEmpty(result)
    finally
        Environment.SetEnvironmentVariable("CRANE_SKIPLIST_PATH", prev)

[<Fact>]
let ``add appends to file when file already has content ending without double newline`` () =
    withTempPath (fun () ->
        add "test.md" "text-completeness" "first entry" |> ignore
        add "test.md" "heading-depth" "second entry" |> ignore

        match list "test.md" with
        | Ok entries -> Assert.Equal(2, entries.Length)
        | Error msg -> Assert.Fail(sprintf "list failed: %s" msg))

[<Fact>]
let ``add creates parent directory when path has subdirectory`` () =
    let tmpBase =
        Path.Combine(Path.GetTempPath(), sprintf "crane-dir-test-%s" (Guid.NewGuid().ToString("N").[..7]))

    let subDir = Path.Combine(tmpBase, "subdir")
    let skiplistPath = Path.Combine(subDir, "skiplist.md")

    try
        Environment.SetEnvironmentVariable("CRANE_SKIPLIST_PATH", skiplistPath)

        match add "test.md" "text-completeness" "dir creation test" with
        | Ok added ->
            Assert.True(added)
            Assert.True(File.Exists(skiplistPath))
        | Error msg -> Assert.Fail(sprintf "add failed: %s" msg)
    finally
        Environment.SetEnvironmentVariable("CRANE_SKIPLIST_PATH", null)

        if Directory.Exists(tmpBase) then
            Directory.Delete(tmpBase, true)
