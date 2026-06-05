# Business Requirements — Adopt Post-Mortem Convention

## Business Goal

Establish a single, authoritative, blameless **post-mortem convention** in `ose-public` so that
when a software incident occurs (CI/CD pipeline failure, Vercel production-site outage, dependency
regression, coverage-threshold regression, generated-artifact guard breakage), the retrospective
has a standard home, a standard shape, and a standard tracking mechanism — and the learning survives
beyond the moment of the incident.

## Business Rationale (Why This Exists)

`ose-infra` already proves the value of a structured post-mortem convention for the broader
open-sharia-enterprise ecosystem. `ose-public` currently has no equivalent. [Repo-grounded:
`docs/explanation/` contains only `README.md` and `software-engineering/` — verified via `ls`.]

Without a convention:

- Incident learning is ad-hoc, scattered across commit messages, or lost entirely.
- There is no consistent severity vocabulary, so "how bad was it?" is re-litigated each time.
- Action items born from an incident are not owned, prioritized, or tracked to closure.
- Repeat incidents (e.g., a generated-artifact guard breaking again) recur without a written
  record of the prior root cause and fix.

The cheapest place to capture incident learning is a written retrospective filed promptly while
details are fresh, in a known location, with a known structure. This convention provides exactly
that — adapted to `ose-public`'s software reality rather than `ose-infra`'s on-premise hardware
reality.

## Business Impact

**Pain points addressed**:

- No standard retrospective format → inconsistent, low-signal incident write-ups. [Judgment call:
  qualitative reasoning — no prior post-mortems exist to measure against.]
- No severity scale → ambiguous incident triage and prioritization.
- No action-item tracking → root-cause fixes silently dropped.

**Expected benefits**:

- Faster, higher-signal retrospectives that name root cause vs. trigger explicitly.
- A reusable severity scale (Sev-1 .. Sev-4) shared across the repo.
- Owned, prioritized, tracked action items linked to `plans/` entries.
- A worked example that demonstrates the blameless, software-flavored style for future authors.

## Affected Roles

This is a solo-maintainer repository — no sign-off ceremonies, sponsors, or stakeholder approval
gates. The roles below describe the hats the maintainer wears and the agents that consume the new
governance surface:

- **Maintainer-as-incident-author** — writes post-mortems after software incidents using the new
  convention and template.
- **Maintainer-as-governance-owner** — maintains the convention and ensures it stays consistent
  with the rest of the `repo-governance/` surface.
- **`repo-rules-maker` / `repo-rules-checker` / `repo-rules-fixer` agents** — author, validate, and
  fix the convention as part of the governance surface (consume the convention as a governed rule).
- **`docs-maker` / `docs-checker` agents** — author and validate the writer-facing template, index,
  and worked example as Diátaxis explanation-tier documentation.

## Business-Level Success Metrics

- **Convention discoverable**: the new convention is linked from every index that enumerates
  structure conventions and from the docs explanation index. [Observable fact after delivery —
  verified by index-update acceptance criteria in `prd.md`.]
- **Governance-consistent**: the `repo-rules-quality-gate` workflow passes at strict mode with
  double-zero, proving the new convention is internally consistent with the governance surface.
  [Observable fact after delivery.]
- **Demonstrably usable**: a complete worked example exists, exercising every mandatory section, a
  severity tier, an action-item table, and an accessible diagram. [Observable fact after delivery.]

No fabricated numeric KPIs are claimed; this is a governance-adoption effort whose success is
structural (the rule exists, is consistent, and is demonstrated), not metric-driven.

## Business-Scope Non-Goals

- Not building incident-detection tooling, alerting, or monitoring (no production infra to monitor).
- Not defining an on-call rotation or escalation policy.
- Not migrating or authoring historical post-mortems beyond the single worked example.
- Not changing application behavior or CI configuration.

## Business Risks and Mitigations

| Risk                                                                    | Mitigation                                                                                                            |
| ----------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| Convention drifts from `ose-infra`'s, causing ecosystem inconsistency   | Keep structure, severity scale, blameless rules, action-item table, and `doc_status` identical; only reframe examples |
| Worked example reads as fabricated, undermining trust in the convention | Ground the example in a real, already-documented `ose-public` issue (the `.amazonq/` Prettier parity-guard bug)       |
| Infra-specific vocabulary leaks in, confusing software-platform readers | Systematic adaptation map in `tech-docs.md`; software-only examples; strict-mode `repo-rules-quality-gate` pass       |
| A secret leaks into the committed worked example                        | The example has no secrets; still reference `no-secrets-in-git.md` and use placeholders for any sensitive token       |

## Cross-Cutting Links

- Testable scenarios for each success metric live in [prd.md](./prd.md).
- The ose-infra → ose-public adaptation mapping and dual-file architecture live in
  [tech-docs.md](./tech-docs.md).
