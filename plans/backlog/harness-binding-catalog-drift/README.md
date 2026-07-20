# Harness Binding Catalog Drift (2026-07-20 audit)

**Status**: Backlog
**Delivery Mode**: `worktree-to-pr`

## Why This Exists

A `repo-harness-compatibility-checker` run at ose-public commit `6aea08047` found that several rows
in the platform-binding catalog describe upstream harness conventions that have since moved. The
catalog is how this repo decides which binding files to emit and which instruction surfaces each
vendor reads, so a stale row is not cosmetic — it can mean we ship a binding file a harness no
longer reads, or omit one it now expects.

Phase 0 of that same run was clean: **all five deterministic parity invariants PASS**. Nothing is
broken between `.claude/`, `.opencode/`, and `.amazonq/`. This plan is entirely about _external_
drift.

## Source of Truth for This Plan

[`findings.md`](./findings.md) in this folder — a verbatim copy of the audit report originally
written to `generated-reports/harness-compat__3b8a20__2026-07-20--19-09__audit.md`.

The copy exists because `generated-reports/` is gitignored (`.gitignore:85`) by the
[Temporary Files convention](../../../repo-governance/development/infra/temporary-files.md) — audit
reports there are deliberately ephemeral. Referencing the original path alone would have left this
plan pointing at evidence that vanishes on the next clean.

**Read the report, not a summary of it.** The agent's own closing summary and the report body
disagree in at least one place: the summary presented JetBrains Junie as a HIGH finding, while the
report records that row as `FALSE_POSITIVE` — `.junie/AGENTS.md` still outranks root `AGENTS.md`,
confirmed verbatim against current vendor docs. Any triage that works from the summary will chase a
non-issue and may trust other summary claims it should not.

## Scope

Triage each Phase 1 finding in the report and decide, per row: update the catalog, update the
emitted binding files, or record the row as deliberately unchanged with a reason.

Findings the report rates highest — each still to be independently re-verified before acting, since
several carry MEDIUM confidence by the checker's own assessment:

- **Windsurf → Devin Desktop** (HIGH/HIGH): reported as a full vendor and product identity change
  as of 2026-06-02, with a component EOL date that has already passed, not a path rename inside the
  same product. The catalog row describes the pre-rebrand product.
- **OpenAI Codex CLI custom-agent declaration** (HIGH criticality, MEDIUM confidence): reported to
  have moved from `[agents.<name>]` sub-tables in `config.toml` to standalone `.codex/agents/*.toml`
  files. This repo has a live `.codex/config.toml` still using the older form. The report notes an
  Nx-tooling provenance caveat on that file — establish who owns it before editing.
- **GitHub Copilot MCP config path** (MEDIUM/HIGH) and **Copilot skills surface** (MEDIUM/MEDIUM):
  Copilot is reported to now read `.claude/skills/`, which if true means this repo's existing skills
  are already live there with no new files needed. The report flags this one as explicitly needing
  follow-up confirmation before action.
- **OpenCode skills surface** (LOW): catalog prose is incomplete rather than incorrect. The repo's
  chosen behavior is unaffected.

Amazon Q → Kiro CLI was logged as no-drift by request and not re-researched in this run; its
succession is already recorded in the catalog.

## Approach Notes

- Delegate the re-verification to `web-researcher` rather than acting on the report's citations
  directly. Vendor docs move, and several of these findings are MEDIUM confidence precisely because
  the checker could not fully settle them.
- A row that turns out to be correct as written should gain a dated note saying it was re-verified,
  so the next audit does not re-litigate it.
- Changes here affect `docs/reference/platform-bindings.md` and possibly the emitters. Any change to
  emitted output must keep the three-repo binding parity invariants green — re-run the checker's
  Phase 0 before merging.

## Related

- [Platform Bindings Reference](../../../docs/reference/platform-bindings.md)
- [Multi-Harness Binding Convention](../../../repo-governance/conventions/structure/multi-harness-binding.md)
