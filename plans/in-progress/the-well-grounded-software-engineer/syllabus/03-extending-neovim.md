# 3 · Extending Neovim (By Example, Lua †)

**prd row**: Pass 0 · Editor Foundations · By Example · Lua † · Learn 103 / Drill 203 · Nvim-ready Yes ·
VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: turn vanilla Neovim ([topic 1](./01-just-enough-nvim.md)) into a real IDE-grade forge
using the Lua learned in [topic 2](./02-just-enough-lua.md) — plugin management, LSP, Treesitter,
autocommands/user commands, and a tiny self-authored plugin. This is the payoff topic of Pass 0; every
later topic assumes this forge exists (DD-17). Neovim and every plugin/LSP used are OSS (Tier-1, DD-21).

## Why this exists · the big idea

- **The problem before the solution**: vanilla Neovim edits text; real work also needs diagnostics,
  syntax-awareness, and a config you can reproduce on a new machine — this topic turns the editor into a
  versioned **forge** every later topic assumes (DD-17).
- **Keep-this-if-you-forget-everything**: your editor is code — the config is a Lua program in git, so your
  environment is reproducible and diffable, not a pile of clicked settings.
- **Big ideas touched**: `mechanism-vs-policy` — you now add the **policy** (plugins, LSP, keymaps) onto
  vanilla Neovim's **mechanism**; `abstraction-and-its-cost` — LSP and Treesitter are language-agnostic
  abstractions that buy uniform tooling across languages.

## Prerequisites

- **Prior topics**: [topic 1 Just Enough Nvim](./01-just-enough-nvim.md) (modal editing fluency) and
  [topic 2 Just Enough Lua](./02-just-enough-lua.md) (config is written in Lua).
- **Tools & environment**: a macOS/Linux terminal; the latest **Neovim** (`nvim --version`); **git** (to
  clone/manage the config and plugins); network access to fetch plugins; a working
  `~/.config/nvim` location. A language runtime for the LSP demo (Python 3.x) installed.
- **Assumed knowledge**: reading/writing basic Lua tables and functions (from topic 02); using the
  terminal and git.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28). **Two fast-moving items below —
> re-check immediately before authoring.**

- 2026-07-12 — verified: current stable Neovim **v0.12.4** (2026-07-05). The recommended LSP path is now
  **native `vim.lsp.config('name', {...})` + `vim.lsp.enable('name')`** (Neovim 0.11+); `nvim-lspconfig`'s
  role has shifted to supplying `lsp/*.lua` config files consumed via `vim.lsp.enable()` (requires
  0.11.3+). **Teach the native API as primary**, not the legacy `require('lspconfig').xyz.setup()`.
  (github.com neovim/nvim-lspconfig)
- 2026-07-12 — verified (CORRECTION, time-sensitive): Neovim 0.12 ships a **built-in native plugin
  manager `vim.pack`** (zero external dep, `packpath` start/opt model) — name it as the raw-form-aligned
  default alongside lazy.nvim (still valid for richer lazy-loading). **`nvim-treesitter` was archived
  2026-04-03**: old `master` is frozen (0.11 compat only); a `main`-branch rewrite requires Nvim 0.12+ and
  its successor/maintenance status is fluid — **re-verify the correct plugin/version to pin at authoring**.
  (github.com nvim-treesitter discussions; echasnovski.com)
- 2026-07-12 — verified: `nvim_create_autocmd(event, opts)` + `nvim_create_user_command(name, command,
opts)` (and `vim.opt`/`vim.g`/`vim.keymap.set`) signatures are current. (neovim.io API docs)
- 2026-07-12 — verified: XDG layout — `$XDG_CONFIG_HOME/nvim/init.lua` (default `~/.config/nvim`), `lua/`
  on `runtimepath`; inspect via `stdpath('config')`. (neovim.io / archwiki)

## Items

- **Neovim's Lua config**: `init.lua`, the `vim.opt`/`vim.g`/`vim.keymap` API, `runtimepath`, XDG config
  layout — all from the terminal.
- **Structuring a config**: a `lua/` module tree, `require`, lazy-loading intuition.
- **Plugin management** with a plugin manager (e.g. lazy.nvim): declaring, loading, configuring plugins.
- **LSP**: `nvim-lspconfig` + built-in LSP client, attaching a language server, diagnostics, code
  actions; note **Neovim 0.11+ native `vim.lsp.config()`/`vim.lsp.enable()`** as the emerging path.
- **Treesitter**: syntax-aware highlighting & text objects.
- **Autocommands & user commands**: `vim.api.nvim_create_autocmd`, `nvim_create_user_command`.
- **Writing a tiny plugin**: a Lua module exposing a command; a minimal statusline/keymap tweak.
- **DAP intro**: debugging from inside the editor (cross-ref the raw-form-first stance, DD-17).

## Worked examples

Colocated under `extending-neovim/learning/code/`; each ends with a **complete runnable config listing**
(DD-20) and states the exact launch command + observable result (DD-30).

- **beginner** — a from-scratch `init.lua` setting options + keymaps, run and reloaded from the terminal
  (`nvim -u init.lua`). Maps to `by-example/beginner`.
- **intermediate** — wire an LSP server + Treesitter for one language; a custom user command in Lua.
  Maps to `by-example/intermediate`.
- **advanced** — a small self-authored plugin (a Lua module with an autocommand + command) placed on the
  `runtimepath`, with a complete runnable config listing at the end. Maps to `by-example/advanced`.

**Materials note**: Neovim and every plugin/LSP used are OSS (Tier-1, DD-21).

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a complete, from-scratch Neovim configuration repository that turns vanilla Neovim into
  an IDE-grade editor for one language (Python, to dovetail with Pass 1), wiring a plugin manager, LSP,
  Treesitter, autocommands, and one self-authored plugin — reproducible from an empty `~/.config/nvim`.
- **Concepts exercised**: [ ] `init.lua` + `lua/` module tree [ ] plugin manager bootstrap [ ] LSP
  attach + diagnostics + code action [ ] Treesitter highlight/text-objects [ ] an autocommand [ ] a
  `nvim_create_user_command` [ ] a self-authored Lua plugin on `runtimepath`.
- **Ordered steps**:
  1. `extending-neovim/learning/capstone/code/nvim/init.lua` — bootstrap the pinned plugin manager;
     verify `nvim --headless "+qa"` exits 0 with the plugin manager installed.
  2. Add `lua/options.lua` + `lua/keymaps.lua`, `require`d from `init.lua`. Verify `:lua print(vim.o.…)`
     reflects the set options.
  3. Wire LSP + Treesitter for Python (pinned, CVE-clean versions). Verify opening a `.py` file shows
     diagnostics and `:Inspect`/`:checkhealth` reports the server attached and parser installed.
  4. `lua/plugins/greet.lua` — a self-authored module registering a `:Greet` user command + a
     `BufWritePost` autocommand. Verify `:Greet` runs and the autocommand fires on save.
  5. Document the full launch (`nvim -u NONE` baseline vs the capstone config) and pin every version.
- **Acceptance criteria**: from an empty config dir, following the listings yields a working IDE-grade
  Neovim: LSP diagnostics on a Python file, Treesitter highlighting, and the `:Greet` command all
  function; `nvim --headless "+checkhealth" "+qa"` reports no missing required dependency.
- **Done bar**: runnable end-to-end + web-verified.

## Capstone spec — inter-topic: `capstone-forge-ready` (Pass-0 boundary)

Anchored here (weight 135, section-root folder `capstone-forge-ready/` with colocated `code/`).
Integrates topics 01–03: vanilla editing fluency + Lua + a real extended config.

- **Goal**: stand up a complete, reproducible personal development **forge** from an empty machine
  profile — a versioned Neovim config repo the reader can `git clone` and use to edit, navigate, search,
  and run code with LSP + Treesitter — and prove editing fluency by driving a scripted refactor in it
  with no mouse/arrow keys.
- **Concepts exercised**: [ ] raw-form editing (01) [ ] Lua modules/closures/metatables (02) [ ] plugin
  manager + LSP + Treesitter + user command + autocommand (03) [ ] a reproducible config repo layout
  [ ] the `:terminal` build/run loop (DD-17).
- **Ordered steps**:
  1. `capstone-forge-ready/code/nvim-config/` — a self-contained config repo (init.lua + `lua/` tree +
     pinned plugin lockfile). Verify a clean `XDG_CONFIG_HOME=$(mktemp -d) nvim --headless
"+checkhealth" "+qa"` bootstraps and reports healthy.
  2. `capstone-forge-ready/code/sample-project/` — a small Python project. Open it in the forge; verify
     LSP diagnostics + Treesitter highlighting appear.
  3. Drive a scripted, mouse-free refactor across the sample project using motions + macros + quickfix
     (reusing the topic-01 workflow), recording the transcript. Verify the refactor lands identically
     from the transcript.
  4. Run the sample project's check from `:terminal` beside the source. Verify it passes.
- **Acceptance criteria**: a reader on a clean machine reproduces the forge from the repo, opens the
  sample project with working LSP+Treesitter, and replays the refactor transcript to the identical
  result — end to end, no hidden setup.
- **Done bar**: runnable end-to-end (clean-machine reproduction) + web-verified.

## Read more

**Papers & articles**

- **Neovim User Documentation (`:help lsp`, `:help treesitter`, `:help lua-guide`)** — Neovim core team. Authoritative reference for the built-in LSP client, Tree-sitter, and the Lua plugin API. <https://neovim.io/doc/user/>
- **Language Server Protocol Specification** — Microsoft (3.17). The formal protocol Neovim's LSP client implements. <https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/>
- **Tree-sitter Documentation** — Max Brunsfeld and maintainers. Official docs for the incremental-parsing library Neovim uses. <https://tree-sitter.github.io/tree-sitter/>
- **nvim-lspconfig** — Neovim core team. Reference collection of default LSP client configs. <https://github.com/neovim/nvim-lspconfig>

---

← Previous: [2 · Just Enough Lua](./02-just-enough-lua.md) · Next: [4 · Just Enough Python](./04-just-enough-python.md) →
