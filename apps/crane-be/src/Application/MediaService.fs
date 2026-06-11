module CraneBe.Application.MediaService

open CraneBe.Core.Ports

let convert (port: IMediaPort) (bytes: byte[]) : Result<string, string> = port.Convert(bytes)
