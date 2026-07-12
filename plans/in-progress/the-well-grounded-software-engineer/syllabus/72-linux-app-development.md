# 72 · Linux App Development ◆ (By Example, Python)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Python ◆ · Learn 172 / Drill 272 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `◆` app-domain — building real Linux applications (CLIs and daemons) as an app developer:
the process/runtime model, filesystem & I/O, argument parsing/config/logging, IPC/subprocess, packaging &
distribution, and daemons/scheduling with graceful shutdown — in Python (no `†`: Python is the native
teaching language here). The kernel-level view is [`75-linux-os`](./75-linux-os.md).

## Why this exists · the big idea

- **The problem before the solution**: a CLI that ignores exit codes or mixes errors into stdout, and a
  daemon that dies mid-work on a signal, are unusable in the pipelines and init systems they live in —
  this topic exists to build programs that behave correctly as citizens of the Unix process model.
- **Keep-this-if-you-forget-everything**: a well-behaved Linux program honours the contract the OS already
  defines — args, exit codes, stdout-vs-stderr, and signals — so it composes in pipelines and shuts down
  cleanly under `systemd`.
- **Big ideas touched**: `layering-and-leaks` — an app rides on the process/runtime model (env, file
  descriptors, signals), and those OS-level contracts leak straight into how your program must behave;
  `coupling-vs-cohesion` — config, logging, and IPC kept as separable concerns let a CLI and its daemon
  share one core without tangling.

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md),
  [topic 5 Just Enough Bash](./05-just-enough-bash.md) (env/args/exit codes/
  signals), and [topic 11 Backend Essentials](./11-backend-essentials.md) (long-running service intuition).
- **Tools & environment**: a **Linux** machine (or WSL/VM); **Python 3.x**; virtualenv/packaging tooling;
  `systemd` (for the daemon lifecycle example) or an equivalent init; Neovim/VSCode (DD-17).
- **Assumed knowledge**: Python functions + files (topic 04); shell env/args/exit codes/signals (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: `pyproject.toml`-based packaging (PEP 517/518/621) + `venv` remain the current
  standard; `subprocess`/signal-handling stdlib idioms, systemd-unit basics, Unix exit-code + stdio
  conventions, and `pytest` are all evergreen/unchanged. No version/license-sensitive claims to correct.

## Items

- Scope: **native Linux apps** — CLIs, daemons, and services that are first-class OS citizens (signals,
  exit codes, systemd), plus a survey of GTK/Qt for native desktop GUI.
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

## Read more

**Books**

- **GTK+/Gnome Application Development** — Havoc Pennington (1999, New Riders). A classic, widely cited GTK/GNOME app-development book written by a core GNOME developer.

**Papers & articles**

- **GTK documentation** — The GTK Project, official. The authoritative API reference for the GTK toolkit. <https://docs.gtk.org/>
- **Qt documentation** — The Qt Project, official. The authoritative reference for the Qt cross-platform toolkit. <https://doc.qt.io/>
- **XDG Base Directory Specification** — freedesktop.org, official standard. The canonical spec governing where Linux apps store config, data, and cache files. <https://specifications.freedesktop.org/basedir/latest/>
- **Debian Policy Manual** — The Debian Project, official. The canonical packaging standard referenced across Debian-derived Linux distributions. <https://www.debian.org/doc/debian-policy/>

---

← Previous: [71 · Windows App Development](./71-windows-app-development.md) · Next: [73 · Building Production CLI Tools](./73-building-production-cli-tools.md) →
