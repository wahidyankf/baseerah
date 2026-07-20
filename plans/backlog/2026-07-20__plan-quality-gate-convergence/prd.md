# Product Requirements — Plan Quality Gate Convergence

## Product Overview

Six coordinated changes to the plan-quality-gate loop and its supporting agents and skills, so the
loop reaches its first zero faster while catching at least everything it catches today.

The product surface is governance text and one CLI validator. There is no user-facing screen or
component under `apps/` or `libs/` that renders to an end user, so the **UI-design-funnel is not
applicable** to this plan — the exemption is stated and justified in
[tech-docs.md §UI-Design-Funnel Exemption](./tech-docs.md#ui-design-funnel-exemption).

## Personas

Hats the solo maintainer wears, plus the agents that consume these surfaces.

| Persona                   | Description                                                                                   |
| ------------------------- | --------------------------------------------------------------------------------------------- |
| **Plan author**           | Writes a new plan; needs to know which acceptance-clause forms are traps before writing them  |
| **`plan-maker`**          | Authoring agent; must simulate every shell-bearing acceptance clause before committing it     |
| **`plan-checker`**        | Validating agent; must run cheap deterministic checks first and partition findings by surface |
| **`plan-fixer`**          | Repairing agent; must remediate at class level and verify its own edits empirically           |
| **Workflow orchestrator** | The calling context running the loop; needs a terminable stopping rule                        |
| **Maintainer**            | Reviews the resulting PR; needs latent work to be visible and owned, not silently dropped     |

## User Stories

### US-1 — Trap registry

As a **plan author**, I want a catalogue of known acceptance-criteria traps with runnable proofs,
so that I stop rediscovering the same `grep` and CommonMark pitfalls in every plan.

### US-2 — Deterministic pre-flight

As the **workflow orchestrator**, I want statically-detectable defects caught by a mechanical pass
before the semantic checker runs, so that the expensive lens is not spent re-finding mechanical
defects.

### US-3 — Symmetric verification at authoring time

As a **plan author** using `plan-maker`, I want every shell-bearing acceptance clause empirically
simulated in both directions before it is written, so that clauses do not describe behavior the
executing agent will never observe.

### US-4 — Symmetric verification at fix time

As **`plan-fixer`**, I want my own edits verified by re-running the clause's own command and
re-rendering any touched fence, so that I stop injecting a new defect at the site I just repaired.

### US-5 — Class-level remediation

As **`plan-fixer`**, I want a finding that instantiates a pattern to oblige me to enumerate and fix
every instance of that pattern in one pass, so that the same class does not recur at new sites in the
next three iterations.

### US-6 — Class closure verification

As **`plan-checker`**, I want to verify that a defect **class** is closed rather than that a single
instance is closed, so that a partially-swept class is caught immediately rather than one iteration
later.

### US-7 — Scope discipline

As the **workflow orchestrator**, I want findings partitioned into change-surface and pre-existing
latent, so that the loop converges on a bounded set instead of mining an unbounded document forever.

### US-8 — Latent work stays owned

As the **maintainer**, I want every latent finding fully reported and filed as a follow-up backlog
plan, so that scope discipline never becomes a way to ship content known to be broken.

### US-9 — Budget shaping

As the **workflow orchestrator**, I want the loop to run a deterministic lens, then an in-surface
semantic lens, then a single non-looping latent sweep, so that expensive passes run last and only
once.

## Acceptance Criteria

Every scenario below binds to at least one RED delivery step in [delivery.md](./delivery.md).

### AC-1 — The registry exists and every entry carries a runnable proof

```gherkin
Scenario: Defect-class registry lists each trap with an executable proof
  Given the file repo-governance/development/quality/plan-acceptance-defect-classes.md exists
  When a reader opens any registry entry
  Then the entry states the trap symptom, a runnable proof command, the safe rewrite form, and the detection method
  And running the proof command reproduces the trap
  And running the safe rewrite form under the same conditions does not reproduce it
```

### AC-2 — `grep -c` line-counting trap is catalogued and demonstrated

```gherkin
Scenario: Multi-term alternation threshold undercounts when terms share a line
  Given a fixture file containing three search terms packed onto a single line
  When the clause "grep -Ec 'alpha|beta|gamma' fixture" is evaluated against a threshold of 3
  Then the command returns 1 and the threshold is not met despite every term being present
  And the registry's safe form "grep -ohE 'alpha|beta|gamma' fixture | sort -u | wc -l" returns 3
  And the safe form returns 3 for a fixture with the same terms on separate lines
```

### AC-3 — Absent-file trap is catalogued, including the masking corollary

```gherkin
Scenario: Pre-edit count claims are wrong for files the plan itself creates
  Given a target file that does not yet exist because a later checkbox creates it
  When "grep -Ec 'pattern' target" runs against it
  Then the command prints nothing to stdout and exits 2 rather than printing 0
  And "grep -Ec 'pattern' existing-file" with no match prints 0 and exits 1
  And the registry records that the safe occurrence-unique form also returns 0 for an absent file, so a "test -f" companion check is mandatory
```

### AC-4 — Environment-dependent flags are forbidden in acceptance clauses

```gherkin
Scenario: The -L flag is rejected in an acceptance clause
  Given an acceptance clause using "grep -L" to assert files-without-match
  When the deterministic pre-flight pass evaluates the clause
  Then the pass reports a finding naming the environment-dependence of -L
  And the finding cites the safe per-file substitute using "grep -q" in a loop
```

### AC-5 — Multi-file `grep -c` is rejected

```gherkin
Scenario: A multi-file count clause is flagged as non-comparable
  Given an acceptance clause of the form "grep -c pattern file1 file2" compared against a single numeric threshold
  When the deterministic pre-flight pass evaluates the clause
  Then the pass reports a finding stating that multi-file grep -c emits per-file "filename:count" output
  And the finding notes the output ordering is not guaranteed stable
```

### AC-6 — Indented fences inside list items are detected

```gherkin
Scenario: A fence indented past its list item content column is caught mechanically
  Given a delivery checkbox whose fenced code block is indented six spaces inside a top-level list item
  When the deterministic pre-flight pass evaluates the document
  Then the pass reports a finding stating the block parses as an indented code block rather than a fenced one
  And the finding notes that Prettier reports the broken form as correctly formatted
  And the finding names the correct content-column indentation as the fix
```

### AC-7 — Non-discriminating acceptance clauses are caught

```gherkin
Scenario: An OR term already satisfied by an earlier checkbox is flagged
  Given a checkbox whose acceptance clause ORs two search terms against a target file
  When an earlier checkbox in the same phase already writes one of those terms into that same target file
  Then the checker reports the clause as non-discriminating
  And the report states that the clause would pass even if this checkbox performed no work
```

### AC-8 — Authoring-time simulation is mandatory

```gherkin
Scenario: plan-maker simulates a shell-bearing clause before writing it
  Given plan-maker is authoring an acceptance clause containing a shell command
  When the clause is about to be written into delivery.md
  Then plan-maker has executed the command against the real repo or a fixture in both the pre-edit and post-edit directions
  And the observed output is what the clause claims
  And an unsimulatable clause is either rewritten into a simulatable form or omitted
```

### AC-9 — Fix-time simulation is mandatory

```gherkin
Scenario: plan-fixer verifies its own edit empirically
  Given plan-fixer has applied a fix that rewrites an acceptance clause
  When the fixer performs self-verification
  Then the fixer has re-run the rewritten clause's own command and recorded the observed output
  And any markdown fence touched by the fix has been rendered through a CommonMark parser to confirm it parses as fenced
  And the fix report records the verification as APPLIED (verified) or FAILED (not applied)
```

### AC-10 — Class-level remediation is enforced

```gherkin
Scenario: A pattern-instantiating finding triggers a whole-class sweep
  Given the checker reports a finding that instantiates a registry defect class
  When plan-fixer remediates it
  Then the fixer enumerates every instance of that class across all plan documents in the same pass
  And the fix report lists each enumerated site with its disposition
  And no instance of that class remains unaddressed in the plan
```

### AC-11 — Class closure is verified, not instance closure

```gherkin
Scenario: A partially swept class is caught in the same iteration
  Given plan-fixer reports a class sweep for a registry defect class
  When plan-checker re-validates
  Then the checker re-runs the class enumeration independently rather than checking only the originally flagged site
  And a residual unswept instance is reported as a class-closure failure
```

### AC-12 — Findings are partitioned by surface

```gherkin
Scenario: Checker labels every finding in-surface or latent
  Given a re-validation iteration with a fix report listing changed files
  When plan-checker reports findings
  Then every finding carries an explicit in-surface or latent label
  And each latent label cites the provenance evidence establishing the content predates this chain
  And any CRITICAL finding is treated as in-surface regardless of provenance
```

### AC-13 — Termination is evaluated on the bounded set

```gherkin
Scenario: The loop terminates on in-surface findings while latent work remains
  Given two consecutive validations report zero in-surface threshold-level findings
  When the workflow evaluates termination
  Then the workflow reports pass only if a follow-up backlog plan capturing every latent finding exists on disk
  And the final report lists every latent finding in full
  And the absence of the follow-up plan blocks the pass status
```

### AC-14 — Scope discipline cannot hide a known-broken checkbox

```gherkin
Scenario: A latent defect inside an executable checkbox is promoted
  Given a latent finding located inside a delivery checkbox that this plan's execution will act on
  When plan-checker classifies the finding
  Then the finding is promoted to in-surface and must be fixed before termination
  And the promotion rationale is recorded in the audit report
```

### AC-15 — Budget shaping runs cheap lenses first

```gherkin
Scenario: The deterministic pass precedes the semantic pass
  Given the plan-quality-gate workflow starts against a plan folder
  When the first iteration runs
  Then the deterministic pre-flight pass executes before plan-checker's semantic validation steps
  And a non-zero deterministic result is remediated before the semantic lens is spent
```

### AC-16 — No existing check was removed

```gherkin
Scenario: The validation step inventory does not shrink
  Given the pre-change inventory of plan-checker validation steps recorded in Phase 0
  When the post-change inventory is taken
  Then every pre-change validation step is still present
  And the post-change step count is greater than or equal to the pre-change count
```

### AC-17 — Bindings stay generated, never hand-edited

```gherkin
Scenario: Secondary harness bindings are regenerated from the primary binding
  Given the .claude/ agent definitions have been modified
  When npm run generate:bindings runs
  Then the .opencode/ and .amazonq/ artifacts reflect the .claude/ changes
  And the harness sync validation reports no drift
```

### AC-18 — Tri-repo propagation preserves rhino-cli byte identity

```gherkin
Scenario: The validator lands byte-identical across all three repositories
  Given the new validator has landed in ose-public
  When the change is propagated to ose-primer and ose-infra
  Then apps/rhino-cli is byte-identical across all three repositories
  And the Gherkin behavior tree under the rhino specs path is byte-identical across all three
```

## Product Scope

### In scope

- The defect-class registry as a governance convention document
- A deterministic pre-flight validator with a Gherkin behavior tree
- Authoring-time and fix-time empirical simulation requirements
- Class-level remediation and class-closure verification contracts
- In-surface / latent partition with anti-loophole guards
- Workflow step-model, termination-criteria and convergence-target rewrite
- Binding regeneration and tri-repo propagation

### Out of scope

- Editing the audited plan that supplied the evidence
- The PR-review quality gate (open question Q5)
- Any relaxation of a check, threshold or criticality level
- Retroactive sweeps of existing `plans/in-progress/` plans (open question Q6)

## Product Risks

| Risk                                                               | Severity | Handling                                                                                     |
| ------------------------------------------------------------------ | -------- | -------------------------------------------------------------------------------------------- |
| Validator false positives add noise                                | Medium   | Existing FALSE_POSITIVE skip-list machinery applies; validator findings are checker-mediated |
| Latent classification is abused                                    | High     | Four guards in tech-docs DD-5; provenance must be cited; CRITICAL never exempt               |
| Agent files grow past the instruction-size budget                  | Medium   | Registry content lives in the governance file; agents link rather than inline                |
| The registry's proof commands rot as tooling changes               | Low      | Proofs are re-run by the registry's own delivery gate                                        |
| Tri-repo propagation partially applied, leaving repos inconsistent | Medium   | Per-repo phases with their own gates; byte-identity check is a gate item                     |
