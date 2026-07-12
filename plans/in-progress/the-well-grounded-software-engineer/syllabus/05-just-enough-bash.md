# 5 · Just Enough Bash (Primer, Bash/shell †)

**prd row**: Pass 1 · Core Foundations · Primer · Bash/shell † · Learn 105 / Drill 205 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: just enough Bash to drive builds, tests, and tooling from the terminal in every later
topic (the raw-form stance, DD-17); PowerShell is folded into [`76-windows-os`](./76-windows-os.md), not
taught here. All tooling is OSS (Tier-1, DD-21).

## Why this exists · the big idea

- **The problem before the solution**: every later topic drives builds, tests, and tooling from the
  terminal (DD-17); without shell fluency you cannot glue those tools together or automate the loop.
- **Keep-this-if-you-forget-everything**: small single-purpose tools piped together beat one big program —
  each command does one thing, and the pipe composes them.
- **Big ideas touched**: `coupling-vs-cohesion` — the Unix pipeline is the idea in miniature: highly
  cohesive tools (`grep`, `sort`, `awk`) loosely coupled through plain text streams.

## Prerequisites

- **Prior topics**: [topic 1 Just Enough Nvim](./01-just-enough-nvim.md) (to edit scripts).
- **Tools & environment**: a macOS/Linux terminal with **Bash** (`bash --version`); the `shellcheck` and
  `shfmt` CLIs installed; standard Unix text tools (`grep`/`sed`/`awk`/`find`). (Windows readers use WSL2
  — DD-25.)
- **Assumed knowledge**: basic terminal navigation; no prior shell scripting required.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28). Re-confirm version pins at authoring.

- 2026-07-12 — verified: current Bash stable **5.3** (July 2025); `set -euo pipefail`, `[[ ]]`, `getopts`
  behavior unchanged; POSIX-`sh`-vs-Bash caveats accurate. (lwn.net / phoronix.com, secondary)
- 2026-07-12 — verified: `shellcheck` **0.11.0** (2026-01-05). `shfmt` **v3.9.0** (mvdan/sh) — one
  secondary source conflicted (3.13.1); **re-fetch github.com/mvdan/sh/releases before authoring** to
  settle. (github.com koalaman/shellcheck, mvdan/sh)

## Items

- **Running the shell raw**: interactive vs script (`#!/usr/bin/env bash`), `chmod +x`,
  `set -euo pipefail`; Bash vs POSIX `sh` note; all from the terminal.
- **Core syntax**: variables & quoting (single-vs-double-quote and `"$var"` word-splitting traps),
  command substitution `$(...)`, arithmetic `$(( ))`, exit codes & `$?`.
- **Control flow**: `if`/`test`/`[[ ]]`, `case`, `for`/`while`/`until` loops, functions, `local`.
- **I/O & redirection**: stdin/stdout/stderr, `>`/`>>`/`2>&1`, pipes, here-docs/here-strings, `read`.
- **Text pipeline tools**: `grep`/`sed`/`awk`/`cut`/`sort`/`uniq`/`tr`/`xargs`/`find` — the composable
  core.
- **Arguments & options**: `$1`/`$@`/`$#`, `shift`, a `getopts` option parser, `--help`/usage.
- **Robustness**: quoting for spaces/globs, `trap` for cleanup, `mktemp`, checking command success.
- **Safety & quality**: `shellcheck` and `shfmt` from the CLI (the same gates this repo enforces).
- **Regular expressions**: POSIX BRE/ERE via `grep -E`/`sed`, character classes, anchors, quantifiers,
  and capture groups — the pattern language underlying the text-pipeline tools above.

## Worked examples

Colocated under `just-enough-bash/learning/code/`; each a complete executable script (DD-20/DD-30),
`shellcheck`-clean.

- **beginner** — a `set -euo pipefail` script taking args, with `--help` and correct exit codes.
- **intermediate** — a text-processing pipeline (`find | grep | awk`) solving a real chore; a `getopts`
  parser.
- **advanced** — a robust script with `trap` cleanup + `mktemp`, passing `shellcheck` clean, ending in a
  complete runnable listing.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: write one robust, `shellcheck`-clean Bash tool (~60–120 lines) that parses options with
  `getopts`, processes text through a pipeline, cleans up with `trap`+`mktemp`, and returns correct exit
  codes — the kind of helper later topics reuse.
- **Concepts exercised**: [ ] `set -euo pipefail` [ ] `getopts` + `--help` [ ] safe quoting [ ] a
  `grep`/`awk`/`sort` pipeline [ ] `trap` cleanup + `mktemp` [ ] correct exit codes.
- **Ordered steps**:
  1. `just-enough-bash/learning/capstone/code/report.sh` — parse `-i <input> -o <output>` via `getopts`,
     print usage on `-h`/bad args. Verify `./report.sh -h` prints usage, exits 0.
  2. Implement the pipeline writing to a `mktemp` scratch file, moved to `-o` on success; `trap` removes
     scratch on any exit. Verify `shellcheck report.sh` is clean and the output file matches expected.
  3. Verify a missing input exits non-zero with a stderr message and leaves no scratch file behind.
- **Acceptance criteria**: `shellcheck` clean; correct output for valid input; correct non-zero exit +
  cleanup on error.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **The Linux Command Line: A Complete Introduction** — William Shotts (5th internet ed., free). Comprehensive free intro from basics through real shell scripting. <https://linuxcommand.org/tlcl.php>

**Papers & articles**

- **GNU Bash Reference Manual** — FSF / Chet Ramey. Official canonical reference for Bash syntax, builtins, behavior. <https://www.gnu.org/software/bash/manual/bash.html>
- **POSIX Shell & Utilities (IEEE Std 1003.1)** — The Open Group / IEEE. Formal standard defining portable `sh` that Bash supersets. <https://pubs.opengroup.org/onlinepubs/9699919799/utilities/contents.html>
- **Google Shell Style Guide** — Google. Widely adopted pragmatic style guide for maintainable production Bash. <https://google.github.io/styleguide/shellguide.html>

---

← Previous: [4 · Just Enough Python](./04-just-enough-python.md) · Next: [6 · Version Control & Git](./06-version-control-and-git.md) →
