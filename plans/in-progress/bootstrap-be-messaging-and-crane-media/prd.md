# Product Requirements: Bootstrap BE Messaging and Crane Media Service

## Product Overview

A walking-skeleton (infra-ready) bundle of three product capabilities:

1. A shared F# media Core library reused by the existing CLI and a new service.
2. A deployable F# media service (`crane-be`) exposing PDF→Markdown over HTTP and NATS.
3. Real NATS messaging in both Rust backends — service RPC over core NATS plus a JetStream
   durable demo that exercises the provisioned streams.

The product is "deployable by the infra plan" and "TDD-clean" rather than feature-rich; it
delivers exactly one media op and one demo per backend.

## Personas

This solo-maintainer repo's personas are the hats worn and the agents/services that consume the
output.

- **Backend maintainer** — wires messaging and the F# service; wants TDD-clean, drift-guard-clean
  code.
- **Infra operator (ose-infra)** — pulls the public GHCR images and deploys them; wants images
  that build, migrate on boot, and connect to NATS.
- **Calling backend service** — `organiclever-be` / `ose-app-be` acting as clients of `crane-be`
  over HTTP and NATS.
- **Media service (`crane-be`)** — subscribes to each backend's NATS and serves HTTP requests.

## User Stories

- As the **backend maintainer**, I want the PDF→Markdown Core in a shared F# library so that both
  `crane-cli` and `crane-be` reuse it without app→app imports.
- As the **infra operator**, I want production Dockerfiles and a GHCR publish workflow so that the
  twin-cluster deployment can pull real images.
- As the **infra operator**, I want both Rust backends to run `sqlx::migrate!` on boot so that a
  fresh cluster database is schema-current without a manual step.
- As a **calling backend service**, I want to convert a PDF to markdown via both an HTTP call and
  a NATS request/reply so that I can choose sync or async integration.
- As the **backend maintainer**, I want each backend to publish to and consume from a JetStream
  durable stream so that the provisioned JetStream is proven working, not assumed.
- As the **backend maintainer**, I want every new env var registered in the drift guard so that
  pre-push and CI stay green.

## Acceptance Criteria (Gherkin)

> Every scenario uses exactly one primary `Given`, one `When`, one `Then`; extras chain with
> `And`/`But`. These scenarios seed the first failing tests for the matching delivery phases.

### Shared library extraction

```gherkin
Scenario: crane-cli consumes the extracted Core library and stays green
  Given the PDF-to-Markdown Core has been extracted to libs/fsharp-crane-core
  And apps/crane-cli references the library instead of its in-app Core
  When the crane-cli unit and integration test suites run
  Then all crane-cli tests pass
  And no app-to-app import exists between crane-cli and crane-be
```

### crane-be health

```gherkin
Scenario: crane-be reports healthy over HTTP
  Given the crane-be service is running on its configured port
  When a client sends GET to /health
  Then the response status is 200
  And the response body indicates the service is healthy
```

### Media PDF to Markdown over HTTP

```gherkin
Scenario: crane-be converts a PDF to markdown over HTTP using the fake adapter
  Given crane-be is configured with the fake media adapter
  When a client sends POST /media/pdf-to-md with sample PDF bytes
  Then the response status is 200
  And the response body contains the canned markdown output
```

```gherkin
Scenario: crane-be converts a real PDF to markdown over HTTP using the real adapter
  Given crane-be is configured with the real PdfPig/Tesseract adapter
  When a client sends POST /media/pdf-to-md with a real sample PDF
  Then the response status is 200
  And the response body contains markdown extracted from the PDF
```

### Media PDF to Markdown over NATS request/reply

```gherkin
Scenario: crane-be answers a NATS core request/reply on crane.convert
  Given crane-be has subscribed to subject crane.convert on a backend NATS server
  When a backend publishes a request to crane.convert with sample PDF bytes
  Then crane-be replies on the auto _INBOX subject with markdown
  And the requesting backend receives the markdown reply
```

### NATS connect and health per backend

```gherkin
Scenario: organiclever-be connects to its NATS server at startup
  Given ORGANICLEVER_BE_NATS_URL points to a running NATS server with JetStream enabled
  When organiclever-be starts up
  Then the NATS connection is established
  And startup fails fast if the NATS URL is missing or unreachable
```

```gherkin
Scenario: ose-app-be connects to its NATS server at startup
  Given OSE_APP_BE_NATS_URL points to a running NATS server with JetStream enabled
  When ose-app-be starts up
  Then the NATS connection is established
  And startup fails fast if the NATS URL is missing or unreachable
```

### JetStream durable publish + consume + ack demo per backend

```gherkin
Scenario: organiclever-be publishes and durably consumes its demo subject with ack
  Given organiclever-be has a JetStream durable stream and consumer for its demo subject
  When organiclever-be publishes a demo message to that subject
  Then the durable consumer receives the message
  And the message is acknowledged
  And the stream reports the message as delivered and acked
```

```gherkin
Scenario: ose-app-be publishes and durably consumes its demo subject with ack
  Given ose-app-be has a JetStream durable stream and consumer for its demo subject
  When ose-app-be publishes a demo message to that subject
  Then the durable consumer receives the message
  And the message is acknowledged
  And the stream reports the message as delivered and acked
```

### Shared single crane-be against two NATS servers

```gherkin
Scenario: the single crane-be serves both backends over independent NATS connections
  Given crane-be has opened one NATS connection to each backend's NATS server
  And each subscription uses the same queue group crane.workers
  When each backend independently issues a crane.convert request
  Then each backend receives a markdown reply from crane-be
  And neither backend's request is delivered to the other backend's NATS server
```

### Env drift guard with new vars

```gherkin
Scenario: env drift guard passes with the new messaging and crane vars
  Given the new env vars are annotated in each app .env.example
  And the new env vars are registered in env-contract.yaml for each surface
  When rhino-cli env validate runs
  Then it reports no drift
  And the pre-push and CI env-validate checks pass
```

### Production images build

```gherkin
Scenario: production Dockerfiles build for all three services
  Given production Dockerfiles exist for organiclever-be, ose-app-be, and crane-be
  When each production image is built locally
  Then each image builds successfully
  And each image is distinct from the existing Dockerfile.integration
```

### Run-on-boot migrations

```gherkin
Scenario: a Rust backend runs sqlx migrations on boot
  Given a backend production image starts against an empty database
  When the backend process boots
  Then sqlx::migrate! applies all pending migrations before serving requests
  And the backend reports healthy after migrations complete
```

### GHCR publish

```gherkin
Scenario: the affected-aware GHCR workflow publishes only changed images
  Given the GHCR publish workflow is triggered by a push to main
  When only crane-be source changed in that push
  Then the workflow builds and publishes the crane-be image
  And it does not republish unchanged backend images
```

## Product Scope

### In Scope (Product Features)

- Shared F# media Core library and crane-cli migration to it.
- `crane-be` service: `/health`, `POST /media/pdf-to-md`, NATS `crane.convert` request/reply, fake
  - real adapters.
- Both backends: NATS client, HTTP + NATS clients to `crane-be`, JetStream durable demo.
- Production Dockerfiles (x3), run-on-boot migrations (x2), affected-aware GHCR publish workflow.
- Spec sets and Gherkin features for the new messaging context and `crane-be`.
- New env vars and drift-guard registration.

### Out of Scope (Product Features)

- Additional media operations beyond PDF→Markdown.
- Authentication/authorization on `crane-be` (internal ClusterIP only).
- Cross-cluster or federated NATS.
- Frontend or web-app changes.
- Production deployment manifests (owned by `ose-infra`).

## Product Risks

- **Adapter parity**: the real adapter must produce markdown comparable to `crane-cli`'s existing
  PDF path; mitigated by reusing the same PdfPig/Tesseract logic via the shared library.
- **NATS test flakiness**: JetStream integration tests depend on a real NATS container; mitigated
  by keeping integration tests non-cacheable and gated behind a healthcheck.
- **Queue-group semantics**: same queue group on two servers must not cross-deliver; mitigated by
  the two-independent-connections design and an explicit isolation scenario above.
