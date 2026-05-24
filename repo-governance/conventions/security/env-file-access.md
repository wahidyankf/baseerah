---
title: "Environment File Access Convention"
description: AI agents must not directly read, write, edit, or commit any .env* file except .env.example. Codifies the guard-env-file-access policy including the script carve-out, trust boundary, git-commit prevention, cross-platform enforcement, and known gaps.
category: explanation
subcategory: conventions
tags:
  - security
  - env-files
  - agents
  - guard-env-file-access
created: 2026-05-24
---

# Environment File Access Convention

AI agents operating in this repository must not directly read, write, edit, or commit any `.env*`
file except `.env.example`. The canonical identifier for this policy is **`guard-env-file-access`**,
which is the name used in hooks, plans, and cross-references throughout the codebase.

Real environment files (`.env`, `.env.local`, `.env.production`, and variants) contain machine-specific
configuration and secrets. They are gitignored, not shared across environments, and are changed
manually by the human maintainer. Exposing or accidentally committing them is a high-severity
secret-disclosure event.

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: The
  policy uses hard deny rules, not heuristics or prompts. Every enforcement mechanism states clearly
  which paths are blocked and which are allowed, leaving no ambiguity for agents or humans.
- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**:
  Multiple complementary automated mechanisms (declarative rules, hooks, gitignore, pre-commit guard)
  enforce the policy without requiring per-commit human review.
- **[Reproducibility First](../../principles/software-engineering/reproducibility.md)**: Keeping
  secrets out of version control is a prerequisite for reproducible, auditable builds. Once a secret
  enters git history, removing it requires destructive rewrites.
- **[Documentation First](../../principles/content/documentation-first.md)**: The policy is codified
  here as a governance rule so it is discoverable and enforced regardless of which agent platform or
  human contributor is involved.

## Purpose

This convention protects the repository from secret exposure via two main threat vectors:

1. An AI agent directly reading, writing, or editing a real `.env*` file — leaking its contents into
   a conversation or overwriting it with generated values.
2. An AI agent (or human using `git add -f`) staging and committing a real `.env*` file, which would
   embed secrets into git history permanently.

The `.env.example` file is a committed template that demonstrates required variables without holding
real values. It is the only `.env*` file agents need direct access to, and it is explicitly allowed.

## Scope

### What This Convention Covers

- Which `.env*` files agents may and may not access directly.
- The script carve-out that permits project scripts (under `apps/`, `libs/`, `scripts/`) to manage
  env files at runtime while still blocking direct agent manipulation.
- The trust boundary accepted by the project's maintainers regarding the carve-out.
- The git-commit prevention mechanisms that block any `.env*` file (except `.env.example`) from
  entering version history, regardless of actor.
- Where cross-platform enforcement details live (without reproducing config syntax).
- Known gaps in enforcement and their accepted compensating controls.

### What This Convention Does NOT Cover

- Server-side secret scanning or CI-based leak detection (separate concern).
- The internal syntax of platform-binding configuration files (that detail lives in the respective
  binding directories).
- How to write or structure `.env.example` content (out of scope for this convention).

## Standards

### Policy Statement

AI agents must not directly read, write, edit, or commit any `.env*` file except `.env.example`.
The human maintainer performs all changes to real environment files manually.

"Directly" means using the agent's own file-access tools (Read, Write, Edit, or equivalent). The
policy does not prohibit agents from invoking project scripts that manage env files as part of their
normal runtime behavior — see the Script Carve-Out section below.

### Allowed: `.env.example`

Agents may freely read, write, edit, and commit `.env.example` without any approval prompt. This
file is the committed template that documents required variables with placeholder values. Template
maintenance (adding new variables, updating documentation comments) is a normal agent task.

### Blocked: All Other `.env*` Files

Every other `.env*` file is hard-blocked for the agent's direct access. The blocked set includes but
is not limited to:

- `.env`
- `.env.local`
- `.env.*.local` (e.g. `.env.development.local`)
- `.env.development`
- `.env.production`
- `.env.staging`
- `.env.test`

The pattern is not a fixed enumeration — any filename matching `.env*` that is not exactly
`.env.example` is blocked. Platform enforcement uses both an enumerated deny list (defense in depth)
and a hook that catches arbitrary names the list does not enumerate.

### Script Carve-Out

Executable project scripts under `apps/`, `libs/`, and `scripts/` may read, write, delete, or
update `.env*` files at runtime as part of their normal function (for example, a setup script that
generates `.env.local` from a template, or a seed script that writes test credentials). Agents may
invoke these scripts.

The guard targets only the agent's _direct_ manipulation. An agent that runs
`bash scripts/setup-env.sh` is permitted; the same agent directly writing `.env.local` with its
own Write tool is not.

### Trust Boundary (Explicit, Accepted)

The script carve-out is bypassable in principle. An agent that can both edit script source under
`apps/`, `libs/`, or `scripts/` and execute those scripts could author a script that writes
`.env.local` and then invoke it.

This is a deliberate design choice made by the project's maintainers. Project scripts are trusted to
manage env files — they are version-controlled, human-reviewed, and serve legitimate purposes. The
residual risk of this bypass path is accepted, documented here, and not engineered away. The
pre-commit guard and gitignore remain as platform-agnostic backstops even if the bypass is used.

### Git-Commit Prevention

Real `.env*` files must never enter git history. Two mechanisms enforce this at the repository level,
applying to all actors (agents and humans alike):

**Gitignore coverage**: The root `.gitignore` excludes `.env`, `.env.local`, `.env.*.local`,
`.env.development`, `.env.production`, `.env.staging`, and `.env.test`. The template file is
force-unignored (`!.env.example`) so it remains committable. Standard `git add` cannot stage
gitignored files.

**Pre-commit guard**: The script at `scripts/check-no-env-staged.sh`, invoked from
`.husky/pre-commit`, inspects all staged files and rejects the commit if any staged path's basename
matches `.env*` and is not `.env.example`. This guard catches `git add -f` force-adds and any
variant that gitignore does not cover. The error message names the offending file and instructs
the contributor to unstage it. The guard exits 0 (allows the commit) when only `.env.example` is
staged.

Together, gitignore blocks unintentional staging and the pre-commit guard blocks intentional
force-adds. Both mechanisms are platform-agnostic — they apply regardless of which agent platform
or tool performed the operation.

### Cross-Platform Enforcement

The policy applies to every AI agent platform wired to this repository. Platform-binding enforcement
details live in the binding directories and must not be reproduced here (per the
[Governance Vendor-Independence Convention](../structure/governance-vendor-independence.md)).
The binding paths are:

- **`.claude/` platform binding**: `.claude/settings.json` (declarative allow and deny rules) and
  `.claude/hooks/block-env-file-access.sh` (PreToolUse hook — authoritative for direct file access
  and best-effort Bash guard).
- **`opencode.json` platform binding**: `opencode.json` `permission` block (path-scoped read and
  edit deny with `.env.example` allow using last-match-wins semantics).

Any future platform binding added to this repository must implement equivalent enforcement before
being considered production-ready for this repo.

### Known Gaps and Compensating Controls

The following gaps exist in the current enforcement posture. They are accepted with documented
compensating controls and are not silently left unaddressed.

**Bash guard is best-effort heuristic**: The PreToolUse hook in the `.claude/` platform binding
inspects Bash command strings using pattern matching. It cannot cover all indirect manipulation
paths — for example, creative piping, `sed -i` patterns, or subshell constructs that achieve the
same effect without spelling out a `.env*` filename literally. The pre-commit guard and gitignore
are the robust backstop for any write that reaches the filesystem.

**`opencode.json` bash permission is coarse-grained**: The `opencode.json` permission schema (as
of this rule's authoring) allows path-scoped deny for read and edit operations but does not support
command-level deny for Bash invocations. The `bash` permission uses `"*": "allow"`, meaning this
binding cannot express "deny Bash commands that directly manipulate `.env*`." The compensating
control is the pre-commit guard, which is platform-agnostic and catches any write that would be
staged, and the declarative read/edit deny, which blocks the more direct file-tool paths.

**Sandbox hardening is a future option**: The only OS-level block for indirect Bash manipulation in
the `.claude/` platform binding would be a filesystem-level sandbox (`denyRead`/`denyWrite`). This
would also block the script carve-out unless explicit allow lists are added for the script
directories. Adopting this is a future hardening option, not delivered as part of the current
policy. If adopted, it would require careful allow-list design to preserve the carve-out.

## Validation

The following checks confirm correct enforcement of this convention:

- An agent Read/Write/Edit on any real `.env*` file (e.g. `.env.local`, `.env.production`) is
  refused on every supported agent platform.
- An agent Read/Write/Edit on `.env.example` succeeds without a prompt.
- An agent invoking `bash scripts/some-setup.sh` or `npm run <task>` is not blocked by the guard.
- A direct Bash command targeting a real `.env*` file (e.g. `cat .env.local`,
  `echo X > .env.local`) is refused on platforms where the Bash guard is implemented.
- Staging a real `.env*` file (including via `git add -f`) and attempting a commit is rejected by
  the pre-commit guard, naming the offending file.
- Staging and committing `.env.example` succeeds.
- `git check-ignore .env.production` prints the path (gitignored); `git check-ignore .env.example`
  exits non-zero (not gitignored, committable).

The hook test harness at `.claude/hooks/block-env-file-access.test.sh` and the pre-commit guard
self-test at `.claude/hooks/guard-pre-commit-env.test.sh` encode these assertions as executable
cases.

## References

**Related Conventions:**

- [Governance Vendor-Independence Convention](../structure/governance-vendor-independence.md) —
  Why platform-binding config syntax is referenced by path here rather than embedded inline.

**Platform-Binding Enforcement Paths:**

- `.claude/settings.json` — primary platform binding declarative allow/deny rules
- `.claude/hooks/block-env-file-access.sh` — primary platform binding PreToolUse hook (authoritative)
- `.claude/hooks/block-env-file-access.test.sh` — hook test harness
- `scripts/check-no-env-staged.sh` — pre-commit guard script
- `.husky/pre-commit` — hook invocation point
- `opencode.json` — secondary platform binding permission block

**Agents:**

- `repo-rules-checker` — Validates that this convention is linked from indices and that the
  platform-binding enforcement files exist at the paths referenced above.
- `repo-rules-fixer` — Applies corrections when `repo-rules-checker` identifies drift from this
  convention.
