# Behavior — rust-commons

Gherkin behavioral specifications for
[rust-commons](../../../../libs/rust-commons/Cargo.toml), the shared Rust utility crate.

## Structure

```
specs/libs/rust-commons/behavior/
└── gherkin/
    └── links/
        └── check-links.feature
```

## Status

No Cucumber/Gherkin runner currently consumes these scenarios — `rust-commons` is exercised via
plain Rust unit tests (`cargo test --lib`) co-located with the `links` module (see the top-level
[README.md](../README.md#status)).
