# ERP Integration Patterns (By Example)

**Course ID**: `erp-integration-patterns` · **Format**: By Example · **Language**: JSON/HTTP (recorded fixtures, no live vendor calls — DD-14).

**Short summary**: Batch/file vs API integration, middleware/ESB patterns, idempotency and retry

**Scope note**: closes Stage A — how an ERP exchanges data with the rest of an enterprise's IT
landscape (course 3's boundary). Per the API-gate posture (DD-14), every worked example here runs on
recorded fixtures or a containerised open-source ERP under the author's own control — **never a live
call to a third-party vendor's SaaS**. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: an ERP rarely stands alone — it exchanges data with a CRM, a
  WMS, an e-commerce front end, or a data warehouse, and each integration pattern trades reliability
  and latency differently.
- **Keep-this-if-you-forget-everything**: idempotency is what makes a retried integration call safe —
  without it, a network timeout followed by a retry can duplicate a transaction.
- **Big ideas touched**: `integration-surface-as-subject-matter-not-shipped-surface` (DD-14); `retry
safety requires idempotency`, not just error handling.

## Prerequisites

- **Prior topics**: [`erp-extension-and-customization`](./erp-extension-and-customization.md).
- **Cross-domain prerequisites**: `event-driven-architecture`, `networking-essentials`,
  `backend-essentials`, `api-design` (all existing library).
- **Assumed knowledge**: the four existing-library courses' own request/response, event, and API-design
  vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Vendor-specific integration-surface details were re-grounded 2026-07-22 (see `tech-docs.md`'s
  verification table). Confirmed and safe to state:
  - **IDoc** is absent from SAP S/4HANA Cloud **Public** Edition (release 2508); on-prem and private
    cloud editions **retain** IDoc
    `[Web-cited: SAP Community — IDOCs are Still Safe for SAP S/4HANA (Clean Core Level B) — https://community.sap.com/t5/enterprise-resource-planning-blog-posts-by-members/idocs-are-still-safe-for-sap-s-4hana-sap-clean-core-extensibility-level-b/ba-p/14225439 ; accessed 2026-07-22]`.
    Any "IDoc is being retired everywhere" framing is blog commentary and stays `[Needs Verification]`
    — not restated as fact.
  - **Dataverse dual-write** is **active and being enhanced** (async dual-write); it is **not**
    deprecated. The Dataverse/Dynamics **Web API is OData v4.0**
    `[Web-cited: Microsoft Learn — Dual-write overview (updated 2026-04-03) — https://learn.microsoft.com/en-us/dynamics365/fin-ops-core/dev-itpro/data-entities/dual-write/dual-write-overview ; accessed 2026-07-22]`.

## Concepts

- **co-01 · batch-file-integration** — periodic file exchange (e.g. a nightly extract), simple but
  high-latency.
- **co-02 · api-based-integration** — synchronous or asynchronous API calls, lower latency, higher
  coupling.
- **co-03 · middleware-esb-pattern** — an intermediary layer translating and routing between systems
  rather than point-to-point integration.
- **co-04 · point-to-point-vs-hub** — direct system-to-system links versus a central integration hub,
  and how the trade-off changes as the number of connected systems grows.
- **co-05 · idempotency** — a retried call producing the same effect as a single call, preventing
  duplicate transactions.
- **co-06 · idempotency-key** — a mechanism (e.g. a client-generated request id) that lets a receiver
  detect and safely ignore a duplicate retried call.
- **co-07 · retry-semantics** — when and how a failed integration call is retried, and why retrying
  blindly without idempotency is unsafe.
- **co-08 · recorded-fixture-methodology** — this course's own worked examples run against recorded
  request/response fixtures or a self-hosted open-source ERP, never a live third-party vendor call
  (DD-14).

## Worked examples

Recorded-fixture-based worked scenarios (JSON request/response payloads; no live vendor calls, per
DD-14). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · batch-file-trace** — given a sample nightly extract file, trace how its records map into
  the ERP's master data. (co-01)
- **ex-02 · api-call-shape** — given a sample API request/response payload (recorded fixture), identify
  its integration pattern. (co-02, co-08)

### Intermediate

- **ex-03 · point-to-point-vs-hub-scaling** — given three systems needing pairwise integration,
  contrast the connection count under point-to-point versus a central hub. (co-03, co-04)
- **ex-04 · idempotency-key-design** — design an idempotency-key scheme for a payment-posting API call.
  (co-05, co-06)

### Advanced

- **ex-05 · retry-without-idempotency-failure** — given a retried call with no idempotency key, show
  how a duplicate transaction results. (co-07)
- **ex-06 · retry-with-idempotency-safety** — repeat ex-05 with an idempotency key present, and show
  the retry is safely ignored. (co-05, co-06, co-07)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a live system (`A6`, and this course's
> own no-live-vendor-call constraint, DD-14).

- **Goal**: design an integration pattern (batch, API, or middleware-routed) for a new
  system-to-system data exchange, with an idempotency scheme, using recorded fixtures only.
- **Concepts exercised**: [ ] pattern choice (co-01–co-04) [ ] idempotency (co-05, co-06) [ ] retry
  safety (co-07).
- **Ordered steps**: 1) choose and justify a pattern; 2) design the idempotency scheme; 3) write a
  recorded-fixture request/response pair demonstrating a safe retry.
- **Acceptance criteria**: the pattern choice is justified against the scenario's actual needs; the
  retry demonstration shows no duplicate effect.
- **Done bar**: a written design with recorded-fixture examples, no live network call, no system
  deployed.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 23 of 27.
- `skills/sharia-erp` — Stage A, course 23 of 30.

---

← Back to the [syllabus index](../README.md)
