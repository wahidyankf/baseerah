# ose-app — System Context (C4 L1)

See [system-context/README.md](./README.md) for the C4 L1 diagram and actor/system inventory.

## Actors

| Actor              | Role                                                         |
| ------------------ | ------------------------------------------------------------ |
| Compliance Officer | Uploads regulatory and policy documents; reviews gap reports |
| Risk Team Member   | Reviews and triages GapItem records                          |

## External Systems

| System                    | Purpose                                      |
| ------------------------- | -------------------------------------------- |
| Regulator Document Store  | Source of regulator-published rule documents |
| LLM Provider (OpenRouter) | AI inference for gap analysis prompts        |
