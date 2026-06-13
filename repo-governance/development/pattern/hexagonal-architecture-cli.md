---
title: Hexagonal Architecture — CLI Apps
description: Hexagonal architecture specialization for CLI apps — commands as inbound adapters, layer responsibilities, and forbidden imports
category: explanation
subcategory: development
tags:
  - architecture
  - hexagonal
  - cli
  - rust
  - fsharp
created: 2026-05-26
---

# Hexagonal Architecture — CLI Apps

CLI apps apply hexagonal architecture with the `commands/` directory acting as the inbound adapter. CLI argument
parsing libraries (Clap, Cobra) belong exclusively in that adapter layer; the domain and application layers know
nothing about flags, subcommands, or exit codes.

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Command handlers
  receive parsed, typed arguments. The application layer is invoked with named domain concepts, not raw `&[String]`
  slices or `os.Args`.

- **[Pure Functions Over Side Effects](../../principles/software-engineering/pure-functions.md)**: Domain logic runs as
  pure functions. File I/O, HTTP requests, and standard-output writes are outbound adapter concerns confined to
  `infrastructure/`.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: Separating argument
  parsing from business logic keeps each layer testable in isolation. Domain tests need no CLI harness.

## Conventions Implemented/Respected

- **[Functional Programming Practices](./functional-programming.md)**: Domain functions are pure and stateless.

## Overview

CLI apps are driven by command-line arguments, which are the inbound signal — the equivalent of an HTTP request in a
web service. `commands/` parses those arguments using the CLI framework (Clap for Rust, Cobra for Go) and delegates to
the application layer. The application layer orchestrates domain logic and calls outbound ports. Infrastructure
implementations satisfy those ports.

## Directory Layout

The table below shows the canonical layout for all four CLI apps.

| Layer              | rhino-cli (Rust)      | crane-cli (F#)      | ose-cli (Rust)        | ayokoding-cli (Rust)  |
| ------------------ | --------------------- | ------------------- | --------------------- | --------------------- |
| Inbound adapter    | `src/commands/`       | `src/Adapters/In/`  | `src/commands/`       | `src/commands/`       |
| Application        | `src/application/`    | `src/Core/Logic/`   | `src/application/`    | `src/application/`    |
| Domain             | `src/domain/`         | `src/Core/Domain/`  | `src/domain/`         | `src/domain/`         |
| Outbound adapters  | `src/infrastructure/` | `src/Adapters/Out/` | `src/infrastructure/` | `src/infrastructure/` |
| I/O port contracts | —                     | `src/Core/Ports.fs` | —                     | —                     |
| Binary entry point | `src/main.rs`         | `src/Program.fs`    | `src/main.rs`         | `src/main.rs`         |

**`src/internal/` backward-compatibility shim**: `rhino-cli` retains a `src/internal/` directory containing
thin re-export modules (e.g., `pub use crate::application::agents::*;`). These exist solely for callers that
were written before the hexagonal migration (P7, 2026-05-23). No new code should import from `src/internal/`;
import from `src/domain/`, `src/application/`, or `src/infrastructure/` directly.

**crane-cli F# layout note**: crane-cli's F# implementation departs from the flat `src/commands/` layout because F#
compile order is explicit — all files must be declared in the `.fsproj` in dependency order. Grouped subdirectories
(`src/Core/Domain/`, `src/Core/Logic/`, `src/Adapters/`) make compile-order intent visible. An additional
`src/Core/Ports.fs` module declares all I/O boundaries as function type aliases (e.g.,
`type ReadPdf = string -> Result<PdfContent, PdfError>`), keeping the Impureim Sandwich pattern explicit: adapters
in `src/Adapters/Out/` satisfy these aliases; `src/Program.fs` is the composition root that wires everything
together.

## Layer Responsibilities

### commands/ — Inbound Adapter

- Parse CLI arguments using Clap (`#[derive(Parser)]`) or Cobra
- Validate argument types and required/optional constraints
- Map parsed arguments to application-layer input types
- Translate application errors to human-readable messages and non-zero exit codes
- Print progress or results to stdout/stderr

### domain/ — Domain Layer

- Business entities and value objects relevant to the CLI's domain
- Pure validation and transformation functions
- Domain error types (no exit codes, no `fmt.Println`)

### application/ — Application Layer

- Use-case functions that orchestrate domain objects and call outbound ports
- Outbound port definitions (repository traits in Rust, interfaces in Go)
- Application-level error types

### infrastructure/ — Outbound Adapters

- File system access (reading input files, writing output files)
- HTTP client calls to external services
- Concrete port implementations

## Forbidden Imports

| Layer             | Forbidden                                                                           |
| ----------------- | ----------------------------------------------------------------------------------- |
| `domain/`         | `clap`, `cobra`, any HTTP framework, any DB driver, `std::io` (Rust stdout)         |
| `application/`    | `clap`, `cobra`, concrete infrastructure types, HTTP framework types                |
| `infrastructure/` | `clap`, `cobra`, business logic — push invariants to `domain/`                      |
| `commands/`       | Direct database drivers, external HTTP SDKs (must use ports through `application/`) |

## Examples

A Rust CLI command delegating to the application layer:

```rust
// src/commands/validate.rs  (inbound adapter — Clap lives here)
use clap::Args;
use crate::application::validate_links::{ValidateLinksInput, validate_links};

#[derive(Args)]
pub struct ValidateArgs {
    /// Root directory to scan
    pub root: std::path::PathBuf,
    /// Fail on the first broken link
    #[arg(long)]
    pub fail_fast: bool,
}

pub fn run(args: ValidateArgs) -> anyhow::Result<()> {
    // Translate CLI args to application input type
    let input = ValidateLinksInput {
        root: args.root,
        fail_fast: args.fail_fast,
    };

    // Call application layer — no Clap types cross this boundary
    let result = validate_links(input)?;

    // Print results (side effect confined to commands/)
    for broken in &result.broken_links {
        eprintln!("BROKEN: {broken}");
    }

    if !result.broken_links.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}
```

```rust
// src/application/validate_links.rs  (application layer — no Clap import)
use crate::domain::link::Link;

pub struct ValidateLinksInput {
    pub root: std::path::PathBuf,
    pub fail_fast: bool,
}

pub struct ValidateLinksOutput {
    pub broken_links: Vec<Link>,
}

pub fn validate_links(input: ValidateLinksInput) -> Result<ValidateLinksOutput, crate::application::AppError> {
    // Orchestrates domain + outbound ports; no Clap types visible here
    todo!()
}
```

## Related

- **[Hexagonal Architecture](./hexagonal-architecture.md)** — Core pattern, dependency rule, and layer definitions
