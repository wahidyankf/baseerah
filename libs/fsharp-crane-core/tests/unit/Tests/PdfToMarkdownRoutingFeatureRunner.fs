module CraneCore.Tests.Unit.Tests.PdfToMarkdownRoutingFeatureRunner

open System
open System.IO
open System.Reflection
open TickSpec
open Xunit
open CraneCore.Tests.Unit.Tests.PdfToMarkdownRoutingSteps

let private assembly = Assembly.GetExecutingAssembly()

let private specsDir =
    let assemblyDir = Path.GetDirectoryName(assembly.Location)
    Path.Combine(assemblyDir, "specs")

let private getFeatureFile (namePart: string) =
    if Directory.Exists(specsDir) then
        Directory.GetFiles(specsDir, "*.feature", SearchOption.AllDirectories)
        |> Array.tryFind (fun f -> f.Contains(namePart))
    else
        None

type private ConvertScenarioServiceProvider() =
    interface IServiceProvider with
        member _.GetService(serviceType: Type) =
            if serviceType = typeof<ConvertState> then
                emptyState :> obj
            else
                null

let private buildScenarioData (namePart: string) : seq<obj[]> =
    match getFeatureFile namePart with
    | Some path ->
        let defs = StepDefinitions(assembly)
        defs.ServiceProviderFactory <- fun () -> ConvertScenarioServiceProvider() :> IServiceProvider
        let lines = File.ReadAllLines(path)
        let feature = defs.GenerateFeature(path, lines)
        feature.Scenarios |> Seq.map (fun scenario -> [| scenario :> obj |])
    | None -> Seq.empty

type ConvertFeatureTests() =
    static member Scenarios() : seq<obj[]> =
        buildScenarioData "pdf-to-markdown-routing" |> Seq.toList :> seq<_>

    [<Theory>]
    [<MemberData("Scenarios")>]
    member _.``PDF to Markdown Routing``(scenario: Scenario) = scenario.Action.Invoke()
