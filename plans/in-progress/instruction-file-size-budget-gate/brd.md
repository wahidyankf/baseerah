# Business Requirements — Instruction-File Size-Budget Gate

## Problem Statement

Auto-loaded AI instruction files are loaded **verbatim into every agent session, before the
first user message, on every harness**. When they grow unbounded they:

1. **Get silently truncated** — OpenAI Codex CLI stops reading `AGENTS.md` at 32,768 bytes
   with no warning, so trailing rules vanish for Codex users. `AGENTS.md` is **41,108 bytes
   today**, meaning the bottom ~8k bytes (roughly everything from the "Web Sites" section
   down) is already invisible to Codex.
2. **Trip the Claude Code 40k warning** — the resolved `CLAUDE.md` + `@AGENTS.md` tree is
   **47,730 bytes**, over the 40,000-character runtime warning. This is the warning that
   triggered this plan.
3. **Burn context budget** — every byte consumes session tokens that could serve the actual
   task. Vendor guidance (Claude Code "target under 200 lines"; Windsurf 12,000-char hard
   cap; Junie "20–40 lines") and a 2026 instruction-adherence study converge on the same
   conclusion: keep instruction files lean.

A single-file gate exists (`rhino-cli convention agents-md-size`) but only watches
`AGENTS.md`, sets its hard limit too high (40k, above Codex truncation), and is not enforced
at pre-push. The repo has **no budget at all** for the other harnesses' instruction surfaces.

## Business Goals

| Goal                                                            | Measure of success                                                                                                  |
| --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| No instruction file silently truncated by any supported harness | Every monitored surface's hard ceiling sits below the tightest harness limit that consumes it                       |
| The Claude Code 40k warning never fires again                   | Resolved `CLAUDE.md` tree stays under a 38k ceiling (≈5% headroom)                                                  |
| Drift is caught before it ships, not after                      | The gate blocks `git push` when an over-budget instruction file is in the pushed range                              |
| The rule is discoverable and governed, not tribal knowledge     | Rule lives as a `repo-governance/` convention, checked by `repo-rules-checker`, listed in the quality-gate workflow |
| `AGENTS.md` is back within budget                               | `AGENTS.md` ≤ 30,000 bytes (target 24,000) after trim                                                               |

## Stakeholders

- **AI coding agents (all harnesses)** — primary consumers; benefit from complete,
  non-truncated, lean instructions.
- **Repository maintainer** — wants drift blocked automatically and the rule self-documenting.
- **Human contributors** — get a fast, local, actionable failure instead of a silent
  degradation discovered later.
- **Downstream `ose-primer` / `ose-infra`** — inherit the convention + gate via the parity
  loop (hand-off only in this plan).

## Business Value

- **Correctness**: eliminates a silent-truncation class of bug where agents act on
  half-loaded instructions.
- **Cost**: trims per-session token overhead across every harness, every session.
- **Governance**: converts an ad-hoc, single-file check into a first-class, propagated
  convention — consistent with the repo's six-layer governance model.
- **Reversibility**: thresholds live in one committed config file; tuning is a one-line edit,
  not a code change.

## Success Criteria (Business-Level)

1. The gate covers the full AGENTS.md-class of auto-loaded instruction surfaces, not just
   `AGENTS.md`.
2. Pushing an over-budget instruction file fails at pre-push with a clear, per-file message.
3. `AGENTS.md` and the Claude resolved tree are both within their new ceilings, so the gate
   ships green and the Claude Code warning is gone.
4. The rule is documented as a convention, propagated to all reference surfaces, validated by
   `repo-rules-checker`, and listed in `repo-rules-quality-gate.md`.
5. `ose-primer` / `ose-infra` parity is explicitly handed off (not silently dropped).

## Constraints

- **No self-failing gate.** The budget must not be merged in a state where the current repo
  fails it — `AGENTS.md` must be trimmed under the new ceiling in the same plan.
- **Byte-based measurement** (matches the existing validator's `fs::metadata` len), so
  behavior is deterministic and identical in hooks and CI.
- **Two-path completeness**: touching `apps/rhino-cli` requires companion `specs/apps/rhino`
  Gherkin in the same change.
- **No time estimates** — outcomes only.

## Out of Scope

- Executing `ose-primer` / `ose-infra` convergence (Phase 6 is a hand-off note).
- Creating instruction files for harnesses that don't yet have them.
- Changing what any instruction file _says_ — except the mechanical `AGENTS.md` trim, which
  moves already-linked content to its canonical `repo-governance/` homes without altering
  rules.
