# Technical Documentation: Bootstrap BE Messaging and Crane Media Service

## Architecture Overview

```mermaid
%% Color-blind-friendly palette: blue #0173B2, orange #DE8F05, green #029E73, purple #CC78BC, grey #808080
flowchart TB
  subgraph SHARED["Shared code"]
    CORE["libs/fsharp-crane-core<br/>Domain + Ports + Logic<br/>PDF -> Markdown"]
  end

  CLI["apps/crane-cli<br/>F# CLI consumer"]

  subgraph CRANE["apps/crane-be (single shared service)"]
    IN_HTTP["Adapter In: Giraffe HTTP<br/>/health, POST /media/pdf-to-md"]
    IN_NATS["Adapter In: NATS subscriber<br/>crane.convert (queue group)"]
    APP["Application service<br/>orchestrates ports"]
    OUT_REAL["Adapter Out:<br/>PdfPig + TesseractOCR"]
    OUT_FAKE["Adapter Out: fake (canned md)"]
  end

  subgraph OLC["organiclever"]
    OLBE["organiclever-be (Rust/Axum)<br/>messaging context"]
    OLN["NATS JetStream (organiclever)"]
  end

  subgraph OAC["ose-app"]
    OABE["ose-app-be (Rust/Axum)<br/>messaging context"]
    OAN["NATS JetStream (ose-app)"]
  end

  CORE --> CLI
  CORE --> APP
  IN_HTTP --> APP
  IN_NATS --> APP
  APP --> OUT_REAL
  APP --> OUT_FAKE

  OLBE -->|HTTP sync| IN_HTTP
  OABE -->|HTTP sync| IN_HTTP
  OLBE -->|request crane.convert| OLN
  OABE -->|request crane.convert| OAN
  IN_NATS -->|conn 1| OLN
  IN_NATS -->|conn 2| OAN
  OLBE -->|JetStream demo publish/consume/ack| OLN
  OABE -->|JetStream demo publish/consume/ack| OAN

  linkStyle default stroke:#808080,stroke-width:1px

  style SHARED fill:#FFFFFF,stroke:#000000,color:#000000
  style CORE fill:#0173B2,stroke:#000000,color:#FFFFFF
  style CLI fill:#DE8F05,stroke:#000000,color:#000000
  style CRANE fill:#FFFFFF,stroke:#000000,color:#000000
  style IN_HTTP fill:#CC78BC,stroke:#000000,color:#000000
  style IN_NATS fill:#CC78BC,stroke:#000000,color:#000000
  style APP fill:#CC78BC,stroke:#000000,color:#000000
  style OUT_REAL fill:#CC78BC,stroke:#000000,color:#000000
  style OUT_FAKE fill:#CC78BC,stroke:#000000,color:#000000
  style OLC fill:#FFFFFF,stroke:#000000,color:#000000
  style OAC fill:#FFFFFF,stroke:#000000,color:#000000
  style OLBE fill:#029E73,stroke:#000000,color:#000000
  style OABE fill:#029E73,stroke:#000000,color:#000000
  style OLN fill:#808080,stroke:#000000,color:#FFFFFF
  style OAN fill:#808080,stroke:#000000,color:#FFFFFF
```

## Shared F# Library: libs/fsharp-crane-core

`[Repo-grounded]` `apps/crane-cli/src/Core/` currently holds `Domain/` (`Finding.fs`,
`PdfMetadata.fs`, `Report.fs`), `Ports.fs`, and `Logic/` (eleven checker/manager modules). The
PDF→Markdown extraction path lives in `Adapters/Out/PdfAdapter.fs` and `Adapters/Out/OcrAdapter.fs`
using `PdfPig` and `TesseractOCR`.

Design decision: extract only the **PDF→Markdown Core** (the Domain types and Ports needed for
conversion, plus the conversion Logic) into `libs/fsharp-crane-core/`. The library is a class
library (`Microsoft.NET.Sdk`, no `OutputType=Exe`), `TargetFramework net10.0`, matching the
crane-cli TFM `[Repo-grounded: crane-cli.fsproj]`. The library exposes a `convertPdfToMarkdown`
port plus the domain model; both `crane-cli` and `crane-be` reference it via `ProjectReference`.

Rationale: Nx forbids app→app imports `[Repo-grounded: AGENTS.md "Apps never import other apps"]`,
so the only sanctioned reuse path is a shared lib. This introduces the repo's first F# library.

### Library project.json

`libs/fsharp-crane-core/project.json` mirrors `libs/rust-commons/project.json`'s target shape:
`build`, `typecheck`, `lint`, `fmt`, `fmt:check`, `test:unit`, `test:quick`, and a `spec-coverage`
that echoes not-applicable for a library (as `rust-commons` does
`[Repo-grounded: rust-commons/project.json line 67-70]`). Coverage threshold for F# unit tests is
`Threshold=95` line, matching crane-cli `[Repo-grounded: crane-cli/project.json test:quick uses
/p:Threshold=95]`.

## crane-be Hexagonal Layout

```
apps/crane-be/
  crane-be.fsproj            # OutputType=Exe, net10.0, ProjectReference to fsharp-crane-core
  project.json               # Nx targets (see below)
  fsharplint.json            # mirrors crane-cli lint config
  Dockerfile                 # production image (Phase 6)
  docker-compose.integration.yml  # NATS + crane-be (Phase 6)
  .env.example               # CRANE_BE_* vars (Phase 2)
  src/
    Core/                    # thin app-level wiring over fsharp-crane-core ports
      Ports.fs               # in/out port signatures used by the service
    Application/
      MediaService.fs        # orchestrates the convert port
    Adapters/
      In/
        HttpHandlers.fs      # Giraffe HttpHandlers: /health, POST /media/pdf-to-md
        NatsSubscriber.fs    # subscribes crane.convert on each connection
      Out/
        FakeMediaAdapter.fs  # canned markdown (TDD first)
        RealMediaAdapter.fs  # delegates to fsharp-crane-core PdfPig/Tesseract
    Config.fs                # env read + fail-fast validation (dotenvy-equivalent intent)
    Program.fs               # composition root: build host, wire adapters, open 2 NATS conns
  tests/
    unit/                    # xUnit/F#, Threshold=95
    integration/             # NATS + crane-be via docker-compose; non-cacheable
```

- **Ports (driving / In)**: HTTP handler and NATS subscriber both call the same
  `MediaService.convert` application function.
- **Ports (driven / Out)**: `IMediaConverter`-style port with `FakeMediaAdapter` (canned) and
  `RealMediaAdapter` (delegates to `fsharp-crane-core`). The fake adapter is wired first for TDD;
  the real adapter is wired in Phase 3.
- **Composition root** (`Program.fs`): reads config, opens two NATS connections (one per backend
  server URL), subscribes `crane.convert` with queue group `crane.workers` on each, and starts the
  Giraffe HTTP host.

### Giraffe / ASP.NET Core

`Giraffe 8.2.0` on ASP.NET Core (.NET 10). `HttpHandler` composition wraps the application port,
keeping the web framework in the In-adapter layer only. The HTTP route `POST /media/pdf-to-md`
accepts PDF bytes (request body) and returns `text/markdown`.

## Messaging Bounded Context (both Rust backends)

Each backend gains a `messaging` module (Rust) implementing:

- **NATS client**: connect at startup using `<APP>_NATS_URL`, fail-fast if missing/unreachable
  (mirrors the existing dotenvy+envy fail-fast pattern `[Repo-grounded: Cargo.toml has dotenvy
0.15.7 + envy 0.4.2]`).
- **crane HTTP client**: `reqwest`-style call to `POST <CRANE_URL>/media/pdf-to-md`.
- **crane NATS client**: core request/reply to `crane.convert` (reply via auto `_INBOX`).
- **JetStream demo**: a durable stream + durable consumer on one demo subject per backend; publish
  → consume → ack, exercising the provisioned JetStream.

Rust NATS crate: `async-nats = "0.47.0"` (see Dependency Clearance). The existing backends already
use `tokio` full and `axum` `[Repo-grounded: Cargo.toml]`, so `async-nats` integrates into the
existing tokio runtime.

### Subject and queue-group naming

- Media RPC subject: `crane.convert` (core NATS request/reply; lowest latency; reply via auto
  `_INBOX`). Core request/reply suits tightly-coupled service RPC
  `[Web-cited: NATS docs — Request-Reply — https://docs.nats.io/nats-concepts/core-nats/reqreply — accessed 2026-06-11]`.
- Queue group (crane-be subscribers): `crane.workers` (same name on both connections).
- JetStream demo subjects: `<domain>.<service>.<action>` style, e.g.
  `organiclever.messaging.demo` and `ose-app.messaging.demo` — dot-separated, alphanumeric plus
  `-`/`_`, no `$` prefix.
- Durable consumer names: `organiclever-messaging-demo`, `ose-app-messaging-demo`.

## Dependency Clearance

Per the
[Dependency Bump Stability & Safety Policy](../../../../../repo-governance/development/workflow/dependency-bump-policy.md),
all bumps follow **Path B** (60-day soak + CVE-clean), no waivers. Exact pins only; no
caret/tilde. CVE sources checked for each: NVD, GitHub Advisories, Snyk DB, vendor/RustSec
security pages, and the CISA KEV feed. Cutoff = execution date minus 60 days; the chosen version's
release date must precede the cutoff.

| Package        | Version                        | Ecosystem    | Release date                                        | Path | 60-day cutoff basis                                | Notes                                                                                                                                                           |
| -------------- | ------------------------------ | ------------ | --------------------------------------------------- | ---- | -------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `async-nats`   | `0.47.0`                       | Rust (Cargo) | 2026-03-31 `[Unverified — confirm at Phase 0]`      | B    | release date ≥ 60d before exec date                | RUSTSEC-2023-0027 patched since 0.29.0; MSRV 1.88 matches both backends `[Repo-grounded: Cargo.toml rust-version 1.88]`. Do NOT use 0.48.x/0.49.x (inside soak) |
| `NATS.Net`     | latest Path-B-eligible `2.7.x` | .NET (NuGet) | `_Unknown — confirm exact 2.7.x + date at Phase 0_` | B    | must be ≥ 60d old + CVE-clean at exec              | Do NOT pin 2.8.0/2.8.1 (inside soak). Recorded as a Phase 0 verification step                                                                                   |
| `Giraffe`      | `8.2.0`                        | .NET (NuGet) | 2025-11-12 `[Unverified — confirm at Phase 0]`      | B    | well past 60d                                      | ASP.NET Core; HttpHandler composes with hexagonal ports                                                                                                         |
| `PdfPig`       | `0.1.14`                       | .NET (NuGet) | reused                                              | B    | already pinned `[Repo-grounded: crane-cli.fsproj]` | consumed via shared lib                                                                                                                                         |
| `TesseractOCR` | `5.5.2`                        | .NET (NuGet) | reused                                              | B    | already pinned `[Repo-grounded: crane-cli.fsproj]` | consumed via shared lib                                                                                                                                         |
| Runtime        | `.NET 10` (`net10.0`) + F# 10  | .NET         | matches crane-cli                                   | n/a  | n/a                                                | `[Repo-grounded: crane-cli.fsproj TargetFramework net10.0]`                                                                                                     |

> **Phase 0 hard requirement**: confirm the exact `NATS.Net` 2.7.x version and its release date are
> ≥ 60 days old and CVE-clean before any .NET NATS code is written. Record the confirmed version
> and date back into this table. Re-confirm `async-nats 0.47.0` and `Giraffe 8.2.0` release dates
> against the computed cutoff. Dates above are marked `[Unverified — confirm at Phase 0]` rather
> than asserted at authoring time.

All waivers: none. If any unpatched CVE for a chosen version appears in the CISA KEV catalog, the
60-day soak is bypassed and the bump escalates to Path C — in that case stop and re-grill before
proceeding (this plan assumes clean Path B).

## Environment Variables

All vars follow the per-app `SCREAMING_SNAKE` + app-prefix rule and must be (a) annotated in the
app's `.env.example` with the required/optional + type + format format, and (b) registered in
`env-contract.yaml` for that surface, or `rhino-cli env validate` fails
`[Repo-grounded: env-contract.yaml]`. `crane-be` is not a Next.js app, so it gets no framework
`PORT`/`HOSTNAME` exemption — `CRANE_BE_PORT` is an explicit app var.

| App / surface   | Var                              | Req/Opt  | Type   | Purpose                                      |
| --------------- | -------------------------------- | -------- | ------ | -------------------------------------------- |
| organiclever-be | `ORGANICLEVER_BE_NATS_URL`       | REQUIRED | string | NATS server URL (JetStream enabled)          |
| organiclever-be | `ORGANICLEVER_BE_CRANE_URL`      | REQUIRED | string | crane-be HTTP base URL                       |
| ose-app-be      | `OSE_APP_BE_NATS_URL`            | REQUIRED | string | NATS server URL (JetStream enabled)          |
| ose-app-be      | `OSE_APP_BE_CRANE_URL`           | REQUIRED | string | crane-be HTTP base URL                       |
| crane-be        | `CRANE_BE_PORT`                  | OPTIONAL | u16    | HTTP listen port (default chosen in Phase 2) |
| crane-be        | `CRANE_BE_ORGANICLEVER_NATS_URL` | REQUIRED | string | organiclever NATS URL (connection 1)         |
| crane-be        | `CRANE_BE_OSE_APP_NATS_URL`      | REQUIRED | string | ose-app NATS URL (connection 2)              |

`env-contract.yaml` gets a new surface entry for `apps/crane-be` (`kind: app`). The `lang` field
currently only documents `rust`/`typescript` `[Repo-grounded: env-contract.yaml header]`; Phase 8
extends the validator/contract to accept an F# app surface (or registers crane-be with an
allowlist-only entry if `lang` must stay rust/typescript — resolve against the validator
implementation in Phase 8, treat as `[Unverified]` until then). crane-be reads env with a
dotenvy-equivalent for .NET and validates fail-fast at startup, mirroring the Rust intent.

> **Agent guardrail** `[Repo-grounded: secrets-and-env-standards guard-env-file-access]`: agents
> must never read, write, edit, or commit real `.env*` files — only `.env.example`. Any real-env
> relocation is a `[HUMAN]` step in delivery.md.

## Docker, Image, and Migration Design

- **Integration compose** `[Repo-grounded: docker-compose.integration.yml currently postgres-only]`:
  add a `nats` service (`-js` for JetStream) to each backend's
  `docker-compose.integration.yml`. crane-be gets its own `docker-compose.integration.yml` with a
  `nats` service plus the crane-be service so the media path is reachable in integration tests.
  Integration tests remain non-cacheable per nx-targets.
- **Production Dockerfiles**: new `apps/<backend>/Dockerfile` (distinct from the existing
  `Dockerfile.integration` `[Repo-grounded: both backends have Dockerfile.integration]`) for
  `organiclever-be`, `ose-app-be`, and `crane-be`. Multi-stage; crane-be image bundles
  `tessdata/eng.traineddata` like crane-cli does `[Repo-grounded: crane-cli.fsproj Content
Include tessdata]`.
- **Run-on-boot migrations**: both Rust backends call `sqlx::migrate!` at startup before serving
  (the backends already depend on `sqlx 0.8` `[Repo-grounded: Cargo.toml]`).
- **GHCR publish workflow**: a new `.github/workflows/*.yml` that is **affected-aware** — builds
  and publishes only the images whose sources changed — producing public images
  `ghcr.io/wahidyankf/{organiclever-be,ose-app-be,crane-be}:latest`.

## Nx Project Configuration

- **`apps/crane-be/project.json`** mirrors `apps/crane-cli/project.json` targets
  `[Repo-grounded: crane-cli has build, typecheck, lint, fmt, fmt:check, run, dev, test:unit,
test:quick, test:integration, spec-coverage]` plus a long-running `dev`/`run` for the service and
  a production image build target. Tags: `domain:crane`, `type:app`.
- **`libs/fsharp-crane-core/project.json`** as described above; tags `domain:crane`, `type:lib`.
- Apps never import apps — that is the reason the lib exists.

## fsharp- Lib-Naming Convention Addition

`[Repo-grounded: docs/reference/monorepo-structure.md lines 179-180 list only ts- and rust-]` The
lib-naming token list documents `ts-` and `rust-`. Phase 8 registers `fsharp-` alongside them in
`docs/reference/monorepo-structure.md` (and the AGENTS.md / monorepo lib-naming note if it
restates the list), so `libs/fsharp-crane-core/` is convention-compliant.

## Deviations / Parity vs ose-infra

- **Parity**: per-backend NATS (one each), one shared crane media service, internal-only reach,
  walking-skeleton ambition — all match the infra plan's staging.
- **Deviation (intentional)**: where infra stages NATS as "infra-ready, not consumed yet", this
  plan **consumes** NATS (media RPC + JetStream demo) to prove provisioning. This is a deliberate
  one-step-further posture, agreed in the handoff.
- **Topology note**: NATS does not federate independent servers, so the single crane-be opens two
  independent connections and subscribes with the same queue group on each — there is no
  cross-server federation, matching the infra topology.
- **Boundary**: production deployment, manifests, and ClusterIP wiring remain owned by `ose-infra`;
  this plan stops at building deployable images + publish pipeline.

## Rollback

Each phase is an independent, revertible commit set. Reverting the GHCR workflow, Dockerfiles, or
the messaging modules restores the prior backends. The library extraction is the only
cross-cutting change; reverting it restores crane-cli's in-app Core. No production state is
mutated by this plan (deployment is owned downstream).
