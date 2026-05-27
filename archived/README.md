# Archived Applications

This directory contains previously active applications that have been superseded by newer implementations.

## Contents

| Directory             | Archived   | Reason                                       | Successor                                          |
| --------------------- | ---------- | -------------------------------------------- | -------------------------------------------------- |
| `organiclever-web/`   | 2026-03-29 | Replaced by fullstack rebuild                | [`apps/organiclever-web`](../apps/organiclever-web/) |
| `rhino-cli/`          | 2026-05-23 | Go binary replaced by Rust rewrite           | [`apps/rhino-cli`](../apps/rhino-cli/)             |
| `ayokoding-cli/`      | 2026-05-25 | Go binary replaced by Rust rewrite           | [`apps/ayokoding-cli`](../apps/ayokoding-cli/)     |
| `ose-cli/`            | 2026-05-25 | Go binary replaced by Rust rewrite           | [`apps/ose-cli`](../apps/ose-cli/)                 |
| `crane-cli/`          | 2026-05-26 | F# source replaced by Rust rewrite           | [`apps/crane-cli`](../apps/crane-cli/)             |

## Notes

- Archived apps are excluded from the Nx workspace, CI pipelines, and Docker builds
- Git history is preserved via `git mv` — use `git log --follow` to trace file history
- These apps are kept for reference only and should not be modified
