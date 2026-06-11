module CraneBe.Config

type Config =
    { Port: uint16
      OrganicLeverNatsUrl: string
      OseAppNatsUrl: string }

let load () : Result<Config, string> =
    let port =
        match System.Environment.GetEnvironmentVariable("CRANE_BE_PORT") with
        | null -> 8300us
        | v ->
            match System.UInt16.TryParse(v) with
            | true, n -> n
            | _ -> 8300us

    let orgNats =
        System.Environment.GetEnvironmentVariable("CRANE_BE_ORGANICLEVER_NATS_URL")

    let oseNats = System.Environment.GetEnvironmentVariable("CRANE_BE_OSE_APP_NATS_URL")

    match orgNats, oseNats with
    | null, _ -> Error "CRANE_BE_ORGANICLEVER_NATS_URL is required but not set"
    | _, null -> Error "CRANE_BE_OSE_APP_NATS_URL is required but not set"
    | orgUrl, oseUrl ->
        Ok
            { Port = port
              OrganicLeverNatsUrl = orgUrl
              OseAppNatsUrl = oseUrl }
