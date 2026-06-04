---
name: repo-dependency-bump-planning
title: "repo-dependency-bump-planning"
goal: >
  Survey every dependency manifest across apps/ and libs/ (and workspace-root language pins),
  classify each candidate bump per the Dependency Bump Stability & Safety Policy, and produce a
  validated backlog plan that — when later executed — updates those dependencies. The deliverable
  is the plan, never the dependency edits.
termination: >
  A grill-validated plan exists at plans/backlog/<YYYY-MM-DD>__<identifier>/, passes
  plan-quality-gate at strict mode, and a dependency clearance report is written to
  generated-reports/. No dependency manifest or lockfile is modified by this workflow.
inputs:
  - name: scope-filter
    type: string
    description: >
      Optional comma-separated glob filter limiting which projects/manifests are inventoried.
      Default is "all manifests under apps/ and libs/, plus the workspace-root language pins
      (root package.json volta block)".
    required: false
  - name: ecosystems
    type: string
    description: >
      Optional comma-separated filter of ecosystems to consider (npm, cargo, dotnet, go, docker).
      Default is all ecosystems present in the inventory.
    required: false
  - name: as-of-date
    type: string
    description: >
      The "today" used for the Path B 60-day cutoff computation (YYYY-MM-DD). Defaults to the
      current date. Recorded verbatim in the clearance report for auditability.
    required: false
  - name: plan-identifier
    type: string
    description: "Slug for the backlog plan folder. Default: dependency-bump."
    required: false
    default: dependency-bump
  - name: push-target
    type: string
    description: "Git push destination for the backlog plan. Forwarded to plan-establishment-execution."
    required: false
    default: "origin main"
outputs:
  - name: clearance-report
    type: file
    pattern: generated-reports/repo-dependency-bump-planning__*__report.md
    description: Inventory + Security & Functional Clearance Status table + cutoff computation. Always written.
  - name: plan-path
    type: string
    description: Path to the created backlog plan in plans/backlog/<YYYY-MM-DD>__<identifier>/
  - name: final-status
    type: enum
    values: [pass, partial, fail]
    description: Final status after the backlog plan's quality gate
---

# Repository Dependency Bump Planning Workflow

**Purpose**: Turn the [Dependency Bump Stability & Safety Policy](../../development/workflow/dependency-bump-policy.md)
into a concrete, validated **backlog plan** for updating dependencies across all of `apps/` and
`libs/`. This workflow performs the policy's survey-and-classify work (Application Workflow steps
1–7: inventory → path classification → recency → functional stability → clearance) and hands the
results to `plan-establishment-execution` to author the plan.

> **The outcome is the plan, not the implementation.** This workflow never edits a manifest,
> never updates a lockfile, and never runs a bump. It produces a proposal in `plans/backlog/`. The
> actual edits happen later, only after a human promotes the backlog plan to `plans/in-progress/`
> and runs the [Plan Execution workflow](../plan/plan-execution.md). The policy's Application
> Workflow steps 8–12 (pin, lockfile, re-audit, document, quality gates) become the plan's
> delivery checklist — they are executed then, not now.

This is a `planning`-type workflow: a single forward procedure whose terminal deliverable is a
plan document. It is **not** an iterative quality gate.

## Execution Mode

**Direct Orchestration** — the calling context (top-level assistant session) orchestrates the
phases, delegating external version/CVE/yank research to `web-research-maker` via the Agent tool,
running the human checkpoint inline (so the user's conversation is preserved), and invoking the
[plan-establishment-execution workflow](../plan/plan-establishment-execution.md) for plan
authoring.

## When to use

- Periodic dependency-hygiene sweep across the monorepo (e.g., a scheduled maintenance cadence).
- Before a release, to capture a snapshot proposal of all eligible bumps for later scheduling.
- When a runtime/language LTS line advances and you want a planned, policy-compliant upgrade.

## Phases

### 0. Pre-flight (Sequential)

**Actions**:

- Confirm the `ose-public` working tree is clean (`git status --porcelain` empty).
- Resolve `as-of-date` (input, else current date). Compute and record the Path B cutoff:
  `cutoff = as-of-date − 60 days`. This is written verbatim into the clearance report per the
  policy's [Cutoff Date Computation](../../development/workflow/dependency-bump-policy.md) section.
- Resolve `scope-filter` and `ecosystems`. Default scope = manifests under `apps/` and `libs/`
  plus the workspace-root language pins.

**Output**: Cutoff date computed. Scope resolved.

**On failure**: If the tree is dirty, abort and ask the user to commit/stash first.

### 1. Inventory (Sequential)

Enumerate every in-scope dependency manifest and capture its currently-pinned versions. Manifests
governed by the policy (intersected with `scope-filter`/`ecosystems`):

- **npm**: workspace-root `package.json` (`volta` block = Node/npm language pins; `dependencies`,
  `devDependencies`, `optionalDependencies`), plus `apps/*/package.json` and `libs/*/package.json`.
- **Cargo**: `apps/*/Cargo.toml` and `libs/*/Cargo.toml` `[dependencies]` (e.g. `organiclever-be`,
  `ose-app-be`, `rhino-cli`, `rust-commons`).
- **.NET**: `apps/*/global.json` `sdk.version` and `*.fsproj`/`*.csproj` `<PackageReference>`
  (e.g. `crane-cli`).
- **Go**: `apps/*/go.mod` Go version + module requirements (e.g. `ayokoding-cli`, `ose-cli`).
- **Docker**: `apps/*/Dockerfile` `FROM` base-image tags.

Use the `nx-workspace` skill / `nx graph` to enumerate projects, then `Grep`/`Glob` for the
manifests. Record a table: project → ecosystem → package → current pinned version.

**Output**: Full inventory of in-scope dependencies with current versions.

**Note**: GitHub Actions `uses:` pins live in `.github/` (repo-wide, outside `apps/`/`libs/`) and
are out of this workflow's default scope; include them only if `scope-filter` explicitly selects
`.github/`.

### 2. Candidate Discovery & Classification (Parallel, delegated)

For each dependency/runtime, determine its policy path and the version to propose. Delegate the
external research to `web-research-maker` — the [default primitive for public-web information
gathering](../../conventions/writing/web-research-delegation.md). **Group research by ecosystem**
(one agent per ecosystem batch) rather than one agent per package, and cap concurrent agents at
**3** per the [Subagent Orchestration Convention](../../development/agents/subagent-orchestration.md).

Each research batch must return, per package:

- Latest version and its release date; whether an LTS line exists (→ **Path A**) and the latest
  LTS patch.
- For non-LTS packages, the latest version released on or before the **cutoff** (→ **Path B**).
- CVE status across all four policy sources (NVD, GitHub Security Advisories, Snyk DB, vendor
  security page). If no version satisfies both the 60-day rule and CVE-cleanness → **Path C**.
- **Rule 5a (recency)**: the most recent eligible version for the chosen path.
- **Rule 5b (functional stability)**: whether the chosen version is yanked/deprecated, carries an
  open release-blocker, or has a widely-reported fatal functional bug — and if so, the most recent
  eligible version that passes.

**Agent**: `web-research-maker` (one invocation per ecosystem batch).

**Output**: Per-package classification: path (A/B/C), proposed target version, CVE status, Rule 5b
status.

### 3. Clearance Table & Decisions (Sequential)

Assemble the results into the policy's **Security & Functional Clearance Status** for every
package, using one of: `CLEAR`, `CLEAR (patch-of)`, `WAIVER`, `FUNCTIONAL-HOLD` (per the policy).
Build the proposed bump table (project → package → current → proposed → path → clearance) and
record the cutoff computation from Phase 0.

Write all of this progressively to
`generated-reports/repo-dependency-bump-planning__<uuid>__<YYYY-MM-DD--HH-MM>__report.md`
(the `clearance-report` output) per the [Temporary Files convention](../../development/infra/temporary-files.md).

**Output**: `clearance-report` written. Bump table + clearance statuses finalized.

### 4. Human Checkpoint (Sequential, Hard Gate)

Present the proposed bump table, the clearance statuses, and — prominently — any `WAIVER` or
`FUNCTIONAL-HOLD` rows. Use `AskUserQuestion` to:

1. Confirm the plan identifier (default `dependency-bump`).
2. Confirm the scope is correct (any packages to exclude/hold).
3. Explicitly approve proceeding to plan authoring.

**Do NOT proceed to Phase 5** until the user approves. The user may trim scope or defer specific
bumps here.

**Output**: Approved bump set + confirmed identifier.

### 5. Backlog Plan Establishment (Sequential)

Invoke the [plan-establishment-execution workflow](../plan/plan-establishment-execution.md) with:

- **Input** `target-stage`: `backlog` (lands at `plans/backlog/<YYYY-MM-DD>__<identifier>/`).
- **Input** `push-target`: forwarded from this workflow's input.
- **Input** `prompt`: a self-contained handoff containing the full inventory, the approved bump
  table, the Security & Functional Clearance Status, the recorded cutoff date, a link to the
  `clearance-report`, and this **Definition of Done** for the plan it must author:
  - Every in-scope manifest is pinned (exact, no `^`/`~`) to its approved target version.
  - Lockfiles regenerated (`npm install`, `cargo update -p`, `go mod tidy`, etc.).
  - Post-bump re-audit clean (`npm audit --audit-level=moderate`, `govulncheck ./...`).
  - All `WAIVER`/`FUNCTIONAL-HOLD` entries propagated to `docs/reference/security-waivers.md`.
  - Affected-project quality gates pass (typecheck, lint, test:quick, spec-coverage).
  - The delivery checklist mirrors the policy's [Application Workflow](../../development/workflow/dependency-bump-policy.md)
    steps 8–12, grouped per ecosystem, TDD-shaped where code changes are required.

Because `plan-establishment-execution` runs its own grill + (optional) research + `plan-maker` +
`plan-quality-gate` + push, this phase yields a strict-gate-passing backlog plan.

**Output**: `plan-path`, `final-status`, `final-report` (from the nested quality gate).

### 6. Hand-back (Sequential)

Emit a user-visible summary: `plan-path`, `clearance-report` path, `final-status`, and a reminder
that **the plan is a snapshot as of the cutoff date**. Per the policy's
[When the Plan Spans Many Days](../../development/workflow/dependency-bump-policy.md) section, if
promotion to `in-progress/` is delayed, the eligibility check must be re-run before execution to
catch newly-eligible versions or newly-disclosed CVEs.

## Gherkin Success Criteria

```gherkin
Feature: repository dependency bump planning

Scenario: Planning sweep produces a backlog plan without touching manifests
  Given the ose-public working tree is clean
  When the workflow runs to completion
  Then a clearance report appears under generated-reports/repo-dependency-bump-planning__*__report.md
  And a plan exists at plans/backlog/<YYYY-MM-DD>__dependency-bump/
  And the backlog plan passes plan-quality-gate at strict mode
  And no package.json, Cargo.toml, go.mod, *.fsproj, Dockerfile, or lockfile is modified

Scenario: Functional-hold is surfaced before authoring
  Given a candidate version is yanked or carries an open release-blocker
  When the workflow classifies that package
  Then the clearance report records it as FUNCTIONAL-HOLD with the skipped and chosen versions
  And the human checkpoint presents the FUNCTIONAL-HOLD before plan authoring

Scenario: User declines at the checkpoint
  Given the proposed bump table is presented
  When the user does not approve
  Then no plan is authored
  And the workflow terminates with the clearance report written
```

## Related Documents

- [Dependency Bump Stability & Safety Policy](../../development/workflow/dependency-bump-policy.md) — the authority this workflow operationalizes (three-path tree, Rule 5a/5b, clearance statuses).
- [plan-establishment-execution workflow](../plan/plan-establishment-execution.md) — invoked in Phase 5 with `target-stage=backlog`.
- [Plan Execution workflow](../plan/plan-execution.md) — runs the plan later, after promotion to `in-progress/`.
- [web-research-maker Agent](../../../.claude/agents/web-research-maker.md) — Phase 2 version/CVE/yank research.
- [security-waivers register](../../../docs/reference/security-waivers.md) — destination for WAIVER / FUNCTIONAL-HOLD entries.

## Principles Implemented/Respected

- **[Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md)**: Classification and clearance precede any proposal; the human checkpoint forces an explicit go/no-go.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Cutoff date, path classification, and clearance status are recorded in writing before the plan is authored.
- **[Reproducibility First](../../principles/software-engineering/reproducibility.md)**: The resulting plan mandates exact pins and lockfile regeneration.
- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: Inventory, research, and clearance assembly are delegated and report-driven.
- **[No Time Estimates](../../principles/content/no-time-estimates.md)**: Outcomes, not durations.

## Conventions Implemented/Respected

- **[Workflow Naming Convention](../../conventions/structure/workflow-naming.md)**: Basename `repo-dependency-bump-planning` parses as scope=`repo`, qualifier=`dependency-bump`, type=`planning`.
- **[Plans Organization Convention](../../conventions/structure/plans.md)**: The backlog plan uses the `YYYY-MM-DD__<identifier>/` creation-date-prefixed folder form.
- **[Web Research Delegation Convention](../../conventions/writing/web-research-delegation.md)**: Version/CVE/yank research delegated to `web-research-maker`.
- **[Subagent Orchestration Convention](../../development/agents/subagent-orchestration.md)**: Research agents capped at 3 concurrent.
- **[Linking Convention](../../conventions/formatting/linking.md)**: Cross-references use GitHub-compatible markdown with `.md` extensions.

```

```
