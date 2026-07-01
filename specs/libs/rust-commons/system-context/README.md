# System Context — rust-commons

C4 Level 1 system context for `rust-commons`.

## Actors and consumers

- **`ayokoding-cli`** — `apps/ayokoding-cli/src/commands/links.rs` calls `rust_commons::links` to
  check AyoKoding content links.
- **`ose-cli`** — `apps/ose-cli/src/commands/links.rs` calls `rust_commons::links` to check ose-web
  content links.

`rust-commons` has no runtime dependency on any backend or network service; `check_links` reads
only the local filesystem.

See [context.md](./context.md) for the C4 context diagram placeholder.
