# 76 · Windows OS (By Example, C + PowerShell †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · C + PowerShell † · Learn 176 /
Drill 276 · Nvim-ready Partial · VSCode-ready Partial. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the Windows OS from the inside — user vs kernel mode, the Win32 API, the object/handle
model, the registry, processes/threads (`CreateProcess`), memory management, Win32 synchronization,
NTFS/async I/O — observed from C (Win32) + PowerShell tooling. The deliberate cross-OS contrast to
[`75-linux-os`](./75-linux-os.md).

## Why this exists · the big idea

- **The problem before the solution**: the same OS jobs — spawn a process, share memory, synchronize
  threads — that Linux does with fds and `fork`, Windows does with handles, objects, and `CreateProcess`;
  an engineer who knows only one model is blind on the other machine. This topic supplies the second lens.
- **Keep-this-if-you-forget-everything**: Windows is an object-and-handle OS — nearly every kernel resource
  is an object you hold by handle — which is the deep structural contrast to Linux's
  file-descriptor-and-`fork` model.
- **Big ideas touched**: `mechanism-vs-policy` — the Win32 API and object manager are the mechanism the OS
  exposes, and how you compose `CreateProcess`, handles, and sync primitives is your policy;
  `layering-and-leaks` — user-mode code sits on the Win32 subsystem over the kernel, and the handle/object
  model plus overlapped I/O are where that layering surfaces in your code.

## Prerequisites

- **Prior topics**: [topic 74 Just Enough C](./74-just-enough-c.md) (the language for Win32 calls),
  [topic 75 Linux OS](./75-linux-os.md) (the OS-concept baseline to contrast), and
  [topic 5 Just Enough Bash](./05-just-enough-bash.md) (shell/PowerShell
  fluency).
- **Tools & environment**: a **Windows** machine; a C toolchain (MSVC or MinGW) for Win32; **PowerShell**;
  Task Manager / Process Explorer for inspection; Neovim/VSCode (DD-17).
- **Assumed knowledge**: C pointers + structs (topic 74); the process/memory/IPC model from Linux to
  contrast (topic 75); shell basics (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: user-vs-kernel mode, Win32 API (`CreateProcess`, handles, object model, registry,
  subsystems), Win32 sync primitives (mutex/event/critical section), NTFS concepts, overlapped I/O,
  PowerShell inspection, and Task Manager / Process Explorer (Sysinternals) are evergreen/unchanged. The
  file pins no Windows release number — good; nothing to correct.

## Items

- Windows architecture: user vs kernel mode, the Win32 API, subsystems, the registry.
- Processes & threads: creation (`CreateProcess`), handles, the object model, scheduling.
- Memory management: virtual memory, working sets, heaps.
- Synchronization: Win32 mutexes / events / critical sections.
- Filesystem & I/O: NTFS concepts, handles, async / overlapped I/O intuition.
- Tooling: PowerShell for inspection; Task Manager / Process Explorer concepts.
- Vendor-neutral OS theory: Win32's handles/objects/working-sets are one implementation of the same
  universal concepts (processes, virtual memory, scheduling, IPC) seen in
  [`75-linux-os`](./75-linux-os.md) — learn the concept, then the vendor's names.

## Worked examples

Colocated under `windows-os/learning/code/`; C (Win32) + PowerShell on Windows (DD-20/DD-30).

- **beginner** — create a process with the Win32 API in C; enumerate processes via PowerShell.
- **intermediate** — a Win32 mutex / critical-section synchronization example.
- **advanced** — handle-based file I/O; inspect a running process's memory/handles with PowerShell +
  tooling.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: write a small Win32 C program that creates a child process (`CreateProcess`), coordinates two
  threads with a mutex/critical section, and does handle-based (overlapped) file I/O, then inspect it from
  PowerShell + Process Explorer — and write a short Windows-vs-Linux contrast against the topic-53 model.
- **Concepts exercised**: [ ] `CreateProcess` + the handle/object model [ ] Win32 thread synchronization
  (mutex/critical section) [ ] handle-based / overlapped file I/O [ ] PowerShell + tooling inspection
  [ ] a Windows-vs-Linux OS contrast.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a program that `CreateProcess`es a child + uses handles. Verify the
     child launches and handles are closed correctly (no leak).
  2. Add two threads synchronized by a mutex/critical section + handle-based file I/O. Verify no data race
     on the shared resource and the file round-trips.
  3. Inspect via PowerShell + Process Explorer, then write `contrast.md` (Windows handles/objects vs Linux
     fd/`/proc`; `CreateProcess` vs `fork`/`exec`). Verify the inspection matches the code and the contrast
     is concrete.
- **Acceptance criteria**: process creation + synchronization + handle I/O work with no leak/race;
  PowerShell inspection matches the running program; the Windows-vs-Linux contrast is concrete.
- **Done bar**: runnable end-to-end (Windows) + observed via tooling + web-verified.

## Read more

**Books**

- **Windows Internals, Part 1**, 7th ed. — Pavel Yosifovich, Alex Ionescu, Mark E. Russinovich, David A. Solomon (2017). The definitive deep-dive into Windows architecture, processes, threads, and memory management.
- **Windows Internals, Part 2**, 7th ed. — Andrea Allievi, Alex Ionescu, Mark E. Russinovich, David A. Solomon (2021). Companion volume covering storage, I/O, networking, the boot process, and Windows management mechanisms.
- **Troubleshooting with the Windows Sysinternals Tools**, 2nd ed. — Mark Russinovich, Aaron Margosis (2016). The official field guide to the Sysinternals toolset for diagnosing real-world Windows problems.
- **Windows System Programming**, 4th ed. — Johnson M. Hart (2010). Long-running canonical reference for the Windows API at the systems-programming level (processes, threads, memory, sync, IPC).

**Papers & articles**

- **Sysinternals** — Mark Russinovich et al. (Microsoft). Official free toolkit and documentation; the de facto standard suite for Windows internals investigation and troubleshooting. <https://learn.microsoft.com/en-us/sysinternals/>

---

← Previous: [75 · Linux OS](./75-linux-os.md) · Next: [77 · System Programming](./77-system-programming.md) →
