# Ubiquitous Language — be-media

**Bounded context**: `be-media`
**Maintainer**: organiclever-be team
**Last reviewed**: 2026-06-12

## Responsibility

Exposes the `POST /api/v1/media/convert` HTTP endpoint that accepts a PDF payload and
delegates PDF-to-Markdown conversion to crane-be via the NATS request/reply path
(`crane.convert`). Stateless — organiclever-be does not persist the conversion result.

## Term index

| Term                   | Code identifier(s) | Used in features                |
| ---------------------- | ------------------ | ------------------------------- |
| media-convert endpoint | `convert`          | messaging/crane-convert.feature |

## Out of scope

- NATS client lifecycle (belongs to the `messaging` bounded context)
- crane-be internals (owned by the crane bounded context)
- Conversion result persistence (intentionally out of scope for this context)
