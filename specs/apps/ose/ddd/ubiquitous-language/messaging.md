# Ubiquitous Language — messaging

**Bounded context**: `messaging`
**Maintainer**: ose-be team
**Last reviewed**: 2026-06-12
**Audience:** Engineers, Technical Product/Project Managers

## One-line summary

NATS-backed integration layer that connects ose-be to the NATS broker, proving
message delivery via JetStream durable consumers at startup.

## Term index

| Term               | Code identifier(s)                        | Used in features                   |
| ------------------ | ----------------------------------------- | ---------------------------------- |
| `NATS subject`     | `ose-app.messaging.demo` (stream subject) | `messaging/nats-connect.feature`   |
| `JetStream`        | `NatsJSContext`                           | `messaging/jetstream-demo.feature` |
| `durable consumer` | `ose-app-messaging-demo` (consumer name)  | `messaging/jetstream-demo.feature` |
| `messaging status` | `SharedMessagingStatus`                   | `messaging/jetstream-demo.feature` |

## Terms in detail

### Term: `NATS subject`

A string channel identifier on the NATS broker to which publishers send messages and
subscribers listen. In this bounded context the primary subject is
`ose-app.messaging.demo`, used for the JetStream stream demonstration.

**Code identifier(s)**: `OSE_APP_MESSAGING_DEMO` (StreamName value in
`apps/ose-be/src/OseBe/Contexts/Messaging/Infrastructure/JetStreamDemo.fs`).

**Used in features**: `messaging/nats-connect.feature`

---

### Term: `JetStream`

The NATS persistent message-streaming subsystem. Unlike core NATS (fire-and-forget),
JetStream retains messages in named streams and tracks consumer delivery via
acknowledgements. The ose-be messaging context uses JetStream to prove durable,
at-least-once delivery at startup.

**Code identifier(s)**: `NatsJSContext` (NATS.Net JetStream context in
`apps/ose-be/src/OseBe/Contexts/Messaging/Infrastructure/JetStreamDemo.fs`).

**Used in features**: `messaging/jetstream-demo.feature`

**Related**: `durable consumer`, `messaging status`

---

### Term: `durable consumer`

A named JetStream consumer that remembers its consumption position across service
restarts. A durable consumer is identified by a consumer name rather than an ephemeral
ID. The ose-be messaging context creates a durable consumer `ose-app-messaging-demo`
on the `OSE_APP_MESSAGING_DEMO` stream at startup.

**Code identifier(s)**: `ose-app-messaging-demo` (ConsumerName value in
`apps/ose-be/src/OseBe/Contexts/Messaging/Infrastructure/JetStreamDemo.fs`).

**Used in features**: `messaging/jetstream-demo.feature`

**Forbidden synonyms in this context**: "push consumer" (a distinct JetStream consumer
kind); "subscription" (core NATS term, not JetStream).

**Related**: `JetStream`

---

### Term: `messaging status`

A startup probe result exposed at `GET /api/v1/system/status/messaging`. Reports the
outcome of the JetStream demo run (stream created, message published, consumer created,
message delivered and acked). Allows operators and e2e tests to verify the NATS
integration without starting a full business workflow.

**Code identifier(s)**: `SharedMessagingStatus` backing the
`GET /api/v1/system/status/messaging` route in
`apps/ose-be/src/OseBe/Contexts/Messaging/`.

**Used in features**: `messaging/jetstream-demo.feature`

**Related**: `JetStream`, `durable consumer`

---

## Forbidden synonyms

- "event bus" — use "NATS broker" or "NATS subject"
- "queue" — use "JetStream stream" (JetStream) or "NATS subject" (core NATS)
- "topic" — use "NATS subject" (NATS does not use the "topic" vocabulary)
