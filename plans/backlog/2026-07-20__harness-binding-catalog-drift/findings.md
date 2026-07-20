# Harness Compatibility Audit Report

**Status**: Complete
**Agent**: repo-harness-compatibility-checker
**Scope**: Full repository audit at commit 6aea08047 (main) — post-fix verification pass for
plans/done/2026-07-20\_\_parallel-orchestration-shared-machine-governance/
**Timestamp**: 2026-07-20--19-09 (UTC+7)
**UUID Chain**: 3b8a20
**Commit under audit**: 6aea08047 (`fix(workflows): use the canonical Delivery Mode vocabulary in
parity planning`)
**Working tree**: clean at audit start (verified via `git status --porcelain`)

**Known context supplied by requester (not to be re-derived)**:

- Amazon Q Developer sunsetting (IDE plugins EOS 2027-04-30), succeeded by Kiro CLI which reads
  `AGENTS.md` natively. Already recorded in `docs/reference/platform-bindings.md`.
- Recent commits of interest: `a158b0843` (edited `.claude/agents/pr-review-fixer.md` +
  `.opencode/` mirror together — verify genuine sync + `.amazonq/` consistency, watch for
  post-staging hook drift), `60d53119b`, `f995df5e9`, `6aea08047` (governance docs / workflow
  files).

**Tool hazard acknowledged**: `grep` in this shell routes to ugrep, which rejects `--glob` and
silently no-ops on `\{2,3\}` interval quantifiers in basic-regex mode. This audit uses `-E` for
intervals, `--include='*.md'` for file filters, and `/opt/homebrew/bin/rg` by absolute path where
appropriate. No sweep whose conclusion is "nothing found" suppresses stderr.

---

## Phase 0 — Cross-Vendor Parity Invariants

### Finding: Invariant 1 — Governance prose vendor-neutrality

**Phase**: Phase 0 — Parity
**Criticality**: N/A (PASS)
**Confidence**: HIGH

**Command**: `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor validate repo-governance/`

**Result**: `GOVERNANCE VENDOR AUDIT PASSED: no violations found` (exit 0)

**Conclusion**: PASS — no drift.

---

### Finding: Invariant 2 — Root instruction surface vendor-neutrality

**Phase**: Phase 0 — Parity
**Criticality**: N/A (PASS)
**Confidence**: HIGH

**Command**: `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor validate AGENTS.md` and same for `CLAUDE.md`

**Result**:

- `AGENTS.md`: `GOVERNANCE VENDOR AUDIT PASSED: no violations found` (exit 0)
- `CLAUDE.md`: `GOVERNANCE VENDOR AUDIT PASSED: no violations found` (exit 0)

**Conclusion**: PASS — no drift.

---

### Finding: Invariant 3 — Binding sync no-op

**Phase**: Phase 0 — Parity
**Criticality**: N/A (PASS)
**Confidence**: HIGH

**Command**: `npm run generate:bindings && git diff --quiet .opencode/ .amazonq/`

**Result**: Sync ran (`Agents: 82 converted`, `Status: SUCCESS`, wrote
`.amazonq/rules/00-agents-md.md` + `.amazonq/cli-agents/ose-default.json`). `git diff --quiet
.opencode/ .amazonq/` exited 0. Full-tree `git status --porcelain` (not just `.opencode/`/
`.amazonq/`) also came back empty — ruling out the requester-flagged hazard of a newly generated
(untracked) mirror file that `git diff --quiet` would silently miss.

**Targeted spot-check for the `a158b0843` concern** (`.claude/agents/pr-review-fixer.md` +
`.opencode/agents/pr-review-fixer.md` edited together): re-read both files post-sync — the
`description` field is byte-identical between the two, and the file pair produced zero diff under
the full sync above. No drift, no post-staging regeneration artifact left uncommitted.

**Conclusion**: PASS — no drift. `.claude/`, `.opencode/`, and `.amazonq/` are genuinely in sync at
commit 6aea08047.

---

### Finding: Invariant 4 — Agent count parity

**Phase**: Phase 0 — Parity
**Criticality**: N/A (PASS)
**Confidence**: HIGH

**Command**: `ls .claude/agents/*.md | wc -l` and same for `.opencode/agents/*.md`; filename diff via
`comm -3 <(ls .claude/agents | sort) <(ls .opencode/agents | sort)`

**Tool-hazard note**: The first attempt used the shell's default `ls`, whose output was wrapped in
OSC-8 terminal hyperlink escape sequences, corrupting the `comm -3` byte comparison and producing a
spurious full-list mismatch (every filename appeared to differ from itself). Re-ran with `/bin/ls`
explicitly (confirmed via `type ls` → `ls is /bin/ls`, so the corruption source was terminal/tool
rendering rather than a `ls` alias) piped through explicit temp files — this is exactly the class of
false "everything drifted" result the requester's tool-hazard warning was guarding against.

**Result** (clean re-run):

- `.claude/agents/*.md` count: 83
- `.opencode/agents/*.md` count: 83
- `comm -3` filename diff: empty (zero entries only-in-`.claude`, zero only-in-`.opencode`)

Both counts include `README.md` (index file, not an agent definition, per the documented known
intentional skip in `converter.rs` ~line 391) — present identically on both sides, so it does not
affect parity. 83 filesystem files − 1 `README.md` = 82 real agent definitions, matching the `Agents:
82 converted` line from the Invariant 3 sync run.

**Conclusion**: PASS — no drift.

---

### Finding: Invariant 5 — Translation-map coverage

**Phase**: Phase 0 — Parity
**Criticality**: N/A (PASS)
**Confidence**: HIGH

**Color map**:

- Distinct `color:` values across `.claude/agents/*.md`: `blue`, `green`, `purple`, `yellow`
- Color Translation Table (`repo-governance/development/agents/ai-agents.md`, "Color Translation
  Table" section, lines ~762-776) covers: `blue`, `green`, `yellow`, `purple`, `red`, `orange`,
  `pink`, `cyan` — a superset. All 4 values in use are covered.

**Tier/model map**:

- Distinct `model:` values across `.claude/agents/*.md`: blank/omitted (5 agents: `plan-maker`,
  `docs-tutorial-maker`, `swe-ui-maker`, `pr-review-maker`, plus the `README.md` template example
  which is not a real agent), `sonnet` (67), `haiku` (11)
- Distinct `model:` values across `.opencode/agents/*.md`: `opencode-go/glm-5.2`,
  `opencode-go/minimax-m3`
- The capability-tier map (`repo-governance/development/agents/model-selection.md`, "Model ID
  Mapping" section, lines ~279-285) covers all three primary-binding forms (`opus` / omit /
  `sonnet` → `opencode-go/glm-5.2`; `haiku` → `opencode-go/minimax-m3`) and both secondary-binding
  values in use.

**Conclusion**: PASS — no drift. Every distinct frontmatter value in use has a corresponding map
entry.

---

### Phase 0 Summary

All five deterministic parity invariants **PASS** at commit `6aea08047`. No CRITICAL/HIGH/MEDIUM/LOW
findings in Phase 0.

Context check on the three additional recent commits named by the requester:

- `a158b0843` (`.claude/agents/pr-review-fixer.md` + `.opencode/` mirror) — confirmed genuinely in
  sync (see Invariant 3 targeted spot-check above); no post-staging drift.
- `60d53119b`, `f995df5e9`, `6aea08047` — governance-doc / workflow-file only changes (N+1
  orchestration model, guard hardening, Delivery Mode vocabulary rename). None touch
  `.claude/agents/`, `.opencode/agents/`, or `.amazonq/`, so they carry no Phase-0-invariant risk
  beyond what Invariants 1-2 already re-confirmed clean at HEAD.

---

## Phase 1 — Harnesses Under Review

Source: `docs/reference/platform-bindings.md` (main table header stamped "Verified 2026-05-24";
Amazon Q / Kiro succession subsection separately stamped "Verified 2026-07-20" — today).

| #   | Harness                       | Binding dir             | Root file (catalog)                                       | MCP config (catalog)                                     | Custom-agent surface (catalog)                 | Skills surface (catalog)           | Status                      |
| --- | ----------------------------- | ----------------------- | --------------------------------------------------------- | -------------------------------------------------------- | ---------------------------------------------- | ---------------------------------- | --------------------------- |
| 1   | Claude Code                   | `.claude/`              | `CLAUDE.md` (shim `@AGENTS.md`)                           | `.mcp.json`                                              | `.claude/agents/*.md`                          | `.claude/skills/*/SKILL.md`        | Active                      |
| 2   | OpenCode                      | `.opencode/`            | none (reads `AGENTS.md` natively)                         | `opencode.json`                                          | `.opencode/agents/*.md`                        | reads `.claude/skills/`            | Active                      |
| 3   | OpenAI Codex CLI              | `.codex/`               | none native + `AGENTS.override.md` (overrides)            | `.codex/config.toml [mcp_servers]`                       | `[agents.<name>]` in `config.toml`             | `.agents/skills/`                  | Partial                     |
| 4   | GitHub Copilot                | `.github/`              | none native                                               | `.vscode/mcp.json`                                       | `.github/agents/*.agent.md`                    | n/a                                | Reserved                    |
| 5   | Cursor                        | `.cursor/`              | none native                                               | `.cursor/mcp.json`                                       | `.cursor/agents/*.md`                          | `.cursor/skills/`                  | Reserved                    |
| 6   | Windsurf                      | `.windsurf/`            | none native                                               | global only                                              | not officially documented                      | `.windsurf/skills/` (unverified)   | Reserved                    |
| 7   | JetBrains Junie               | `.junie/`               | `.junie/AGENTS.md` (outranks root)                        | `.junie/mcp/mcp.json`                                    | `.junie/agents/`, `.agents/`                   | `.junie/skills/<name>/SKILL.md`    | Reserved                    |
| 8   | Amazon Q Developer → Kiro CLI | `.amazonq/` (generated) | Q: none native → Kiro: `AGENTS.md` native                 | Q: `.amazonq/mcp.json` → Kiro: `.kiro/settings/mcp.json` | Q: JSON in `.amazonq/` → Kiro: `.kiro/agents/` | Q: none → Kiro: `.kiro/skills/`    | Sunsetting (EOS 2027-04-30) |
| 9   | Google Antigravity CLI        | `.agent/`               | `GEMINI.md` (outranks)                                    | global only                                              | runtime-orchestrated                           | `.agents/skills/<name>/SKILL.md`   | Reserved                    |
| 10  | Pi (pi.dev)                   | `.pi/`                  | `.pi/AGENTS.md`, `.pi/SYSTEM.md` (also reads `CLAUDE.md`) | none                                                     | none built-in                                  | `.agents/skills/` or `.pi/skills/` | Reserved                    |
| 11  | Aider                         | n/a                     | `CONVENTIONS.md` (opt-in only)                            | n/a                                                      | n/a                                            | n/a                                | Reserved                    |

**Scoping decision for this run**: No `scope` filter was requested — all 11 rows are in scope.
Item 8 (Amazon Q → Kiro) uses the requester's supplied context (succession already verified today,
2026-07-20, in the catalog itself) rather than re-delegating fresh web research — see Finding below.
Items with committed binding files (`.claude/`, `.opencode/`, `.codex/config.toml`) receive D1-D6;
`Reserved`-status harnesses with no committed binding files receive D1-D5 only (D6 is vacuous — no
files to check).

---

### Finding: D1-D5 Amazon Q Developer → Kiro CLI — [INFO] No drift detected (not re-researched)

**Phase**: Phase 1 — Amazon Q Developer (superseded by Kiro CLI)
**Criticality**: N/A
**Confidence**: FALSE_POSITIVE (not a finding — logged per skill's FALSE_POSITIVE/no-drift handling)

**Current value**: Catalog already documents: Q Developer CLI does not read `AGENTS.md` natively
(open feature request aws/amazon-q-developer-cli#2712); Kiro CLI reads `AGENTS.md` natively at
workspace root or `~/.kiro/steering/`; sunset milestones 2026-05-15 (new sign-ups closed),
2026-05-29 (Opus 4.6 removed from Q Developer Pro), 2027-04-30 (IDE plugins/paid subscriptions EOS).
Repo's `.amazonq/` bridge (`rules/00-agents-md.md` pointer + `cli-agents/ose-default.json`) stays
in place pending a deliberate future retirement decision.

**Expected / Upstream value**: Same — this subsection of `docs/reference/platform-bindings.md`
carries its own "Verified 2026-07-20 against AWS and Kiro primary sources" stamp, dated **today**,
per the requester's supplied context. Re-delegating `web-researcher` for this specific claim would
duplicate work completed in the same day.

**Drift description**: None. Per-request instruction: this item is a known, already-current entry —
not rediscovered in this run.

**Recommendation**: No action. Re-verify only when a new milestone in the 2026-05-15 → 2027-04-30
sunset window passes, or if `.amazonq/` retirement becomes an active plan (at which point this
catalog section and the generated bridge files both need coordinated updates).

---

## Phase 1 — Claude Code

### Finding: D1-D5 Claude Code — [INFO] No drift detected

**Phase**: Phase 1 — Claude Code
**Criticality**: N/A
**Confidence**: FALSE_POSITIVE (no drift; logged per skill's no-drift handling, not counted in
findings total)

**Current value** (catalog): root file `CLAUDE.md` (shim `@AGENTS.md`), binding dir `.claude/`, MCP
config `.mcp.json`, custom-agent surface `.claude/agents/*.md`, skills surface
`.claude/skills/*/SKILL.md`.

**Upstream value** (web-researcher, [Verified] against `code.claude.com/docs/en/{memory,sub-agents,mcp,skills}`):
identical on all five dimensions. Notable: the official docs' own worked example for importing a
pre-existing `AGENTS.md` is literally `@AGENTS.md` — this repo's `CLAUDE.md` matches the documented
pattern verbatim. `context: fork` skill delegation mode confirmed current.

**Drift description**: None (D1-D5 all match). Two **non-drift additions** noted for awareness
only, not actionable:

- A `fable` model alias now appears alongside `sonnet`/`opus`/`haiku`/`inherit` in the sub-agent
  `model:` field docs. Not currently used by any `.claude/agents/*.md` file in this repo (Invariant
  5 already confirmed the in-use set is `sonnet`/`haiku`/omit only) — no catalog or map update
  required unless an agent adopts it.
- Several new **optional** sub-agent frontmatter fields exist upstream (`disallowedTools`,
  `permissionMode`, `maxTurns`, `mcpServers`, `hooks`, `memory`, `background`, `effort`,
  `isolation`, `initialPrompt`) beyond the repo's five-field baseline (`name`, `description`,
  `tools`, `model`, `color`, `skills`). All are additive/optional — none deprecate or replace an
  existing field, so this is not D6 drift (D6 only flags fields the harness has _removed_).

**Recommendation**: No action required.

---

## Phase 1 — OpenCode

### Finding: D1-D4 OpenCode — [INFO] No drift detected

**Phase**: Phase 1 — OpenCode
**Criticality**: N/A
**Confidence**: FALSE_POSITIVE (no drift; not counted in findings total)

**Current value** (catalog): no root file (reads `AGENTS.md` natively), binding dir
`.opencode/agents/`, MCP config `opencode.json`, custom-agent surface `.opencode/agents/*.md` with
a `permission` object and theme-token colors.

**Upstream value** (web-researcher, [Verified] against `opencode.ai/docs/{rules,config,agents,mcp-servers}`):
identical on D1-D4. `permission` values confirmed as strings only (`allow`/`ask`/`deny`) — no
boolean form documented anywhere, current or legacy, consistent with this repo's existing prose
("the older boolean flags form ... is deprecated/legacy — still accepted by OpenCode but no longer
emitted"). `color` field confirmed to accept the seven theme tokens already in this repo's
translation table, plus hex codes as an explicit alternative — already covered by this repo's
documented "unrecognized/hex passed through" escape hatch, so not new drift.

**Drift description**: None on D1-D4.

**Recommendation**: No action required for D1-D4.

---

### Finding: D5 OpenCode Skills Surface — catalog prose now incomplete (not incorrect)

**Phase**: Phase 1 — OpenCode
**Criticality**: LOW — informational; the repo's chosen behavior (rely solely on `.claude/skills/`,
never populate an OpenCode-native skills directory) still functions correctly per current docs, and
the existing `validate:sync` guard against a stale `.opencode/skills/` mirror is still valid
defensive tooling. No functional break.
**Confidence**: MEDIUM — web-researcher tagged the finding [Verified] with a citation
(`opencode.ai/docs/skills`), but this is the kind of "additive capability" claim best hand-confirmed
before prose changes are made, per this skill's conservative drift threshold guidance.

**Current value**: `CLAUDE.md` §Multi-harness configuration states: "**Skills**: NOT mirrored —
OpenCode reads `.claude/skills/{name}/SKILL.md` natively per opencode.ai/docs/skills. The
validate:sync 'No Synced Skill Mirror' check fails if a stale `.opencode/skill/` or
`.opencode/skills/<claude-name>` mirror reappears." Catalog's Skills surface column for OpenCode:
"reads `.claude/skills/`".

**Expected / Upstream value**: Per `web-researcher` ([Verified], `opencode.ai/docs/skills/`),
OpenCode now documents **six** discovery paths union-loaded (not override-precedence): project
`.opencode/skills/<name>/SKILL.md`, global `~/.config/opencode/skills/<name>/SKILL.md`, project/global
Claude-compat `.claude/skills/`, and project/global agent-compat `.agents/skills/`. A native,
first-class `.opencode/skills/` directory is now a documented discovery path — not merely a
deprecated/stale mirror concept.

**Drift description**: The repo's prose frames any `.opencode/skills/` directory as a "stale mirror"
to be guarded against. That framing was accurate when OpenCode had no native skills directory of its
own and only read `.claude/skills/`. Per current docs, `.opencode/skills/` is now a legitimate
_native_ (not mirrored) discovery path OpenCode would read directly if populated. The repo's actual
behavior (single source of truth in `.claude/skills/`, `.opencode/skills/` never populated) remains
correct and functional — this is a documentation-completeness gap, not a functional break: the prose
doesn't yet acknowledge that OpenCode _could_ read a project-native `.opencode/skills/` directory,
it just doesn't have to.

**Affected files**: `CLAUDE.md` (§Multi-harness configuration, Skills bullet); possibly
`docs/reference/platform-bindings.md` (OpenCode row, Skills surface column: "reads `.claude/skills/`"
— technically still true but incomplete).

**Recommendation**: Human/maker review — consider updating the CLAUDE.md skills bullet to note that
`.opencode/skills/` is OpenCode's own native path (currently and deliberately left unpopulated by
this repo's single-source-of-truth policy) rather than solely framing it as a "stale mirror" risk.
Not urgent; does not affect current functionality.

---

## Phase 1 — OpenAI Codex CLI

### Finding: D1-D3, D5 OpenAI Codex CLI — [INFO] No drift detected

**Phase**: Phase 1 — OpenAI Codex CLI
**Criticality**: N/A
**Confidence**: FALSE_POSITIVE (no drift; not counted in findings total)

**Current value** (catalog): native `AGENTS.md` read (since Apr 2025) with `AGENTS.override.md`
ranking above it; config `.codex/config.toml`; MCP under `[mcp_servers]` table; skills surface
`.agents/skills/`.

**Upstream value** (web-researcher, [Verified] against `learn.chatgpt.com/docs/{agent-configuration/agents-md,config-file/config-reference,build-skills}`,
noting the `developers.openai.com/codex/*` URLs 308-redirect there): D1 confirmed unchanged
(precedence: global override → global AGENTS.md → project root-to-cwd walk, override-before-plain
per directory, files concatenate). D2 confirmed unchanged (`.codex/config.toml`, project-scope only
loaded for trusted projects). D3 confirmed same location, expanded fields
(`enabled`, `default_tools_approval_mode`, `enabled_tools`/`disabled_tools`,
`startup_timeout_sec`) — additive, not breaking. D5 confirmed unchanged (`.agents/skills/`, scanned
at cwd/one-level-up/repo-root/`$HOME/.agents/skills`).

**Drift description**: None on D1, D2, D3, D5.

**Recommendation**: No action required for these four dimensions.

---

### Finding: D4 OpenAI Codex CLI — Custom-agent declaration mechanism superseded

**Phase**: Phase 1 — OpenAI Codex CLI
**Criticality**: HIGH — D4 default; if the old declaration form is silently ignored by current
Codex CLI (unconfirmed either way by the research), the `ci-monitor-subagent` entry becomes
unreachable via Codex CLI without any error surfaced to a contributor.
**Confidence**: MEDIUM — web-researcher tagged the mechanism-change claim [Verified] against
`learn.chatgpt.com/docs/agent-configuration/subagents`, but did not confirm (and the source page
apparently doesn't state) whether the superseded `[agents.<name>]` sub-table form in `config.toml`
is still read for backward compatibility or now silently ignored — that ambiguity caps confidence
at MEDIUM rather than HIGH.

**Current value** (verified live in this repo at commit 6aea08047):

```toml
[agents.ci-monitor-subagent]
description = "CI helper for /monitor-ci. Fetches CI status, retrieves fix details, or updates self-healing fixes. Executes one MCP tool call and returns the result."
config_file = "ci-monitor-subagent.toml"
```

in `.codex/config.toml`, pointing at `.codex/ci-monitor-subagent.toml`. Catalog documents this
exact pattern: "`[agents.<name>]` in `config.toml` (with optional `config_file` pointer to a TOML
layer, e.g. `.codex/<name>.toml`)".

**Expected / Upstream value**: Per current Codex CLI docs ([Subagents](https://learn.chatgpt.com/docs/agent-configuration/subagents)),
custom agents are now declared as **standalone TOML files** — `.codex/agents/*.toml` (project) or
`~/.codex/agents/*.toml` (personal), one file per agent, with required fields `name`, `description`,
`developer_instructions`; the agent's `name` field (not the filename) is the source of truth.
`[agents]` inside the main `config.toml` is now documented as reserved for **global settings only**
(e.g. `agents.max_threads`) — not per-agent sub-tables.

**Drift description**: The catalog's documented custom-agent surface for OpenAI Codex CLI
(`[agents.<name>]` sub-table + optional `config_file` pointer) matches this repo's live
`.codex/config.toml`, but both now describe a superseded declaration form per current upstream docs.

**Affected files**:

- `docs/reference/platform-bindings.md` — OpenAI Codex CLI row, "Custom-agent surface" column
- `.codex/config.toml`, `.codex/ci-monitor-subagent.toml` — live files using the superseded form

**Provenance caveat**: Per this catalog's own "Provenance of pre-existing partial bindings" section,
`.codex/config.toml` is Nx/Codex tooling infrastructure, **not** produced by `rhino-cli agents
sync` and not hand-authored by this repo's agent pipeline — `rhino-cli agents sync` does not write
to `.codex/`. Remediation of the live file therefore falls outside the `repo-harness-compatibility-
fixer`'s normal auto-remediation scope (catalog/binding updates) and may require coordinating with
whichever tooling generates/maintains `.codex/config.toml`, or a manual migration to
`.codex/agents/ci-monitor-subagent.toml`.

**Recommendation**: (1) Confirm live in a current Codex CLI install whether the old
`[agents.<name>]` form still resolves `/monitor-ci` correctly or has gone silently inert — this is
the single fact that would upgrade or downgrade this finding's criticality. (2) If confirmed broken
or deprecated-with-a-removal-date, migrate `.codex/ci-monitor-subagent.toml` to
`.codex/agents/ci-monitor-subagent.toml` with `name`/`description`/`developer_instructions` fields
and update the catalog row's Custom-agent surface column accordingly. (3) Either way, update the
catalog row to document the current (`.codex/agents/*.toml`) form as primary, keeping the legacy
form noted only if independently confirmed still functional.

---

## Phase 1 — GitHub Copilot

### Finding: D1, D2, D4 GitHub Copilot — [INFO] No drift detected

**Phase**: Phase 1 — GitHub Copilot
**Criticality**: N/A
**Confidence**: FALSE_POSITIVE (no drift; not counted in findings total)

**Current value** (catalog): reads root `AGENTS.md` natively (nearest file wins), tool-specific
surface `.github/copilot-instructions.md` + `.github/instructions/*.instructions.md`, custom-agent
surface `.github/agents/*.agent.md`.

**Upstream value** (web-researcher, [Verified] against `docs.github.com/copilot/{customizing-copilot/adding-custom-instructions-for-github-copilot,reference/custom-instructions-support,reference/custom-agents-configuration}`):
D1 confirmed — nearest-`AGENTS.md`-wins precedence for multiple `AGENTS.md` files at different
depths; also now reads `CLAUDE.md`/`GEMINI.md` additively (not overriding); this repo ships none of
those extra files, so no interaction. D2 confirmed unchanged. D4 confirmed unchanged path; full
frontmatter schema now documented (required `description`; optional `name`, `target`, `tools`,
`model`, `disable-model-invocation`, `user-invocable`, `mcp-servers`, `metadata`) — this repo has no
`.github/agents/*.agent.md` files committed (Reserved status), so no D6 conformance check applies.

**Drift description**: None on D1, D2, D4.

**Recommendation**: No action required for these three dimensions.

---

### Finding: D3 GitHub Copilot — MCP config path expanded beyond catalog description

**Phase**: Phase 1 — GitHub Copilot
**Criticality**: MEDIUM (D3 default)
**Confidence**: HIGH — sourced from three current official GitHub docs pages with explicit URLs;
GitHub's docs are continuously updated (no versioned staleness risk for this class of claim)

**Current value**: Catalog documents GitHub Copilot's Project MCP config as `.vscode/mcp.json`
only.

**Expected / Upstream value**: `.vscode/mcp.json` remains correct for the VS Code IDE surface, but
per [Extending Copilot Chat with MCP](https://docs.github.com/copilot/customizing-copilot/using-model-context-protocol/extending-copilot-chat-with-mcp),
[CLI config dir reference](https://docs.github.com/en/copilot/reference/copilot-cli-reference/cli-config-dir-reference),
and [Configure MCP servers for your repository](https://docs.github.com/copilot/how-tos/agents/copilot-coding-agent/extending-copilot-coding-agent-with-mcp),
Copilot now has three distinct MCP configuration surfaces: VS Code (`.vscode/mcp.json`,
unchanged), Copilot CLI (project-level `.mcp.json` or `.github/mcp.json`, taking precedence over
user-level `~/.copilot/mcp-config.json`), and the cloud coding agent (no committed file at all —
configured via repo **Settings → Copilot → MCP servers** web UI).

**Drift description**: The catalog's single-path description is incomplete for the harness's full
current surface set. This repo has status "Reserved" for GitHub Copilot (no committed binding files
for this harness at all), so there is no functional break — only a catalog-accuracy gap that would
matter once/if this harness moves from Reserved to Active.

**Affected files**: `docs/reference/platform-bindings.md` — GitHub Copilot row, "Project MCP config"
column.

**Recommendation**: Update the catalog cell to note the three distinct MCP surfaces (VS Code file,
CLI file, cloud-agent web UI) rather than a single path, or add a footnote. Low urgency given
Reserved status.

---

### Finding: D5 GitHub Copilot — Skills surface listed as n/a may now be inaccurate

**Phase**: Phase 1 — GitHub Copilot
**Criticality**: MEDIUM (D5 default)
**Confidence**: MEDIUM — web-researcher itself flagged this as needing follow-up confirmation before
acting on it ("recommend a follow-up check... before updating this repo's 'Reserved' status note")

**Current value**: Catalog's GitHub Copilot row: "Skills surface: n/a". Status: "Reserved (reads
root `AGENTS.md` natively; `.github/` is CI-only)".

**Expected / Upstream value**: Per [Adding agent skills for GitHub Copilot](https://docs.github.com/en/copilot/how-tos/copilot-on-github/customize-copilot/customize-cloud-agent/add-skills)
([Verified]), GitHub Copilot now documents skill discovery at `.github/skills`, `.claude/skills`, or
`.agents/skills` (project scope; personal-scope equivalents also exist), each skill a subdirectory
with `SKILL.md` (required frontmatter `name`, `description`; optional `license`) — distinguishing
skills (loaded only when relevant) from always-in-context custom instructions.

**Drift description**: This repo already populates `.claude/skills/<name>/SKILL.md` for Claude Code
and OpenCode. If GitHub Copilot's cloud coding agent genuinely discovers `.claude/skills/` by file
presence alone (no separate opt-in), this repo's skills may **already be live for GitHub Copilot**
with zero additional files — which would make the catalog's "Skills surface: n/a" and the
"Reserved... no binding files shipped by default" framing inaccurate for this one dimension. The
web-researcher explicitly could not confirm whether repo-presence alone is sufficient or whether an
org/repo-level opt-in toggle gates it, so this is flagged MEDIUM confidence pending that one
follow-up check.

**Affected files**: `docs/reference/platform-bindings.md` — GitHub Copilot row, "Skills surface"
column and "Status" column; potentially `AGENTS.md` §Platform Bindings Catalog (GitHub Copilot
listed among "no per-tool instruction file shipped by default").

**Recommendation**: Human/maker follow-up — confirm via a live GitHub Copilot cloud-agent session
(or a single targeted WebFetch of the skills doc's opt-in section) whether `.claude/skills/`
presence alone activates skill discovery for this repo. If confirmed active-by-default, update the
catalog's Skills surface column from `n/a` to `.claude/skills/*/SKILL.md` (reads natively, same as
OpenCode) and reconsider whether GitHub Copilot's Status cell should note this capability explicitly
rather than implying zero integration exists.

---

## Phase 1 — Cursor

### Finding: D1-D4 Cursor — [INFO] No drift detected

**Phase**: Phase 1 — Cursor
**Criticality**: N/A
**Confidence**: FALSE_POSITIVE (no drift; not counted in findings total)

**Current value** (catalog): native `AGENTS.md` read, rules `.cursor/rules/*.mdc` + legacy
`.cursorrules`, MCP `.cursor/mcp.json`, custom-agent surface `.cursor/agents/*.md` (also reads
`.claude/agents/`, `.codex/agents/`).

**Upstream value** (web-researcher, [Verified] against `cursor.com/docs/{rules,mcp,subagents}` and
`cursor.com/help/customization/rules` — noting `docs.cursor.com` now 308-redirects to
`cursor.com/docs`, a URL-only change, not a convention change): all four dimensions confirmed
unchanged. `.mdc` frontmatter still only `description`/`globs`/`alwaysApply`. `.cursorrules`
confirmed legacy-but-functional (deprecated wording, not removed). Custom-agent discovery confirmed
unchanged with an added detail: user-level equivalents (`~/.cursor/agents/`, `~/.claude/agents/`,
`~/.codex/agents/`) exist alongside the project-level paths already recorded, `.cursor/` wins on
name conflicts.

**Drift description**: None on D1-D4. One citation-hygiene note: any future doc update citing
`docs.cursor.com/...` deep links should use the `cursor.com/docs/...` equivalents (redirect works
today but the old subdomain is not the canonical form going forward).

**Recommendation**: No action required functionally. Optional low-priority cleanup: none of this
repo's own docs currently cite `docs.cursor.com` deep links (verified — Cursor is Reserved status,
no binding files), so no edit is actually needed.

---

### Finding: D5 Cursor — Skills surface catalog description incomplete

**Phase**: Phase 1 — Cursor
**Criticality**: MEDIUM (D5 default)
**Confidence**: HIGH — sourced from current official `cursor.com/docs/skills`, [Verified]

**Current value**: Catalog's Cursor row, Skills surface column: "`.cursor/skills/`" (single path).

**Expected / Upstream value**: Per [Agent Skills | Cursor Docs](https://cursor.com/docs/skills),
Cursor now documents four native locations — project (`.agents/skills/`, `.cursor/skills/`) and
user (`~/.agents/skills/`, `~/.cursor/skills/`) — plus compatibility reads of `.claude/skills/`,
`.codex/skills/`, and their user-level equivalents.

**Drift description**: Same pattern as the GitHub Copilot D5 finding above — this repo's
`.claude/skills/<name>/SKILL.md` files may already be natively discoverable by Cursor via its
`.claude/skills/` compatibility read, which the single-path catalog cell doesn't convey. Reserved
status means no functional break today, only a catalog-completeness gap.

**Affected files**: `docs/reference/platform-bindings.md` — Cursor row, "Skills surface" column.

**Recommendation**: Update the catalog cell to "`.cursor/skills/` (also reads `.claude/skills/`
compatibility)" or similar, consistent with the GitHub Copilot D5 recommendation above. Low urgency
given Reserved status.

---

## Phase 1 — Windsurf

### Finding: D1/D2 Windsurf — Vendor rebrand to Devin Desktop; catalog row may be stale for the entire product identity

**Phase**: Phase 1 — Windsurf
**Criticality**: HIGH (D1/D2 default — this is not a path rename within the same product, it's a
full vendor/product identity change with an already-passed component-EOL date)
**Confidence**: HIGH — sourced from official `docs.devin.ai/desktop/*` pages (confirmed via 307
redirect from the legacy `docs.windsurf.com` domain) plus a dated vendor blog post
(`devin.ai/blog/windsurf-is-now-devin-desktop`)

**Current value**: Catalog lists this harness as "Windsurf", tool-specific surface
`.windsurf/rules/*.md` + `.windsurf/workflows/`, MCP "global only", custom-agent surface "not
officially documented", skills `.windsurf/skills/` "(unverified)". Status: "Reserved".

**Expected / Upstream value**: Cognition (Windsurf's owner) **rebranded Windsurf to "Devin Desktop"
on 2026-06-02**. The agent formerly called Cascade is on a documented path to end-of-life
**2026-07-01** — a date that has already passed as of this audit (today: 2026-07-20) — being
replaced by "Devin Local." `docs.windsurf.com` now 307-redirects to `docs.devin.ai/desktop/*`.
Legacy `.windsurf/rules/*.md`, `.windsurf/workflows/`, and `.windsurfrules` still function as
documented **fallbacks**, but the new preferred rules location is `.devin/rules/` (workflows'
`.devin/` equivalent unconfirmed by the research). MCP remains global-only for the Desktop product
(`~/.codeium/windsurf/mcp_config.json`, path itself apparently unchanged despite the rebrand) — a
separate Devin CLI product has its own project-level `.devin/config.json`, not to be conflated.
Skills are now officially documented at `.windsurf/skills/<name>/` (workspace) +
`~/.codeium/windsurf/skills/` (global), removing the prior "(unverified)" caveat — plus
cross-agent discovery of `.agents/skills/` and, if enabled, `.claude/skills/`. Custom-agent
discovery remains ambiguous for the Desktop surface specifically: Devin Local documentation
references "spawns independent subagents" and shares a harness with Devin CLI (which documents
`.devin/agents/AGENT.md`, importing Claude Code agent frontmatter), but no Desktop-specific page
explicitly confirms this directory applies outside the CLI product.

**Drift description**: The single largest finding in this audit. The catalog's entire row identity
("Windsurf") may now refer to a discontinued/rebranded product name. Whether this repo's catalog
should be updated to a "Windsurf (rebranded to Devin Desktop, 2026-06-02)" framing — mirroring how
the Amazon Q → Kiro CLI succession is already handled in this same catalog — is a judgment call for
a human/maker, not something this checker resolves. Given Reserved status (no binding files
committed), there is no functional break in this repo today, but the catalog text itself is now
describing a legacy/fallback surface as if it were the current one, which is exactly the class of
staleness this audit exists to catch.

**Affected files**: `docs/reference/platform-bindings.md` — Windsurf row (all columns) and Status
column; potentially the row header/name itself.

**Recommendation**: Human/maker review — treat this the same way the Amazon Q Developer → Kiro CLI
succession was handled in this catalog: add a "Windsurf (superseded by Devin Desktop — see below)"
framing with a dedicated succession subsection citing the 2026-06-02 rebrand, the 2026-07-01
Cascade EOL date (already passed), and the `.devin/rules/` preferred-location change, while noting
that legacy `.windsurf/*` paths remain functional fallbacks for now. Do not silently rename the row
without a documented justification, per this catalog's own established pattern.

---

## Phase 1 — JetBrains Junie

### Finding: D1 JetBrains Junie — [INFO] No drift detected

**Phase**: Phase 1 — JetBrains Junie
**Criticality**: N/A
**Confidence**: FALSE_POSITIVE (no drift; not counted in findings total)

**Current value** (catalog): `.junie/AGENTS.md` outranks root `AGENTS.md`.

**Upstream value** (web-researcher + self-confirmation WebFetch, [Verified] against
`junie.jetbrains.com/docs/guidelines-and-memory.html`): confirmed unchanged. Documented discovery
order verbatim: "1. `.junie/AGENTS.md` file in the project root. 2. `AGENTS.md` file in the project
root. 3. `.junie/guidelines.md` file or `.junie/guidelines/` folder – Junie's legacy format for
guidelines (still supported)." `.junie/AGENTS.md` still outranks root `AGENTS.md` exactly as
cataloged. Project-level also outranks the global `~/.junie/AGENTS.md` equivalent.

**Drift description**: None on D1.

**Recommendation**: No action required.

---

### Finding: D2 JetBrains Junie — `.junie/rules/*.md` no longer the documented mechanism

**Phase**: Phase 1 — JetBrains Junie
**Criticality**: HIGH (D2 default)
**Confidence**: HIGH — self-confirmed via direct `WebFetch` of
`junie.jetbrains.com/docs/guidelines-and-memory.html`, quoting the exact three-tier discovery list
verbatim; the page explicitly enumerates all three tiers with no fourth "`.junie/rules/`" tier
present.

**Current value**: Catalog's JetBrains Junie row, "Tool-specific instruction surface" column:
"`.junie/AGENTS.md`, `.junie/rules/*.md` (imports `.claude/agents/`, `.codex/agents/`,
`.claude/skills/`)".

**Expected / Upstream value**: Per current official docs, the discovery mechanism is a three-tier
precedence chain — `.junie/AGENTS.md` → root `AGENTS.md` → `.junie/guidelines.md` file or
`.junie/guidelines/` folder ("Junie's legacy format for guidelines, still supported"). No
`.junie/rules/*.md` tier is documented anywhere on this page. The cross-tool import behavior
(detecting `.cursor/agents/`, `.claude/agents/`, `.codex/agents/` and suggesting import) is real and
confirmed, but lives under the separate **Custom subagents** doc
(`junie-cli-subagents.html`), not under "rules" — i.e., the import behavior the catalog attributes
to a rules directory actually belongs to the agent-discovery and skill-discovery mechanisms (D4/D5,
unaffected — see below), not to a `.junie/rules/` guidelines directory.

**Drift description**: The catalog cell conflates two things that are now (or always were,
per this research) documented as functionally distinct: (a) the guidelines/instructions discovery
chain, which never included a `.junie/rules/*.md` tier per this fetch, ending instead at
`.junie/guidelines.md`/`.junie/guidelines/` (legacy), and (b) the cross-tool import suggestions
that actually live on the agent/skill discovery pages. This repo has status "Reserved" for Junie
(no `.junie/` files committed — confirmed, only narrative/governance mentions exist), so there is
no functional break today, only a catalog-accuracy gap.

**Affected files**: `docs/reference/platform-bindings.md` — JetBrains Junie row, "Tool-specific
instruction surface" column.

**Recommendation**: Update the catalog cell to reflect the three-tier chain (`.junie/AGENTS.md` →
root `AGENTS.md` → `.junie/guidelines.md`/`.junie/guidelines/` legacy format) and move the
cross-tool import parenthetical to the D4/D5 (agent/skill surface) cells where it factually belongs,
or drop the parenthetical from this cell entirely since it's not part of the rules-discovery
mechanism itself.

---

### Finding: D3-D5 JetBrains Junie — [INFO] No drift detected

**Phase**: Phase 1 — JetBrains Junie
**Criticality**: N/A
**Confidence**: FALSE_POSITIVE (no drift; not counted in findings total)

**Current value** (catalog): MCP `.junie/mcp/mcp.json`, custom-agent surface `.junie/agents/`,
`.agents/`, skills surface `.junie/skills/<name>/SKILL.md`.

**Upstream value** (web-researcher, [Verified] against
`junie.jetbrains.com/docs/{junie-cli-mcp-configuration,junie-cli-subagents,agent-skills}.html`):
all three confirmed unchanged verbatim, including the cross-tool detection/import-suggestion
behavior for `.cursor/agents/`, `.claude/agents/`, `.codex/agents/` (D4) and `.cursor/skills/`,
`.claude/skills/`, `.codex/skills/` (D5).

**Drift description**: None on D3-D5.

**Recommendation**: No action required.

---

## Phase 1 — Google Antigravity CLI

### Finding: D1-D5 Google Antigravity CLI — primary docs unreachable; secondary-source signals suggest possible D3/D4 drift, unconfirmed

**Phase**: Phase 1 — Google Antigravity CLI
**Criticality**: MEDIUM (D3 default) for the MCP-path signal; HIGH (D4 default) for the
custom-agent-format signal — both provisional pending primary-source confirmation, so treat these
criticality labels as upper bounds, not settled
**Confidence**: MEDIUM across all five dimensions — web-researcher explicitly could not reach a
single primary Google doc page with rendered text (all `antigravity.google/docs/*` pages returned
bare titles with no body, apparently JS-rendered SPAs). Every claim below is sourced from secondary
material (official GitHub repo issues/discussions, Google's AI Developers Forum) with the
researcher explicitly flagging several other secondary sources it found as likely AI-generated
content-farm sites it discounted. Per this checker's confidence-propagation rule, [Needs
Verification] tags map to MEDIUM confidence — none of this reaches HIGH.

**Current value** (catalog): native `AGENTS.md` read since v1.20.3 with `GEMINI.md` ranking above
it; rules `.agent/rules/*.md`; MCP `~/.gemini/antigravity/mcp_config.json` (global only, "no
confirmed project-level path"); custom-agent surface "runtime-orchestrated (no declarative file)";
skills `.agents/skills/<name>/SKILL.md`.

**Expected / Upstream value (unconfirmed, secondary sources only)**:

- D1 (root file): Convergent secondary sources still describe `AGENTS.md` read with `GEMINI.md`
  winning conflicts — no change signal. The "v1.20.3" figure appears in one forum thread but a
  direct changelog fetch returned implausible data (2024 dates, version 1.1.4 — evidently a render
  failure, not real content). **No actionable drift** — treat as unchanged pending better sourcing.
- D2 (rules dir): `.agent/rules/*.md` still cited once, but other sources describe skills having
  moved from legacy singular `.agent/skills` to canonical `.agents/skills` — raising unconfirmed
  doubt whether rules made the same singular-to-plural move (`.agent/rules/` → `.agents/rules/`).
- D3 (MCP config) — **strongest signal of the five**: an official-repo GitHub issue
  (`google-antigravity/antigravity-cli` #60) shows the CLI now _discovers_ a project-level path,
  `<workdir>/.antigravitycli/mcp_config.json`, though it's reported as silently ignoring the
  `mcpServers` field (open bug) — i.e., a project-level path may now exist, contradicting the
  catalog's "no confirmed project-level path," but in a broken/unreliable state. The global path is
  also cited two conflicting ways in secondary sources
  (`~/.gemini/antigravity-cli/mcp_config.json` vs. `~/.gemini/config/mcp_config.json`), neither
  matching the catalog's `~/.gemini/antigravity/mcp_config.json` verbatim.
- D4 (custom-agent surface): changelog snippets and a GitHub Discussion thread suggest subagents
  moved from JSON to a Markdown+YAML-frontmatter `agent.md` declarative format, which would
  contradict the catalog's "runtime-orchestrated (no declarative file)" claim if confirmed.
- D5 (skills): `.agents/skills/<name>/SKILL.md` was the most consistently corroborated item across
  independent secondary sources — **no change signal**.

**Drift description**: Two possible substantive changes (D3, D4) surfaced only through
lower-confidence secondary sourcing; two dimensions (D1, D5) show no change signal; one (D2) has a
weak, unconfirmed doubt. Given Reserved status (no binding files committed for this harness), there
is zero functional break in this repo today regardless of outcome.

**Affected files**: `docs/reference/platform-bindings.md` — Google Antigravity CLI row (all
columns, pending confirmation).

**Recommendation**: Do not update the catalog on this evidence alone — confidence is capped at
MEDIUM specifically because primary sources were unreachable. Follow-up options: (a) re-run the
research with a JS-rendering-capable fetch tool against `antigravity.google/docs/*`, or (b) treat
this as low priority given Reserved status and defer re-verification until this harness is
un-reserved (i.e., until this repo actually ships `.agent/`/`.antigravitycli/` binding files, at
which point accurate D3/D4 documentation becomes load-bearing).

---

## Phase 1 — Pi (pi.dev)

### Finding: D1/D2 Pi — `.pi/AGENTS.md` is not a documented path; project `AGENTS.md` is root-level only

**Phase**: Phase 1 — Pi (pi.dev)
**Criticality**: HIGH (D1/D2 default)
**Confidence**: HIGH — sourced from `pi.dev/docs/latest/{quickstart,settings}` plus the upstream
GitHub repo docs (`github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/settings.md`),
[Verified]; the tool was confirmed real and well-documented (project `earendil-works/pi`, npm
`@earendil-works/pi-coding-agent`), ruling out the "wrong Pi product" risk the research prompt was
built to guard against.

**Current value**: Catalog's Pi row, "Tool-specific instruction surface" column: "`.pi/AGENTS.md`,
`.pi/SYSTEM.md` (also reads `CLAUDE.md`)".

**Expected / Upstream value**: Pi loads `AGENTS.md` or `CLAUDE.md` at startup from three
concatenated locations — global (`~/.pi/agent/AGENTS.md`), then walking up parent directories, then
the current directory (i.e., project-level `AGENTS.md` lives at the **repo root**, not inside a
`.pi/` subdirectory). `.pi/AGENTS.md` as a project-level path is **not documented anywhere** in
current Pi docs. Only the global counterpart lives under a `.pi/`-style path
(`~/.pi/agent/AGENTS.md`). `.pi/settings.json` and `.pi/SYSTEM.md` (project system-prompt override,
with an `APPEND_SYSTEM.md` variant) are confirmed correct as-is.

**Drift description**: The catalog conflates two different things: root-level `AGENTS.md` discovery
(no `.pi/` prefix) and genuinely `.pi/`-scoped files (`settings.json`, `SYSTEM.md`). This isn't
necessarily new drift (unclear whether `.pi/AGENTS.md` was ever a real path or was a documentation
error at catalog-authoring time) — either way, it doesn't match current docs. This repo has status
"Reserved" for Pi (no binding files committed), so there is no functional break today, only a
catalog-accuracy gap.

**Affected files**: `docs/reference/platform-bindings.md` — Pi row, "Tool-specific instruction
surface" column, and the "Reads root AGENTS.md natively?" column (should clarify root, not
`.pi/`-scoped).

**Recommendation**: Correct the catalog cell to: root `AGENTS.md` (repo root or any ancestor
directory, concatenated with global `~/.pi/agent/AGENTS.md`) or `CLAUDE.md`; `.pi/SYSTEM.md`
(project system-prompt override, `.pi/APPEND_SYSTEM.md` variant also exists). Drop `.pi/AGENTS.md`
from the cell entirely.

---

### Finding: D3-D5 Pi — [INFO] No drift detected, one clarification

**Phase**: Phase 1 — Pi (pi.dev)
**Criticality**: N/A
**Confidence**: FALSE_POSITIVE (no drift; not counted in findings total)

**Current value** (catalog): MCP "none (intentionally no native MCP)"; custom-agent surface "none
built-in (extension-based)"; skills surface "`.agents/skills/` or `.pi/skills/`".

**Upstream value** (web-researcher, [Verified] against `pi.dev/docs/latest/{extensions,skills}`):
D3 and D4 confirmed unchanged verbatim — no native MCP by deliberate design (third-party community
extensions exist but are unofficial), custom agents remain extension-based
(`~/.pi/agent/extensions/`, `.pi/extensions/`, TypeScript modules). D5 confirmed functionally
correct with one wording clarification: both `.agents/skills/` and `.pi/skills/` are simultaneously
valid discovery paths (an "and," not the catalog's implied "or" — both load together, they are not
mutually exclusive alternatives), plus global equivalents `~/.agents/skills/` and
`~/.pi/agent/skills/` not previously recorded.

**Drift description**: None functionally on D3-D5; the "or" in the catalog's skills cell is a minor
wording imprecision, not a substantive divergence (below this audit's conservative drift threshold
for flagging).

**Recommendation**: No action required for D3/D4. Optional low-priority wording tweak for D5
("`.agents/skills/` and `.pi/skills/`" instead of "or") — not urgent, does not meet the substantive
drift bar on its own.

---

## Phase 1 — Aider

### Finding: D1-D5 Aider — [INFO] No drift detected

**Phase**: Phase 1 — Aider
**Criticality**: N/A
**Confidence**: FALSE_POSITIVE (no drift; not counted in findings total)

**Current value** (catalog): `CONVENTIONS.md` requires explicit `--read` or `.aider.conf.yml`; no
MCP, no custom-agent surface, no skills surface.

**Upstream value** (web-researcher, [Verified] against `aider.chat/docs/{usage/conventions,config/options}.html`
and site-restricted searches for AGENTS.md/MCP mentions, both zero hits): all five dimensions
confirmed unchanged. Exact quote confirming D1/D2: "It's best to load the conventions file with
`/read CONVENTIONS.md` or `aider --read CONVENTIONS.md`," with `.aider.conf.yml` `read:` key as the
persistent alternative. No AGENTS.md auto-discovery. No MCP, custom-agent, or skill mechanisms
exist natively (third-party unofficial bridges exist for MCP but are not part of Aider itself).

**Drift description**: None.

**Recommendation**: No action required. This repo's "Reserved (`CONVENTIONS.md` not yet provided)"
status remains accurate.

---

## Phase 1 — D6 Committed Binding File Conformance

### Finding: D6 Claude Code — required-field completeness confirmed; one stylistic `tools:` format outlier

**Phase**: Phase 1 — Claude Code (D6)
**Criticality**: LOW — demonstrably non-breaking (see verification below); pure style
inconsistency, not a functional degradation
**Confidence**: HIGH — directly observed via `Grep` + `Read`, mechanical comparison

**Current value**: All 82 `.claude/agents/*.md` files have both required fields (`name`,
`description`) present — zero missing, confirmed by sweeping every file. 81 of 82 use the
documented comma-separated-string form for `tools:` (e.g., `tools: Read, Edit, Write, Bash, Grep,
Glob`). One file, `repo-setup-manager.md`, uses YAML flow-sequence array notation instead:
`tools: [Read, Bash, Glob, Grep]`.

**Expected / Upstream value**: Per the Claude Code Phase 1 research ([Verified],
`code.claude.com/docs/en/sub-agents`), the current official file-based frontmatter examples show
`tools:` as a comma-separated string; the bracket-array form is documented only for the separate
`--agents` CLI/JSON declaration path, not the `.claude/agents/*.md` file format.

**Drift description**: Purely a style-consistency outlier in one file, not a functional break —
verified by re-running `npm run generate:bindings` in Invariant 3 above, which produced a
byte-identical `.opencode/agents/repo-setup-manager.md` output (`permission: {bash: allow, glob:
allow, grep: allow, read: allow}`), confirming `rhino-cli`'s YAML parser correctly handles both the
comma-separated-string and flow-sequence-array forms for this field. No agent's tool grants are
mis-parsed.

**Affected files**: `.claude/agents/repo-setup-manager.md` (line with `tools: [Read, Bash, Glob,
Grep]`).

**Recommendation**: Optional low-priority normalization — change to `tools: Read, Bash, Glob, Grep`
for fleet-wide consistency with the other 81 files and with the documented file-based convention.
Not urgent; zero functional impact confirmed.

---

### Finding: D6 OpenCode — [INFO] No drift detected

**Phase**: Phase 1 — OpenCode (D6)
**Criticality**: N/A
**Confidence**: FALSE_POSITIVE (no drift; not counted in findings total)

**Current value**: Swept all 82 `.opencode/agents/*.md` `permission:` blocks — zero use the
deprecated boolean-flags form (`true`/`false`); all use the current string form
(`allow`/`ask`/`deny`). Swept all `color:` values — four distinct values in use (`primary`,
`secondary`, `success`, `warning`), all valid current OpenCode theme tokens per the Phase 1
OpenCode research above (no named CSS colors like `blue` leaked through the converter).

**Drift description**: None.

**Recommendation**: No action required.

---

### Finding: D6 OpenAI Codex CLI — see D4 finding above

**Phase**: Phase 1 — OpenAI Codex CLI (D6)
**Criticality**: HIGH (cross-reference)
**Confidence**: MEDIUM (cross-reference)

The committed `.codex/config.toml` / `.codex/ci-monitor-subagent.toml` pair uses the
`[agents.<name>]` sub-table + `config_file` pointer declaration form, which the Phase 1 OpenAI
Codex CLI research above found to be superseded by standalone `.codex/agents/*.toml` files. This
is simultaneously a D4 (custom-agent surface) and D6 (committed binding file conformance) finding —
already fully documented under "D4 OpenAI Codex CLI — Custom-agent declaration mechanism
superseded" above, including the provenance caveat that this file is Nx/Codex tooling
infrastructure rather than `rhino-cli`-managed. Not re-duplicated here; see that entry for full
detail and recommendation.

---

## Summary

**Phase 0 (parity invariants)**: 0 findings (all 5 invariants PASS — CRITICAL: 0, HIGH: 0, MEDIUM: 0,
LOW: 0). No cross-vendor drift between `.claude/`, `.opencode/`, `.amazonq/` at commit
`6aea08047`. The `a158b0843` pr-review-fixer.md + mirror pair specifically confirmed genuinely
in sync — the prior-session post-staging-drift hazard did not recur.

**Phase 1 (external drift)**: 10 actionable findings (CRITICAL: 0, HIGH: 4, MEDIUM: 3, LOW: 2,
1 mixed/low-confidence bundle) + 1 pre-known item logged as no-drift per requester instruction +
numerous `[INFO] No drift detected` entries not counted toward the total (FALSE_POSITIVE
confidence, per the skill's no-drift handling).

**By harness** (Phase 1):

- **Claude Code**: 1 finding (LOW:1) — `repo-setup-manager.md` `tools:` array-style outlier
  (D6), demonstrably non-breaking
- **OpenCode**: 1 finding (LOW:1) — Skills surface catalog prose incomplete re: native
  `.opencode/skills/` path (D5)
- **OpenAI Codex CLI**: 1 finding (HIGH:1) — custom-agent declaration mechanism superseded,
  live `.codex/config.toml` uses the old form; Nx-tooling provenance caveat applies (D4/D6)
- **GitHub Copilot**: 2 findings (MEDIUM:2) — MCP config path incomplete (D3); Skills surface
  listed `n/a` may now be inaccurate, repo's `.claude/skills/` may already be live for Copilot
  (D5)
- **Cursor**: 1 finding (MEDIUM:1) — Skills surface catalog description incomplete re:
  `.claude/skills/` compatibility read (D5)
- **Windsurf**: 1 finding (HIGH:1) — **largest finding in this audit**: vendor rebranded to
  "Devin Desktop" 2026-06-02, Cascade EOL date (2026-07-01) already passed as of this audit;
  catalog row describes a legacy/fallback surface as current (D1/D2)
- **JetBrains Junie**: 1 finding (HIGH:1) — `.junie/rules/*.md` not a documented mechanism;
  actual chain is `.junie/AGENTS.md` → root `AGENTS.md` → `.junie/guidelines.md`/
  `.junie/guidelines/` legacy (D2), self-confirmed via direct WebFetch quote
- **Amazon Q Developer → Kiro CLI**: 0 findings — pre-known item, catalog already carries a
  same-day "Verified 2026-07-20" stamp per requester instruction, not re-researched
- **Google Antigravity CLI**: 1 finding (MEDIUM/HIGH mixed, capped MEDIUM confidence) — primary
  docs unreachable (JS-rendered SPA), secondary sources suggest possible D3 (project-level MCP
  path) and D4 (declarative agent format) changes, unconfirmed
- **Pi (pi.dev)**: 1 finding (HIGH:1) — `.pi/AGENTS.md` is not a documented path; project
  `AGENTS.md` is root-level, concatenated with global `~/.pi/agent/AGENTS.md` (D1/D2)
- **Aider**: 0 findings — full parity confirmed, no drift on any dimension

**Total findings**: 10 actionable (Phase 0: 0, Phase 1: 10)

**Overall status**: PASS WITH WARNINGS. No CRITICAL findings and no functional break in this
repo today (every harness with a Phase-1 HIGH finding — OpenAI Codex CLI, Windsurf, JetBrains
Junie, Pi — is either "Partial"/Nx-infrastructure-provenance or "Reserved" status with zero
committed binding files for the affected dimension, so nothing in this repo's live tooling is
currently broken). The Windsurf rebrand (HIGH) is the standout item worth prioritizing for human/
maker review given the already-passed Cascade EOL date. Recommend routing this report to
`repo-harness-compatibility-fixer` for the catalog-update-eligible findings (all Phase 1 findings
above are catalog/prose updates, not code fixes) and flagging the OpenAI Codex CLI D4/D6 finding
and the Windsurf D1/D2 finding for explicit human sign-off given their higher stakes (Nx-tooling
provenance boundary; full vendor-identity change respectively).

**Status**: Complete
**Completed**: 2026-07-20--19-43 (UTC+7)
