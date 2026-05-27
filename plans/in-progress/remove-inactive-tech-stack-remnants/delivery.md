# Delivery Checklist

## Worktree

Worktree path: `worktrees/remove-inactive-tech-stack-remnants/`

Provision before execution (run from repo root):

```bash
claude --worktree remove-inactive-tech-stack-remnants
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline

- [ ] Provision worktree: `claude --worktree remove-inactive-tech-stack-remnants` (creates
      `worktrees/remove-inactive-tech-stack-remnants/` in repo root)
- [ ] Initialize toolchain: `npm install && npm run doctor -- --fix` — exits 0, all tools
      present
- [ ] Verify baseline tests pass: `npx nx affected -t typecheck lint test:quick` — exits 0
      before any changes
- [ ] Verify markdown clean: `npm run lint:md` — exits 0 before any changes
- [ ] Fix any preexisting failures before proceeding (root cause, not bypass)

---

## Phase 1: Dotnet (F# / C#) Cleanup

### 1a: Delete dotnet files

- [ ] Delete `open-sharia-enterprise.sln`: `rm open-sharia-enterprise.sln` — file gone, `ls
*.sln` returns nothing
- [ ] Delete `.github/actions/setup-dotnet/` directory:
      `rm -rf .github/actions/setup-dotnet/` — directory gone
- [ ] Delete `scripts/format-csharp.sh`: `rm scripts/format-csharp.sh` — file gone
- [ ] Delete C# docs: `rm -rf docs/explanation/software-engineering/programming-languages/c-sharp/`
      — directory gone
- [ ] Delete F# docs: `rm -rf docs/explanation/software-engineering/programming-languages/f-sharp/`
      — directory gone
- [ ] Delete F# generated contracts:
      `rm -rf apps/organiclever-be/generated-contracts/OpenAPI/` — directory gone
- [ ] Delete `.claude/agents/swe-csharp-dev.md` and `.opencode/agents/swe-csharp-dev.md`:
      `rm .claude/agents/swe-csharp-dev.md .opencode/agents/swe-csharp-dev.md` — both gone
- [ ] Delete `.claude/agents/swe-fsharp-dev.md` and `.opencode/agents/swe-fsharp-dev.md`:
      `rm .claude/agents/swe-fsharp-dev.md .opencode/agents/swe-fsharp-dev.md` — both gone
- [ ] Delete `.claude/skills/swe-programming-csharp/`:
      `rm -rf .claude/skills/swe-programming-csharp/` — directory gone
- [ ] Delete `.claude/skills/swe-programming-fsharp/`:
      `rm -rf .claude/skills/swe-programming-fsharp/` — directory gone

### 1b: Replace Dockerfile.be.dev with Rust image

- [ ] Overwrite `infra/dev/ose-app/Dockerfile.be.dev` with content:

  ```
  FROM rust:1.95-slim
  ```

  Verify: `head -1 infra/dev/ose-app/Dockerfile.be.dev` outputs `FROM rust:1.95-slim`

### 1c: Modify config and workflow files

- [ ] Edit `package.json`: remove `"*.cs": "scripts/format-csharp.sh"` from the
      `lint-staged` object. Verify: `grep '"*.cs"' package.json` returns nothing.
- [ ] Edit `infra/dev/ose-app/docker-compose.ci.yml`: remove the `ASPNETCORE_URLS`
      environment variable entry under `ose-app-be`. Verify:
      `grep ASPNETCORE infra/dev/ose-app/docker-compose.ci.yml` returns nothing.
- [ ] Edit `infra/dev/ose-app/README.md`: change "F#/Giraffe REST API backend" to
      "Rust/Axum REST API backend" in the Services table. Verify: `grep "F#" infra/dev/ose-app/README.md`
      returns nothing.
- [ ] Edit `.github/workflows/crane-cli-integration.yml`: remove the line
      `- uses: ./.github/actions/setup-dotnet`. Verify:
      `grep setup-dotnet .github/workflows/crane-cli-integration.yml` returns nothing.
- [ ] Edit `.github/workflows/pr-quality-gate.yml` — remove dotnet detection and gate:
  - Remove `has-dotnet: ${{ steps.detect.outputs.has-dotnet }}` from `outputs:`
  - Remove `echo "has-dotnet=false" >> "$GITHUB_OUTPUT"` from detect step
  - Remove `lang:fsharp|lang:csharp) echo "has-dotnet=true" ...` case from detect step
  - Remove `tag:lang:fsharp,tag:lang:csharp` from the TypeScript `--exclude=` list
  - Remove the entire `dotnet:` job block (name through final step)
  - Remove `dotnet` from the `quality-gate` job's `needs:` list
  - Remove `dotnet` from the `for job in ...` loop in `quality-gate`
  - Verify: `grep -i "dotnet\|fsharp\|csharp" .github/workflows/pr-quality-gate.yml`
    returns nothing
- [ ] Edit `AGENTS.md`: remove `swe-csharp-dev, swe-fsharp-dev` from the **Development**
      agents list (line ~371). Verify: `grep "swe-csharp-dev\|swe-fsharp-dev" AGENTS.md`
      returns nothing.

### 1d: Quality gate + commit

- [ ] Run `npm run lint:md` — exits 0 (no broken links from removed C#/F# doc dirs)
- [ ] Run `npx nx affected -t typecheck lint` — exits 0
- [ ] Run `npm run generate:bindings` — exits 0; `.opencode/agents/` no longer has
      `swe-csharp-dev.md` or `swe-fsharp-dev.md` (already deleted manually in 1a; this
      validates sync state)
- [ ] Commit: `chore(cleanup): remove dotnet (F#/C#) remnants from ose-public`

---

## Phase 2: JVM (Java / Kotlin) Cleanup

### 2a: Delete JVM files

- [ ] Delete Java docs:
      `rm -rf docs/explanation/software-engineering/programming-languages/java/` — directory gone
- [ ] Delete Kotlin docs:
      `rm -rf docs/explanation/software-engineering/programming-languages/kotlin/` — directory gone
- [ ] Delete `.claude/agents/swe-java-dev.md` and `.opencode/agents/swe-java-dev.md`:
      `rm .claude/agents/swe-java-dev.md .opencode/agents/swe-java-dev.md` — both gone
- [ ] Delete `.claude/agents/swe-kotlin-dev.md` and `.opencode/agents/swe-kotlin-dev.md`:
      `rm .claude/agents/swe-kotlin-dev.md .opencode/agents/swe-kotlin-dev.md` — both gone
- [ ] Delete `.claude/skills/swe-programming-java/`:
      `rm -rf .claude/skills/swe-programming-java/` — directory gone
- [ ] Delete `.claude/skills/swe-programming-kotlin/`:
      `rm -rf .claude/skills/swe-programming-kotlin/` — directory gone

### 2b: Modify pr-quality-gate.yml for JVM

- [ ] Edit `.github/workflows/pr-quality-gate.yml` — remove JVM detection and gate:
  - Remove `has-jvm: ${{ steps.detect.outputs.has-jvm }}` from `outputs:`
  - Remove `echo "has-jvm=false" >> "$GITHUB_OUTPUT"` from detect step
  - Remove `lang:java|lang:kotlin) echo "has-jvm=true" ...` case from detect step
  - Remove `tag:lang:java,tag:lang:kotlin` from the TypeScript `--exclude=` list
  - Remove the entire `jvm:` job block
  - Remove `jvm` from the `quality-gate` job's `needs:` list
  - Remove `jvm` from the `for job in ...` loop in `quality-gate`
  - Verify: `grep -i "jvm\|lang:java\|lang:kotlin" .github/workflows/pr-quality-gate.yml`
    returns nothing
- [ ] Edit `AGENTS.md`: remove `swe-java-dev, swe-kotlin-dev` from Development agents list.
      Verify: `grep "swe-java-dev\|swe-kotlin-dev" AGENTS.md` returns nothing.

### 2c: Quality gate + commit

- [ ] Run `npm run lint:md` — exits 0
- [ ] Run `npx nx affected -t typecheck lint` — exits 0
- [ ] Commit: `chore(cleanup): remove JVM (Java/Kotlin) remnants from ose-public`

---

## Phase 3: Other ose-primer Langs (Elixir, Clojure, Dart, Python)

### 3a: Delete Elixir, Clojure, Dart, Python files

- [ ] Delete Elixir docs:
      `rm -rf docs/explanation/software-engineering/programming-languages/elixir/` — directory gone
- [ ] Delete Clojure docs:
      `rm -rf docs/explanation/software-engineering/programming-languages/clojure/` — directory gone
- [ ] Delete Dart docs:
      `rm -rf docs/explanation/software-engineering/programming-languages/dart/` — directory gone
- [ ] Delete Python docs:
      `rm -rf docs/explanation/software-engineering/programming-languages/python/` — directory gone
- [ ] Delete Elixir/Clojure/Dart/Python agent + opencode mirror files:

  ```bash
  rm .claude/agents/swe-elixir-dev.md .opencode/agents/swe-elixir-dev.md
  rm .claude/agents/swe-clojure-dev.md .opencode/agents/swe-clojure-dev.md
  rm .claude/agents/swe-dart-dev.md .opencode/agents/swe-dart-dev.md
  rm .claude/agents/swe-python-dev.md .opencode/agents/swe-python-dev.md
  ```

  Verify: `ls .claude/agents/ | grep -E "elixir|clojure|dart|python"` returns nothing

- [ ] Delete skill directories:

  ```bash
  rm -rf .claude/skills/swe-programming-elixir/
  rm -rf .claude/skills/swe-programming-clojure/
  rm -rf .claude/skills/swe-programming-dart/
  rm -rf .claude/skills/swe-programming-python/
  ```

  Verify: `ls .claude/skills/ | grep -E "elixir|clojure|dart|python"` returns nothing

- [ ] Delete `libs/clojure-openapi-codegen/` (source already removed; remaining tracked file
      is `LICENSE` plus gitignored build artifacts):
      `rm -rf libs/clojure-openapi-codegen/` — directory gone.
      Verify: `ls libs/ | grep clojure` returns nothing.
- [ ] Edit `libs/README.md`: remove the `clojure-openapi-codegen/` line from the libs listing.
      Verify: `grep clojure libs/README.md` returns nothing.
- [ ] Edit `.gitignore`: remove the `# Clojure classpath cache` comment line and the
      `.cpcache/` entry below it (no Clojure code remains after this cleanup).
      Verify: `grep cpcache .gitignore` returns nothing.

### 3b: Modify pr-quality-gate.yml for remaining langs

- [ ] Edit `.github/workflows/pr-quality-gate.yml` — remove Python gate + vestigial detection:
  - Remove `has-python: ${{ steps.detect.outputs.has-python }}` from `outputs:`
  - Remove `has-elixir: ${{ steps.detect.outputs.has-elixir }}` from `outputs:`
  - Remove `has-clojure: ${{ steps.detect.outputs.has-clojure }}` from `outputs:`
  - Remove `has-dart: ${{ steps.detect.outputs.has-dart }}` from `outputs:`
  - Remove `echo "has-python=false" >> "$GITHUB_OUTPUT"` from detect step
  - Remove `echo "has-elixir=false" >> "$GITHUB_OUTPUT"` from detect step
  - Remove `echo "has-clojure=false" >> "$GITHUB_OUTPUT"` from detect step
  - Remove `echo "has-dart=false" >> "$GITHUB_OUTPUT"` from detect step
  - Remove `lang:python) echo "has-python=true" ...` case from detect step
  - Remove `lang:elixir) echo "has-elixir=true" ...` case from detect step
  - Remove `lang:clojure) echo "has-clojure=true" ...` case from detect step
  - Remove `lang:dart) echo "has-dart=true" ...` case from detect step
  - Remove `tag:lang:python,tag:lang:elixir,tag:lang:clojure,tag:lang:dart` from TypeScript
    `--exclude=` list
  - Remove the entire `python:` job block
  - Remove `python` from the `quality-gate` job's `needs:` list
  - Remove `python` from the `for job in ...` loop in `quality-gate`
  - Verify: `grep -iE "lang:(python|elixir|clojure|dart)" .github/workflows/pr-quality-gate.yml`
    returns nothing
- [ ] Edit `AGENTS.md`: remove `swe-elixir-dev, swe-dart-dev, swe-clojure-dev, swe-python-dev`
      from Development agents list. Verify:
      `grep -E "swe-(elixir|clojure|dart|python)-dev" AGENTS.md` returns nothing.

### 3c: Sync OpenCode bindings

- [ ] Run `npm run generate:bindings` — exits 0. Verify:
      `ls .opencode/agents/ | grep -E "csharp|fsharp|java|kotlin|elixir|clojure|dart|python"`
      returns nothing.

### 3d: Quality gate + commit

- [ ] Run `npm run lint:md` — exits 0
- [ ] Run `npx nx affected -t typecheck lint` — exits 0
- [ ] Commit: `chore(cleanup): remove ose-primer lang (Elixir/Clojure/Dart/Python) remnants`

---

## Phase 4: Cross-Cutting Cleanup

### 4a: Rewrite programming-languages README

- [ ] Edit
      `docs/explanation/software-engineering/programming-languages/README.md`:
  - Remove **Skills Available** entries for all 8 removed langs
    (`swe-programming-csharp`, `swe-programming-fsharp`, `swe-programming-java`,
    `swe-programming-kotlin`, `swe-programming-elixir`, `swe-programming-clojure`,
    `swe-programming-dart`, `swe-programming-python`)
  - Remove the 💠 C#, 🔷 F#, ☕ Java, 🟠 Kotlin, 💜 Elixir, 🎯 Dart, 🐍 Python, and
    🎸 Clojure language sections
  - Remove C#, F#, Java, Kotlin, Elixir, Clojure, Dart, Python rows from the
    "Current Language Usage" table
  - Remove C#, F# from the "Domain-Specific Standards Pattern" example language list
  - Update the "Quick Decision" table: remove "Complex domain logic with DDD (future) → Java/Kotlin/F#"
    row or update to reflect active stacks only
  - Update "Platform Guidance" bullets to list only active langs
  - Verify: `grep -iE "c-sharp|f-sharp|java|kotlin|elixir|clojure|dart|python"
docs/explanation/software-engineering/programming-languages/README.md` returns nothing
    (except cross-links to ose-primer if any are kept)
  - _Suggested executor: `docs-maker`_

### 4b: Final AGENTS.md verification

- [ ] Run: `grep -E "swe-(csharp|fsharp|java|kotlin|elixir|clojure|dart|python)-dev" AGENTS.md`
      — must return nothing. If any remain, remove them.
- [ ] Verify active dev agents present: `grep -E "swe-(golang|typescript|rust|e2e)-dev" AGENTS.md`
      — must show results.

### 4c: Final link verification

- [ ] Run `npm run lint:md` — exits 0 (validates no dead internal links from removed doc dirs)
- [ ] Spot-check: `grep -r "programming-languages/c-sharp\|programming-languages/f-sharp\|programming-languages/java\|programming-languages/kotlin\|programming-languages/elixir\|programming-languages/clojure\|programming-languages/dart\|programming-languages/python" docs/ repo-governance/ AGENTS.md`
      — review any hits; fix or remove stale cross-links

### 4d: Quality gate + commit

- [ ] Run `npx nx affected -t typecheck lint test:quick` — exits 0
- [ ] Run `npm run lint:md` — exits 0
- [ ] Commit: `chore(cleanup): rewrite programming-languages README, final cross-cutting cleanup`

---

## Phase 5: Post-Push CI Verification

- [ ] Push to `origin main`: `git push origin main`
- [ ] Monitor GitHub Actions: `gh run list --limit 5` — check status every 3 minutes
- [ ] Verify `PR - Quality Gate` workflow (if triggered) completes with success or skip for
      all jobs — particularly confirm no `dotnet`, `jvm`, `python` jobs appear
- [ ] Verify `crane-cli-integration` workflow (if triggered) completes without setup-dotnet
      errors
- [ ] If any CI job fails: diagnose root cause, fix, push follow-up commit, re-monitor

---

## Plan Archival

- [ ] Verify ALL delivery checklist items above are ticked
- [ ] Verify ALL quality gates pass (local + CI)
- [ ] Move plan: `git mv plans/in-progress/remove-inactive-tech-stack-remnants plans/done/2026-05-27__remove-inactive-tech-stack-remnants`
      (use actual completion date)
- [ ] Update `plans/in-progress/README.md` — remove this plan's entry
- [ ] Update `plans/done/README.md` — add this plan's entry with completion date
- [ ] Commit: `chore(plans): move remove-inactive-tech-stack-remnants to done`
