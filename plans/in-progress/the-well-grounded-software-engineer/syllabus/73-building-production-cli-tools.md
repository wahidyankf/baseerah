# 73 · Building Production CLI Tools (By Example, Go + Rust †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Go + Rust † · Learn 173 / Drill 273 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: CLIs people actually enjoy using — argument parsing, subcommands, the config/flags/env
precedence chain, good help/errors/exit codes, honest TTY-vs-pipe behavior, and distribution as a single
binary. Cross-platform and anchored in Go and Rust, the two languages that dominate modern CLI tooling.
The telemetry instincts from [`59-analytics-and-experimentation`](./59-analytics-and-experimentation.md)
inform how a tool reports itself, and the shell fluency from
[`05-just-enough-bash`](./05-just-enough-bash.md) is what your tool has to compose with. `†`: Go and Rust
building native, statically-linkable binaries.

## Why this exists · the big idea

- **The problem before the solution**: the terminal is full of tools that are technically correct and
  miserable to use — cryptic flags, no `--help`, silent failures, exit code 0 on error, color codes dumped
  into a pipe, and installation that means "clone this and hope." A tool that ignores CLI convention becomes
  a tool people avoid or misuse.
- **Keep-this-if-you-forget-everything**: a good CLI is a contract with both a human at a keyboard and a
  script in a pipeline — it obeys convention (flags, exit codes, stdout-for-data/stderr-for-messages),
  detects whether it is talking to a terminal or a pipe, and fails loudly and specifically. Design for the
  pipe as carefully as for the person.
- **Big ideas touched**: `mechanism-vs-policy` (the tool's engine is the mechanism; flags, config, and env
  vars are how the user sets policy — keeping them separate is what makes a tool scriptable and
  composable), `coupling-vs-cohesion` (subcommands keep each verb's logic cohesive, while a clean core/CLI
  boundary keeps the tool's engine decoupled from its argument-parsing shell).

## Prerequisites

- **Prior topics**: [topic 59 Analytics & Experimentation](./59-analytics-and-experimentation.md)
  (honest measurement and self-reporting) and [topic 5 Just Enough Bash](./05-just-enough-bash.md) (pipes,
  exit codes, stdout/stderr, the shell environment a CLI lives in).
- **Tools & environment**: a macOS/Linux/Windows terminal; the **Go toolchain** (`go`) and/or the **Rust
  toolchain** (`cargo`), pinned to a current stable; a mature arg-parsing library per language (a
  `cobra`/`urfave`-style parser for Go, a `clap`-style parser for Rust); Neovim/VSCode with the Go/Rust LSP
  (DD-17).
- **Assumed knowledge**: pipes, exit codes, and stdout-vs-stderr (topic 05); building and running a native
  binary (topics 60/78); reading Go or Rust well enough to follow a small program (topics 60/78).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the design conventions are stable and correctly unpinned — POSIX/GNU argument
  syntax, the flags-over-env-over-config precedence idea, `0`-for-success exit codes, stdout-for-data /
  stderr-for-diagnostics, and TTY detection (`isatty`) for color/progress are long-settled and documented
  in `clig.dev` and the POSIX Utility Conventions. Keep both toolchains at "a current stable" in shipped
  text.
- 2026-07-12 — verified (GAP for plan owner): the body references arg-parsing libraries by role rather than
  a pinned version — re-verify the specific Go and Rust parser package names/versions once the worked
  examples are drafted; their public API is stable but the exact version should be current at authoring
  time.

## Items

- Argument parsing & subcommands: positional args, short/long flags, `--`, subcommand trees, and generated
  help.
- Configuration precedence: the flags → environment → config-file → defaults chain, and making it explicit
  and predictable.
- Help, errors, and exit codes: discoverable `--help`, actionable error messages on stderr, and meaningful
  non-zero exit codes.
- TTY vs pipe behavior: detecting an interactive terminal to decide color, progress bars, and prompts;
  emitting clean machine-readable output when piped.
- Ergonomics: sensible defaults, `--version`, shell completion, and quiet/verbose modes.
- Distribution: building a single static binary per platform, cross-compilation, and packaging/install
  paths.

## Tensions & trade-offs — when NOT to reach for this

- **When a script would do**: not every automation deserves a compiled, flag-parsed, cross-compiled binary.
  A twenty-line shell or Python script is the right tool for a one-off or a personal utility — building a
  "production CLI" for it is over-engineering.
- **Feature creep kills composability**: the Unix philosophy is do-one-thing-well for a reason. A CLI that
  grows an interactive menu, its own config DSL, and a plugin system becomes an application wearing a
  terminal costume — harder to script and harder to reason about than the small tools it replaced.
- **Convention over cleverness**: reinventing flag syntax, exit-code meanings, or output format because you
  think you can do better breaks every user's muscle memory and every downstream script. The "when not" here
  is: do not deviate from POSIX/GNU convention without a reason your users will thank you for.

## Lineage — why it beat the alternative

- CLI conventions were forged in early Unix: small single-purpose tools composed through pipes, with stdout
  as the data channel and exit codes as the success signal — the design that made the shell a programmable
  environment. The scripting-language era (Perl/Python CLIs) added rich arg-parsing but often shipped as
  "install the interpreter and these dependencies first." The Go and Rust generation closed that last gap:
  a single statically-linked binary you can drop onto any machine, with mature parser libraries
  (`cobra`/`clap`) that make convention the path of least resistance. Each step kept the Unix contract and
  removed a distribution or ergonomics tax. The mechanism/policy separation and clean binaries built here
  carry straight into the lower-level tooling of [`74-just-enough-c`](./74-just-enough-c.md) and every
  systems tool you ship afterward.

## Worked examples

Colocated under `building-production-cli-tools/learning/code/`; each runnable, built and exercised from the
CLI in Go and/or Rust (DD-20/DD-30).

- **beginner** — a single-command tool with flags, a generated `--help`, `--version`, and a correct
  non-zero exit code on failure.
- **intermediate** — a subcommand tree with the full config precedence chain (flags → env → file →
  defaults) and stderr-for-errors / stdout-for-data separation.
- **advanced** — TTY-aware output (color + progress bar interactively, clean machine output when piped) plus
  a cross-compiled single-binary build for two platforms with shell completion.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: ship one small production-quality CLI (Go or Rust) with a subcommand tree, the full
  configuration-precedence chain, discoverable help and meaningful exit codes, TTY-aware output that stays
  clean in a pipe, and a cross-compiled single-binary release for at least two platforms.
- **Concepts exercised**: [ ] subcommands + flags + generated help [ ] flags → env → config → defaults
  precedence [ ] stdout-for-data / stderr-for-diagnostics + non-zero exit codes [ ] TTY-vs-pipe detection
  for color/progress [ ] `--version` + shell completion [ ] a cross-compiled single-binary build for two
  targets.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a subcommand tree with flags, `--help`, and `--version`. Verify each
     subcommand's help renders and an unknown flag exits non-zero.
  2. Implement the config precedence chain. Verify a flag overrides an env var, which overrides a config
     file, which overrides the default — and that the resolved value is what runs.
  3. Split output: data on stdout, diagnostics on stderr, meaningful exit codes. Verify piping stdout yields
     clean machine-readable output while errors still surface on stderr with a non-zero code.
  4. Add TTY-aware color/progress and cross-compile a single binary for two platforms with completion.
     Verify color/progress appear interactively but not when piped, and that both binaries run on their
     targets.
- **Acceptance criteria**: subcommands and help work; precedence resolves in the documented order;
  stdout/stderr and exit codes obey convention; output adapts to TTY vs pipe; the tool builds to a single
  binary on two platforms.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **The Art of Unix Programming** — Eric S. Raymond (2003, Addison-Wesley; free CC-licensed edition
  author-hosted). The classic articulation of the Unix philosophy underlying good CLI tool design.
  <http://www.catb.org/esr/writings/taoup/html/>

**Papers & articles**

- **Command Line Interface Guidelines** — Aanand Prasad, Ben Firshman, Carolyn Zeller, et al. (ongoing,
  open source). The modern, widely cited canonical guide to designing CLI tools. <https://clig.dev/>
- **POSIX.1 / The Open Group Base Specifications (Utility Conventions)** — IEEE / The Open Group, official
  standard. The formal standard defining conventional CLI argument syntax that most Unix-family tools
  follow. <https://pubs.opengroup.org/onlinepubs/9699919799/>
- **The Linux man-pages project** — Michael Kerrisk et al., official (kernel.org project). The canonical
  documentation model and reference for well-documented CLI tools on Linux.
  <https://man7.org/linux/man-pages/>

---

← Previous: [72 · Linux App Development](./72-linux-app-development.md) · Next: [74 · Just Enough C](./74-just-enough-c.md) →
