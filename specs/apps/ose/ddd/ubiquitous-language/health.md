# Ubiquitous Language — health

**Bounded context**: `health`
**Maintainer**: ose-be team
**Last reviewed**: 2026-05-27

## Responsibility

Exposes a liveness endpoint (`GET /api/v1/health`) that allows orchestrators and monitoring
systems to determine whether the service is ready to handle traffic.

## Term index

| Term          | Code identifier(s)       | Used in features |
| ------------- | ------------------------ | ---------------- |
| health status | `HealthStatus`, `status` | health.feature   |
| get health    | `getHealth`              | health.feature   |

## Out of scope

- Component-level health (database connectivity, external service checks) — not exposed
  in this endpoint
- Distributed tracing or metrics export — belongs to infrastructure concerns
