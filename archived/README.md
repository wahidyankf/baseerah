# Archived Applications

This directory contains previously active applications that have been superseded by newer implementations.

## Contents

| Directory             | Archived   | Reason                                | Successor                                      |
| --------------------- | ---------- | ------------------------------------- | ---------------------------------------------- |
| `ayokoding-web-hugo/` | 2026-03-24 | Replaced by Next.js 16 implementation | [`apps/ayokoding-web`](../apps/ayokoding-web/) |
| `rhino-cli/`          | 2026-05-23 | Go binary replaced by Rust rewrite    | [`apps/rhino-cli`](../apps/rhino-cli/)         |

## Notes

- Archived apps are excluded from the Nx workspace, CI pipelines, and Docker builds
- Git history is preserved via `git mv` — use `git log --follow` to trace file history
- These apps are kept for reference only and should not be modified
