# BRD — Unify rhino-cli, SDLC & Repo Structure (Second Pass)

## Business Goal

Close the gap between the first plan's _claimed_ `"identical"` end-state and the _actual_ divergence
across `ose-public`, `ose-primer`, and `ose-infra`, so that the entire repo structure — the SDLC
machinery **and the `rhino-cli` tool itself** — is genuinely `"identical"`. A contributor (human or
AI agent) who learns any surface in one repo applies that knowledge unchanged in the other two, and
`ose-public` (the upstream source of truth) propagates to the siblings with zero per-repo translation.

## Why It Matters

- **The first pass under-delivered on its headline.** The archived plan reported `"identical"`, but a
  fresh audit shows rhino-cli is three different codebases (public 155 / primer 231 / infra 235 src
  files; infra differs in 100 of ~155 files) and ose-infra's hooks/CI use a different invocation
  mechanism throughout. Green CI in three repos still means three different things. [Repo-grounded —
  see [tech-docs §2](./tech-docs.md#2-current-state-verified-2026-07-02)]
- **Stale "done" notes erode trust in the plan record.** delivery.md items marked done (e.g.
  rhino-cli command-set "identical") do not match reality. This pass re-audits against the working
  tree and treats the code, not the notes, as ground truth. A second, deeper sweep even corrected this
  plan's own first-draft current-state (the cucumber `.feature` trees diverge structurally, not just
  in count; primer is on the _older_ cucumber `0.22.1`), reinforcing the code-over-notes discipline.
  [Repo-grounded]
- **rhino-cli drift is the highest-leverage divergence.** It is the one tool every gate in every repo
  invokes. If its source, its `Cargo.lock`, and its `project.json` commands differ, every downstream
  gate can behave differently even when the wiring looks the same. Making the tool byte-identical
  makes every gate that calls it identical by construction. [Judgment call]
- **Cucumber coverage is asymmetric.** primer has a wired BDD harness for rhino-cli's own behaviour
  (11 `[[test]]` blocks); public and infra declare the dependency but don't register the harness. Yet
  the `.feature` surface is _more_ divergent than that: public carries `ddd`/`specs` feature dirs
  unique to it (public and primer share `workflows`, which only infra lacks), while primer/infra
  additionally carry `contracts`/`java`/`test-coverage` dirs public lacks — so the canonical BDD
  surface is the reconciled union of all three, and the tool that
  enforces spec coverage everywhere is itself only harness-wired in one of three repos. [Repo-grounded]
- **Parity-loop cost.** The [multi-repo parity workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)
  cannot be near-mechanical while the three rhino-cli codebases are structurally different. Byte
  identity is what makes propagation a copy, not a translation. [Repo-grounded]

## Affected Roles

- **Repo maintainer / solo operator** — edits all three repos; primary beneficiary of a single mental
  model that now extends into the tool's own source.
- **AI coding agents** (`ci-checker`, `repo-harness-compatibility-checker`, `swe-rust-dev`) — validate
  and edit this surface; benefit from one canonical rhino-cli to reason about.
- **Contributors to `ose-primer`** (downstream template consumers) — inherit a coherent, identical
  tool + gate model.

## Business-Level Success Metrics

- **rhino-cli byte-identity (new north star)**: `diff -rq apps/rhino-cli/src` between any two repos is
  empty; `Cargo.toml`, `Cargo.lock`, `project.json`, and `LICENSE` are 100% byte-identical with **zero
  carve-outs** (infra relicensed to MIT with an `apps/rhino-cli/`-scoped MIT `LICENSE`; env-validation
  scan paths data-driven from `repo-config.yml`), carrying the full command superset in every repo.
  Observable check: Phase 5's `diff -rq` + `diff` matrix shows no differences at all. [Repo-grounded]
- **Cucumber parity**: the same wired cucumber-rs harness (migrated to `0.23.0`) + the same **reconciled
  union** `.feature` tree for rhino-cli's own behaviour exist and pass in all three repos. Observable
  check: `cargo test` runs the cucumber suites in each repo; the `tests/*.rs` set and the
  `specs/apps/rhino/behavior/rhino-cli/gherkin` tree are identical across repos. [Repo-grounded]
- **Round-trip integrity**: primer ⊇ current-primer after the primer→public→primer round trip.
  Observable check: the Phase-0 primer behavior baseline (cucumber + golden + CLI output) passes
  against canonical-primer at the Phase 3 gate, and the file-accounting ledger accounts for every
  current-primer file as ported/merged/explicitly-dropped. [Repo-grounded]
- **Config schema parity**: all three `repo-config.yml` files carry an identical key set (values may
  differ), enforced by a new `rhino-cli repo-config validate` command — a strict-deserialize check
  against the byte-identical canonical schema — run at each repo's **pre-commit** (fast path, fires
  only when `repo-config.yml` is staged) and pre-push/PR (defense-in-depth), so the byte-identical
  source cannot break at runtime on a missing or misspelled key. Observable check: `rhino-cli
repo-config validate` exits 0 in all three repos, and rejects a deliberately corrupted
  `repo-config.yml` at commit time. [Repo-grounded]
- **SDLC mechanism parity (zero `⚠️`)**: every gate is invoked through the identical mechanism in all
  three repos — no repo using `npx nx run rhino-cli:*` where another uses direct `cargo run`, no
  inline tool-lint where another uses lint-staged. Observable check: the Phase 5 parity table shows ✅
  on every mechanics row with **no `⚠️` rows remaining**. [Repo-grounded]
- **Full `namedInputs.specs` coverage**: every Nx-registered project in every repo — enumerated via
  `nx show projects`, which includes the `*-contracts` projects rooted under
  `specs/apps/*/containers/contracts/` (invisible to a directory-only `apps`/`libs` scan) — wires the
  specs input. Observable check: the count of projects with `namedInputs.specs` equals the total
  `nx show projects` count in each repo (currently 16/29, 20/26, 6/8). [Repo-grounded]
- **Complete mandatory-target coverage**: no project missing any mandatory target in any repo
  (currently 6 infra projects have gaps, including the `coralpolyp-contracts` project which — like
  the other `*-contracts` projects — sits outside `apps`/`libs`). Observable check: the
  mandatory-target `jq` loop (enumerated via `nx show projects`) prints no `MISSING` in any repo.
  [Repo-grounded]
- **Latent bugs fixed**: the agent-naming validator fires (trigger path `.opencode/agents/`), and the
  PR gate runs `gherkin-cardinality` in public — both dry-run in Phase 0 before arming. Observable
  check: a renamed agent file trips the naming validator; the public PR-gate specs job lists
  `gherkin-cardinality`. [Repo-grounded]
- **Reality-grounded record**: Phase 0 produces committed audit evidence; every delivery item cites a
  concrete verification (diff, jq, grep), not a prior "done" note. [Judgment call]
- **One plan, three repos**: this single plan executes end-to-end across all three repos (Phases
  2/3/4) with per-repo granular steps. [Repo-grounded]
- **Zero regressions**: after convergence each repo's pre-push and PR quality gate pass on a no-op
  change. [Repo-grounded]

## Business-Scope Non-Goals

- Not changing **what** any validator checks (no new lint rules, thresholds, or validator logic) —
  the one new gate is the `repo-config.yml` schema-parity check, which enforces an existing invariant
  (identical key sets) rather than adding a new rule surface.
- Not unifying the **app set** or **language set** across repos.
- Not building **new automated parity-enforcement tooling** for rhino-cli byte-identity — mission is
  verify-&-closeout, not a drift-guard. A parity-check command remains a possible future follow-up.

## Business Risks

- **Risk: the infra rhino-cli port is large and could destabilize a working repo.** Mitigation: it is
  isolated as gated **Phase 4** behind a full pre-push + PR-gate verification. It is **required, not
  descopable** (Decision 7) — if large, it is still completed, and Phases 1–3 stand on their own and
  are never unwound.
- **Risk: byte-identity conflicts with genuinely repo-specific bits.** Mitigation: every such bit
  (env-validation scan paths, domain/ddd areas) is driven from `repo-config.yml` data (with a
  schema-parity gate guaranteeing the key set is shared), and infra's CLI is relicensed to MIT with an
  `apps/rhino-cli/`-scoped `LICENSE` — leaving `apps/rhino-cli` with zero carve-outs. The only
  divergence anywhere is app/language set and the CI runner label.
- **Risk: pulling primer's advances back into public regresses public — or the round trip regresses
  primer.** Mitigation: the synthesis lands behind rhino-cli's own unit + cucumber suites (Phase 1
  wires the union cucumber suite into public) and the regenerated golden-master test; the primer
  round-trip is guarded by the Phase-0 behavior baseline + the file-accounting ledger (Decision 8).
- **Risk: "done" notes mislead again.** Mitigation: every item is verified against the working tree;
  the delivery checklist requires an evidence command per item; the second-pass sweep already caught
  and corrected two stale current-state claims in this plan's own first draft.
