module CraneBe.Tests.Unit.Tests.ConfigTests

open Xunit
open CraneBe.Config

[<Fact>]
let ``Config fails when CRANE_BE_ORGANICLEVER_NATS_URL is missing`` () =
    let orig1 =
        System.Environment.GetEnvironmentVariable("CRANE_BE_ORGANICLEVER_NATS_URL")

    let orig2 = System.Environment.GetEnvironmentVariable("CRANE_BE_OSE_APP_NATS_URL")

    System.Environment.SetEnvironmentVariable("CRANE_BE_ORGANICLEVER_NATS_URL", null)
    System.Environment.SetEnvironmentVariable("CRANE_BE_OSE_APP_NATS_URL", "nats://x")

    try
        match load () with
        | Error msg -> Assert.Contains("CRANE_BE_ORGANICLEVER_NATS_URL", msg)
        | Ok _ -> Assert.Fail("Expected config error")
    finally
        System.Environment.SetEnvironmentVariable("CRANE_BE_ORGANICLEVER_NATS_URL", orig1)
        System.Environment.SetEnvironmentVariable("CRANE_BE_OSE_APP_NATS_URL", orig2)

[<Fact>]
let ``Config fails when CRANE_BE_OSE_APP_NATS_URL is missing`` () =
    let orig1 =
        System.Environment.GetEnvironmentVariable("CRANE_BE_ORGANICLEVER_NATS_URL")

    let orig2 = System.Environment.GetEnvironmentVariable("CRANE_BE_OSE_APP_NATS_URL")

    System.Environment.SetEnvironmentVariable("CRANE_BE_ORGANICLEVER_NATS_URL", "nats://x")
    System.Environment.SetEnvironmentVariable("CRANE_BE_OSE_APP_NATS_URL", null)

    try
        match load () with
        | Error msg -> Assert.Contains("CRANE_BE_OSE_APP_NATS_URL", msg)
        | Ok _ -> Assert.Fail("Expected config error")
    finally
        System.Environment.SetEnvironmentVariable("CRANE_BE_ORGANICLEVER_NATS_URL", orig1)
        System.Environment.SetEnvironmentVariable("CRANE_BE_OSE_APP_NATS_URL", orig2)
