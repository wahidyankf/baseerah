# 1 · Just Enough Nvim (Primer, Neovim §)

**prd row**: Pass 0 · Editor Foundations · Primer · Neovim § · Learn 101 / Drill 201 · Nvim-ready Yes ·
VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: **vanilla latest Neovim with zero plugins/extensions** — editing fluency built entirely
on what ships in the box. Plugin management, LSP, DAP, Treesitter, and completion are deliberately **out
of scope here** and belong to [`03-extending-neovim`](./03-extending-neovim.md). Neovim is Apache-2.0
(Tier-1, DD-21); this primer precedes [`02-just-enough-lua`](./02-just-enough-lua.md), so configuration
is shown as `:set`/ex-commands, **not** Lua code.

## Why this exists · the big idea

- **The problem before the solution**: every later topic drives build/run/test/git from the terminal
  (DD-17); without a modal editor under your fingers you fight your tools instead of the problem.
- **Keep-this-if-you-forget-everything**: modal editing separates _moving and selecting_ from _inserting_,
  so plain keystrokes become a composable editing language (operator + motion + text object).
- **Big ideas touched**: `mechanism-vs-policy` — vanilla Neovim is pure **mechanism**; the **policy** (your
  config, plugins, LSP) is deliberately deferred to [`02`](./02-just-enough-lua.md) and
  [`03`](./03-extending-neovim.md).

## Prerequisites

**This is the entry point — it assumes no prior programming.**

- **Prior topics**: none.
- **Tools & environment**: a computer with a **macOS/Linux-compatible terminal** and the latest **Neovim**
  installed (`nvim --version`); nothing else. (Windows readers use WSL2 or Git Bash — DD-25.)
- **Assumed knowledge**: how to open a terminal and run a command; the willingness to learn modal editing.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28). Re-confirm version pins at authoring
> time (fast-moving).

- 2026-07-12 — verified: current stable Neovim is **v0.12.4** (2026-07-05). Vanilla `nvim` ships
  `:checkhealth`, built-in `:terminal`, and `:help` with zero plugins (all core). (neovim.io / github.com)
- 2026-07-12 — verified: Neovim license is **Apache-2.0** with Vim-license-derived portions dual-licensed;
  Tier-1 free-to-teach (DD-21). (github.com neovim/LICENSE.txt)
- 2026-07-12 — verified (stable, not literally re-quoted): default keymaps `<C-v>` blockwise, `<C-w>`
  window prefix, `gt` tab-next are unchanged Vim-heritage defaults; spot-check `:help` at authoring.

## Items

- **Install & launch**: getting the latest Neovim, `nvim`/`nvim <file>` from the terminal, the built-in
  `:help` system, `:checkhealth` (no config file needed).
- **Modes**: normal / insert / visual (charwise, linewise `V`, blockwise `<C-v>`) / command-line /
  replace; entering and leaving each; why modal editing exists.
- **Motions & operators**: `h/j/k/l`, word/WORD (`w`/`b`/`e`), line (`0`/`^`/`$`), file (`gg`/`G`), find
  (`f`/`t`/`;`/`,`), paragraph/sentence; the operator+motion grammar (`d`/`c`/`y` + motion), text objects
  (`iw`/`aw`/`i"`/`ip`/`i(`), counts.
- **Editing**: insert variants (`i`/`a`/`o`/`O`), change/delete/yank/put (`p`/`P`), `.` repeat,
  `u`/`<C-r>` undo/redo, join `J`, indent `>>`/`<<`, case toggles.
- **Buffers, windows, tabs**: `:e`/`:b`/`:bn`/`:bd`, splits (`:sp`/`:vsp`, `<C-w>` navigation/resize), tab
  pages (`:tabnew`/`gt`); the buffer-vs-window-vs-tab mental model.
- **Ex-commands**: ranges, `:w`/`:wa`/`:q`/`:x`, `:%s///` substitution with flags, `:g/pattern/cmd`
  global commands, `:normal`, `:read`/`:!`.
- **Search & replace**: `/`/`?`/`n`/`N`, `*`/`#`, incremental search, very-magic patterns, `:s` with
  capture groups & `\zs`/`\ze`.
- **Registers**: named/numbered/yank/black-hole/clipboard (`"+`), viewing `:reg`, pasting from a register.
- **Marks & jumps**: `m{a-z}`, `` `a ``/`'a`, the jumplist (`<C-o>`/`<C-i>`), the changelist.
- **Macros**: record `q{reg}`, replay `@{reg}`/`@@`, count-prefixed replay, editing a macro via a
  register.
- **Quickfix & location lists**: `:vimgrep`/`:grep`, `:copen`, `:cnext`/`:cprev`; the built-in
  `:terminal`.

## Worked examples

All in vanilla Neovim, no plugins → colocated under `just-enough-nvim/learning/code/`. Each example is a
before/after file pair plus the exact keystroke transcript (DD-30 follow-along).

- **beginner** — Edit a file using only motions+operators (no arrow keys/mouse): a `:%s///g` refactor
  with confirmation; undo/redo a mistake. Maps to `by-example/beginner`.
- **intermediate** — A recorded macro applied across many lines with a count; a `:g/…/normal` transform;
  split windows editing two buffers side by side. Maps to `by-example/intermediate`.
- **advanced** — A `:vimgrep` → quickfix → `:cnext`-driven multi-file edit; a register-composed edit; a
  `:terminal` build/run loop beside the source — the raw-form workflow every later topic assumes (DD-17).
  Maps to `by-example/advanced`.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: perform one non-trivial refactor of a small multi-file text project **entirely in vanilla
  Neovim** — no plugins, no mouse, no arrow keys — driving find/replace, macros, and the quickfix list,
  and capture the full keystroke transcript so the session is reproducible.
- **Concepts exercised**: [ ] modal editing [ ] operator+motion grammar [ ] text objects [ ] `:%s///`
  with capture groups [ ] a recorded macro replayed with a count [ ] `:vimgrep`→quickfix→`:cnext`
  multi-file edit [ ] registers [ ] `:terminal` build/run loop.
- **Ordered steps**:
  1. `just-enough-nvim/learning/capstone/code/before/` — seed 3 small text/source files with a repeated
     symbol to rename and a list to transform. Verify `ls` shows the seed files.
  2. Open in Neovim; rename the symbol across all files via `:vimgrep /oldName/ **/*` → `:copen` →
     `:cdo s/oldName/newName/g | update`. Verify `:cnext` walks every hit and `git diff`/`diff -r` shows
     the rename applied everywhere.
  3. Record a macro `qa … q` that reformats one list line; replay with a count `10@a`. Verify all lines
     reformatted identically.
  4. Run the project's check from `:terminal` (e.g. `python3 -m py_compile *.py` or a `grep` assertion)
     beside the source. Verify the terminal reports success.
  5. Save the keystroke transcript to `just-enough-nvim/learning/capstone/code/transcript.md`.
- **Acceptance criteria**: the `after/` tree differs from `before/` exactly by the intended refactor; the
  transcript reproduces it from scratch; no plugin, mouse, or arrow key was used.
- **Done bar**: runnable end-to-end (a reader following the transcript reaches the identical `after/`
  tree) + web-verified.

## Read more

**Books**

- **Practical Vim: Edit Text at the Speed of Thought** — Drew Neil (2nd ed., 2015). The classic guide to Vim's composable command grammar, still the most recommended path to editing fluency.
- **Learning the vi and Vim Editors** — Arnold Robbins, Elbert Hannah, Linda Lamb (8th ed., O'Reilly). The long-running comprehensive vi/Vim reference, basics to power-user scripting.

**Papers & articles**

- **Neovim User Documentation (`:help`)** — Neovim core team. Authoritative version-matched reference for Neovim's modal model, commands, options. <https://neovim.io/doc/user/>
- **Vim/Neovim built-in tutorial (`vimtutor` / `:Tutor`)** — Bram Moolenaar; Neovim team. Original hands-on modal-editing intro shipped with the editor.

---

← Previous: [README (syllabus index)](./README.md) · Next: [2 · Just Enough Lua](./02-just-enough-lua.md) →
