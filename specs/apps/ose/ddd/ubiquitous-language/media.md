# Ubiquitous Language — media

**Bounded context**: `media`
**Maintainer**: ose-app-be team
**Last reviewed**: 2026-06-11

## Responsibility

Converts uploaded PDF files into Markdown text via crane-be (Content Retrieval And
Normalization Engine) using NATS request/reply. The media context is stateless —
it delegates all conversion work to crane-be and returns the result to the caller.

## Term index

| Term             | Code identifier(s)            | Used in features      |
| ---------------- | ----------------------------- | --------------------- |
| media conversion | `convert`, `convert_via_nats` | crane-convert.feature |
| crane-be         | `crane_client`                | crane-convert.feature |
| convert endpoint | `POST /api/v1/media/convert`  | crane-convert.feature |

## Out of scope

- Persistence of conversion results — media context is stateless
- OCR post-processing — handled by crane-be internals
- Document format support beyond PDF — future work
