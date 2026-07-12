# 2 · Just Enough Lua (Primer, Lua †)

**prd row**: Pass 0 · Editor Foundations · Primer · Lua † · Learn 102 / Drill 202 · Nvim-ready Yes ·
VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: just enough Lua to be productive configuring and extending Neovim in
[`03-extending-neovim`](./03-extending-neovim.md); **not** full mastery. Lua is the `†` language
exception here because it is the config language of the editor the whole book is built around. All
tooling is OSS (Lua is MIT-licensed) — Tier-1 per DD-21.

## Why this exists · the big idea

- **The problem before the solution**: [`03`](./03-extending-neovim.md) configures Neovim in Lua; learning
  the config language and the editor-extension concept at once doubles the difficulty — so get just-enough
  Lua first (DD-13).
- **Keep-this-if-you-forget-everything**: in Lua a single structure — the **table** — is array, map,
  object, and module at once; master the table and the rest of the language is small.
- **Big ideas touched**: `abstraction-and-its-cost` — one universal abstraction (the table) buys a tiny,
  learnable language and charges you the specialized types a bigger language would give.

## Prerequisites

- **Prior topics**: [topic 1 Just Enough Nvim](./01-just-enough-nvim.md) (to edit and run files
  comfortably).
- **Tools & environment**: a macOS/Linux terminal; the standalone **`lua`** interpreter installed
  (`lua -v`) for running scripts outside Neovim.
- **Assumed knowledge**: basic terminal use; no prior programming language required (this is a first
  language for some readers).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28). Re-confirm version pins at authoring.

- 2026-07-12 — verified (CORRECTION): PUC-Lua's current major is now **5.5.0** (2025-12-22), superseding
  the 5.4 line (latest 5.4.x patch 5.4.8). Neovim still embeds **LuaJIT 2.1** (Lua-5.1 semantics), so the
  "LuaJIT vs PUC-Lua" gap is now 5.1-vs-5.5 — widen the version note in Items accordingly. (lua.org)
- 2026-07-12 — verified: Lua remains **MIT**-licensed; Neovim bundles it as MIT (Tier-1, DD-21). (lua.org)
- 2026-07-12 — verified (stable): the `vim` global + `require` semantics match current embedded Lua; no
  change found.

## Items

- **Running Lua raw**: the `lua` interpreter & REPL, running a script from the terminal (no IDE); LuaJIT
  vs PUC-Lua note; where Neovim's embedded Lua differs (the `vim` global).
- **Core syntax**: variables, `nil`/boolean/number/string, operators, string library basics.
- **Tables — the one data structure**: arrays, maps, nested tables, `ipairs`/`pairs`, length `#`.
- **Functions**: first-class functions, closures, multiple return values, varargs, method-call sugar
  (`:`).
- **Control flow**: `if`/`elseif`/`else`, numeric & generic `for`, `while`, `repeat`.
- **Modules**: `require`, returning a table as a module; metatables & `__index` at a glance.
- **Errors**: `pcall`/`error`; idiomatic `nil, err` returns.

## Worked examples

Colocated under `just-enough-lua/learning/code/`; each a complete runnable `.lua` script run with
`lua <file>` (DD-20, DD-30 — full listing on the page, exact command, expected output shown).

- **beginner** — a runnable script exercising tables + `for`; REPL exploration of metatables.
- **intermediate** — a module returning a table of functions; closures as counters/config.
- **advanced** — a metatable-backed "class" with `__index`; `pcall` error handling around it.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: write one small, self-contained Lua program (~60–120 lines) that uses tables, closures, a
  `require`d module, a metatable, and `pcall` error handling **together** — a mini "config-value store"
  the reader recognizes as the shape of Neovim config to come.
- **Concepts exercised**: [ ] tables as records+arrays [ ] `ipairs`/`pairs` [ ] closures capturing state
  [ ] a module returning a function table [ ] `__index` metatable defaulting [ ] `pcall`/`nil, err`.
- **Ordered steps**:
  1. `just-enough-lua/learning/capstone/code/store.lua` — a module returning `{ new = function() … end }`
     where `new()` returns a closure-backed store with `get`/`set`. Verify `lua -e 'require("store")'`
     loads without error.
  2. Add a `defaults` table wired via `setmetatable(store, { __index = defaults })` so missing keys fall
     back. Verify a `get` on an unset key returns the default.
  3. `just-enough-lua/learning/capstone/code/main.lua` — `require("store")`, set/get several keys, and
     wrap a deliberately failing lookup in `pcall`, printing `nil, err` cleanly.
  4. Run `lua main.lua`. Verify stdout matches the documented expected output block exactly.
- **Acceptance criteria**: `lua main.lua` exits 0 and prints the expected lines; the failing path is
  caught by `pcall` (no uncaught error); every listing on the page is complete and runnable as shown.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Programming in Lua** — Roberto Ierusalimschy (4th ed., 2016; Lua 5.3). Official book by the language's chief architect; free first edition online. <https://www.lua.org/pil/>

**Papers & articles**

- **Lua 5.4 Reference Manual** — Ierusalimschy, de Figueiredo, Celes (Lua.org). Canonical spec of syntax, semantics, standard libraries. <https://www.lua.org/manual/5.4/manual.html>
- **"Lua — An Extensible Extension Language"** — Ierusalimschy, de Figueiredo, Celes (1996, Software: Practice and Experience). Original paper on Lua's design as a small embeddable language. <https://www.lua.org/spe.html>
- **"The Evolution of Lua"** — Ierusalimschy, de Figueiredo, Celes (2007, HOPL III). The authors' own design history; context for why Neovim adopted Lua. <https://www.lua.org/doc/hopl.pdf>

---

← Previous: [1 · Just Enough Nvim](./01-just-enough-nvim.md) · Next: [3 · Extending Neovim](./03-extending-neovim.md) →
