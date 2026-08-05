# Orphaned harness-binding artifacts with no hand-authored source

One-line summary: `ose-private` carries a stale `.opencode/agents/ci-monitor-subagent.md` with no
`.claude/` source behind it — either restore the source or delete the mirror, and close the guard gap
that lets the whole class of source-less binding artifacts pass validation silently.

> Demoted from a full `backlog/` plan to a two-pager on 2026-08-05.

## Problem / context

The multi-harness binding model in this ecosystem has exactly one hand-authored surface, `.claude/`,
and treats `.opencode/`, `.cursor/`, and `.amazonq/` as mechanically emitted mirrors of it. That
invariant is what makes "never hand-edit a mirror" a safe rule: every mirror file is supposed to be
reproducible from a source file. A mirror with no source breaks the invariant in the direction the
tooling does not look. Nothing regenerates it, nothing validates it, and nothing tells a reader
whether it is a deliberate carve-out or leftover debris. `ose-private` has one such file — an
OpenCode agent mirror for a CI-monitoring subagent whose `.claude/agents/` counterpart no longer
exists — surfaced during the Cursor platform-binding adoption work and deferred rather than resolved.

The same class exists in this repository, in a different shape. The exact artifact is absent: neither
`.claude/agents/` nor `.opencode/agents/` here carries a `ci-monitor-subagent.md`, and those two
directories are otherwise file-for-file identical. But `.opencode/` also holds two surfaces that have
no `.claude/` counterpart at all:

- `.opencode/skills/` contains seven skill directories — `link-workspace-packages`, `monitor-ci`,
  `nx-generate`, `nx-import`, `nx-plugins`, `nx-run-tasks`, `nx-workspace` — and **none** of the
  seven has a matching `.claude/skills/<name>/SKILL.md`.
- `.opencode/commands/monitor-ci.md` exists, while `.claude/commands/` does not exist as a directory
  at all.

These are Nx tooling provisions rather than emitter output, which is a legitimate reason for them to
exist — but no document says so, and two guards that look like they would catch the condition do not.
`validate_no_synced_skills` in `apps/rhino-cli/src/application/agents/sync_validator.rs` reports a
`.opencode/skills/<name>` directory as an offender **only when** a `.claude/skills/<name>/SKILL.md`
exists to mirror; all seven source-less directories therefore pass clean. And
`validate_catalog_coverage` in `apps/rhino-cli/src/application/agents/bindings.rs` checks catalog
coverage at top-level directory granularity — it asks whether the catalog text contains `.opencode`,
which the OpenCode row trivially satisfies — so no subdirectory under a listed binding root is ever
required to appear in the catalog. `docs/reference/platform-bindings.md` describes OpenCode's binding
as `.opencode/agents/` plus native reads of `.claude/skills/`; it mentions neither `.opencode/skills/`
nor `.opencode/commands/`.

The catalog does handle the analogous cases well elsewhere, which shows the shape of the fix: it
carries an explicit "Provenance of pre-existing partial bindings" section explaining that `.codex/`
is Codex/Nx infrastructure the emitter never writes, and it records that the GitHub Copilot
CI-monitor artifacts were deliberately removed. Both of those decisions are legible. The
`ose-private` orphan and this repo's `.opencode/` extras are not.

## Why now

The Cursor platform-binding adoption added a fourth generated mirror, so the cost of an ambiguous
"is this generated or not?" artifact now multiplies across four harness surfaces instead of two. Every
future harness adoption re-asks the same question of every unexplained file. The finding is also
already localized to a single named file in `ose-private`, which is the cheapest state it will ever be
in — once someone hand-edits it, or a harness starts reading it, the decision stops being reversible.
Separately, both guard gaps are small, well-bounded code changes in a CLI that already owns the
validation surface, so the remediation does not depend on any upstream vendor timeline.

## Prior art / precedents

- **Multi-Harness Binding convention** — defines the two-tier binding model and the primary/mirror
  relationship that a source-less mirror violates.
  [multi-harness-binding](../../repo-governance/conventions/structure/multi-harness-binding.md)
- **Platform Bindings reference** — already carries a "Provenance of pre-existing partial bindings"
  section for `.codex/`; the natural home for an equivalent entry covering tool-provisioned
  `.opencode/` subdirectories. [platform-bindings](../../docs/reference/platform-bindings.md)
- **`repo-harness-compatibility-checker`** — the agent that validates cross-vendor parity invariants
  and would be the natural owner of an orphan-detection check.
  [checker agent](../../.claude/agents/repo-harness-compatibility-checker.md)
- **Agent-skills README** — documents the "OpenCode reads `.claude/skills/` natively, no mirror"
  rule and the `No Synced Skill Mirror` check whose predicate the orphans slip past.
  [skills README](../../.claude/skills/README.md)
- **Harness binding catalog drift two-pager** — the sibling brief covering _external_ catalog drift;
  this one covers _internal_ artifacts absent from the catalog, and the two should be triaged
  together. [harness-binding-catalog-drift](./harness-binding-catalog-drift.md)

## Proposed direction (sketch)

Treat the `ose-private` file as one instance and the guard gap as the actual deliverable.

- **Decide the single file first.** For the `ose-private` OpenCode CI-monitor mirror, pick one of
  three: restore the `.claude/agents/` source and let the emitter regenerate it; delete the mirror
  outright; or keep it and record it as a documented tool-provisioned carve-out. Deletion is the
  reversible default when no harness demonstrably reads it.
- **Generalize the skill-mirror predicate.** Change the check so a directory under `.opencode/skills/`
  with no `.claude/` source is _reported_ rather than ignored — as a distinct outcome from the
  existing mirror-of-a-live-source failure, since the remedy differs (document versus delete).
- **Push catalog coverage down a level.** Require named binding subdirectories, not just binding
  roots, to appear in the catalog, so a new tool-provisioned surface forces a catalog sentence.
- **Add an allowlist with reasons, not exemptions by silence.** Tool-provisioned surfaces stay, but
  each is listed once with a dated reason in the catalog — the same treatment `.codex/` already gets.
- **Apply the sweep per repository.** Each of the four repos has its own harness surface and its own
  set of tool provisions; a fix that only names the sites this brief cites is not a fix of the class.

## Rough scope & non-goals

In scope: resolving the named `ose-private` orphan; broadening the skill-mirror check to detect
source-less directories; deepening catalog coverage to named subdirectories; adding catalog prose for
tool-provisioned `.opencode/` surfaces; a per-repo audit producing an explicit verdict per unmatched
artifact.

Out of scope:

- Changing what the emitter writes for `.claude/` → `.opencode/`, `.cursor/`, `.amazonq/` agent
  mirrors — that pipeline is working and is not implicated.
- Any change to `.codex/` provenance, which the catalog already documents.
- Vendoring, editing, or re-authoring the Nx-provisioned `monitor-ci` and `nx-*` skill content; this
  is about whether the artifacts are accounted for, not about their contents.
- External harness-convention drift — that belongs to the harness-binding-catalog-drift brief.
- The Amazon Q to Kiro CLI succession and any other vendor-lifecycle work.
- Anything touching `ose-private` infrastructure configuration; the governance/tooling shape of the
  problem is the entire subject here.

## Risks & open questions

- Whether any harness actually reads `.opencode/skills/` and `.opencode/commands/`, or whether
  OpenCode's native `.claude/skills/` read makes them dead weight. If they are read, deleting them
  removes working capability. (open)
- Whether the Nx tooling **rewrites** these directories on install or generator runs. If it does, a
  deletion silently reappears and any guard that fails on their presence becomes a recurring false
  alarm. (open)
- Whether the `ose-private` file has been hand-edited since it lost its source — if so, deleting it
  discards content that exists nowhere else, and the decision needs a diff first. (open)
- Broadening the skill-mirror check could start failing in the sibling repos, whose tool-provisioned
  sets may differ; the change needs to land with each repo's allowlist populated, not before.
- Subdirectory-level catalog coverage is a stricter gate that raises the cost of every future tool
  integration. Worth confirming the stricter rule is wanted before implementing it, rather than
  discovering it as friction later.
- Guard changes touch `apps/rhino-cli`, which sits under a cross-repo byte-identity boundary for three
  of the four repos while this repo carries a fork — so the same source change may need different
  landing mechanics per repo. (open)

## What success looks like + promotion signal

Success: every file under a generated binding directory in every repo either has a `.claude/` source
that reproduces it, or is named in the platform-bindings catalog with a dated reason for existing
without one. A newly appearing source-less artifact fails a guard instead of sitting unnoticed, and
the failure message distinguishes "delete this stale mirror" from "document this tool provision".

Promotion signal: promote to a `backlog/` plan once two things are settled — a confirmed answer on
whether the Nx tooling regenerates `.opencode/skills/` and `.opencode/commands/` on install (which
decides delete-versus-document, and therefore the whole shape of the guard), and a per-repo inventory
of source-less binding artifacts across all four repos, so the allowlist can be written as part of
the same change rather than discovered during it.
