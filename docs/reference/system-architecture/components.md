---
title: Components & Code Architecture
description: C4 Level 3 component diagrams and Level 4 code architecture
category: reference
tags:
  - architecture
  - c4-model
  - components
created: 2025-11-29
---

# Components & Code Architecture

C4 Level 3 component diagrams and Level 4 code architecture for the Baseerah platform.

> **2026 Baseerah repo reset**: the `ose-www`, `ayokoding-cli`, and `ayokoding-www` components
> previously documented here were deleted along with their apps. `rhino-cli` is the sole
> surviving app; `baseerah-fe` and `baseerah-be` are planned but not yet scaffolded, so no
> component diagram exists for them yet. See [applications.md](./applications.md).

## C4 Level 3: Component Diagrams

Shows the internal components within each container. Components are groupings of related functionality behind a well-defined interface.

### rhino-cli Components (Rust CLI Tool)

```mermaid
graph TB
    subgraph "CLI Interface"
        RHINO_ROOT[Root Command<br/>Repository automation]
        RHINO_FLAGS[Flags Parser<br/>Command-line arguments]
    end

    subgraph "Automation Modules"
        AUTO_MODULE[Automation Module<br/>Extensible automation]
    end

    subgraph "Infrastructure"
        RHINO_CONFIG[Config Loader<br/>Configuration]
        RHINO_LOGGER[Logger<br/>Logging]
    end

    RHINO_ROOT --> AUTO_MODULE
    RHINO_ROOT --> RHINO_FLAGS
    AUTO_MODULE --> RHINO_CONFIG
    AUTO_MODULE --> RHINO_LOGGER

    style RHINO_ROOT fill:#0077b6,stroke:#03045e,color:#ffffff
    style AUTO_MODULE fill:#2a9d8f,stroke:#264653,color:#ffffff
```

**Component Responsibilities:**

- **Root Command**: CLI entry point for repository automation tasks
- **Automation Module**: Extensible module system for automation workflows
- **Config Loader**: Load repository-hygiene configuration

## C4 Level 4: Code Architecture

Shows implementation details for critical components. No Level 4 breakdown is documented yet
beyond the `rhino-cli` Level 3 diagram above; this section will gain entries for `baseerah-fe`
and `baseerah-be` once those apps are scaffolded.
