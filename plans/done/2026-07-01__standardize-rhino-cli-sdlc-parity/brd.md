# BRD — Standardize rhino-cli Checks & SDLC Commands

## Business Goal

Make the SDLC quality machinery behave `"identically"` across `ose-public`, `ose-primer`, and
`ose-infra` so that a contributor (human or AI agent) who learns the gate mechanics in one repo
applies that knowledge unchanged in the other two, and so that `ose-public` (the upstream source of
truth) can be propagated to the siblings without per-repo translation friction.

## Why It Matters

- **Cognitive load** — today the same gate has three names (`commons-quality-gate.yml` /
  `pr-quality-gate.yml`) and three invocation styles (inline shellcheck vs. an Nx-wrapped
  `rhino-cli:shell:lint` target). Every cross-repo edit forces a re-learn. [Repo-grounded]
- **Parity-loop cost** — the [multi-repo parity workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)
  has to absorb structural drift on every sync. Identical mechanics make propagation near-mechanical. [Repo-grounded]
- **Dead-command risk** — rhino-cli ships subcommands that no lifecycle automation invokes
  (e.g. `convention validate license`, `md validate frontmatter-dates`). Without a triage, nobody
  knows whether a gate is genuinely enforced or merely available. This plan makes the wired/not-wired
  status explicit and reviewable. [Repo-grounded]
- **Trust in green CI** — when "the markdown gate" runs three different validator sets in three
  repos, a green check means three different things. Identical mechanics make green mean the same
  thing everywhere. [Judgment call]

## Affected Roles

- **Repo maintainer / solo operator** — edits hooks and workflows across all three repos; primary beneficiary.
- **AI coding agents** (`ci-checker`, `repo-harness-compatibility-checker`, `swe-rust-dev`) — validate and edit this surface; benefit from one mental model.
- **Contributors to `ose-primer`** (downstream template consumers) — inherit a coherent, documented gate model.

## Business-Level Success Metrics

- **Identical cross-repo experience (north star)**: the entire standardization layer is identical across `ose-public`, `ose-primer`, and `ose-infra` — the same rhino-cli command (verb-last) does the same thing, the same `:`-separated Nx target resolves the same way, `repo-config.yml` holds the same kind of config under the same schema, the hooks/gates/CI workflows carry the same names and order. Working cross-repo is identical, logical, and intuitive. The only divergence is each repo's project/app set. Observable check: the Phase 5 parity table shows ✅ on every standardization row. [Judgment call — invariant]
- **Gate-mechanics parity**: for every gate in the [target standard](./tech-docs.md#7-target-standard-best-of-three-synthesis), the gate name, the workflow filename, the hook step ordering, and the invocation mechanism are identical across all three repos (modulo documented allowed divergence). Observable check: the cross-repo parity table in [delivery.md Phase 5](./delivery.md#phase-5-cross-repo-parity-verification--archival) shows ✅ on every mechanics row. [Repo-grounded]
- **rhino-cli command naming**: every CLI leaf command is verb-last (`{domain} {noun…} {verb}`) and identical across repos; Nx targets stay `:`-separated. Observable check: `cargo run -- --help` (recursive) shows only verb-last leaves matching the triage target column; no verb-middle form remains. [Repo-grounded]
- **Native coverage gate**: every project with a real `test:unit` enforces `test:coverage` at **≥ 90% line** via its native runner (no Codecov, no rhino-cli `test-coverage`). Observable check: `test:coverage` present wherever `test:unit` is real; a sub-90% project fails the gate. [Repo-grounded]
- **Command triage published**: a committed reference doc lists every rhino-cli command with a wired/not-wired status and the exact invocation site for each wired command, with the **target** (end-state) command column placed before the current-form column. Observable check: `docs/reference/rhino-cli-command-triage.md` exists and covers every leaf subcommand in `apps/rhino-cli/src/cli.rs`; the target column precedes the current column. [Repo-grounded]
- **Surface rationalization (merge/drop recommendations)**: the triage carries a per-command keep/merge/drop/wire verdict that collapses redundancy without breaking the identical-command-set invariant. Observable check: §3.3 lists a verdict for every command; the net-effect summary (drops, merges, promotions) is explicit and every verdict is **ratified** (Decided ✅ in the §3 triage table) after one-by-one maintainer review. [Judgment call]
- **All-harness coverage (every command)**: **every** rhino-cli `harness` command — `bindings`, `naming`, `instruction-size`, `duplication`, `audit` — accounts for every supported harness (11 across source/generated/native tiers), driven by the `harness:` section in `repo-config.yml` rather than a hard-coded directory list, so none is limited to Claude + OpenCode. Observable check: the [§3.2 per-command coverage matrix](./tech-docs.md#32-harness-binding-coverage-standard-all-supported-harnesses) is satisfied — `harness bindings validate` asserts generated byte-parity + native no-shadowing; `harness naming validate` covers Amazon Q `.amazonq/cli-agents/` with an N-way mirror; `harness instruction-size validate` budgets every native surface; adding a 12th harness is a one-line `repo-config.yml` edit picked up by all commands. [Judgment call — coverage standard; current gaps Repo-grounded]
- **Unit/integration folder separation**: every project with both a real `test:unit` and a real `test:integration` keeps the two suites in physically separate folders. Observable check: in each such project the `test:unit` and `test:integration` source globs share no path (TS/F#: `tests/unit` vs `tests/integration`; Rust: co-located `#[cfg(test)]` + external `tests/`). [Repo-grounded — current layouts already separate]
- **Symmetric per-project targets**: every project (direct child of `apps/`/`libs/`) in all three repos exposes the mandatory six targets (`echo` where N/A), plus native `test:coverage`, `specs:behavior:coverage`, and `specs:domain:coverage` on every project listed in the explicit `specs.domain-areas` allowlist in `repo-config.yml` (not folder-presence, not the `*-be` suffix — explicit-config principle; today the listed set is the `*-be` backends) (`test:quick` composes `test:coverage` + `specs:behavior:coverage`, so both are present on every project — `echo` where N/A), plus the cross-language dependency targets `deps:audit` (real on **every** project, including `*-e2e`) and `compat:min-version` (real on Rust + Python; `echo` elsewhere). Observable check: the mandatory-six `jq` loop prints no `MISSING` in any repo (delivery Phase gates). [Repo-grounded]
- **Identical specs structure (all projects, all repos)**: every spec area uses one identical C4 tree (`product` + `system-context` + `containers` + `components` + `behavior/.../gherkin`, mandatory for apps and libs alike; gherkin in every area; `ddd/` only for the explicit `specs.ddd-areas` allowlist). App specs are **domain-keyed** — one `specs/apps/<domain>/` tree serves the whole `apps/<domain>-*` family — while lib specs are per-project. Observable check: `specs:structure-validation` passes for every project in all three repos against the one structure. [Repo-grounded — domain/lib tree already present; full-C4-everywhere + the ddd-areas allowlist are this plan's additions]
- **Formatting & config consolidation**: no per-project `format`/`format:check` target (formatting is file-type lint-staged); the three root config files are merged into one `repo-config.yml`; Codecov is fully removed. Observable check: `grep -ri codecov` returns only `ExcludeFromCodeCoverage`; `repo-config.yml` exists and the 3 old files are absent in every repo. [Repo-grounded]
- **Standardized GitHub CI for every project**: the plan completes only when every project across the three repos is covered by a GitHub CI named per the ose-public convention (`pr-quality-gate.yml`, `validate-env.yml`, `main-ci.yml` — markdown validation folds into the gates, **no** standalone `validate-markdown.yml`). Observable check: the three canonical workflows exist with those names in each repo, no `validate-markdown.yml` remains, and every project is covered by `main-ci.yml` (it runs `nx run-many --all`, so coverage is total by construction). [Repo-grounded]
- **Uniform test→deploy lifecycle (one check set, two scopes)**: the same check set composes every gate — the **PR gate runs the union of what pre-commit ∪ pre-push run, for _affected_ projects** (`nx affected`), and **post-merge `main-ci.yml` runs that identical set for _all_ projects** (`nx run-many --all`); the only gate-to-gate difference is scope (affected vs. all), never the check list. **Pre-commit stays fast** (format + tool-lint + guards, no `test:quick`); **none** of the four gates runs `test:integration`/`test:e2e`; every gate runs its independent checks **in parallel**. The heavy levels and the staging/prod deploy that depends on them run **only** in the scheduled CRON pipelines (`*-test-local-deploy-stag.yml` → staging; `*-test-stag.yml` → prod), per-app isolated — identical mechanism in every repo (deploy leg per each repo's actual deployables). Observable check: `grep test:integration\|test:e2e` finds no gate invocation in any hook or `main-ci.yml`/`pr-quality-gate.yml`; `pr-quality-gate.yml` uses `nx affected` while `main-ci.yml` uses `nx run-many --all`; only the CRON pipelines run the heavy tiers. [Repo-grounded]
- **One plan, three repos**: this single plan executes end-to-end across `ose-public`, `ose-primer`, and `ose-infra` (Phases 2/3/4) with per-repo granular steps — not a public-only plan deferred for later propagation. [Repo-grounded]
- **Worktree-agnostic guardrails**: every guardrail (the hooks, the rhino-cli commands, lint-staged, and the Nx targets they invoke) runs green from **both** the primary checkout and a linked worktree (`worktrees/<name>/`); `ose-infra` (bare, worktree-only) exercises the worktree path as its sole context. Observable check: the full pre-push + PR-gate command set exits 0 when run from a linked worktree **and** from the primary checkout, and a rhino-cli regression test runs a guardrail from a synthetic linked worktree. [Repo-grounded — `git rev-parse --show-toplevel` already used; the invariant, the regression test, and the per-repo verification are new]
- **Zero regressions**: after convergence each repo's pre-push and PR quality gate pass on a no-op change. [Repo-grounded]

## Business-Scope Non-Goals

- Not changing **what** any validator checks (no new lint rules, no threshold changes).
- Not unifying the **app set** or **language set** across repos.
- Not net-new validator behavior — the §3.3 ratified command removals, merges, and wirings **are** executed by this plan (not deferred to a follow-up); but no surviving validator gains a new rule or threshold. The only deferred item is an optional automated parity-enforcement check (see Business Risks).

## Business Risks

- **Risk: convergence breaks a working gate.** Mitigation: each repo converges in its own phase behind a full pre-push + PR-gate verification before the phase gate passes.
- **Risk: over-standardization erases a legitimate infra-only gate.** Mitigation: the [divergence policy](./tech-docs.md#71-divergence-policy-allowed-vs-drift) is fixed up front; IaC and app-set gates are explicitly protected.
- **Risk: drift re-appears after convergence.** Mitigation: the committed standard doc becomes the reference `ci-checker` / `repo-harness-compatibility-checker` validate against; a follow-up can add an automated parity check.
