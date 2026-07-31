# Technical Design: Cross-Repo Port Registry

## Defect Class

A cross-repo resource-allocation concern (port numbers) currently tracked only in per-repo prose
tables (`docs/reference/monorepo-structure.md`). Nothing machine-readable spans repos, so
collision detection depends entirely on a human reading four separate tables correctly.

## Proposed Investigation

- Enumerate every currently-allocated port across the four repos (`ose-public`, `ose-primer`,
  `ose-private`, `baseerah`) by reading each repo's `docs/reference/monorepo-structure.md`.
- Decide the registry's home: a shared file synced via the existing `ose-public`/`ose-primer`
  parity loop, a `repo-config.yml` key checked independently per repo, or a dedicated
  cross-repo file outside the parity loop (given `baseerah` and `ose-private` don't participate in
  it).
- Decide the validator's home: a new `rhino-cli` subcommand, or a lightweight script wired into an
  existing Nx target (e.g. a `repo-config.yml` validation step).
- Decide enforcement point: CI-blocking on port allocation, or a checker-report warning.
