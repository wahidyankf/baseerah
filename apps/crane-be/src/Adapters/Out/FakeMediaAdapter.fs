module CraneBe.Adapters.Out.FakeMediaAdapter

open CraneBe.Core.Ports

type FakeMediaAdapter() =
    interface IMediaPort with
        member _.Convert(_bytes) =
            Ok "# Fake Markdown\n\nThis is canned output from the fake adapter."
