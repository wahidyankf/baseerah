# Product Requirements Document

## Product Overview

A one-time cleanup sweep that removes inactive-tech-stack remnants from `ose-public`. The
deliverable is a clean repo where only the three active stacks (TypeScript, Go, Rust) have
agents, skills, CI gates, docs, and toolchain support.

## Personas

**Contributor (human or AI agent)** — reads docs and agent catalogs to understand which stacks
are supported. Currently sees 8 non-active language directories and agents, creating confusion.

**CI system** — runs `pr-quality-gate.yml` on every PR. Currently parses dotnet, JVM, Python
detection jobs that always produce `has-X=false` and skip.

## User Stories

1. As a contributor, I want the language docs directory to list only active stacks so I know
   which languages to use for new work.
2. As an AI agent, I want the agent catalog to list only agents for active stacks so I don't
   invoke a non-applicable agent for a task.
3. As a CI pipeline, I want no detection/gate jobs for stacks with zero tagged projects so
   workflows are clean and unambiguous.
4. As a developer running local infra, I want `docker compose up` for `ose-app` to succeed using
   the correct Rust image, not a stale dotnet one.

## Product Scope

### In-Scope

All artifacts listed in `README.md §Scope In-Scope`.

### Out-of-Scope

All artifacts listed in `README.md §Scope Out-of-Scope`.

### Product Risks

| Risk                                                     | Severity | Mitigation                                     |
| -------------------------------------------------------- | -------- | ---------------------------------------------- |
| Doc links from other files pointing at removed lang dirs | Medium   | `npm run lint:md` validates links post-cleanup |
| AGENTS.md skill references to removed skills             | Medium   | Grep + manual update in Phase 4                |

## Acceptance Criteria

### Scenario 1: Inactive language docs removed

```gherkin
Given the repo has been cleaned up per this plan
When I run: ls docs/explanation/software-engineering/programming-languages/
Then I see only: golang/, rust/, typescript/, README.md
And no directories for c-sharp, f-sharp, java, kotlin, elixir, clojure, dart, or python
```

### Scenario 2: Inactive agent files removed

```gherkin
Given the repo has been cleaned up per this plan
When I run: ls .claude/agents/ | grep swe-
Then I see only: swe-e2e-dev.md, swe-golang-dev.md, swe-rust-dev.md, swe-typescript-dev.md
And no swe-csharp-dev, swe-fsharp-dev, swe-java-dev, swe-kotlin-dev,
    swe-elixir-dev, swe-clojure-dev, swe-dart-dev, or swe-python-dev files
```

### Scenario 3: Inactive skill directories removed

```gherkin
Given the repo has been cleaned up per this plan
When I run: ls .claude/skills/ | grep swe-programming-
Then I see only: swe-programming-golang/, swe-programming-rust/, swe-programming-typescript/
And no swe-programming-csharp, swe-programming-fsharp, swe-programming-java,
    swe-programming-kotlin, swe-programming-elixir, swe-programming-clojure,
    swe-programming-dart, or swe-programming-python directories
```

### Scenario 4: CI quality gate has no dotnet/JVM/Python/inactive-lang jobs

```gherkin
Given the repo has been cleaned up per this plan
When I read .github/workflows/pr-quality-gate.yml
Then there are no jobs named: dotnet, jvm, python
And there are no detect outputs for: has-dotnet, has-jvm, has-python, has-elixir, has-clojure, has-dart
And the quality-gate needs list references only: typescript, golang, rust
  (plus format, markdown, naming, specs-gate, detect)
```

### Scenario 5: .github/actions/setup-dotnet removed

```gherkin
Given the repo has been cleaned up per this plan
When I run: ls .github/actions/
Then setup-dotnet/ is not present
And crane-cli-integration.yml does not reference setup-dotnet
```

### Scenario 6: Dotnet toolchain scripts and config removed

```gherkin
Given the repo has been cleaned up per this plan
When I run: ls scripts/ | grep csharp
Then no results
And: grep '"*.cs"' package.json returns no results
And: ls | grep '.sln' returns no results
```

### Scenario 7: ose-app Dockerfile uses Rust image

```gherkin
Given the repo has been cleaned up per this plan
When I read infra/dev/ose-app/Dockerfile.be.dev
Then the base image is rust:1.95-slim (not dotnet SDK)
And infra/dev/ose-app/docker-compose.ci.yml contains no ASPNETCORE_URLS
```

### Scenario 8: OpenCode bindings synced

```gherkin
Given agent files have been removed from .claude/agents/
When I run: npm run generate:bindings
Then .opencode/agents/ contains no mirrors for the 8 removed agents
And no diff remains between .claude/agents/ and .opencode/agents/ (aside from format translation)
```

### Scenario 9: All local quality gates pass

```gherkin
Given all cleanup changes are applied
When I run: npx nx affected -t typecheck lint test:quick
Then all targets exit 0 with no errors
And: npm run lint:md exits 0 (no broken markdown links from removed doc dirs)
```

### Scenario 10: CI passes after push

```gherkin
Given the cleanup changes are pushed to origin main
When GitHub Actions runs pr-quality-gate.yml and related workflows
Then all jobs complete with success or skip (no failures)
```
