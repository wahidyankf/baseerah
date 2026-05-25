---
title: "BRD: Harness/Vendor Neutrality Blueprint — Phase 1"
---

# Business Requirements Document: Harness/Vendor Neutrality Blueprint — Phase 1

## Business Goal

Establish a **harness/vendor neutrality blueprint** for the ose-\* ecosystem — a framework
defining where vendor names are permitted, where they are forbidden, how violations are
detected, and how compliance propagates to downstream repositories.

The first concrete deliverable: replace the vendor-locked `sync:claude-to-opencode` npm script
with a unified, vendor-neutral `generate:bindings` script that regenerates **all** secondary
binding artifacts (OpenCode + Amazon Q) in a single command. Remove the old script completely
(hard delete — no alias, no passthrough).

## Business Impact

### Current Pain Points

**1. Silent correctness gap** [Repo-grounded]

`npm run sync:claude-to-opencode` only runs `agents sync` (OpenCode). It never runs
`agents emit-bindings` (Amazon Q). Every agent, hook, and documentation instruction that says
"run `sync:claude-to-opencode` after editing agents" silently leaves Amazon Q bindings stale.

The Phase 0 Invariant 3 check in `repo-harness-compatibility-checker` uses:

```bash
npm run sync:claude-to-opencode && git diff --quiet .opencode/
```

It passes even when `.amazonq/` is out of date — a correctness hole in the parity gate.
[Repo-grounded: `repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md`
line ~150]

**2. Vendor-locked naming** [Repo-grounded]

`sync:claude-to-opencode` encodes two vendor names (Claude, OpenCode) in a script name that
lives in the shared `package.json`. The repo's governance explicitly requires vendor neutrality
in shared artifacts (see Multi-Harness Binding Convention). This creates friction when
onboarding new harnesses — contributors ask "does this script handle my harness?"

**3. No formal blueprint** [Judgment call]

The repo has a governance vendor-independence convention and a multi-harness-binding convention,
but no single document that defines what "harness neutrality" means as a system: which zones
are neutral, which are vendor-specific, how violations are detected, and how the rule propagates
across the ose-\* ecosystem. This gap means violations (like `sync:claude-to-opencode`) emerge
naturally over time with no systematic enforcement.

**4. Ecosystem name clarity** [Judgment call]

The word `sync` describes a push/pull operation with a remote, not artifact generation. The
word `emit` is compiler-internal terminology (rustc, tsc) — not idiomatic for user-facing npm
scripts. The `generate:` namespace is used in comparable pipelines (Prisma, GraphQL Code
Generator, OpenAPI Generator) for artifact generation, though the exact `generate:bindings`
form is a design decision, not an ecosystem standard. [Judgment call — not a web-confirmed
idiom; our naming choice is principled but not yet widespread.]

## Affected Roles

- **AI agents** — every agent definition that instructs agents to run `sync:claude-to-opencode`
  after editing `.claude/` files; these will use `generate:bindings` after this plan
- **Pre-commit hook** (via `rhino-cli git pre-commit`) — already calls rhino-cli directly, not
  via the npm script; no change needed [Repo-grounded: `.husky/pre-commit`]
- **Harness compatibility checker** — Phase 0 Invariant 3 uses the npm script; must be updated
  to use `generate:bindings` [Repo-grounded:
  `repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md`]
- **Human contributors** reading docs — will find `generate:bindings` in instructions
- **ose-primer** — downstream template; receives propagated changes via PR from
  `repo-ose-primer-propagation-maker`
- **ose-infra** — private infra repo; adopts `generate:bindings` if/when it ships agent
  bindings (conditional adoption; see README)

## Business-Level Success Metrics

1. **Observable fact**: `grep -r "sync:claude-to-opencode" --include="*.md" --include="*.json" . | grep -v "node_modules\|\.git\|target/\|generated-reports/\|plans/"` returns zero matches
   after migration. The old name is completely absent — no alias, no passthrough, no stale reference.

2. **Observable fact**: `npm run generate:bindings && git diff --quiet .opencode/ .amazonq/`
   exits 0 immediately after a fresh edit-and-generate cycle (no stale Amazon Q files).

3. **Observable fact**: `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance vendor-audit repo-governance/` exits 0 — governance prose is clean of vendor names outside exempt sections.

4. **Judgment call**: Future agent instructions will be unambiguous — `generate:bindings` is
   sufficient for all harnesses; contributors no longer wonder whether their harness is covered.

## Business-Scope Non-Goals

- Renaming `rhino-cli` CLI subcommands (`agents sync`, `agents emit-bindings`) — implementation
  details not visible in npm scripts
- Removing `sync:agents` / `sync:skills` / `sync:dry-run` targeted scripts — valid for focused
  operations
- Adding new harnesses — this plan does not change which harnesses are supported
- Exhaustive audit of all vendor-name violations (only the npm script violation is in scope for
  Phase 1; the blueprint framework enables future phases to address other violations)

## Business Risks and Mitigations

| Risk                                                               | Mitigation                                                                                                         |
| ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| Old `sync:claude-to-opencode` name in long-lived generated reports | Reports are historical; checkers skip `generated-reports/` by convention; no active use                            |
| Missing a documentation reference during bulk rename               | Delivery checklist includes explicit zero-match grep-verify step BEFORE committing; any miss is caught before push |
| `generate:bindings` runs `emit-bindings` which adds latency        | `agents emit-bindings` is fast (deterministic overwrite, no web I/O); acceptable                                   |
| ose-primer propagation delay — old name persists in primer briefly | propagation-maker opens a PR immediately; primer's old name is in a draft PR, not merged                           |
| Blueprint too narrow — new violations emerge later                 | Blueprint defines enforcement mechanisms (vendor-audit + quality gates) that catch future violations continuously  |
