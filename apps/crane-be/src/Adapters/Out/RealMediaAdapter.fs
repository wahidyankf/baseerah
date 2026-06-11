module CraneBe.Adapters.Out.RealMediaAdapter

open System.IO
open CraneBe.Core.Ports
open CraneCore.Adapters.Out.PdfAdapter
open CraneCore.Adapters.Out.OcrAdapter

type RealMediaAdapter() =
    let pdfAdapter = RealPdfAdapter()
    let ocrAdapter = RealOcrAdapter()

    interface IMediaPort with
        member _.Convert(bytes: byte[]) =
            let baseTmp = Path.GetTempFileName()
            let tmpPath = Path.ChangeExtension(baseTmp, ".pdf") |> string

            try
                File.WriteAllBytes(tmpPath, bytes)
                CraneCore.Convert.convertPdfToMarkdown pdfAdapter ocrAdapter tmpPath
            finally
                if File.Exists(tmpPath) then
                    File.Delete(tmpPath)

                if File.Exists(baseTmp) then
                    File.Delete(baseTmp)
