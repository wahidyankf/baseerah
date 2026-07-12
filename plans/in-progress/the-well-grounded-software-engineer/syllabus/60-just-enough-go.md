# 60 · Just Enough Go (Primer §, Go †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · Go † · Learn 160 / Drill 260 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` tool/language primer — **just enough Go** to be productive in the next topic
([`61-csp-style-concurrency`](./61-csp-style-concurrency.md)), no more. The toolchain, syntax, structs,
interfaces, the error-value convention, and a goroutine/channel _preview_ only (concurrency depth belongs
to topic 61). This opens Pass 4.

## Why this exists · the big idea

- **The problem before the solution**: Pass 4 is about concurrency, and studying CSP needs a language whose
  runtime makes goroutines and channels first-class — this primer gets you productive in Go without
  detouring into mastery you don't yet need.
- **Keep-this-if-you-forget-everything**: Go trades expressive power for a small, explicit surface — errors
  are values you check (`if err != nil`), not exceptions you catch, and that plainness is the feature.
- **Big ideas touched**: `abstraction-and-its-cost` — Go deliberately hides little (explicit error values,
  no inheritance, few keywords), so what does leak stays minimal and legible.

## Prerequisites

- **Prior topics**: general programming fluency from Pass 1/2 — especially
  [topic 4 Just Enough Python](./04-just-enough-python.md) (a first language to contrast) and
  [topic 5 Just Enough Bash](./05-just-enough-bash.md) (driving a toolchain).
- **Tools & environment**: a macOS/Linux terminal; the **Go toolchain** (`go`), pinned to a current stable
  release; Neovim/VSCode with Go LSP (DD-17).
- **Assumed knowledge**: variables/functions/types in some language (topic 04); running CLI tools + a build
  loop (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: keep the Go version unpinned in shipped text (file already says "current stable").
  Current stable is **Go 1.26.5** (2026-07-07); Go 1.26 (2026-02-10) enabled the Green Tea GC by default.
  `go run`/`build`/`test`/`mod` subcommands and the `if err != nil` error-value convention are unchanged.
  Re-pull the exact version at authoring time. (go.dev/doc/devel/release)

## Items

- The Go toolchain from the CLI: `go run` / `build` / `test` / `mod`; project layout.
- Syntax & types: variables, structs, slices, maps, pointers intro.
- Functions, methods, interfaces; the error-value convention (`if err != nil`).
- Packages & modules; a goroutine/channel **preview** (depth in `csp-style-concurrency`).

## Worked examples

Colocated under `just-enough-go/learning/code/`; each runnable via the Go toolchain (DD-20/DD-30).

- **beginner** — a runnable program + a `go test`.
- **intermediate** — a struct + an interface example.
- **advanced** — a small CLI with a goroutine/channel preview.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: build a small idiomatic Go CLI that exercises the primer's surface — structs + interfaces,
  the error-value convention, packages/modules, and a single goroutine/channel hand-off — with a `go test`,
  proving readiness for CSP-style concurrency.
- **Concepts exercised**: [ ] modules + package layout [ ] structs + methods + an interface [ ] error-value
  handling [ ] a goroutine + channel hand-off [ ] a `go test`.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a CLI with a struct + an interface + explicit error handling. Verify
     `go build` + `go run` work and errors surface via the error value.
  2. Add a single goroutine + channel hand-off. Verify the value crosses the channel and the program exits
     cleanly.
  3. `main_test.go` — a `go test`. Verify the test passes.
- **Acceptance criteria**: the CLI builds and runs; errors are handled via the error value; the
  goroutine/channel hand-off works; `go test` passes.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **The Go Programming Language** — Alan A. A. Donovan, Brian W. Kernighan (2015). The definitive, most widely recommended book on Go, co-authored by a legendary computer scientist.
- **Learning Go** — Jon Bodner (2nd ed., 2024). Widely recommended modern, idiomatic guide to Go covering generics and current tooling.

**Papers & articles**

- **Effective Go** — The Go Team (ongoing). The official guide to writing idiomatic Go, maintained directly by the language's creators. <https://go.dev/doc/effective_go>
- **The Go Memory Model** — The Go Team (ongoing). Official specification of Go's concurrency and memory-ordering guarantees, essential for correct concurrent code. <https://go.dev/ref/mem>

---

← Previous: [59 · Analytics & Experimentation](./59-analytics-and-experimentation.md) · Next: [61 · CSP-Style Concurrency](./61-csp-style-concurrency.md) →
