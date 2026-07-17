# Learnings — rhino-cli Source-Drift Reconciliation

> Scaffold per the [Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md).
> Capture learnings as they surface during execution; triage each to a home (convention, doc,
> `plans/ideas.md`) or discard in Phase 5.

## Per-file canonical decisions (Phase 1)

_(populate during Phase 1 — one entry per drifted file: canonical form summary + classification as
union-surface gap or hardcoded per-repo value moved to `repo-config.yml`)_

- `docs/naming.rs`: _(pending)_
- `doctor/checker.rs`: _(pending)_
- `doctor/tools.rs`: _(pending)_
- `repo_governance/instruction_size.rs`: _(pending)_
- `tests/doctor.rs`: _(pending)_

## Candidate learnings (populate during execution)

- **Per-file canonical decisions** — for each drifted file, record whether it was a union-surface gap
  or a value moved to `repo-config.yml`, and why.
- **Standing tri-repo src-diff gate** — evaluate whether a periodic/CI tri-repo `diff` over the
  rhino-cli boundary should be added so this class of drift is caught automatically rather than by
  manual audit. Candidate `plans/ideas.md` entry if adopted.
- **`tests/doctor.rs` boundary question** — whether `tests/` should be pulled into the codified
  byte-identity boundary alongside `src/`.

## Triage log (Phase 5)

- **5 per-file canonical decisions** (`naming.rs`, `checker.rs`, `tools.rs`,
  `instruction_size.rs`, `tests/doctor.rs`) — **kept in place.** Litmus test passed (durable record
  of what/why for this exact reconciliation), but their home is this plan's own record, not a
  separate governance surface. No further routing — they stay in this `learnings.md`, which
  archives with the plan.
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
