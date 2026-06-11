# crane — Components: BE

The crane-be service exposes two inbound adapters sharing a single `CraneCore` library port:

- **HTTP adapter** (`POST /media/pdf-to-md`) — accepts PDF bytes from any HTTP client and
  returns the converted Markdown body with `Content-Type: text/markdown`
- **NATS adapter** — subscribes to the `crane.convert` subject (queue group `crane.workers`)
  on two independent NATS connections (one per tenant: organiclever, ose-app) and replies on
  the auto `_INBOX` subject with the converted Markdown payload

Both adapters delegate to `MediaService.convert`, which calls the `IMediaPort` out-port
backed by `RealMediaAdapter` in production (and `FakeMediaAdapter` in unit tests).

## Hexagonal boundary

```
[HTTP client]   [NATS organiclever]  [NATS ose-app]
      |                 |                   |
  HttpHandler      NatsSubscriber (organiclever conn)
  (Giraffe)        NatsSubscriber (ose-app conn)
      \                  |                  /
       \___________ MediaService ___________/
                         |
                    IMediaPort (out-port)
                         |
              RealMediaAdapter (production)
              FakeMediaAdapter (unit tests)
                         |
              CraneCore.convertPdfToMarkdown
              (libs/fsharp-crane-core)
```

## Health adapter

`GET /health` is a stateless Giraffe handler — no port dependency. Returns
`{"status":"healthy"}` with HTTP 200.
