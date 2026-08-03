module BeaverNestBe.Tests.Unit.Tests.HttpConfigurationTests

open Xunit
open BeaverNestBe.Domain.HttpConfiguration

let private environment entries key =
    entries |> Map.tryFind key |> Option.toObj

[<Fact>]
let ``listener defaults to loopback on the production port`` () =
    let result = parse (environment Map.empty)
    Assert.Equal(Ok { Address = "127.0.0.1"; Port = 19300 }, result)

[<Fact>]
let ``listener accepts the Nx development loopback override`` () =
    let result =
        parse (environment (Map.ofList [ "BEAVER_NEST_BE_HTTP_LISTEN_PORT", "19320" ]))

    Assert.Equal(Ok { Address = "127.0.0.1"; Port = 19320 }, result)

[<Fact>]
let ``wildcard listener requires an explicit container runtime`` () =
    let hostResult =
        parse (environment (Map.ofList [ "BEAVER_NEST_BE_HTTP_LISTEN_ADDRESS", "0.0.0.0" ]))

    let containerResult =
        parse (
            environment (
                Map.ofList
                    [ "BEAVER_NEST_BE_HTTP_LISTEN_ADDRESS", "0.0.0.0"
                      "DOTNET_RUNNING_IN_CONTAINER", "true" ]
            )
        )

    Assert.True(Result.isError hostResult)
    Assert.Equal(Ok { Address = "0.0.0.0"; Port = 19300 }, containerResult)
