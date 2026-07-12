# 58 · IT Governance, Risk & Compliance ‡ (Annotated-concept, no-code)

**prd row**: Pass 3 · Build for the Real World · Annotated-concept · ‡ no-code · Learn 158 / Drill 258 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `‡` leadership/no-code — governing technology responsibly: frameworks (COBIT/ISO 27001/NIST
CSF/SOC 2 intuition), risk management (identify/assess/treat), compliance & privacy (GDPR/data-protection
concepts), policy vs control, audit trails, and how security posture ([`55-it-and-application-security`](./55-it-and-application-security.md),
[`57-defensive-security`](./57-defensive-security.md)) rolls up into organizational assurance. Deliverables
are **decision/governance artifacts**, not code.

## Why this exists · the big idea

- **The problem before the solution**: controls, audits, and policies accumulate ad hoc until no one can
  say whether the organization is actually protected or merely busy — GRC exists to turn scattered security
  activity into defensible, org-level assurance.
- **Keep-this-if-you-forget-everything**: a control is worthless unless it traces back to a named risk and
  forward to auditable evidence — governance is that traceability, not the paperwork.
- **Big ideas touched**: `mechanism-vs-policy` — separates the machinery (controls, security operations)
  from the decisions (who owns which risk, what the org will accept); `correctness-vs-pragmatism` — risk
  treatment is disciplined compromise (mitigate / transfer / accept), never the fantasy of zero risk.

## Prerequisites

- **Prior topics**: [topic 55 IT / Application Security](./55-it-and-application-security.md) (CIA, threat modeling, controls),
  [topic 57 Defensive Security](./57-defensive-security.md) (detection, IR, the blue-team view that GRC
  rolls up), and [topic 9 Project Management](./09-project-management.md)
  (policy, process, working within an org).
- **Tools & environment**: no toolchain — a text editor for the governance artifacts (risk register, policy,
  control mapping); Neovim/VSCode (DD-17). No paid account, no code (DD-20).
- **Assumed knowledge**: security controls + threat modeling (topic 55); detection/IR posture (topic 57);
  how teams adopt process (topic 09).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified (framework versions): **COBIT 2019** is still current (6 principles, 40 objectives
  across EDM/APO/BAI/DSS/MEA — no 2024/25 revision; ISACA's Feb-2026 **ITAF 5th ed.** is a companion audit
  update, not a COBIT bump — re-check at authoring time). **ISO/IEC 27001:2022** is the sole current
  baseline (2013 certs expired 2025-10-30; minor **Amd 1:2024** adds climate-change clauses). **NIST CSF
  2.0** (2024-02-26) is current — added the **Govern** function → 6 functions total. **SOC 2** (AICPA Trust
  Services Criteria) is not version-numbered — un-versioned reference is correct.
- 2026-07-12 — verified: **GDPR** (Regulation (EU) 2016/679) in force, unchanged; risk methodology
  (identify → assess likelihood×impact → treat: accept/mitigate/transfer/avoid, ISO 31000 framing) and
  policy/control/procedure + preventive/detective/corrective control taxonomy are evergreen.

## Items

- Governance frameworks: COBIT, ISO/IEC 27001, NIST CSF, SOC 2 — what each is for (intuition, not
  certification).
- Risk management: identify → assess (likelihood × impact) → treat (accept/mitigate/transfer/avoid); the
  risk register.
- Policy vs control vs procedure; preventive/detective/corrective controls; control mapping.
- Compliance & privacy: GDPR/data-protection concepts, data classification, retention, auditability.
- Privacy by design: data minimization, purpose limitation, and privacy defaults baked into the system
  rather than bolted on.
- Software licensing & OSS IP: permissive vs copyleft (MIT/Apache vs GPL/AGPL), license compatibility,
  SBOMs, and third-party-dependency IP risk.
- Assurance: audit trails, evidence, how security operations roll up to org-level assurance.

## Worked examples

Colocated under `it-governance-grc/learning/artifacts/` (no `code/` — governance
deliverables per the `‡` shape, DD-27/DD-30).

- **beginner** — a risk register entry: an identified risk assessed (likelihood × impact) with a treatment.
- **intermediate** — a control mapping: tie a handful of controls to a framework's categories.
- **advanced** — a short policy + its supporting procedures + the audit evidence that would demonstrate
  compliance.

## Capstone spec — intra-topic (leadership → governance/decision artifact, no code)

- **Goal**: produce a **coherent GRC artifact set** for a small system — a risk register (risks identified,
  assessed by likelihood × impact, and treated), a control mapping to a named framework, and a short policy
  with supporting procedures + the audit evidence that would satisfy it — demonstrating that security
  posture rolls up into organizational assurance. **No code.**
- **Concepts exercised**: [ ] a risk register (identify → assess likelihood × impact → treat) [ ] a control
  mapping to a named framework (ISO 27001 / NIST CSF / SOC 2) [ ] policy vs control vs procedure [ ] an
  audit-evidence / auditability trail [ ] the security-ops → org-assurance roll-up.
- **Ordered steps**:
  1. `.../learning/capstone/artifacts/risk-register.md` — identify + assess (likelihood × impact) + treat
     a realistic set of risks for a small system. Verify each risk has an owner, a rating, and a treatment
     decision.
  2. `control-mapping.md` — map controls to a named framework's categories. Verify each mapped control
     traces to a real risk and a framework category.
  3. `policy.md` + `evidence.md` — a short policy with supporting procedures and the audit evidence that
     would demonstrate compliance. Verify the policy is enforceable, the procedures operationalize it, and
     the evidence is concrete and auditable.
- **Acceptance criteria**: risks are assessed and treated with owners; controls map to a real framework and
  back to risks; the policy/procedure/evidence chain is coherent and auditable; the artifact set holds
  together as org-level assurance. No code.
- **Done bar**: complete governance artifact set + internally coherent + web-verified.

## Read more

**Books**

- **IT Governance: How Top Performers Manage IT Decision Rights for Superior Results** — Peter Weill, Jeanne W. Ross (2004). The foundational business-school text defining IT governance as a discipline.

**Papers & articles**

- **NIST Cybersecurity Framework (CSF) 2.0** — National Institute of Standards and Technology (2024). Foundational risk-management framework central to most modern GRC programs. <https://www.nist.gov/cyberframework>
- **ISO/IEC 27001:2022 — Information Security Management Systems** — ISO/IEC (2022). The internationally recognized standard for information security management systems (ISMS), central to most GRC programs. <https://www.iso.org/standard/27001>
- **COBIT 2019 Framework** — ISACA (2018). The widely adopted framework specifically for IT governance and management objectives. <https://www.isaca.org/resources/cobit>

---

← Previous: [57 · Defensive Security](./57-defensive-security.md) · Next: [59 · Analytics & Experimentation](./59-analytics-and-experimentation.md) →
