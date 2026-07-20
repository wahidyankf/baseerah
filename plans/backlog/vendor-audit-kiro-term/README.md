# Vendor-Audit: Add `Kiro` to the Vendor-Term List (Tri-Repo)

> **Status**: Backlog — filed by the Knowledge Capture phase of
> [`parallel-orchestration-shared-machine-governance`](../../done/) (merged as `60d53119b`).
>
> **Delivery Mode**: `worktree-to-pr` (repo default), executed as a **tri-repo parity** change.
>
> **Boundary note**: touches `apps/rhino-cli/**`, required to be **byte-identical** across
> `ose-public`, `ose-primer`, and `ose-infra` per the
> [SDLC Gate Standard](../../../docs/reference/sdlc-gate-standard.md#rhino-cli-byte-identity-boundary).

The vendor-audit scanner enforces vendor-neutrality in `repo-governance/**` by matching a fixed
list of vendor terms. **"Kiro" is not in that list.** A Kiro mention leaking into governance prose
would pass the scanner silently — exactly the failure the scanner exists to prevent.

## Context

The
[Governance Vendor-Independence Convention](../../../repo-governance/conventions/structure/governance-vendor-independence.md)
keeps `repo-governance/**` vendor-neutral, with concrete tool integrations confined to
platform-binding directories. Enforcement is a term-list scan implemented in
`apps/rhino-cli/src/application/repo_governance/vendor_audit.rs`, matching `Claude Code`,
`OpenCode`, `\bCursor\b`, `\bAmazon Q\b`, `\bAntigravity\b`, and similar. [Repo-grounded]

"Kiro" / "Kiro CLI" entered this repository's vocabulary as the Amazon Q Developer succession. The
scanner does not know the term.

**This gap is preventive, not corrective.** Verified at filing time: `grep -rn "Kiro"
repo-governance/` returns nothing, so there is no live leak to clean up.

**Why it was not fixed in the originating plan**: editing `vendor_audit.rs` from a single-repo plan
would break the byte-identity boundary. Editing only the convention's documented term table would
be strictly worse — the table would then describe terms the scanner does not actually match, so the
documentation would lie about the tool. This is the enumeration-based-guard failure mode in its
canonical form: a denylist that fails open on every term nobody has added yet.

## Scope

**In scope**:

- Add `\bKiro\b` and the `\.kiro/` path prefix to the vendor-term list in `vendor_audit.rs`.
- Update the companion term table in the Governance Vendor-Independence Convention so the doc and
  the tool agree.
- Companion Gherkin under `specs/apps/rhino/behavior/rhino-cli/gherkin/**`, per the
  [Feature Change Completeness Convention](../../../repo-governance/development/quality/feature-change-completeness.md).
- Land byte-identically in all three repos together.
- Add `.kiro/` to the platform-bindings catalog if a Kiro binding is actually shipped.

**Consider while executing** — the term list is a **denylist, and denylists fail open**. Evaluate
whether the scanner should instead assert that governance prose contains no proper-noun tool
reference outside an allowlisted set, which would fail closed on the next unnamed vendor rather
than requiring a plan per vendor. See
[Anti-Pattern 10: Enumeration-Based Guards](../../../repo-governance/development/agents/anti-patterns.md#anti-pattern-10-enumeration-based-guards-denylist-guards-that-fail-open).
If that redesign is out of appetite, record the decision explicitly — the next vendor will hit this
same plan.

## Acceptance Criteria

```gherkin
Feature: The vendor-audit scanner recognizes Kiro

  Scenario: A Kiro mention in governance prose is flagged
    Given a file under repo-governance/ containing the word "Kiro"
    When rhino-cli repo-governance vendor-audit runs
    Then the finding names that file and the term "Kiro"
    And the command exits non-zero

  Scenario: A Kiro mention under a Platform Binding Examples heading is skipped
    Given a file whose "Platform Binding Examples" section mentions Kiro
    When the vendor-audit runs
    Then no finding is reported for that section

  Scenario: Governance prose without any vendor term still passes
    Given repo-governance/ contains no vendor term
    When the vendor-audit runs
    Then the command exits 0

  Scenario: The documented term table matches the scanner
    Given the Governance Vendor-Independence Convention term table
    When it is diffed against the term list in vendor_audit.rs
    Then the two sets are identical

  Scenario: The change is byte-identical across all three repos
    Given the rhino-cli source in ose-public, ose-primer, and ose-infra
    When the vendor_audit.rs files are compared
    Then they are byte-identical
```

The third scenario is the falsifiability control — a scanner broken into flagging everything would
pass the first scenario and fail this one. The fourth is the doc/tool-agreement check that the
originating plan identified as the reason not to fix the table alone.
