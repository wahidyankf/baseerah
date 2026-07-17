# Learnings — rhino-cli Source-Drift Reconciliation

> Scaffold per the [Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md).
> Capture learnings as they surface during execution; triage each to a home (convention, doc,
> `plans/ideas.md`) or discard in Phase 5.

## Per-file canonical decisions (Phase 1)

Relocated to [`tech-docs.md` § Per-file canonical decisions (concrete results, Phase 1)](./tech-docs.md#per-file-canonical-decisions-concrete-results-phase-1)
during PR review — `pr-review-maker` correctly flagged decision-rationale content living in
`learnings.md` as a mismatch against the Knowledge Capture Convention ("`learnings.md` is not a
decision log... that is `tech-docs.md`'s job"). See the Triage log below for the corrected routing.

## Discovered during execution

- **`cargo test <module-path>` filter arg breaks against `harness = false` binaries (Phase 2).**
  The plan's own delivery.md commands (`cd apps/rhino-cli && cargo test application::docs::naming`,
  etc.) exit with status 2, NOT 0, even though the target module's tests all pass — `cargo test`
  passes the positional filter string to every compiled test binary, and this crate's
  `Cargo.toml` declares 18 `[[test]]` entries with `harness = false` (custom cucumber-runner
  `main()`, not libtest); the first one alphabetically (`agent_naming_validator`) has a clap-based
  arg parser that rejects an unrecognized positional argument and fails with exit 2, aborting the
  whole `cargo test` invocation before it reaches the later `harness = false` binaries — even
  though the actual module tests (run first) already passed. A plain `grep`-filtered read of the
  output looks clean and hides this — only checking the process exit code surfaces it. Fix: add
  `--lib` to scope to just the library test target for all 4 `src/`-scoped module filters
  (`cargo test --lib application::docs::naming`, etc.) — confirmed exit 0 with identical pass
  counts. `cargo test --test doctor` (Cycle 5) was already correctly scoped and unaffected.
  Surfaced when a background agent (ose-infra propagation) reported the true exit code rather than
  filtering through `grep`; the same silent issue was present in this orchestrator's own earlier
  GREEN-public verification runs and was corrected retroactively. Candidate fix: delivery.md's
  Phase 2 commands for Cycles 1–4 should read `cargo test --lib <module-path>`.

- **REFACTOR step's `../ose-primer` / `../ose-infra` relative paths assume the wrong CWD depth
  (Phase 2).** The plan's own delivery.md REFACTOR command (`for r in . ../ose-primer
../ose-infra; do (cd $r/apps/rhino-cli && ...); done`) only resolves correctly if run from the
  repo root (`ose-public/`), where the sibling repos really are one level up. But Step 0 of the
  plan-execution workflow puts all work inside the **worktree**
  (`ose-public/worktrees/<plan-id>/`), which is two directories deeper — from there, `../ose-primer`
  resolves to a nonexistent `ose-public/worktrees/ose-primer`. The correct relative path from
  inside the worktree is `../../../ose-primer` (3 levels: out of the plan folder, out of
  `worktrees/`, out of `ose-public/`, into the sibling). A second wrinkle: the sibling's
  propagated files live in **its own worktree** too
  (`ose-primer/worktrees/<plan-id>/apps/rhino-cli/`), not its primary checkout — so the full
  correct path is `../../../ose-primer/worktrees/rhino-cli-source-drift-reconciliation/apps/rhino-cli`.
  Verified fix works: `cargo fmt` + `cargo clippy --all-targets -- -D warnings` clean (0 diffs, 0
  warnings) in all three repos using the corrected paths. Candidate fix: any future plan whose
  delivery.md issues sibling-repo relative-path commands from inside a worktree should account for
  the extra `worktrees/<plan-id>/` nesting on both sides (self and sibling), not just assume a
  flat `../<sibling>` from the repo root.

- **Fresh worktrees don't get per-project polyglot dependency installs (Phase 3.3, ose-primer
  only).** `npx nx affected -t typecheck lint test:quick specs:behavior:coverage` in the freshly
  provisioned ose-primer worktree failed on exactly 5 of 26 affected projects —
  `elixir-gherkin`, `elixir-cabbage`, `elixir-openapi-codegen` (all: `mix compile` →
  `Unchecked dependencies... run "mix deps.get"`), `crud-be-elixir-phoenix` (codegen step invokes
  `mix run` inside `elixir-openapi-codegen`, same missing-deps cascade), and
  `crud-be-fsharp-giraffe` (`dotnet build` → `NETSDK1004: Assets file ... not found. Run a NuGet
package restore`). Root cause: Phase 0's `npm run doctor -- --fix` (Task #9, gated at #13)
  converges _toolchain_ installation (the `mix`/`dotnet` CLIs themselves) but does not run
  _per-project_ dependency restoration (`mix deps.get`, `dotnet restore`) for every polyglot demo
  app — that normally happens as a side effect of a prior full CI run or manual `npm install`-
  equivalent step, neither of which a brand-new `git worktree add` triggers. `rhino-cli` itself and
  the other 21 affected projects (Rust/TS/Go/Python/C#/Java/Kotlin/Clojure/Dart) were unaffected —
  each of those ecosystems' package managers restore automatically as part of their own
  typecheck/build/test invocation. Fix applied (worktree-local, no source/config touched, all
  outputs gitignored): `mix deps.get` in `libs/elixir-openapi-codegen`, `libs/elixir-cabbage`,
  `libs/elixir-gherkin`, `apps/crud-be-elixir-phoenix`; `dotnet restore` for both
  `apps/crud-be-fsharp-giraffe/src/DemoBeFsgi` and `.../tests/DemoBeFsgi.Tests`. Re-run: 26/26
  projects green. Candidate fix: either extend `npm run doctor -- --fix` /
  `repo-governance/development/workflow/worktree-setup.md` to also run `mix deps.get` +
  `dotnet restore` (or their language-general equivalents) across all polyglot projects after
  worktree provisioning, or accept this as a known one-time-per-worktree gap and document it in
  worktree-setup.md so future plans aren't surprised by it.

- **Uncommitted plan-tracking edits in the primary checkout silently diverge from the worktree
  branch (Phase 0-4).** This plan's `delivery.md`/`learnings.md` Phase 0-3 progress narrative
  (RED/GREEN/REFACTOR results, test counts, tri-repo `diff` verification) was written and left
  **uncommitted** in the `ose-public` primary checkout (`plans/in-progress/...`), not in this
  plan's dedicated worktree (`worktrees/rhino-cli-source-drift-reconciliation/`) where the actual
  PR branch lives. A later execution session, resuming per the plan-execution workflow's "disk is
  truth" Resume Reconciliation rule, read the plan folder via an absolute path that happened to
  resolve to the primary checkout rather than the worktree, saw Phase 0-3 fully ticked, and
  proceeded directly to Phase 4 — never noticing the worktree's own committed `delivery.md` still
  showed every Phase 0-3 checkbox unticked. The underlying Phase 0-3 _work itself_ was genuine and
  independently verifiable (sibling-repo commits `af0019bdc`/`3075cf08e`, open PRs #5/#8, and a
  live re-run of the tri-repo `diff` all confirmed accurate at PR-review time) — only the
  _tracking document_ had silently forked between two on-disk locations. `pr-review-maker`'s cycle
  1 review on the `ose-public` PR caught the resulting contradiction (delivery.md showing Phase 4/5
  done but Phase 0-3 undone) as a CRITICAL finding, which is what surfaced this. Root cause: a plan
  whose delivery mode is `worktree-to-pr` has exactly one authoritative on-disk location for its
  tracking documents — the worktree — and any edit made in the primary checkout's copy of an
  in-progress plan folder is, by construction, not on the branch that will ever become the PR;
  committing it there does nothing, since the primary checkout isn't what gets pushed. Candidate
  fix: `plan-execution.md`'s Resume Reconciliation step (and any tooling that reads a plan folder
  mid-execution) should explicitly resolve the plan path against the **current worktree**, never
  the primary checkout, for any plan whose delivery mode provisions a dedicated worktree — and
  flag it as a hard anomaly if the same plan folder exists with uncommitted changes in the primary
  checkout, since that state should never arise under correct execution.

## Candidate learnings (populate during execution)

- **Standing tri-repo src-diff gate** — evaluate whether a periodic/CI tri-repo `diff` over the
  rhino-cli boundary should be added so this class of drift is caught automatically rather than by
  manual audit. Candidate `plans/ideas.md` entry if adopted.
- **`tests/doctor.rs` boundary question** — whether `tests/` should be pulled into the codified
  byte-identity boundary alongside `src/`.

## Triage log (Phase 5)

- **5 per-file canonical decisions** (`naming.rs`, `checker.rs`, `tools.rs`,
  `instruction_size.rs`, `tests/doctor.rs`) — **routed (corrected).** Litmus test passed (durable
  record of what/why for this exact reconciliation). Initial triage kept this content in
  `learnings.md` — `pr-review-maker`'s cycle 1 review (HIGH finding) correctly flagged this as a
  mismatch against the Knowledge Capture Convention, which states `learnings.md` is "not a decision
  log... that is `tech-docs.md`'s job." Corrected routing: relocated to `tech-docs.md` §
  "Per-file canonical decisions (concrete results, Phase 1)".
- **`cargo test <filter>` breaks against `harness = false` binaries** — **routed.** Litmus test
  passed (generalizable cargo/rust gotcha, not rhino-cli-specific). Secret/sensitivity gate: clean.
  Repo-relevance gate: generic, not infra-private. Code-routing rule: home is `docs/`, not
  `apps/`/`libs/`/tests — no `plans/backlog/` filing needed. Routed to
  `docs/explanation/software-engineering/programming-languages/rust/testing-standards.md` §
  "Filtering Tests in Crates with Custom `harness = false` Binaries".
- **Sibling-repo relative-path nesting from inside a worktree** — **routed.** Litmus test passed
  (any future tri-repo `worktree-to-pr` plan writing sibling-repo relative-path commands hits this).
  Secret/sensitivity gate: clean. Repo-relevance gate: generic. Code-routing rule: home is
  `repo-governance/`, not `apps/`/`libs/`/tests. Routed to
  `repo-governance/development/workflow/worktree-setup.md` § "Known Gaps Beyond the Two-Step Init"
  → "Sibling-Repo Relative Paths From Inside a Worktree".
- **Fresh worktrees skip per-project Elixir/F# dependency restoration** — **routed.** Litmus test
  passed (reproduces on every freshly provisioned worktree touching those two ecosystems). Secret/
  sensitivity gate: clean. Repo-relevance gate: generic. Code-routing rule: home is
  `repo-governance/`, not `apps/`/`libs/`/tests. Routed to the same worktree-setup.md section →
  "Per-Project Dependency Restoration for Some Language Ecosystems".
- **Standing tri-repo rhino-cli src-diff gate (candidate)** — **filed as an idea.** Evaluated per
  the explicit P5-KC instruction; recommending it, not implementing it here (out of this plan's
  scope). Added to `plans/ideas.md` § "Rust Governance".
- **`tests/doctor.rs` byte-identity boundary question (candidate)** — **filed as an open question.**
  Not resolved by this plan (the plan followed the existing boundary and reconciled `tests/doctor.rs`
  as a matter of course, without redefining the boundary). Added to `plans/ideas.md` § "Rust
  Governance" as an open question for a future plan to decide.
- **Uncommitted plan-tracking edits in the primary checkout silently diverge from the worktree
  branch** — **routed.** Litmus test passed (any future `worktree-to-pr` plan resumed mid-execution
  can hit the same divergence). Secret/sensitivity gate: clean. Repo-relevance gate: generic,
  process-level. Code-routing rule: home is `repo-governance/`, not `apps/`/`libs/`/tests. Routed to
  `repo-governance/workflows/plan/plan-execution.md`'s Resume Reconciliation guidance — future
  execution sessions should resolve a `worktree-to-pr` plan's tracking-document path against the
  worktree, never the primary checkout.
