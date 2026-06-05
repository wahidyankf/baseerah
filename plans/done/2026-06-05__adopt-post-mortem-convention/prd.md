# Product Requirements — Adopt Post-Mortem Convention

## Product Overview

Deliver a blameless, software-flavored post-mortem capability to `ose-public` as governance +
documentation. The "product" is three new markdown surfaces plus three index updates:

1. **Authoritative convention** — `repo-governance/conventions/structure/post-mortems.md`.
2. **Writer-facing template + index** — `docs/explanation/post-mortems/README.md`.
3. **Worked example** — `docs/explanation/post-mortems/<incident-date>-<system>-<short-failure>.md`,
   grounded in the real `.amazonq/` Prettier parity-guard breakage.

Plus index updates so the new surfaces are discoverable from the governance and docs indexes.

## Personas

Solo-maintainer repo — personas are hats the maintainer wears and agents that consume the surface:

- **Incident author** (maintainer) — needs a copy-paste template and clear rules to file a
  retrospective quickly after a software incident.
- **Convention reader** (maintainer / future contributor / agent) — needs an authoritative rule
  that defines mandatory sections, severity scale, naming, and lifecycle.
- **Governance agents** (`repo-rules-maker`, `repo-rules-checker`, `repo-rules-fixer`) — consume the
  convention as a governed rule and must find it consistent with the rest of `repo-governance/`.
- **Docs agents** (`docs-maker`, `docs-checker`) — consume the template/index/example as Diátaxis
  explanation-tier documentation.

## User Stories

- **As an incident author**, I want a single authoritative convention defining how to write a
  post-mortem, so that every retrospective has the same structure and severity vocabulary.
- **As an incident author**, I want a copy-paste template in the docs explanation tier, so that I
  can start a new post-mortem without re-reading the full convention.
- **As a convention reader**, I want a complete software-flavored worked example, so that I can see
  the convention applied to a real `ose-public` incident rather than an abstract infra scenario.
- **As a governance owner**, I want the new convention listed in every relevant index, so that it is
  discoverable and not orphaned.
- **As a governance owner**, I want the new convention to pass `repo-rules-quality-gate` at strict
  mode, so that I know it is consistent with the rest of the governance surface.

## Acceptance Criteria (Gherkin)

### Scenario: Authoritative convention exists with all mandatory sections

```gherkin
Given the post-mortem convention has been adopted
When I open repo-governance/conventions/structure/post-mortems.md
Then the file exists
And it declares itself the authoritative governance rule
And it points to docs/explanation/post-mortems/README.md as the working surface
And it states "when the two disagree, the convention wins"
And it contains a Location and Naming section with the pattern "YYYY-MM-DD-<system>-<short-failure>.md"
And it contains a Blameless Principle section applying the "second story" framing
And it contains a Mandatory Sections section listing every required section in order
And it contains an authoritative four-tier Severity Scale (Sev-1 through Sev-4)
And it contains an Action Items table structure with Owner, Priority (P0/P1/P2), Ticket, Status columns
And it documents the doc_status lifecycle (draft to reviewed to closed)
And it contains a No Secrets Rule referencing repo-governance/conventions/security/no-secrets-in-git.md
And it contains a Diagrams section citing the WCAG AA palette hex codes
```

### Scenario: Convention is framed for software incidents, not infrastructure

```gherkin
Given the convention has been adapted from the ose-infra original
When I read its Purpose, Scope, and example sections
Then incident examples reference software reality only
And examples include CI/CD pipeline failures, Vercel production-site outages, dependency-bump regressions, coverage-threshold regressions, and generated-artifact byte-equality guard breakages
And no example references Proxmox, Tailscale, dual-WAN routers, or on-premise hardware
And no reference to ose-infra's no-secrets-in-committed-files.md filename appears
And the no-secrets reference uses ose-public's no-secrets-in-git.md path
```

### Scenario: Dual structure is present and cross-linked

```gherkin
Given the dual-file structure has been created
When I open docs/explanation/post-mortems/README.md
Then it exists and provides a copy-paste post-mortem template
And it provides an index listing the worked example
And it links to the authoritative convention
And it states the convention wins when the two disagree
And the authoritative convention links back to this writer-facing README
```

### Scenario: Worked example follows naming and contains all mandatory sections

```gherkin
Given the worked example post-mortem has been written
When I open the file under docs/explanation/post-mortems/
Then its filename matches "YYYY-MM-DD-<system>-<short-failure>.md" in lowercase kebab-case
And it has frontmatter including a doc_status field
And it has a metadata table immediately after the H1
And it classifies severity using the authoritative Sev-N scale
And it contains all mandatory sections in the specified order
And its Root Cause section is distinct from its Trigger section
And it contains an Action Items table with at least one owned, prioritized item
And every timestamp in the Timeline uses absolute time with the WIB UTC+7 timezone stated
```

### Scenario: Worked example is grounded in a real ose-public incident

```gherkin
Given the worked example must be a real-pattern incident, not fiction
When I read the worked example
Then the incident is the Prettier reformatting of generated .amazonq binding artifacts
And the symptom is the cross-vendor parity byte-equality guard failing in CI / pre-commit
And the documented fix is adding emitter-generated files to .prettierignore
And the Root Cause distinguishes the systemic condition from the proximate Trigger
And no secret value appears anywhere in the document
```

### Scenario: Worked example includes at least one accessible diagram

```gherkin
Given accessibility is required for diagrams
When I inspect the worked example's Mermaid diagram
Then it uses only the WCAG AA palette hex codes #0173B2 #DE8F05 #029E73 #CC78BC #CA9161 #808080
And every node has sufficient contrast per the color-accessibility convention
And the diagram clarifies the causal chain from trigger to guard failure
```

### Scenario: Index files are updated

```gherkin
Given the new surfaces must be discoverable
When I open repo-governance/conventions/structure/README.md
Then it lists the Post-Mortem Convention entry
And when I open repo-governance/conventions/README.md
Then its structure-conventions enumeration includes the Post-Mortem Convention entry
And when I open docs/explanation/README.md
Then its Documentation Index includes a Post-Mortems entry linking to docs/explanation/post-mortems/README.md
```

### Scenario: Governance surface remains consistent

```gherkin
Given the convention and docs have been authored
When the repo-rules-quality-gate workflow runs at strict mode
Then it reaches double-zero (two consecutive checks with zero CRITICAL/HIGH/MEDIUM findings)
And no broken cross-links remain
And markdown lint passes for all new and modified files
```

### Scenario: Changes are harness-neutral

```gherkin
Given the plan touches repo-governance/ and docs/
When I inspect the new convention file
Then it contains no vendor-specific harness syntax outside any "Platform Binding Examples" heading
And the convention prose is vendor-neutral per the Governance Vendor-Independence Convention
```

## Product Scope

**In-scope features**:

- Authoritative convention file (software-flavored).
- Writer-facing template + index.
- One worked-example post-mortem (real `.amazonq/` parity-guard incident).
- Three index updates.
- Strict-mode `repo-rules-quality-gate` validation.

**Out-of-scope features**:

- Incident-response runbooks; on-call/escalation policy.
- Automated post-mortem generation or templating tooling.
- Historical post-mortem backfill.
- Application or CI-config changes.

## Product Risks

| Risk                                                 | Likelihood | Mitigation                                                                   |
| ---------------------------------------------------- | ---------- | ---------------------------------------------------------------------------- |
| Infra vocabulary leaks into adapted convention       | Medium     | Adaptation map in tech-docs.md; strict-mode quality gate scans for residue   |
| Wrong no-secrets filename (ose-infra's) carried over | Medium     | Explicit delta: use `no-secrets-in-git.md`; acceptance criterion checks this |
| Worked-example diagram fails WCAG AA palette check   | Low        | Use only the six approved hex codes; color-accessibility convention enforced |
| New convention orphaned (not indexed)                | Low        | Dedicated index-update phase with per-file acceptance criteria               |
