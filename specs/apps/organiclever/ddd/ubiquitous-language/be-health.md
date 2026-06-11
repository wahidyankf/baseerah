# Ubiquitous Language — be-health

**Bounded context**: `be-health`
**Maintainer**: organiclever-be team
**Last reviewed**: 2026-06-12

## Responsibility

Exposes a liveness endpoint (`GET /api/v1/health`) that allows orchestrators and monitoring
systems to determine whether the service is ready to handle traffic.

## Term index

| Term          | Code identifier(s)       | Used in features     |
| ------------- | ------------------------ | -------------------- |
| health status | `HealthStatus`, `status` | health-check.feature |
| get health    | `get_health`             | health-check.feature |

## Out of scope

- Component-level health (database connectivity, external service checks) — not exposed
  in this endpoint
- Distributed tracing or metrics export — belongs to infrastructure concerns
