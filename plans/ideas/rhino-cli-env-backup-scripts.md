# .env backup scripts for rhino-cli

One-line summary: scripted backup/restore of the gitignored `.env*` files rhino-cli local development
depends on, so a lost or clobbered env is recoverable.

> Idea, added 2026-07-21 (original capture undated).

## Problem / context

rhino-cli local development relies on gitignored `.env*` files that, by policy, never enter git. That
policy is correct, but it also means a deleted or overwritten `.env` has no recovery path.
**Data point:** 0 backup/restore tooling exists today, so a clobbered `.env` is unrecoverable (no
baseline measured — the failure simply hasn't been counted).

## Why now

The env-file-access guardrails make ad-hoc manual copying awkward (agents cannot touch real `.env*`),
so a sanctioned scripted path is the clean way to make backups routine.

## Proposed direction (sketch)

- A script under `scripts/` (exempt from the agent env-file guardrail) that backs up and restores
  rhino-cli's `.env*` to a gitignored local location.
- Never commits, never prints secret values — just moves files between gitignored paths.

## Rough scope & non-goals

In scope: backup/restore of gitignored env files for local rhino-cli dev.

Out of scope (for now): a secrets manager; syncing env across machines; touching `.env.example` (which
is committed and needs no backup).

## Risks & open questions

- Where do backups live so they stay uncommitted and off any world-readable path? (open)
- Scope to rhino-cli only, or generalize to all apps' `.env*`? (open)

## What success looks like + promotion signal

Success: a clobbered rhino-cli `.env` is restorable from a local backup in one command, with no secret
ever entering git. Ready to promote once the backup-location question is settled.
