module CraneBe.Core.Ports

type IMediaPort =
    abstract member Convert: bytes: byte[] -> Result<string, string>
