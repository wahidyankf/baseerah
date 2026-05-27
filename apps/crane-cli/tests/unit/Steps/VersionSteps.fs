module CraneCli.Tests.Unit.Steps.VersionSteps

open TickSpec
open Xunit

let mutable private versionString: string = ""

[<When>]
let ``I read the assembly version`` () = ()

[<Then>]
let ``the version string matches a SemVer-shaped pattern`` () = ()
