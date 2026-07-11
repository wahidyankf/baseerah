# 51 · Linux App Development ◆ (By Example, Python)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Python ◆ · Learn 151 / Drill 251 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `◆` app-domain — building real Linux applications (CLIs and daemons) as an app developer:
the process/runtime model, filesystem & I/O, argument parsing/config/logging, IPC/subprocess, packaging &
distribution, and daemons/scheduling with graceful shutdown — in Python (no `†`: Python is the native
teaching language here). The kernel-level view is [`53-linux-os`](./53-linux-os.md).

## Prerequisites

- **Prior topics**: [topic 04 Just Enough Python](./04-just-enough-python.md),
  [topic 05 Just Enough Bash](./05-just-enough-bash.md) (env/args/exit codes/
  signals), and [topic 09 Backend Essentials](./09-backend-essentials.md) (long-running service intuition).
- **Tools & environment**: a **Linux** machine (or WSL/VM); **Python 3.x**; virtualenv/packaging tooling;
  `systemd` (for the daemon lifecycle example) or an equivalent init; Neovim/VSCode (DD-17).
- **Assumed knowledge**: Python functions + files (topic 04); shell env/args/exit codes/signals (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: `pyproject.toml`-based packaging (PEP 517/518/621) + `venv` remain the current
  standard; `subprocess`/signal-handling stdlib idioms, systemd-unit basics, Unix exit-code + stdio
  conventions, and `pytest` are all evergreen/unchanged. No version/license-sensitive claims to correct.

## Items

- The Linux process/runtime model as seen from an app: env, args, exit codes, signals.
- Filesystem & I/O: paths, permissions, file descriptors, streams, temp files.
- Building CLIs: argument parsing, config, logging, exit-code discipline.
- IPC & processes: `subprocess`, pipes, sockets (survey), environment/config.
- Packaging & distribution: virtualenvs, dependencies, systemd-unit intuition, containers (survey).
- Daemons & scheduling: long-running services, cron, graceful shutdown on signals.
- **Applied testing**: `pytest` over the CLI/daemon; testing signal handling & `subprocess` calls.

## Worked examples

Colocated under `linux-app-development/learning/code/`; each runnable + tested on Linux (DD-20/DD-30).

- **beginner** — a well-behaved CLI (args, `--help`, exit codes, stderr vs stdout).
- **intermediate** — a script that shells out via `subprocess` with error handling; signal-handled graceful
  shutdown (+ a test).
- **advanced** — a small long-running daemon with logging + a systemd-style lifecycle; a pipe/IPC example.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a well-behaved Linux CLI **and** a companion long-running daemon in Python — proper
  argument parsing/config/logging, exit-code discipline, `subprocess`/IPC, and a signal-handled graceful
  shutdown with a systemd-style lifecycle — packaged as a distributable, covered by `pytest`.
- **Concepts exercised**: [ ] a CLI with args/`--help`/exit codes/stdio discipline [ ] config + logging
  [ ] `subprocess`/pipe IPC with error handling [ ] a long-running daemon [ ] signal-handled graceful
  shutdown [ ] packaging (venv/pyproject) [ ] `pytest` over the CLI + daemon.
- **Ordered steps**:
  1. `.../learning/capstone/code/cli.py` — the CLI (args, `--help`, exit codes, stderr/stdout, config,
     logging). Verify `--help` works and a bad input exits non-zero to stderr.
  2. Add `subprocess`/pipe IPC with error handling. Verify a failed child process is handled and surfaced.
  3. `daemon.py` — a long-running daemon with a SIGTERM-handled graceful shutdown + a systemd-style
     lifecycle. Verify it starts, logs, and shuts down cleanly on signal.
  4. Package it (venv/pyproject) + `pytest`. Verify it installs into a clean venv and the tests (incl.
     signal handling) pass.
- **Acceptance criteria**: the CLI follows exit-code + stdio discipline; IPC errors are handled; the daemon
  shuts down gracefully on SIGTERM; the package installs cleanly; `pytest` passes.
- **Done bar**: runnable end-to-end (Linux) + tests green + web-verified.

---

← Previous: [50 · Windows App Development](./50-windows-app-development.md) · Next: [52 · Just Enough C](./52-just-enough-c.md) →
