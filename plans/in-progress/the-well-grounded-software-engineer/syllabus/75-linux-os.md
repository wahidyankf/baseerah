# 75 · Linux OS (By Example, C + shell †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · C + shell † · Learn 175 / Drill 275 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the Linux OS from the inside — kernel vs user space, syscalls, the process model
(fork/exec/wait), signals, virtual memory/paging, filesystems (inodes/fd/VFS), scheduling, and IPC —
observed from C + shell tooling (`/proc`, `strace`, `ps`/`top`). The app-developer view is
[`72-linux-app-development`](./72-linux-app-development.md); cross-OS contrast is
[`76-windows-os`](./76-windows-os.md).

## Why this exists · the big idea

- **The problem before the solution**: every program you run is lied to by the kernel — it believes it owns
  the CPU and all of memory — and when performance, concurrency, or a crash forces you underneath that
  illusion, you need to know what the OS is actually doing. This topic goes inside.
- **Keep-this-if-you-forget-everything**: the kernel provides mechanism — fork/exec, virtual memory, the
  VFS, scheduling — through a small syscall interface, and user space decides policy on top; the boundary
  between them is the whole design of the OS.
- **Big ideas touched**: `mechanism-vs-policy` — the kernel supplies the machinery (process creation,
  paging, fd/VFS) while leaving what and when to user space, and the syscall boundary is that split made
  concrete; `layering-and-leaks` — virtual memory and the process abstraction hide the hardware until
  paging, context switches, or `strace` make the layer visible.

## Prerequisites

- **Prior topics**: [topic 74 Just Enough C](./74-just-enough-c.md) (the language for syscalls) and
  [topic 5 Just Enough Bash](./05-just-enough-bash.md) (`/proc`, `ps`,
  `strace`).
- **Tools & environment**: a **Linux** machine (or VM/WSL2); **gcc/clang** + make; `strace`, `/proc`,
  `ps`/`top`; Neovim/VSCode (DD-17).
- **Assumed knowledge**: C pointers + structs + a `make` build (topic 74); shell process/job basics
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
- Vendor-neutral OS theory: these Linux mechanisms are one implementation of universal concepts —
  processes, virtual memory, scheduling, filesystems, and IPC — that recur (differently named) in every OS.

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

## Read more

**Books**

- **The Linux Programming Interface** — Michael Kerrisk (2010, No Starch Press). The canonical, comprehensive Linux/UNIX systems-programming reference.
- **How Linux Works**, 3rd ed. — Brian Ward (2021, No Starch Press). The widely recommended canonical guide to Linux internals and administration for working engineers.
- **Linux Kernel Development**, 3rd ed. — Robert Love (2010, Addison-Wesley). A classic, accessible guide to kernel internals by a Linux/Android kernel engineer.
- **Advanced Programming in the UNIX Environment**, 3rd ed. — W. Richard Stevens & Stephen A. Rago (2013, Addison-Wesley). "APUE" — the classic, still-foundational UNIX/Linux systems-programming reference.

**Papers & articles**

- **The Linux man-pages project** — Michael Kerrisk et al., official (kernel.org project). The canonical, free reference for Linux syscalls and library calls. <https://man7.org/linux/man-pages/>

---

← Previous: [74 · Just Enough C](./74-just-enough-c.md) · Next: [76 · Windows OS](./76-windows-os.md) →
