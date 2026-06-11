module CraneBe.Adapters.In.HttpHandlers

open Giraffe
open CraneBe.Core.Ports
open CraneBe.Application.MediaService

let healthHandler: HttpHandler =
    fun _ ctx -> ctx.WriteJsonAsync {| status = "healthy" |}

/// PDF magic bytes: %PDF = 0x25 0x50 0x44 0x46
let private isPdf (bytes: byte[]) =
    bytes.Length >= 4
    && bytes.[0] = 0x25uy
    && bytes.[1] = 0x50uy
    && bytes.[2] = 0x44uy
    && bytes.[3] = 0x46uy

let pdfToMdHandler (port: IMediaPort) : HttpHandler =
    fun _ ctx ->
        task {
            use ms = new System.IO.MemoryStream()
            do! ctx.Request.Body.CopyToAsync(ms)
            let bytes = ms.ToArray()

            if bytes.Length = 0 then
                ctx.Response.StatusCode <- 400
                return! ctx.WriteStringAsync "PDF payload missing"
            elif not (isPdf bytes) then
                ctx.Response.StatusCode <- 422
                return! ctx.WriteStringAsync "Payload could not be parsed as a PDF"
            else
                match convert port bytes with
                | Ok markdown ->
                    ctx.Response.Headers["Content-Type"] <- "text/markdown"
                    return! ctx.WriteStringAsync markdown
                | Error e ->
                    ctx.Response.StatusCode <- 500
                    return! ctx.WriteStringAsync e
        }

let webApp (port: IMediaPort) : HttpHandler =
    choose
        [ GET >=> route "/health" >=> healthHandler
          POST >=> route "/media/pdf-to-md" >=> pdfToMdHandler port ]
