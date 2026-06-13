# ose-app-be — Component Spec (C4 L3)

See [components/README.md](../README.md) for the full C4 L3 overview.

## Container

F#/Giraffe REST API (`ose-app-be`) at `api.oseplatform.com`.

## Bounded-Context Modules

| Module              | Responsibility                                          |
| ------------------- | ------------------------------------------------------- |
| `regulatory-source` | Ingest and store regulator-published rule documents     |
| `internal-policy`   | Ingest and store company-internal policy documents      |
| `gap-analysis`      | Compare corpora and emit GapItem records                |
| `ai-orchestration`  | Wrap LLM calls (OpenRouter), prompt management, retries |
