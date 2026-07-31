# Technical Design: Coverage Artifact Relative Paths

## Defect Class

A generated build/test artifact tracked in git that embeds environment-specific state (an absolute
filesystem path) instead of being either untracked or environment-independent. This class recurs
anywhere a coverage/build tool writes absolute paths into an output file that then gets committed.

## Proposed Investigation

- Confirm the exact mechanism: which .NET coverage tool (Coverlet) writes
  `libs/fsharp-crane-core/tests/unit/coverage.json`, and whether it has a relative-path or
  path-mapping option.
- Search the repo for other tracked coverage/build artifacts with the same risk
  (`**/coverage.json`, `**/coverage.xml`, `**/*.lcov` under `libs/*/tests/**`, `apps/*/tests/**`).
- Decide the fix per artifact class found:
  - **Gitignore**: stop tracking the file entirely; regenerate on demand, never commit.
  - **Relative-path emission**: configure the tool (or post-process the output) to emit paths
    relative to the repo root, making the file safe to track.
- If any coverage-gate tooling (e.g. `test:coverage` threshold enforcement) reads the tracked file
  rather than regenerating it, confirm the chosen fix doesn't break that gate.
