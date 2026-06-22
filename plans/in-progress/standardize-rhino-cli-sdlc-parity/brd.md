# BRD — Standardize rhino-cli Checks & SDLC Commands

## Business Goal

Make the SDLC quality machinery behave `"identically"` across `ose-public`, `ose-primer`, and
`ose-infra` so that a contributor (human or AI agent) who learns the gate mechanics in one repo
applies that knowledge unchanged in the other two, and so that `ose-public` (the upstream source of
truth) can be propagated to the siblings without per-repo translation friction.

## Why It Matters

- **Cognitive load** — today the same gate has three names (`commons-quality-gate.yml` /
  `pr-quality-gate.yml`) and three invocation styles (inline shellcheck vs. an Nx-wrapped
  `rhino-cli:shell:lint` target). Every cross-repo edit forces a re-learn. [Repo-grounded]
- **Parity-loop cost** — the [multi-repo parity workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)
  has to absorb structural drift on every sync. Identical mechanics make propagation near-mechanical. [Repo-grounded]
- **Dead-command risk** — rhino-cli ships subcommands that no lifecycle automation invokes
  (e.g. `convention validate license`, `md validate frontmatter-dates`). Without a triage, nobody
  knows whether a gate is genuinely enforced or merely available. This plan makes the wired/not-wired
  status explicit and reviewable. [Repo-grounded]
- **Trust in green CI** — when "the markdown gate" runs three different validator sets in three
  repos, a green check means three different things. Identical mechanics make green mean the same
  thing everywhere. [Judgment call]

## Affected Roles

- **Repo maintainer / solo operator** — edits hooks and workflows across all three repos; primary beneficiary.
- **AI coding agents** (`ci-checker`, `repo-harness-compatibility-checker`, `swe-rust-dev`) — validate and edit this surface; benefit from one mental model.
- **Contributors to `ose-primer`** (downstream template consumers) — inherit a coherent, documented gate model.

## Business-Level Success Metrics

- **Gate-mechanics parity**: for every gate in the [target standard](./tech-docs.md#1-target-standard-best-of-three-synthesis), the gate name, the workflow filename, the hook step ordering, and the invocation mechanism are identical across all three repos (modulo documented allowed divergence). Observable check: the cross-repo parity table in [delivery.md Phase 5](./delivery.md#phase-5-cross-repo-parity-verification--archival) shows ✅ on every mechanics row. [Repo-grounded]
- **Command triage published**: a committed reference doc lists every rhino-cli command with a wired/not-wired status and the exact invocation site for each wired command. Observable check: `docs/reference/rhino-cli-command-triage.md` exists and covers every leaf subcommand in `apps/rhino-cli/src/cli.rs`. [Repo-grounded]
- **Zero regressions**: after convergence each repo's pre-push and PR quality gate pass on a no-op change. [Repo-grounded]

## Business-Scope Non-Goals

- Not changing **what** any validator checks (no new lint rules, no threshold changes).
- Not unifying the **app set** or **language set** across repos.
- Not removing not-wired commands — triage only; any removal/wiring decision is a follow-up.

## Business Risks

- **Risk: convergence breaks a working gate.** Mitigation: each repo converges in its own phase behind a full pre-push + PR-gate verification before the phase gate passes.
- **Risk: over-standardization erases a legitimate infra-only gate.** Mitigation: the [divergence policy](./tech-docs.md#3-divergence-policy-allowed-vs-drift) is fixed up front; IaC and app-set gates are explicitly protected.
- **Risk: drift re-appears after convergence.** Mitigation: the committed standard doc becomes the reference `ci-checker` / `repo-harness-compatibility-checker` validate against; a follow-up can add an automated parity check.
