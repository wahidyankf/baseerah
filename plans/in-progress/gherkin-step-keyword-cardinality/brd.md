# Business Requirements — Gherkin Step-Keyword Cardinality Rule

## Business Goal

Make the canonical Gherkin scenario shape an **explicit, enforceable rule** instead of
an implicit convention demonstrated only by example, so that every scenario authored by
a human or an AI agent uses exactly one primary `Given`, one `When`, and one `Then`,
with all extras chained via `And` / `But`.

## Business Rationale (WHY)

- The repo already treats "one action / one behavior per scenario" as a norm (see the
  Best Practices and "Multiple Behaviors in One Scenario" anti-pattern sections of
  [`acceptance-criteria.md`](../../../repo-governance/development/infra/acceptance-criteria.md))
  [Repo-grounded], but it never states the keyword-cardinality form of that norm as a
  rule. A norm shown only by example is silently violable. [Judgment call]
- Multiple primary `When`/`Then` lines in one scenario create ambiguity in the
  BDD-to-test mapping ([`bdd-spec-test-mapping.md`](../../../repo-governance/development/infra/bdd-spec-test-mapping.md))
  [Repo-grounded] — it becomes unclear which action a step definition binds to.
- An explicit rule plus a **deterministic linter** removes interpretation from both AI
  agents and human contributors, which is consistent with the repo's
  "Explicit Over Implicit" and "Automation Over Manual" principles. [Repo-grounded]

## Business Impact

**Pain points addressed**:

- Inconsistent scenario structure across 124 `specs/**/*.feature` files [Repo-grounded].
- AI agents (plan-maker, content makers) can emit non-conforming Gherkin because no rule
  forbids it. [Judgment call]
- Reviewers must catch cardinality drift by eye, with no automated gate. [Judgment call]

**Expected benefits**:

- One unambiguous, machine-checked scenario shape repo-wide.
- Reduced reviewer burden — the linter and CI catch violations deterministically.
- Sharper BDD-to-test mapping (one action per scenario → one clear binding).

## Affected Roles

This is a solo-maintainer repository; "roles" denote the hats the maintainer wears and
the agents that consume the governance surface. No sign-off ceremonies apply.

- **Governance author hat** — authors the rule and runs the broad sweep (delegated to
  `repo-rules-maker`).
- **Tooling/Rust hat** — builds the deterministic linter (delegated to `swe-rust-dev`).
- **Spec maintainer hats** — retrofit per-app `.feature` files + step defs.
- **Consuming agents** — `plan-maker`, `plan-checker`, `repo-rules-checker`, the two
  affected skill packages, and any content maker that emits Gherkin.

## Business-Level Success Metrics

- **Rule presence**: the HARD rule text appears verbatim in
  `acceptance-criteria.md` and is referenced by every Gherkin-touching governance doc and
  agent prompt that the sweep covers. [Judgment call — verified observationally at execution]
- **Enforcement**: the new `gherkin-keyword-cardinality` audit category exists, has
  passing Rust unit/cucumber tests, and is wired into the preflight + CI. [Judgment call]
- **Zero offenders**: after retrofit, the linter reports **zero** violations across
  `specs/**/*.feature`. [Judgment call — observable via linter exit code]
- **Double-zero gate**: `repo-rules-quality-gate` (strict) terminates with `pass`.
  [Judgment call — observable via workflow status]

No fabricated numeric KPIs are claimed; all metrics above are observable checks performed
at execution time.

## Business-Scope Non-Goals

- Not changing what scenarios _test_, only how their keyword lines are structured.
- Not introducing a new test framework or BDD harness.
- Not deferring offenders — violating `.feature` files are fixed in this plan.
- Not adding vendor-specific content to any `repo-governance/` file.

## Business Risks and Mitigations

| Risk                                                                               | Likelihood | Mitigation                                                                                                                                                                         |
| ---------------------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Retrofit edits silently break a project's step-definition bindings                 | Medium     | Per-app phased delivery; each phase gate runs that project's `test:unit`/`test:quick` + `spec-coverage validate` before proceeding. [Repo-grounded — gates defined in delivery.md] |
| Linter false positives on `Background` / `Scenario Outline`                        | Medium     | Exemptions are part of the rule spec and covered by dedicated RED tests before GREEN.                                                                                              |
| Governance sweep misses a Gherkin-referencing surface                              | Low        | `repo-rules-quality-gate` (strict) double-zero pass validates repo-wide consistency after the sweep.                                                                               |
| Broad sweep introduces inconsistency between with/without `repo-rules-maker` edits | Low        | Distinct phases with explicit file lists; final strict gate cross-checks.                                                                                                          |
