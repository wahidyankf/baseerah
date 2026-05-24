---
title: Scenario By-Example Tutorial Convention
description: Standards for scenario-domain by-example tutorials using annotated documents, decisions, and governance artifacts — extends the SWE By-Example Convention for any non-code domain
category: explanation
subcategory: conventions
tags:
  - convention
  - tutorial
  - by-example
  - scenario
  - governance
  - decision-making
created: 2026-05-21
---

# Scenario By-Example Tutorial Convention

## Purpose

This convention **extends the [SWE By-Example Tutorial Convention](./swe-by-example.md) for any
domain where learning happens through annotated documents, decisions, or governance artifacts
rather than runnable code**.

**Base requirements**: Scenario by-example tutorials inherit all standards from the
[SWE By-Example Convention](./swe-by-example.md) and override only the differences documented
below.

**Target audience**: Practitioners — engineers, tech leads, managers — who learn best through
realistic annotated scenarios grounded in real organizational contexts.

**Applicable domains** (not exhaustive):

- Security governance and leadership (CISO)
- Software architecture decisions (ADRs, design reviews)
- Legal and compliance scenarios
- Project management and delivery decisions
- Risk management and business continuity

---

## How It Differs from SWE By-Example

### Artifact type

| SWE By-Example                 | Scenario By-Example                                                            |
| ------------------------------ | ------------------------------------------------------------------------------ |
| Runnable source code           | Annotated policy, risk register, decision record, governance table, case study |
| `go run`/`python` verification | Scenario plausibility and organizational realism                               |
| `// =>` on variable state      | `# =>` or `<!-- => -->` on document lines explaining reasoning                 |

### Self-containment definition

**SWE by-example**: Copy-paste-runnable with a single command.

**Scenario by-example**: Fully standalone with complete organizational context. Each example
must include:

- **Scenario Context** — organization type, size, industry, and decision-maker role
- **Complete artifact** — the full policy excerpt, risk register row, or decision document; no
  "see Example N for the template" cross-references
- **All annotations** — inline comments on every substantive line explaining the reasoning,
  trade-off, or decision rationale

### Annotation semantics

**SWE by-example** (`// =>` on code): Documents variable state and return values.

**Scenario by-example** (`# =>` on document lines or `<!-- => -->` in markdown tables):
Documents the reasoning, constraint, or trade-off behind each element.

```yaml
# Risk Register Entry
risk_id: RISK-003
asset: Customer PII database # => Scope: production database only (dev DBs are separate)
threat: Unauthorized access by insider
vulnerability: Over-privileged DBA accounts
likelihood: 3 # => Medium: 1 DBA with excessive access; IAM review outstanding
impact: 5 # => Catastrophic: 200K records, GDPR notification required
risk_score: 15 # => likelihood × impact = HIGH band (12–19)
# => Exceeds risk appetite threshold of 12 — treatment required, cannot accept
treatment: Mitigate # => Implement least-privilege IAM, not accept/transfer
# => Accept rejected: score > appetite. Transfer rejected: no cyber insurance covering insider.
owner: Head of Engineering
due_date: 2026-07-31
```

**For markdown tables**, use a "Rationale" column or trailing comment row:

```markdown
| Control                    | Status      | Rationale                                                           |
| -------------------------- | ----------- | ------------------------------------------------------------------- |
| MFA on all admin accounts  | Implemented | => Reduces credential stuffing risk; required by ISO 27001 A.9.4.2  |
| Vulnerability scan monthly | Partial     | => Scanner deployed but cloud assets excluded; gap being remediated |
```

### Coverage metric

**SWE by-example**: 95% of programming language/framework features.

**Scenario by-example**: Coverage of domain competency — the breadth of decisions, frameworks,
and scenarios a practitioner in that domain regularly encounters.

Coverage percentages per level:

- Beginner: 0–40% (foundational concepts, simple decisions)
- Intermediate: 40–75% (production scenarios, compliance/frameworks in context)
- Advanced: 75–95% (complex multi-stakeholder decisions, crisis scenarios, program-level leadership)

### Mermaid diagram use cases

Scenario by-example diagrams visualize:

- **Decision trees**: Branching decision logic (e.g., breach notification decision tree)
- **Workflow diagrams**: Process flows (e.g., vendor onboarding, incident escalation)
- **Organizational charts**: RACI matrices, governance structures
- **Risk matrices**: Heat maps showing likelihood vs impact
- **Timeline diagrams**: Regulatory deadlines, program milestones
- **Compliance mapping**: Framework control mapping across multiple standards

Same color-blind palette applies (Blue #0173B2, Orange #DE8F05, Teal #029E73, Purple #CC78BC,
Brown #CA9161).

### No "core features first" constraint

The core-features-first principle from SWE by-example does not apply — scenario content has no
"dependency installation" concern. Instead, apply:

**Frameworks-last principle**: Introduce specific frameworks (ISO 27001, NIST CSF, FAIR) only
after the underlying concept is established. Do not open beginner examples by applying a framework
without first explaining what problem it solves.

Example:

```markdown
## PASS: Concept first, then framework

### Example 5: Scoring Risk Likelihood and Impact (Beginner)

A 5×5 risk matrix lets you compare risks by multiplying two factors...
[Example shows a blank matrix and manual scoring]

### Example 36: Applying FAIR for Quantitative Risk (Intermediate)

The FAIR model (Factor Analysis of Information Risk) structures the likelihood/impact
factors from Example 5 into financial loss estimates...
```

```markdown
## FAIL: Framework before concept

### Example 5: FAIR Risk Quantification (Beginner)

FAIR is the international standard for quantitative cyber risk...
(Reader hasn't learned what risk likelihood and impact mean yet)
```

---

## Five-Part Format (scenario-adapted)

### Part 1: What This Covers (2-3 sentences)

Same as SWE by-example. Must answer:

- What governance concept, decision type, or framework element does this example demonstrate?
- Why does it matter to the practitioner's role?
- When would this decision or artifact arise in a real organization?

### Part 2: Scenario Context (1-2 sentences)

Replace "Scenario" with organizational framing:

- Organization type and size (e.g., "You are the security manager at AcmeSoft, a 200-person SaaS
  company")
- Decision-maker role and the immediate business context
- Use fictional but plausible organization names (AcmeSoft, Nexatech, Meridian Health, etc.)

```markdown
**Scenario:** You are the newly appointed security manager at AcmeSoft, a 200-person SaaS
company with no formal risk register. Your CTO has asked for a first pass before the board
meeting next quarter.
```

### Part 3: Annotated Document or Decision

The core artifact — fully annotated with `# =>` or `<!-- => -->`:

- Show the complete policy excerpt, risk register entry, compliance table, or decision record
- Every substantive line has an annotation explaining reasoning, constraint, or trade-off
- Use realistic fictional values (plausible dollar amounts, risk scores, dates)
- Density target: same 1.0–2.25 annotation lines per substantive non-blank content line per example

**Annotation quality**: Annotations explain WHY a decision was made, not just WHAT the field is.

```yaml
# FAIL: Annotation describes field, not reasoning
likelihood: 3  # => This is the likelihood score

# PASS: Annotation explains reasoning
likelihood: 3  # => Medium: attack requires insider access + active exploitation of a
               # => known vulnerability — not easily automated by external attacker
```

### Part 4: Key Takeaway (1-2 sentences)

Same as SWE by-example. The core decision insight to retain:

```markdown
**Key Takeaway:** A risk register is not a documentation exercise — it is a prioritization
tool. Every entry without a named owner and a due date is a finding waiting to be ignored.
```

### Part 5: Why It Matters (50-100 words)

Same as SWE by-example. Production-focused, active voice, specific to the scenario.

---

## Coverage Levels

### Beginner (Examples 1–28, 0–40%)

**Focus**: Foundational governance concepts and simple decisions.

- Core vocabulary: risk, threat, vulnerability, control, compliance, governance
- Simple artifacts: basic risk register, AUP, asset classification table
- Fundamental frameworks introduced conceptually (not applied in depth)
- Decisions are single-factor (one risk, one policy, one vendor)

**Self-containment**: Each example requires no prior framework knowledge.

### Intermediate (Examples 29–57, 40–75%)

**Focus**: Production scenarios, compliance framework application, program-level decisions.

- Applying frameworks in context: ISO 27001 SoA, NIST CSF gap analysis, FAIR quantification
- Multi-factor decisions: vendor risk scoring, tabletop exercise design, budget justification
- Regulatory compliance: GDPR ROPA, breach notification decision trees, PCI DSS scoping
- Measuring and reporting: security KPIs, board dashboards, maturity models

### Advanced (Examples 58–85, 75–95%)

**Focus**: Complex multi-stakeholder decisions, crisis scenarios, program leadership.

- Crisis management: ransomware response, regulatory notification timelines, board escalation
- Program-level leadership: operating model design, M&A due diligence, red team program strategy
- Emerging domains: AI governance, supply chain security programs, DORA/NIS2 compliance
- Career and organizational: succession planning, CISO competency frameworks, C-suite reporting

---

## Applies To

This convention governs scenario by-example content in ayokoding-web:

- `information-security/roles/ciso/by-example/` — CISO security governance track

Future applications (not yet created):

- Architecture decision record tutorials
- Legal/compliance scenario tutorials
- Project and delivery management tutorials

The Foundations, Red Team, and Blue Team tracks are governed by the
[Security By-Example Tutorial Convention](./security-by-example.md), not this convention.

---

## Validation Criteria

Extend the [SWE By-Example validation checklist](./swe-by-example.md#quality-checklist) with:

- [ ] Organizational scenario clearly stated (company type, size, decision-maker role)
- [ ] Fictional but plausible organization names and values used
- [ ] Annotations explain reasoning/trade-off, not just field names
- [ ] Every substantive document line is annotated
- [ ] Frameworks introduced after the underlying concept (frameworks-last)
- [ ] No executable code required — scenario fully standalone
- [ ] Decision or artifact is complete (no "see Example N for the template" cross-references)
- [ ] Dollar amounts, risk scores, and dates are realistic for the stated organization size

---

## Principles Implemented/Respected

- **[Progressive Disclosure](../../principles/content/progressive-disclosure.md)** — Coverage
  levels (Beginner/Intermediate/Advanced) layer complexity progressively; readers start with simple
  single-factor decisions and advance to complex multi-stakeholder scenarios.
- **[No Time Estimates](../../principles/content/no-time-estimates.md)** — Coverage expressed
  as percentages of domain competency breadth rather than time-based estimates; practitioners set
  their own pace.
- **[Accessibility First](../../principles/content/accessibility-first.md)** — Color-blind
  friendly Mermaid palette (Blue #0173B2, Orange #DE8F05, Teal #029E73, Purple #CC78BC, Brown
  #CA9161) required for all diagrams; WCAG AA compliance throughout.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**
  — Each example must be fully self-contained with complete organizational context, explicit
  scenario framing, and all annotations inline; no "see Example N" cross-references permitted.

---

## Related Documentation

- [SWE By-Example Tutorial Convention](./swe-by-example.md) — base convention this extends
- [Security By-Example Tutorial Convention](./security-by-example.md) — for security tool/lab content
- [General Tutorial Convention](./general.md) — base tutorial standards
- [Diagrams Convention](../formatting/diagrams.md) — Mermaid diagram standards
