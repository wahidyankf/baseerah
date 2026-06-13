# Pseudostate and stereotype counting

```mermaid
stateDiagram-v2
  direction LR
  [*] --> <<choice>>
  <<choice>> --> <<fork>>
  <<fork>> --> <<join>>
  <<join>> --> Extra
  Extra --> [*]
```
