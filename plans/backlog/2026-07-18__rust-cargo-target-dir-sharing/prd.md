# Product Requirements — Rust `target/` Directory Sharing

## Product overview

A repo-init-time mechanism that redirects each Rust crate's `target/` directory to a shared,
persistent cache via a symlink, so multiple git worktrees of the same repo share one physical build
directory per crate. The mechanism is a small POSIX shell helper (`scripts/cargo-target-share.sh`)
chained into the repo's `doctor` step and executed during worktree provisioning. It is local-dev
only and no-ops under CI.

This is not a UI-bearing or API-bearing change — it touches build tooling, shell scripts,
`package.json`, `project.json`, and governance docs. No web screens, no HTTP endpoints.

## Personas

- **Local developer (maintainer)** — runs many worktrees; wants disk reclaimed and builds intact.
- **`repo-setup-manager` agent** — executes Phase 0 and `npm run doctor -- --fix`; needs the symlink
  wiring to be idempotent and side-effect-safe.
- **CI custodian** — needs an ironclad guarantee the mechanism never activates on the runner.

## User stories

- **US-1** — As a local developer, I want each crate's `target/` to point at a shared cache, so that
  ten worktrees do not store ten copies of the same build artifacts.
- **US-2** — As a repo/toolchain owner, I want the symlink created idempotently by `npm run doctor`
  and by worktree provisioning, so that any worktree I enter is set up correctly with no manual step.
- **US-3** — As a CI custodian, I want the symlink logic to no-op under CI, so that the shared target
  never worsens the self-hosted rustup/cargo concurrency race.
- **US-4** — As a maintainer, I want builds and tests to work unchanged through the symlink, so that
  the dedup carries zero functional cost.
- **US-5** — As a maintainer of three sibling repos, I want the same mechanism in `ose-public`,
  `ose-primer`, and `ose-infra`, so that all my machines' worktrees benefit uniformly.
- **US-6** — As a maintainer, I want a documented cleanup path, so that the shared cache does not
  silently regrow to fill the disk.

## Acceptance criteria

### Scenario: doctor symlinks a crate's target into the shared cache

```gherkin
Given a Rust crate exists at `apps/rhino-cli` with a plain `target/` directory
When the developer runs `sh scripts/cargo-target-share.sh` outside CI
Then `apps/rhino-cli/target` is a symlink into `$HOME/.cache/ose-cargo-target/<repo>/rhino-cli`
And `readlink apps/rhino-cli/target` resolves under the shared cache root
```

### Scenario: the symlink step is idempotent

```gherkin
Given `apps/rhino-cli/target` is already the correct symlink into the shared cache
When the developer runs `sh scripts/cargo-target-share.sh` a second time
Then the command exits 0 without recreating or altering the symlink
And `readlink apps/rhino-cli/target` is unchanged
```

### Scenario: an existing plain target directory is replaced by a symlink

```gherkin
Given `apps/rhino-cli/target` is a plain (rebuildable) directory containing stale artifacts
When the developer runs `sh scripts/cargo-target-share.sh` outside CI
Then `apps/rhino-cli/target` becomes a symlink into the shared cache
And the previous plain directory no longer occupies the worktree path
```

### Scenario: the mechanism no-ops under CI

```gherkin
Given the environment variable `CI` is set to `1`
When the developer runs `sh scripts/cargo-target-share.sh`
Then no symlink is created at `apps/rhino-cli/target`
And the command exits 0 with a message that CI was detected
```

### Scenario: builds resolve through the symlink and still emit dist

```gherkin
Given `apps/rhino-cli/target` is a symlink into the shared cache
When the developer runs `nx run rhino-cli:build`
Then the command exits 0
And `apps/rhino-cli/dist/rhino-cli` exists as a freshly copied binary
```

### Scenario: tests pass through the symlinked target

```gherkin
Given `apps/rhino-cli/target` is a symlink into the shared cache
When the developer runs `nx run rhino-cli:test:unit`
Then the command exits 0
And all unit tests pass without reference to a per-worktree target directory
```

### Scenario: two worktrees of the same repo share one physical target

```gherkin
Given two worktrees of `ose-public` each have `apps/rhino-cli/target` symlinked by the mechanism
When both symlinks are resolved with `readlink`
Then both point at the same `$HOME/.cache/ose-cargo-target/ose-public/rhino-cli` directory
And a `du -sh` across the worktrees counts that directory only once
```

### Scenario: the rhino-cli byte-identity boundary is untouched by the core mechanism

```gherkin
Given the core symlink phases are complete on a branch
When `git diff --stat main` is inspected for that branch
Then no file under `apps/rhino-cli/` appears in the diff
And the change set is limited to `scripts/`, `package.json`, non-rhino `project.json`, and docs
```

### Scenario: Nx build caching is unaffected for crates that emit only dist

```gherkin
Given `apps/ayokoding-cli` and `apps/ose-cli` no longer list `{projectRoot}/target` in build outputs
When `nx run ayokoding-cli:build` runs twice with no source change
Then the second run is served from the Nx cache
And `apps/ayokoding-cli/dist/ayokoding-cli` is present after both runs
```

### Scenario: the same mechanism lands in all three repos

```gherkin
Given the mechanism is delivered to `ose-public`, `ose-primer`, and `ose-infra`
When `sh scripts/cargo-target-share.sh` runs in each repo outside CI
Then `apps/rhino-cli/target` is symlinked into that repo's own shared-cache namespace
And each repo's `<repo>` cache segment is derived from its own git common directory
```

## Product scope

**In scope (features)**

- `scripts/cargo-target-share.sh` — the symlink helper with CI guard and idempotency.
- `scripts/cargo-target-share.test.sh` — self-contained verification of guard + idempotency + link.
- Chaining the helper into each repo's `package.json` `doctor` script (before the doctor/build).
- Removing `{projectRoot}/target` from the three ose-public crates' `build.outputs`.
- Governance-doc updates (worktree setup, reproducible environments, cleanup guidance).

**Out of scope (features)**

- Editing `apps/rhino-cli/src/**` or its manifests in the core mechanism.
- Installing `cargo-sweep` via `rhino-cli doctor` (documented/manual cleanup instead).
- Any CI runner or workflow change.
- Optional Phase 5 (`[profile.dev]` debuginfo trim) is delivered separately and may be dropped
  wholesale by the maintainer.

## Product-level risks

- **False CI detection locally** — if a developer's shell exports `CI`, the symlink silently no-ops.
  Mitigated by the `OSE_CARGO_TARGET_CACHE`-independent guard message and documentation.
- **Cross-repo cache-name collision** — two repos with the same directory basename would share a
  cache namespace. Low risk (the three repos have distinct directory names); documented in
  `tech-docs.md`.
