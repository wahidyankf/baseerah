# Business Requirements — Rust `target/` Directory Sharing

## Business goal

Reclaim disk consumed by duplicated Rust `target/` directories across git worktrees, and keep it
reclaimed, without slowing local builds or destabilizing CI — while leaving the `apps/rhino-cli`
byte-identity boundary untouched.

## Business rationale

The monorepo's Rust crates produce large `target/` directories, and the repo's worktree-heavy
workflow (each plan gets its own worktree + PR to maximize parallelization) multiplies that cost:
every worktree recompiles and stores its own copy of essentially identical artifacts. The observed
result is tens of gigabytes of largely-redundant build output on the maintainer's machine.
[Judgment call: based on the maintainer's local `du` observation, not an instrumented benchmark.]

A per-crate symlink into a shared cache collapses N per-worktree copies into 1 physical directory
per repo+crate. Because `target/` is already gitignored [Repo-grounded — `.gitignore:114` `target/`],
the mechanism touches **zero tracked build configuration** in the core design, which is what makes it
low-risk and boundary-safe.

## Business impact

**Pain points addressed**

- **Disk exhaustion**: worktree proliferation drives `target/` duplication that can fill the disk.
- **Wasted recompilation**: identical crates rebuilt per worktree burn CPU and time.
- **Cache never GC'd**: `target/*/incremental/` grows unbounded with no maintenance path.

**Expected benefits**

- One shared `target/` per repo+crate instead of one per worktree → cross-worktree duplication
  eliminated. [Judgment call — mechanism-implied, to be confirmed by the before/after `du` gate.]
- Warm shared cache means a crate already built in one worktree is not rebuilt from scratch in
  another. [Judgment call]
- A documented cleanup path (`cargo clean` / `cargo sweep`) gives an explicit lever against regrowth.

## Affected roles

Solo-maintainer repo — no sign-off ceremony. The maintainer wears these hats:

- **Local developer** — the primary beneficiary; faster, lighter multi-worktree development.
- **Repo/toolchain owner** — owns `npm run doctor` and worktree provisioning, where the symlink is
  wired.
- **CI custodian** — must ensure the mechanism never runs on the self-hosted runner (see risks).

Consuming agents: `repo-setup-manager` (runs Phase 0 + doctor), the `plan-execution` workflow
(worktree provisioning), and any agent that runs `npm run doctor -- --fix`.

## Business-level success metrics

- **Disk dedup observed**: after the mechanism is applied, a before/after `du -sh` across worktrees
  shows the shared cache is counted once rather than per worktree. [Observable fact — verified by the
  disk-verification gate in `delivery.md`.]
- **Zero build regressions**: `nx run <crate>:build` still emits the expected `dist/<bin>`, and
  `test:unit` / `test:quick` still pass through the symlinked target. [Observable fact — gated.]
- **CI unaffected**: the doctor symlink logic no-ops under CI, so the known rustup/cargo concurrency
  race is not worsened. [Observable fact — gated by the CI-guard test.]
- **Boundary preserved**: `apps/rhino-cli/**` byte-identity holds because the core mechanism edits
  none of it. [Observable fact — `git diff --stat` gate.]

## Business-scope non-goals

- Not a general build-performance optimization program; the aim is disk dedup with no regressions.
- Not a change to how CI builds or caches Rust — CI is explicitly excluded.
- Not a rewrite of `rhino-cli doctor`; the mechanism sits beside it, not inside it.

## Business risks and mitigations

| Risk                                                                                 | Likelihood        | Mitigation                                                                                                                                                                                                                           |
| ------------------------------------------------------------------------------------ | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Shared target on CI worsens the self-hosted rustup/cargo `.partial` concurrency race | High if unguarded | Hard CI guard (`$CI` / `$GITHUB_ACTIONS`) no-ops the symlink; first-class acceptance criterion + dedicated test. [Judgment call — recalled from a prior session, not documented in-repo]                                             |
| Concurrent local builds of the SAME crate in two worktrees contend on the cargo lock | Low               | Cargo serializes on its own `target` lock (blocks, does not corrupt); documented as an accepted local trade-off in `tech-docs.md` §Accepted trade-off: concurrent local builds of the same crate across worktrees [Web-cited there]. |
| Nx caches/restores a symlinked `target` for crates that list it as an output         | Medium            | Remove `{projectRoot}/target` from the three ose-public crates' `build.outputs` (rhino-cli already excludes it). [Repo-grounded — project.json]                                                                                      |
| Optional debuginfo-trim edit to `apps/rhino-cli/Cargo.toml` breaks byte-identity     | Medium            | Phase 5 is optional and, if taken, applied byte-identically across all three repos in the same cycle; core phases never touch that file.                                                                                             |
| A stray `scripts/*.sh` trips shell-lint on pre-push across the three repos           | Medium            | Scripts are shellcheck-clean at `--severity=warning`; verified in local gates. [Judgment call — recalled from a prior session, not documented in-repo]                                                                               |

## Cross-references

- Testable scenarios for each success metric: [`prd.md` §Acceptance Criteria](./prd.md#acceptance-criteria).
- Design rationale and rejected alternatives: [`tech-docs.md`](./tech-docs.md).
