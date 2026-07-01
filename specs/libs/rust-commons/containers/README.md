# Containers — rust-commons

C4 Level 2 containers for `rust-commons`.

`rust-commons` ships as a single container: a Rust library crate (`rust_commons`) compiled into
each consuming CLI binary (`ayokoding-cli`, `ose-cli`) at build time. It has no separate
deployable runtime of its own — it is a `[lib]` crate, not a binary.

See [container.md](./container.md) for the C4 container diagram placeholder.
