# crud-be-rust-axum `specs:behavior:coverage` Vacuity Check (Phase 0 Audit — Deliverable 4)

## Command

Target confirmed verbatim in `apps/crud-be-rust-axum/project.json`:

```bash
npx nx run crud-be-rust-axum:specs:behavior:coverage
# underlying command:
# cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- specs behavior-coverage \
#   validate --shared-steps --exclude-dir test-support --exclude-dir codegen \
#   specs/apps/crud/behavior/crud-be/gherkin apps/crud-be-rust-axum
```

Run twice: once from Nx cache, once with `--skip-nx-cache` to force a genuine fresh execution.

## Result

| Run                       | Exit code | Output                                                                    |
| ------------------------- | --------- | ------------------------------------------------------------------------- |
| Cached                    | 0         | `✓ Spec coverage valid! 13 specs, 89 scenarios, 333 steps — all covered.` |
| Fresh (`--skip-nx-cache`) | 0         | `Spec coverage valid! 13 specs, 89 scenarios, 333 steps — all covered.`   |

Both runs pass. **This is a genuine pass, not a total no-op** — but it is passing for a different
reason than the plan assumes, and it is _not_ exercising the `@covers`-marker mechanism at all.

## Root-cause trace: the pass is real, but bypasses `@covers` entirely

`git grep -c "@covers" -- apps/crud-be-rust-axum` returns **zero** — `crud-be-rust-axum` has no
`@covers` markers anywhere in its source. Yet the command reports "all covered." Tracing why:

1. `apps/rhino-cli/src/cli.rs:821-824` wires the `specs behavior-coverage validate` CLI subcommand to
   `specs_coverage::run(args, output_format)` in `apps/rhino-cli/src/commands/specs_coverage.rs` — **not**
   to `application::behavior_coverage::validator::validate()`.
2. `specs_coverage.rs` (`//! Port of apps/rhino-cli/cmd/spec_coverage_validate.go`) delegates to
   `application::speccoverage::checker::check_all()` — a legacy, Go-ported **step-text pattern-matching
   scanner**. In `--shared-steps` mode (used by every `crud-be-*`/`crud-fe-*` Nx target) it extracts every
   step-definition regex found anywhere under the app's source tree, then checks that every Gherkin
   step's literal text matches _some_ extracted regex — with no notion of which scenario or feature file
   a step definition "belongs" to, and no `@covers` marker lookup at all (`apps/rhino-cli/src/application/speccoverage/checker.rs:156-192`).
3. Separately, `apps/rhino-cli/src/application/behavior_coverage/` implements the **actual per-scenario
   `@covers`-marker + per-scenario-level-tag engine** the plan wants to generalize
   (`validator.rs:35-41`: an untagged, non-`@wip` scenario is a hard `UntaggedScenario` violation — it
   would flag all 89 scenarios here, since none carry `@unit`/`@integration`/`@e2e` tags). This module
   has 6 unit tests, all green, all self-referencing via `// @covers` comments pointing at rhino-cli's
   own spec file — but **nothing in `cli.rs`'s command dispatch calls it**.
   `git grep -rn "behavior_coverage" apps/rhino-cli/src/commands apps/rhino-cli/src/cli.rs` finds only
   two CLI-arg-parsing test function names; the only real caller is
   `application::domain_coverage::mod.rs`, whose own `specs domain-coverage validate` subcommand
   (`specs_coverage.rs:290-321`, `run_domain()`) _also_ just falls through to the same legacy `run()` /
   `checker::check_all()` path after an eligibility-allowlist check — it never reaches
   `behavior_coverage::validator::validate()` either.

**Net effect**: the newer `@covers` + per-scenario-level-tag validator is dead code from a runtime
enforcement standpoint — built and unit-tested, but unreachable from any Nx target any of the 25
registered projects actually run. The live gate every project's `specs:behavior:coverage` target calls
is the older step-text scanner, which cannot see `@covers` markers or per-scenario level tags and does
not require either.

## Is the legacy checker itself vacuous?

No — it is a real (if weaker) check. `apps/rhino-cli/src/commands/specs_coverage.rs:354-364` has an
existing regression test (`run_returns_err_with_gaps_when_specs_missing_test_files`) proving the legacy
checker genuinely fails when pointed at a directory with no matching step implementations. And
`crud-be-rust-axum` does execute its Gherkin scenarios for real: it depends on the official `cucumber`
crate (`cucumber = { version = "0.21.1", features = ["libtest"] }` in `Cargo.toml`) with hand-written
`#[given]`/`#[when]`/`#[then]` step regexes under `apps/crud-be-rust-axum/tests/unit/steps/*.rs` that
directly execute the `.feature` files — so "some matching step regex exists" is a meaningful (if
loose) signal for a direct-Gherkin-execution framework like this one. It is a materially weaker
guarantee than "this exact scenario is explicitly and traceably wired to this exact test," and it
provides zero signal about per-scenario test-level assignment (unit vs. integration vs. e2e) — that
information currently comes entirely from the registry's `levels:` field (see deliverable 1), with no
cross-check that a scenario tagged for two levels actually has two independent test implementations.

## Conclusion for the plan

`crud-be-rust-axum:specs:behavior:coverage` passing "all covered" is genuine-but-weak, not vacuous. The
real vacuity is architectural: rhino-cli already built the stronger `@covers`-marker validator this plan
wants to generalize repo-wide, but it isn't wired into any live command path yet — so **zero of the 25
registered ose-primer projects are actually gated by it today**, including rhino-cli's own specs. Wiring
`application::behavior_coverage::validator::validate()` into the live `specs behavior-coverage validate`
command (or a new subcommand) is a prerequisite the plan needs to account for, not an assumption it can
build on top of.
