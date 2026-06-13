# rhino-cli Specs

Gherkin behavioral specifications for
[rhino-cli](../../../apps/rhino-cli/README.md) — the Repository Hygiene &
INtegration Orchestrator CLI.

## Purpose

These specs define the **observable behavior** of every rhino-cli command:
what inputs the command accepts, what it writes to stdout, and what exit code
it returns. They are the single source of truth for correctness and serve as
the contract between the CLI implementation and its consumers.

## Structure

Feature files live under `behavior/rhino-cli/gherkin/`, organized into domain subdirs:

```
specs/apps/rhino/
├── README.md
├── product/          # C4 L1 product framing
├── system-context/   # C4 L1 actors and external systems
├── containers/       # C4 L2 deployable units
├── components/
│   └── cli/          # C4 L3 CLI internals
└── behavior/
    └── cli/
        └── gherkin/
            ├── agents/           # agents subcommand family (4 features)
            ├── ddd/              # ddd subcommand family (2 features)
            ├── docs/             # docs subcommand family (5 features)
            ├── env/              # env subcommand family (3 features)
            ├── git/              # git subcommand family (1 feature)
            ├── repo-governance/  # repo-governance subcommand family (9 features)
            ├── spec-coverage/    # spec-coverage subcommand family (1 feature)
            ├── specs/            # specs subcommand family (4 features)
            ├── system/           # system commands — doctor (1 feature)
            ├── test-coverage/    # test-coverage subcommand family (3 features)
            └── workflows/        # workflows subcommand family (1 feature)
```

See [behavior/rhino-cli/gherkin/README.md](./behavior/rhino-cli/gherkin/README.md) for the full file inventory.

## Running the Tests

Unit tests live as `#[cfg(test)]` modules inside `src/`; binary integration tests live in
`tests/cli/`. A cucumber-rs harness (`tests/cucumber/`) is scaffolded but deferred — the
existing 754-test suite plus shadow-diff already establishes full behavioral parity. See
the `project-rhino-cli-rust-cucumber-gap` memory entry for context.

```bash
# Run all unit tests with coverage gate (≥90% line coverage)
nx run rhino-cli:test:quick

# Run unit tests directly (no coverage threshold)
cargo test --manifest-path apps/rhino-cli/Cargo.toml --lib

# Run all binary integration tests
nx run rhino-cli:test:integration

# Run a specific integration test during development
cargo test --manifest-path apps/rhino-cli/Cargo.toml --tests -- <test_name>
```

The `test:integration` target re-runs when `src/**/*.rs`, `tests/**/*.rs`, or
`specs/apps/rhino/**/*.feature` change. `test:quick` (unit + coverage) is also
cache-invalidated when spec files change.

## Adding New Specs

1. Create `specs/apps/rhino/behavior/rhino-cli/gherkin/<domain>/<domain>-<action>.feature`
2. Add unit coverage inside the relevant module in `apps/rhino-cli/src/`:
   - Add a `#[cfg(test)]` block to the module under test
   - Include a doc-comment citing the Gherkin scenario name on each `#[test]` function
   - Use in-source mocks and pure-function calls to cover all scenario branches
3. Add a binary integration test at `apps/rhino-cli/tests/cli/<domain>_<action>.rs`
   using `assert_cmd` and `predicates`:
   - Call the real binary via `Command::cargo_bin("rhino-cli")`
   - Assert stdout, stderr, and exit code from the feature file scenarios
   - Wire cucumber-rs step definitions into `tests/cucumber/` once that harness lands
4. Verify:

   ```bash
   nx run rhino-cli:test:quick
   nx run rhino-cli:test:integration
   ```

## Dual Consumption

Every feature file is consumed at two test levels. Step implementations differ; Gherkin
scenarios are identical:

| Level       | Test File Pattern                    | Step Implementation                         | Nx Target          |
| ----------- | ------------------------------------ | ------------------------------------------- | ------------------ |
| Unit        | `src/**/*.rs` (`#[cfg(test)]` block) | In-source mocks via pure functions          | `test:quick`       |
| Integration | `tests/cli/<domain>_<action>.rs`     | Real binary via `assert_cmd` + `predicates` | `test:integration` |
| Cucumber    | `tests/cucumber/` (harness deferred) | cucumber-rs step definitions (future)       | `specs:coverage`   |

Coverage is measured at the unit level only (≥90% line coverage via `cargo llvm-cov`).

## Convention

See
[BDD Spec-to-Test Mapping Convention](../../../repo-governance/development/infra/bdd-spec-test-mapping.md)
for the mandatory 1:1 mapping between commands and `@tags`, file naming patterns, and coverage
enforcement rules.
