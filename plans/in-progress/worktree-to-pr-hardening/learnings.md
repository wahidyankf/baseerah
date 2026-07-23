<!-- Knowledge Capture running log — append entries during execution. -->
<!-- Triage every entry (or record the explicit "none" escape) before archival. -->

# Learnings: worktree-to-pr-hardening

## Phase 8 — Dogfooding the new PR-review pipeline on its own PR

- **The redesign reviewed itself.** Because the monolith was retired at cutover (Phase 4), PR #88 was
  reviewed by the new specialist-plus-coordinator pipeline it introduces. Three sequential CI-gated
  cycles ran; the discipline split was exercised with the D12 risk-tier fan-out (a LOW-risk
  docs/governance diff weighted toward docs/governance/architecture/instruction/logic, with
  security/performance/integrity correctly finding nothing rather than manufacturing nits).
- **The final cycle earned its keep — no early exit.** Cycles 1–2 concluded 0 CRITICAL/0 HIGH; the
  mandated third cycle surfaced a genuine HIGH both prior passes missed: all eight specialist
  agent-defs had inherited the monolith's "post directly to GitHub" section verbatim, contradicting
  the coordinator-sole-poster contract. This is concrete evidence for the fixed-3-cycle /
  no-early-exit predictability policy (Phase 5): a "clean after 2" heuristic would have shipped the
  defect. Fix: specialists became read-only finding producers; the coordinator is the sole poster.
- **Over-inheritance, not a coverage hole.** The monolith→specialist mapping had no missing rule; the
  one defect was a responsibility (posting) that must NOT be inherited. Worth remembering when
  splitting any monolith: audit for over-inherited responsibilities, not just gaps.
- **Tester-gate exemption (precondition e).** This plan changes no user- or caller-reachable behavior
  (docs / agent-definitions / governance only). The surface-conditional tester gates are therefore
  exempt by the no-reachable-behavior clause; exemption recorded here and surfaced in the PR #88
  merge summary.

## Phases 9–10 — Propagating to the divergent sibling repos

- **The siblings were NOT clean identical-port targets.** The plan assumed "port the identical change
  set"; in reality 11 of 12 shared governance/agent files differed from ose-public's pre-change
  version, so the port had to be a **semantic per-file repoint** (retire `pr-review-maker` → the
  pipeline) onto each divergent base, not a `git apply` of the ose-public diff. New files copy
  verbatim; divergent files need the same _change_ re-derived from the oracle diff.
- **Link-target cascade.** The new convention links to `plans/ideas/pr-review-bot-identity.md` and
  `plans/backlog/merge-queue-adoption/README.md`; neither existed in the siblings, and the latter
  cross-links the ose-public-only `worktree-to-pr-hardening` plan. Porting a convention pulls its link
  targets, whose own links can dangle in the destination. Resolution: port the referenced artifacts and
  adapt their outbound cross-links to destination-resident governance targets.
- **Class-fix must include the binding files.** The Phase-8 HIGH fix (specialists never post) missed
  the `CLAUDE.md` Delivery-Mode paragraph, which still said "specialists … write only via the Reviews
  API". The primer Cycle-2 reviewer caught it as a byte-identical faithful-port artifact — i.e. the
  source (ose-public) still carried the defect. Fixed in all three repos. Reinforces the existing
  "fix the class, not the named sites" practice: a cross-file class-fix must sweep `AGENTS.md`/`CLAUDE.md`
  bindings too, not only `.claude/agents/*`.
- **Self-hosted runner Rust-toolchain-download flake.** ose-infra's cargo-based CI jobs (Quality gate,
  Harness duplication) failed on `could not download … channel-rust-stable.toml … operation timed out`
  during Rust-toolchain install — a transient network flake on a congested self-hosted runner, not a
  content defect (the validation logic never ran; local run was clean). Correct remediation per
  ci-blocker-resolution: confirm the root cause in the log, then `gh run rerun --failed` — never bypass.

## Knowledge Capture Triage

Litmus test applied to every entry above; secret/sensitivity gate applied (no secrets, tokens, or
private hostnames appear — the infra flake references only a public rust-lang.org URL); repo-relevance
gate applied (no infra-private content routed here). No entry routes to a code home (`apps/`/`libs/`),
so none requires a separate backlog plan.

- _Dogfooding / final-cycle-caught-HIGH / over-inheritance_ → **routed (already durable)**: the
  fixed-3-cycle no-early-exit rationale and the specialists-never-post contract are now normative in
  [`pr-review-disciplines.md`](../../../repo-governance/development/quality/pr-review-disciplines.md)
  (Phases 5 + Cycle-3 fix). Retained inline as the concrete evidence that motivated them.
- _Semantic-port + link-target cascade_ → **routed inline (small, non-code)**: a durable multi-repo
  propagation gotcha; kept here as the terminal home rather than expanding the parity workflow doc,
  since it is a one-off observation, not a recurring rule.
- _Class-fix must sweep binding files_ → **routed (already durable)**: reinforces the existing
  "fix the class, not the named sites" practice; no new surface needed.
- _Self-hosted runner toolchain-download flake_ → **routed (already durable)**: same class as the
  known CI rustup-concurrency-race infra flake; remediation already documented in
  [ci-blocker-resolution](../../../repo-governance/development/quality/ci-blocker-resolution.md). No
  new surface needed.
