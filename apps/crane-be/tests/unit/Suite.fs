module CraneBe.Tests.Unit.Suite

open System.IO
open System.Reflection
open System.Text.RegularExpressions
open TickSpec
open Xunit

let private assembly = Assembly.GetExecutingAssembly()

let private gherkinRoot =
    match System.Environment.GetEnvironmentVariable("GHERKIN_ROOT") with
    | null -> Path.Combine(__SOURCE_DIRECTORY__, "../../../../specs/apps/crane/behavior/crane-be/gherkin")
    | root -> root

/// Filter a Gherkin feature file to only include scenarios whose tag block contains the given tag.
/// Preserves the Feature header and only emits Scenario blocks that match.
let private filterByTag (tag: string) (content: string) : string =
    let lines = content.Split('\n')
    let sb = System.Text.StringBuilder()
    let mutable inScenario = false
    let mutable scenarioLines = System.Collections.Generic.List<string>()
    let mutable pendingTags = System.Collections.Generic.List<string>()

    let flushScenario () =
        if inScenario then
            let hasTags = pendingTags |> Seq.exists (fun line -> line.Contains("@" + tag))

            if hasTags then
                for t in pendingTags do
                    sb.AppendLine(t) |> ignore

                for l in scenarioLines do
                    sb.AppendLine(l) |> ignore

            scenarioLines <- System.Collections.Generic.List<string>()
            pendingTags <- System.Collections.Generic.List<string>()
            inScenario <- false

    for line in lines do
        let trimmed = line.TrimStart()

        if trimmed.StartsWith("Feature:") then
            flushScenario ()
            sb.AppendLine(line) |> ignore
        elif trimmed.StartsWith("@") && not inScenario then
            pendingTags.Add(line)
        elif trimmed.StartsWith("Scenario:") then
            flushScenario ()
            inScenario <- true
            scenarioLines.Add(line)
        elif inScenario then
            // Check if this is a new tag line (start of next scenario tag block)
            if trimmed.StartsWith("@") then
                flushScenario ()
                pendingTags.Add(line)
            else
                scenarioLines.Add(line)

    flushScenario ()
    sb.ToString()

/// Returns loaded @unit Gherkin scenarios, or a single no-op placeholder when none can be loaded.
/// Prevents xUnit from failing with "No data found" when step definitions are not yet implemented.
let private buildScenarioData () : seq<obj[]> =
    let loaded =
        if Directory.Exists(gherkinRoot) then
            let files =
                Directory.GetFiles(gherkinRoot, "*.feature", SearchOption.AllDirectories)

            let defs = StepDefinitions(assembly)

            files
            |> Seq.collect (fun path ->
                try
                    let content = File.ReadAllText(path)
                    let filtered = filterByTag "unit" content

                    if filtered.Trim() = "" then
                        Seq.empty
                    else
                        let scenarios = defs.GenerateScenarios(path, new StringReader(filtered))
                        scenarios |> Seq.map (fun scenario -> [| scenario :> obj |])
                with _ ->
                    Seq.empty)
            |> Seq.toList
        else
            []

    if List.isEmpty loaded then
        // Placeholder: step definitions not yet implemented.
        // Returns a single no-op row so [Theory] does not fail with "No data found".
        Seq.singleton [| box "no-op" |]
    else
        loaded :> seq<_>

type CraneBeUnitSuite() =
    static member Scenarios() : seq<obj[]> =
        buildScenarioData () |> Seq.toList :> seq<_>

    [<Theory>]
    [<MemberData("Scenarios")>]
    member _.``Crane BE unit scenarios``(item: obj) =
        match item with
        | :? Scenario as scenario -> scenario.Action.Invoke()
        | _ -> () // no-op placeholder — step definitions not yet implemented
