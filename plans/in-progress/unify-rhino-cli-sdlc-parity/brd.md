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
  tree and treats the code, not the notes, as ground truth. [Repo-grounded]
- **rhino-cli drift is the highest-leverage divergence.** It is the one tool every gate in every repo
  invokes. If its source, its `Cargo.lock`, and its `project.json` commands differ, every downstream
  gate can behave differently even when the wiring looks the same. Making the tool byte-identical
  makes every gate that calls it identical by construction. [Judgment call]
- **Cucumber coverage is asymmetric.** primer has a fully-wired BDD harness for rhino-cli's own
  behaviour (11 `[[test]]` blocks + real `.feature` specs); public and infra declare the dependency
  and wire nothing. The tool that enforces spec coverage everywhere is itself only spec-covered in one
  of three repos. [Repo-grounded]
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
  empty; `Cargo.toml`, `Cargo.lock`, and `project.json` are 100% byte-identical with **zero carve-outs**
  (infra relicensed to MIT; env-validation scan paths data-driven from `repo-config.yml`). Observable
  check: Phase 5's `diff -rq` + `diff` matrix shows no differences at all. [Repo-grounded]
- **Cucumber parity**: the same wired cucumber-rs harness + the same `.feature` specs for rhino-cli's
  own behaviour exist and pass in all three repos. Observable check: `cargo test` runs the cucumber
  suites in each repo; the `tests/*.rs` set and the `specs/apps/rhino/behavior/rhino-cli/gherkin` tree
  are identical across repos. [Repo-grounded]
- **SDLC mechanism parity (zero `⚠️`)**: every gate is invoked through the identical mechanism in all
  three repos — no repo using `npx nx run rhino-cli:*` where another uses direct `cargo run`, no
  inline tool-lint where another uses lint-staged. Observable check: the Phase 5 parity table shows ✅
  on every mechanics row with **no `⚠️` rows remaining**. [Repo-grounded]
- **Full `namedInputs.specs` coverage**: every Nx project in every repo wires the specs input.
  Observable check: the count of projects with `namedInputs.specs` equals the total project count in
  each repo (currently 16/27, 20/25, 6/7). [Repo-grounded]
- **Complete mandatory-target coverage**: no project missing any mandatory target in any repo
  (currently 5 infra projects have gaps). Observable check: the mandatory-target `jq` loop prints no
  `MISSING` in any repo. [Repo-grounded]
- **Latent bugs fixed**: the agent-naming validator fires (trigger path `.opencode/agents/`), and the
  PR gate runs `gherkin-cardinality` in public. Observable check: a renamed agent file trips the
  naming validator; the public PR-gate specs job lists `gherkin-cardinality`. [Repo-grounded]
- **Reality-grounded record**: Phase 0 produces committed audit evidence; every delivery item cites a
  concrete verification (diff, jq, grep), not a prior "done" note. [Judgment call]
- **One plan, three repos**: this single plan executes end-to-end across all three repos (Phases
  2/3/4) with per-repo granular steps. [Repo-grounded]
- **Zero regressions**: after convergence each repo's pre-push and PR quality gate pass on a no-op
  change. [Repo-grounded]

## Business-Scope Non-Goals

- Not changing **what** any validator checks (no new lint rules, thresholds, or validator logic).
- Not unifying the **app set** or **language set** across repos.
- Not building **new automated parity-enforcement tooling** — mission is verify-&-closeout, not a
  drift-guard. A parity-check command remains a possible future follow-up.

## Business Risks

- **Risk: the infra rhino-cli port is large and could destabilize a working repo.** Mitigation: it is
  isolated as gated **Phase 4** behind a full pre-push + PR-gate verification, and can be descoped to
  a documented divergence without unwinding Phases 1–3.
- **Risk: byte-identity conflicts with genuinely repo-specific bits.** Mitigation: every such bit
  (env-validation scan paths, domain/ddd areas) is driven from `repo-config.yml` data, and infra's CLI
  is relicensed to MIT — leaving `apps/rhino-cli` with zero carve-outs. The only divergence anywhere is
  app/language set and the CI runner label.
- **Risk: pulling primer's advances back into public regresses public.** Mitigation: the synthesis
  lands behind rhino-cli's own unit + cucumber suites (Phase 1 wires the cucumber suite into public as
  part of the synthesis) and the golden-master test.
- **Risk: "done" notes mislead again.** Mitigation: every item is verified against the working tree;
  the delivery checklist requires an evidence command per item.
