# Business Requirements: Coverage Artifact Relative Paths

## Problem Statement

A git-tracked, generated coverage file (`libs/fsharp-crane-core/tests/unit/coverage.json`) bakes in
the absolute filesystem path of whichever checkout last regenerated it. Any checkout at a different
path — a second clone, a worktree, a different developer's home directory — dirties the tree the
moment it runs `test:quick`, with a diff that carries zero information about actual code changes.
This is pure noise that erodes trust in `git status` and `git diff` output.

## Impact

**Affected roles**: any engineer or AI agent running `test:quick`/`test:coverage` on
`fsharp-crane-core` (or any other lib with a similarly-tracked .NET coverage artifact) from a
checkout path different from whoever last committed the file — including the routine multi-repo,
multi-worktree setup already in use under `/Users/wkf/ose-projects/`.

## Success Metrics

Running `test:quick`/`test:coverage` from any checkout path produces zero unrelated diff in
coverage artifacts, verified by running the suite from two different absolute paths and diffing
`git status` output.

## Risks

- **Undecided approach could stall the fix.** Gitignoring vs. relative-path emission are both
  reasonable; Phase 1 resolves which one before any change lands.
- **Scope creep to other libs/apps.** This learning was observed on one file
  (`fsharp-crane-core`); Phase 1 must confirm whether the same tracked-absolute-path pattern exists
  elsewhere before deciding whether the fix is single-file or repo-wide.
