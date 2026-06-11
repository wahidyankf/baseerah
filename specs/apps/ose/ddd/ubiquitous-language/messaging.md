# Ubiquitous Language — messaging

**Bounded context**: `messaging`
**Maintainer**: ose-app-be team
**Last reviewed**: 2026-06-12
**Audience:** Engineers, Technical Product/Project Managers

## One-line summary

NATS-backed integration layer that connects ose-app-be to the NATS broker and to
crane-be, proving message delivery via JetStream and exposing PDF-to-Markdown
conversion through a crane NATS request/reply client.

## Term index

| Term                     | Code identifier(s)                        | Used in features                   |
| ------------------------ | ----------------------------------------- | ---------------------------------- |
| `NATS subject`           | `ose-app.messaging.demo` (stream subject) | `messaging/nats-connect.feature`   |
| `JetStream`              | `async_nats::jetstream`                   | `messaging/jetstream-demo.feature` |
| `durable consumer`       | `ose-app-messaging-demo` (consumer name)  | `messaging/jetstream-demo.feature` |
| `crane-convert`          | `crane.convert` (NATS subject)            | `messaging/crane-convert.feature`  |
| `media-convert endpoint` | `POST /api/v1/media/convert`              | `messaging/crane-convert.feature`  |
| `messaging status`       | `GET /api/v1/system/status/messaging`     | `messaging/jetstream-demo.feature` |

## Terms in detail

### Term: `NATS subject`

A string channel identifier on the NATS broker to which publishers send messages and
subscribers listen. In this bounded context the primary subject is
`ose-app.messaging.demo`, used for the JetStream stream demonstration. The
`crane.convert` subject is a separate request/reply channel owned by crane-be.

**Code identifier(s)**: `OSE_APP_MESSAGING_DEMO` (Rust constant in
`apps/ose-app-be/src/messaging/`).

**Used in features**: `messaging/nats-connect.feature`

---

### Term: `JetStream`

The NATS persistent message-streaming subsystem. Unlike core NATS (fire-and-forget),
JetStream retains messages in named streams and tracks consumer delivery via
acknowledgements. The ose-app-be messaging context uses JetStream to prove durable,
at-least-once delivery at startup.

**Code identifier(s)**: `async_nats::jetstream` (Rust crate module in
`apps/ose-app-be/src/messaging/`).

**Used in features**: `messaging/jetstream-demo.feature`

**Related**: `durable consumer`, `messaging status`

---

### Term: `durable consumer`

A named JetStream consumer that remembers its consumption position across service
restarts. A durable consumer is identified by a consumer name rather than an ephemeral
ID. The ose-app-be messaging context creates a durable consumer `ose-app-messaging-demo`
on the `OSE_APP_MESSAGING_DEMO` stream at startup.

**Code identifier(s)**: `ose-app-messaging-demo` (consumer name in
`apps/ose-app-be/src/messaging/`).

**Used in features**: `messaging/jetstream-demo.feature`

**Forbidden synonyms in this context**: "push consumer" (a distinct JetStream consumer
kind); "subscription" (core NATS term, not JetStream).

**Related**: `JetStream`

---

### Term: `crane-convert`

The NATS request/reply subject (`crane.convert`) to which crane-be subscribes under
queue group `crane.workers`. A caller sends a PDF payload as the request body and
receives the converted Markdown text in the reply. The ose-app-be messaging context
uses this subject indirectly via the `media-convert endpoint`.

**Code identifier(s)**: `CRANE_CONVERT_SUBJECT` (Rust constant in
`apps/ose-app-be/src/messaging/crane_client.rs`).

**Used in features**: `messaging/crane-convert.feature`

**Related**: `media-convert endpoint`

---

### Term: `media-convert endpoint`

The HTTP endpoint `POST /api/v1/media/convert` exposed by ose-app-be. Accepts a PDF
file upload, drives the crane NATS request/reply (`crane.convert`) path, and returns
the resulting Markdown to the caller. Stateless — ose-app-be does not persist the
conversion result.

**Code identifier(s)**: `apps/ose-app-be/src/contexts/media/api/http.rs`.

**Used in features**: `messaging/crane-convert.feature`

**Related**: `crane-convert`

---

### Term: `messaging status`

A startup probe result exposed at `GET /api/v1/system/status/messaging`. Reports the
outcome of the JetStream demo run (stream created, message published, consumer created,
message delivered and acked). Allows operators and e2e tests to verify the NATS
integration without starting a full business workflow.

**Code identifier(s)**: `GET /api/v1/system/status/messaging` route in
`apps/ose-app-be/src/messaging/status.rs`.

**Used in features**: `messaging/jetstream-demo.feature`

**Related**: `JetStream`, `durable consumer`

---

## Forbidden synonyms

- "event bus" — use "NATS broker" or "NATS subject"
- "queue" — use "JetStream stream" (JetStream) or "NATS subject" (core NATS)
- "topic" — use "NATS subject" (NATS does not use the "topic" vocabulary)
