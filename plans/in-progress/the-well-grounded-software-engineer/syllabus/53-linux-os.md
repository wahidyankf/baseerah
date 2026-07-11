# 53 · Linux OS (By Example, C + shell †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · C + shell † · Learn 153 / Drill 253 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the Linux OS from the inside — kernel vs user space, syscalls, the process model
(fork/exec/wait), signals, virtual memory/paging, filesystems (inodes/fd/VFS), scheduling, and IPC —
observed from C + shell tooling (`/proc`, `strace`, `ps`/`top`). The app-developer view is
[`51-linux-app-development`](./51-linux-app-development.md); cross-OS contrast is
[`54-windows-os`](./54-windows-os.md).

## Prerequisites

- **Prior topics**: [topic 52 Just Enough C](./52-just-enough-c.md) (the language for syscalls) and
  [topic 05 Just Enough Bash](./05-just-enough-bash.md) (`/proc`, `ps`,
  `strace`).
- **Tools & environment**: a **Linux** machine (or VM/WSL2); **gcc/clang** + make; `strace`, `/proc`,
  `ps`/`top`; Neovim/VSCode (DD-17).
- **Assumed knowledge**: C pointers + structs + a `make` build (topic 52); shell process/job basics
  (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: kernel-vs-user space, syscalls (fork/exec/wait, signals, `mmap`), virtual
  memory/paging, filesystems (inodes, fd, VFS, permissions, mounts), `/proc`, `ps`/`top`, `strace` are
  evergreen OS interfaces/terminology, unchanged. The file pins no kernel/distro version — good; nothing to
  correct.

## Items

- Kernel vs user space; system calls; the process model (fork/exec/wait), PIDs, signals.
- Memory: virtual memory, paging, the process address space, `mmap` intuition.
- Filesystems: inodes, file descriptors, the VFS, permissions, mounts.
- Scheduling: processes vs threads, context switches, priorities (concept).
- IPC: pipes, signals, shared memory, sockets.
- The shell & tooling: `ps`/`top`/`strace`/`/proc`, observing a running system from the terminal.

## Worked examples

Colocated under `linux-os/learning/code/`; C + shell against a live Linux system (DD-20/DD-30).

- **beginner** — `fork`/`exec`/`wait` in C; inspect a process via `/proc` and shell tools.
- **intermediate** — signal handling in C; a pipe between two processes.
- **advanced** — shared-memory IPC; `strace` a program and read its syscalls; an `mmap`'d file example.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: write a small C program that spawns children (`fork`/`exec`/`wait`), coordinates them with
  signals + a pipe and shares data via shared memory, then observe the whole thing from the shell —
  inspecting `/proc`, tracing its syscalls with `strace`, and confirming its memory map — a hands-on tour
  of the process/memory/IPC model.
- **Concepts exercised**: [ ] `fork`/`exec`/`wait` process control [ ] signal handling [ ] a pipe between
  processes [ ] shared-memory IPC [ ] `/proc` + `ps` inspection [ ] `strace` syscall reading + an `mmap`
  view.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a parent that `fork`/`exec`/`wait`s a child + handles a signal. Verify
     the child runs, the parent reaps it, and the signal is handled.
  2. Add a pipe + a shared-memory segment between two processes. Verify data crosses the pipe and the shared
     segment.
  3. Observe: inspect the running process via `/proc` + `ps`, `strace` it, and view its `mmap`. Verify the
     `strace` output shows the expected syscalls and `/proc` reflects the process state.
- **Acceptance criteria**: process control + signals + pipe + shared memory all work; `/proc`/`ps`
  inspection and `strace` show the expected syscalls; the memory map is explained.
- **Done bar**: runnable end-to-end (Linux) + observed via tooling + web-verified.

---

← Previous: [52 · Just Enough C](./52-just-enough-c.md) · Next: [54 · Windows OS](./54-windows-os.md) →
