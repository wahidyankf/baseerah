# Doc Command Existence Validation

> **Status**: Backlog — design decisions settled (see
> [Design Decisions](#design-decisions)); ready for execution.
>
> **Naming note**: this folder intentionally omits the `YYYY-MM-DD__` date prefix that the current
> [Plans Organization Convention](../../../repo-governance/conventions/structure/plans.md) still
> documents for `backlog/`. The maintainer has directed that date prefixes be dropped from
> `backlog/` and `in-progress/` (kept only in `done/`); the convention text update is tracked as
> separate work. Do not "correct" this folder name back to a dated form.

A new `rhino-cli` validator — `md commands validate` — that mechanically detects
**documentation-cited commands that do not exist**, closing a verified drift gap that produced
three independent defects in a single working session.

## Context

The repository already treats several classes of documentation claim as machine-checkable:
internal links (`md links validate`), heading hierarchy (`md heading-hierarchy validate`), file
naming (`md naming validate`), README index completeness (`md readme-index validate`), and Gherkin
keyword cardinality (`specs gherkin-cardinality validate`). [Repo-grounded]

**A command cited in a doc is the same shape of claim** — an assertion about repository reality
that can be verified against an authoritative oracle. It is currently the gap.

### The motivating incident (verified)

Three independent surfaces cited `rhino-cli` Nx targets that do not exist:

| #   | Surface                                                                          | Cited (nonexistent)                                                                                     | Status        |
| --- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- | ------------- |
| 1   | `AGENTS.md:63-64`                                                                | `rhino-cli:mermaid:validation`, `rhino-cli:links:validation`, `rhino-cli:headings:hierarchy-validation` | Fixed         |
| 2   | `plans/in-progress/parallel-orchestration-shared-machine-governance/delivery.md` | the above three plus `rhino-cli:validate:sync`, `rhino-cli:vendor-audit`                                | Fixed         |
| 3   | `repo-governance/development/infra/nx-targets.md` (~L146-148)                    | the above three, presented as canonical                                                                 | **Not fixed** |

Ground truth: `npx nx show project rhino-cli --json` resolves **21 targets**, none matching.
[Repo-grounded — verified this session]

The real mechanism is raw cargo invocation, wired into `.husky/pre-commit`, `.husky/pre-push`, and
the `markdown-per-file` job in `.github/workflows/main-ci.yml`: [Repo-grounded]

```bash
cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate
```

### Drift is broader than the three cited targets

Surveying `repo-governance/development/infra/nx-targets.md` during planning surfaced that its
"Canonical governance and validation targets (defined on `rhino-cli`)" table lists **six**
targets absent from the resolved graph, not three: [Repo-grounded]

- `specs:domain:coverage`
- `links:validation`
- `mermaid:validation`
- `headings:hierarchy-validation`
- `cross-vendor:parity-validation`
- `harness:bindings-validation`

That table is framed as a canonical claim but functions as an aspirational roadmap. **Phase 3
deletes all six rows outright** (maintainer decision, Q7 — see
[Design Decisions](#design-decisions)), leaving the table asserting only targets verified
to resolve against the live graph. No replacement "planned targets" table is created: a canonical
reference doc asserts what exists, and roadmap intent belongs somewhere that is not load-bearing for
execution. The six removed names, and the fact that none was ever implemented, are preserved in
[learnings.md](./learnings.md) so the intent survives the deletion.

Defect #2 is the sharpest evidence of harm: those citations were _verbatim executable gate
acceptance criteria_. Two independent `plan-checker` runs flagged them CRITICAL. Left unfixed,
an executor would have stalled mid-gate running a command that cannot succeed.

## Scope

### In scope

- New `rhino-cli` subcommand `md commands validate` (see
  [Design Decisions](#design-decisions)) scanning tracked markdown.
- Three detector families, each with an authoritative in-repo oracle:
  - **Nx targets** — `nx run <project>:<target>`, `npx nx run …`, `nx run-many -t <target>`,
    validated against the **resolved** project graph (inferred targets included, not just
    literal `project.json` entries).
  - **npm scripts** — `npm run <script>` validated against the relevant `package.json`.
  - **rhino-cli subcommands** — `cargo run … -- <chain>` and bare `rhino-cli <chain>`, validated
    against rhino-cli's own **clap command tree**, introspected at runtime.
- A deliberate two-tier exemption mechanism (inline per-occurrence annotation + config path
  allowlist).
- Conservative-by-default detection with an opt-in `--strict` mode.
- Wiring into `.husky/pre-push` and the `markdown-per-file` CI job, in all three repos.
- Remediation of the known existing violations so the validator lands green.
- Byte-identical propagation to `ose-primer` and `ose-infra`.

### Out of scope

- Shell script (`./scripts/*.sh`) and `make` target citations — deferred; these are the
  highest-false-positive surface and would jeopardize adoption. Revisit once the three core
  detectors have proven their precision in production.
- Validating command _arguments and flags_ (e.g. that `--exclude` is a real flag). Existence
  only; flag validation is a materially harder problem with a much weaker oracle.
- Validating commands for other repositories, external tools (`git`, `docker`, `jq`), or
  installed binaries.
- Auto-fixing violations. The validator reports; a human or agent decides whether the doc or
  the tooling is wrong.

## Approach summary

A functional-core / imperative-shell Rust command in the existing `md <subject> validate` family.
The core is a pure function `(markdown corpus, capability snapshot) -> findings`; the shell
gathers the capability snapshot from three sources and walks the file tree.

The precision-first stance is deliberate and load-bearing: **a noisy validator gets disabled, and
a disabled validator has negative value** — it consumes maintenance while providing false
assurance. Default mode detects only high-confidence citations; `--strict` opts into the wider
net.

## Navigation

- [brd.md](./brd.md) — business rationale, impact, risks
- [prd.md](./prd.md) — personas, user stories, Gherkin acceptance criteria
- [tech-docs.md](./tech-docs.md) — architecture, design decisions, false-positive strategy
- [delivery.md](./delivery.md) — phased delivery checklist
- [learnings.md](./learnings.md) — Knowledge Capture running log

## Design Decisions

All decisions below are **settled**. Q2, Q3, and Q7 were resolved by maintainer grill answers; the
rest were authored against the recommended option and stand unless revisited.

| #   | Decision                    | Settled answer                                                         | Source                                 |
| --- | --------------------------- | ---------------------------------------------------------------------- | -------------------------------------- |
| 1   | CLI shape                   | `md commands validate`                                                 | Recommended, unchallenged              |
| 2   | Detector scope              | Nx targets + npm scripts + rhino-cli subcommands (shell/make deferred) | **Grill answer — as recommended**      |
| 3   | Hook placement              | `pre-push` + CI `markdown-per-file`                                    | **Grill answer — as recommended**      |
| 4   | Exemption mechanism         | Inline annotation **and** config path allowlist                        | Recommended, re-justified post-Q7      |
| 5   | Precision/recall            | Conservative default, opt-in `--strict`                                | Recommended, unchallenged              |
| 6   | Nx oracle                   | `nx show projects --json` snapshot via subprocess, cached per run      | Recommended, unchallenged              |
| 7   | `nx-targets.md` remediation | **Delete the six nonexistent rows outright; no planned-targets table** | **Grill answer — maintainer override** |

**Q7 is a maintainer override** of the recommended split-into-exists/planned approach. The
recommendation was to preserve roadmap intent inside `nx-targets.md` via an explicitly-labelled
"Planned targets" table; the maintainer chose outright deletion instead, so the canonical doc
asserts only what exists. Roadmap intent is preserved out-of-band in
[learnings.md](./learnings.md). See [tech-docs.md](./tech-docs.md) DD-6.

**Q7 does not weaken Q4.** The exemption mechanism was originally justified in large part by the
`nx-targets.md` case; that case is now gone. The two-tier design is re-justified on the remaining
surfaces independently in [tech-docs.md](./tech-docs.md) DD-5 — frozen `plans/done/` trees,
deliberately-malformed test fixtures, illustrative fences, and other-repo commands all still
require an escape hatch.
