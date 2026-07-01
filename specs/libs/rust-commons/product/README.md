# Product — rust-commons

C4 Level 1 product framing for `rust-commons`. See
[Specs Directory Structure Convention](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical layout.

## Overview

`rust-commons` is the shared Rust utility crate for `ose-public`'s Rust CLI tools. Its first
module, `links`, walks a content directory and checks internal markdown links used by the
Next.js-based `*-www` sites, so link-checking logic is written once and reused by every CLI that
needs it.

See [overview.md](./overview.md) for the full product overview.
