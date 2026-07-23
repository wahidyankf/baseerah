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
