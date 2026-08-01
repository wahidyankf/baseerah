<!-- Knowledge Capture running log — append entries during execution. -->
<!-- Triage every entry (or record the explicit "none" escape) before archival. -->

# Learnings: beaver-nest-rebrand

## Phase 11: a literal `/` in Gherkin step text is a Cucumber Expression alternation delimiter, not a literal slash

The new scenario's step text (authored in Phase 10, per the plan's own literal wording) reads
`Then no بصيرة/wawasan-style etymology chip is present`. Registering this verbatim as a
`createBdd()` `Then("no بصيرة/wawasan-style etymology chip is present", ...)` step in
`apps/beaver-nest-fe-e2e/steps/landing.steps.ts` compiled fine but `bddgen` reported it as a
**missing** step definition — Cucumber Expressions treat an unescaped `/` between two words as an
**alternative-text delimiter** (`cat/dog` matches "cat" or "dog"), so `بصيرة/wawasan-style` compiled
to an alternation regex with no literal slash in it at all, which never matches the literal Gherkin
line (which does contain the slash character). Fix: escape the slash in the step-definition string —
`Then("no بصيرة\\/wawasan-style etymology chip is present", ...)` — which restores literal-slash
matching; `bddgen` and `specs:e2e:coverage` both pass clean afterward. Ruled out a stale
`.features-gen/` cache as the cause first (deleted it, reran with `--skip-nx-cache`, same failure) —
this is a genuine Cucumber Expression syntax gotcha, not a caching artifact.

**Generalizable rule**: any Gherkin step text containing a literal `/` between two words must escape
it (`\/`) in the corresponding `createBdd()` step-definition string, or the step will silently fail
to bind despite looking identical to the eye. The plain-string no-op step-def files (e.g. the FE
unit-test side's `landing.steps.ts`, which only registers dummy `Given`/`When`/`Then` functions with
no real Cucumber Expression parsing) don't hit this — only real `playwright-bdd` `createBdd()` step
registries do, so check e2e step files specifically whenever a Gherkin line contains a bare `/`.

## Phase 8: `git add` on an invalid multi-pathspec call can stage some paths with STALE content, silently

While staging Phase 8, an initial `git add -A <path1> <path2> ... <badpath> ...` call errored on a
pathspec that no longer existed (`baseerah.sln`, already `git mv`'d to `beaver-nest.sln`). Git
processes pathspecs left-to-right and aborts on the first invalid one — but paths listed BEFORE the
bad one (`apps/beaver-nest-be`, both e2e `project.json` files) were still staged before the abort. The
committed content for those already-staged paths turned out to be the content as it existed **at that
exact `git add` call**, not the fully-CANONICAL-SED'd content confirmed correct moments earlier by
`git grep -lic baseerah` — because that verification ran against the **working tree**, and a later,
separate content edit had already landed in the working tree but not yet been re-staged. The result:
`git commit` silently committed pre-sed `BaseerahBe`/`BASEERAH_BE_*` strings inside every file under
`apps/beaver-nest-be/`, even though `git status` showed clean rename entries and the pre-commit hook
ran green (fantomas/prettier reformatted the stale content, they don't check semantic correctness).
Caught only by re-running `git grep -lic baseerah` against `HEAD` (not the working tree) after the
commit — the working tree still showed the corrected content as a fresh unstaged diff. Fixed with a
follow-up commit (`425ad4679`) staging the corrected content, then re-verified `test:quick` green
against the corrected `HEAD`.

**Generalizable rule**: after any commit that follows a `git mv` + content-sed sequence, re-verify the
grep/structural check against `git show HEAD:<path>` (or `git diff HEAD` showing empty), not just the
working tree — a working-tree-only check can pass while the actual commit still holds stale content,
especially after a multi-pathspec `git add` call that partially failed. Never batch a rename-path and
a not-yet-renamed path in the same `git add` invocation; stage each already-confirmed-final path
individually or verify the command's exit code before trusting anything got staged from it.

## Phase 6 (addendum): the cross-phase RED/GREEN/REFACTOR design conflicts with `.husky/pre-push`, blocking the push itself (not just CI)

Phase 6's Post-Push CI Verification note anticipated `beaver-nest-fe`/`beaver-nest-be`
`specs:behavior:coverage` showing red **in CI** after the push. In practice the push never reaches
CI: `.husky/pre-push` runs `npx nx affected -t test:quick`, and `test:quick` for all four affected
projects (`baseerah-be`, `baseerah-be-e2e`, `baseerah-fe`, `baseerah-fe-e2e`) nests
`specs:behavior:coverage`/`specs:e2e:coverage` as a dependency (via `test:specs`), so the hook itself
fails and aborts the push with `husky - pre-push script failed (code 1)` before `git push` ever
contacts `origin`. The plan's design (deliberate multi-phase RED, tolerated at the CI level) and this
repo's blocking pre-push policy (`Never skip hooks unless explicitly requested`) are in direct
conflict — no combination of local fixes resolves it without either bypassing the hook or restructuring
when pushes happen.

Surfaced to the user with four options (bypass hook for this documented window, collapse phases into
one push, temporarily no-op the red targets, or stop). **User decision: collapse the per-phase push
cadence** — commit locally through Phases 6 through however far the RED persists (Phase 8 fixes
`baseerah-be`, Phase 9 fixes `baseerah-be-e2e`, Phase 10 fixes `baseerah-fe`, Phase 11 fixes
`baseerah-fe-e2e` — full GREEN isn't reached until Phase 11), and push once for real once
`nx affected -t test:quick` is clean across all of them. Every phase's own non-push steps (content
changes, Local Quality Gates typecheck/lint, Commit Guidelines, Phase Gate checks not requiring a
push) still execute and get recorded per-phase as normal; only the literal "push to origin main +
verify CI" step is deferred and re-attempted once per this whole stretch instead of once per phase.

**Generalizable rule**: before designing (or executing) any cross-phase RED/GREEN/REFACTOR cycle in
a repo with a blocking pre-push hook, check whether the hook's target set (here `test:quick`) nests
the very target the cycle deliberately reds out — if so, the push cadence itself must collapse to
match the cycle's span, not just the phase-by-phase content plan.

## Phase 1: pre-push `md links validate` blocks on ANY repo-wide broken link, not just the current phase's file set

Renaming `repo-governance/vision/baseerah.md` → `beaver-nest.md` immediately broke 27 inbound
markdown links from files entirely outside Phase 1's scope (agent-fleet mirrors under
`.claude/agents/`, `.cursor/agents/`, `.opencode/agents/`, `plans/ideas/*`,
`specs/apps/baseerah/product/README.md`, and this plan's own `README.md`/`brd.md`). The repo's
pre-push husky hook runs `md links validate` across the whole tree and fails the push on any
broken link, regardless of which phase "owns" the referencing file's full content sweep.

**Generalizable rule for every later phase that `git mv`s a path** (Phases 6, 8, 9, 10, 11, 12 all
rename directories): in the SAME commit as the rename, also repoint (not fully content-sweep) every
repo-wide inbound link to the old path — a targeted single-string sed scoped to just that link
string, leaving the referencing file's own full `baseerah`→`beaver-nest` prose sweep to its
designated phase. Verify with `git grep -l "<old-path>"` before push to catch every reference, and
re-run `md links validate` before every push, not just at the phase whose own gate mentions it.

## Phase 3: content sed can rename an outbound link's target path before the target file itself moves

`docs/reference/system-architecture/deployment.md` linked to
`../../../plans/ideas/baseerah-first-deploy.md`. Phase 3's blind `<CANONICAL-SED>` pass rewrote the
link text to `beaver-nest-first-deploy.md`, but that file isn't `git mv`'d until Phase 4 — so the
link broke immediately, failing `md links validate` before Phase 3 could even push. Reverted the one
link back to `baseerah-first-deploy.md` (mirroring the Decision-12 GitHub-URL pattern: keep the old
path text until the real move happens), and added a step to Phase 4's idea-brief-rename item to
repoint this same link once the file actually moves — the mirror-image of the Phase 1 rule (that
rule was about inbound links breaking when a path moves; this is an outbound link's target text being
renamed before the path moves). **Generalizable rule**: whenever a phase's content sweep touches a
markdown link whose _target_ is renamed by a _later_ phase, revert that one link's text in the
current phase and add an explicit repoint step to the later phase's `git mv` item.

## Phase 6: renamed Gherkin prose orphans EVERY step in a file, not just the one deleted scenario — and `bddgen` hard-fails typecheck for e2e projects

Phase 6's `<CANONICAL-SED>` pass renames "Baseerah" → "BeaverNest" (and `baseerah-be`/`baseerah-fe` →
`beaver-nest-be`/`beaver-nest-fe`) throughout the Gherkin feature-file prose, per its own explicit
design. The plan's RED step anticipated exactly ONE orphaned scenario (the deleted brand-chip
scenario) in `apps/baseerah-fe/src/test/landing.steps.ts`. In practice, that file's step
implementations use `[exact]` literal-string matching (not parameterized Cucumber Expressions), so
**every** step whose Gherkin text mentions the renamed strings goes orphan — 9 orphans / 10 gaps for
`baseerah-fe`, not 1. The same mechanism also hit `baseerah-be`'s unit-test steps (2 orphans / 4 gaps
in `GreetingSteps.fs`/`HealthSteps.fs`, e.g. background line + greeting response body text) — a
project the plan's Phase 6 RED callout never named at all, though its own Post-Push CI Verification
note (line ~657) does correctly anticipate both `beaver-nest-fe`/`beaver-nest-be` failing.

Worse, the two e2e projects (`baseerah-be-e2e`, `baseerah-fe-e2e`) use `playwright-bdd`'s `bddgen`
codegen step, invoked as a prerequisite inside their `typecheck`/`test:quick` targets
(`npx bddgen && npx tsc --noEmit` for be-e2e; `npx bddgen && ...` inside `specs:e2e:coverage` for
both). `bddgen` hard-errors (non-zero exit, no partial output) the moment ANY Gherkin step lacks a
matching TS step implementation — so `baseerah-be-e2e:typecheck` itself fails (not just its specs
coverage target), a wider blast radius than the plan's Local Quality Gates text implies ("typecheck
lint" should stay green with only `specs:behavior:coverage` red).

**Full confirmed RED footprint at end of Phase 6** (all expected, none require Phase-6 fixes):

| Project           | Target(s) red             | Cause                                                     | Resolves at |
| ----------------- | ------------------------- | --------------------------------------------------------- | ----------- |
| `baseerah-be`     | `test:quick`              | 2 orphan / 4 gap unit steps                               | Phase 8     |
| `baseerah-be-e2e` | `typecheck`, `test:quick` | `bddgen` hard-fail, 1 missing e2e step                    | Phase 9     |
| `baseerah-fe`     | `test:quick`              | 9 orphan / 10 gap unit steps                              | Phase 10    |
| `baseerah-fe-e2e` | `test:quick`              | `bddgen`-driven `specs:e2e:coverage`, 8 missing e2e steps | Phase 11    |

**Also discovered and fixed forward** (targeted single-string fixes, not full sweeps, since these
files are owned by later phases): `apps/baseerah-be/project.json` and `apps/baseerah-fe/project.json`
both had `dependsOn`/`implicitDependencies` referencing `baseerah-contracts` by name, which Phase 6
renamed to `beaver-nest-contracts` — this broke the Nx project graph for EVERY command until fixed.
Also both apps' and both e2e apps' `project.json` (`codegen`/`specs:behavior:coverage`/
`specs:e2e:coverage` targets, `namedInputs.specs`) and both e2e apps' `playwright.config.ts`
(`featuresRoot`/`features`) hardcoded the old `specs/apps/baseerah/behavior/baseerah-{be,fe}` path,
which Phase 6's `git mv` broke — same generalizable rule as the Phase 1/3/4 findings, just for
project-graph names and path strings instead of markdown links.

**Generalizable rule**: renaming a shared Gherkin/spec tree ahead of its consuming app's step-definition
files does not merely orphan the one intentionally-changed scenario — it orphans every literal-text-matching
step touching the renamed strings, and for `playwright-bdd`-based e2e projects it can hard-fail
`typecheck` itself (via `bddgen`), not just the specs-coverage target. Confirm the full RED footprint
project-by-project after any such phase, don't assume the plan's own RED callout is complete.

## Phase 5: `repo-config.yml` path-referencing fields must NOT rename ahead of their physical directory rename

Phase 5's own instructions said to rename `env-contract.surfaces[].root`, `env-injection.apps[].keys-from`
(and its `app:` label), and the CORS allowlist env-var name to their `beaver-nest-*` forms. But
`apps/baseerah-be/` and `apps/baseerah-fe/` are not physically renamed until Phase 8/10, and their
`.env.example` files still declare `BASEERAH_BE_CORS_ORIGINS` (not renamed until Phase 8 touches the
app's own env vars). Renaming these fields early broke the pre-push `env validate` hook two ways:
(1) `root`/`keys-from` pointed at a directory that doesn't exist yet (`cannot read
apps/beaver-nest-be/.env.example`), and (2) the CORS allowlist entry name no longer matched the key
actually declared in the `.env.example` (`declared-but-unread` drift). **Fix**: reverted `root:`,
`keys-from:`, the `app:` labels, and the CORS allowlist entry back to their `baseerah-*` forms —
these four fields will flip to `beaver-nest-*` in Phase 8/10 alongside the actual directory/env-var
renames, not before. By contrast, `coverage.projects[].specs` glob (pointing at the not-yet-renamed
`specs/apps/beaver-nest/`) was safe to rename now: confirmed via `repo-config validate` and the specs
CLI that no validator checks that glob's filesystem existence, only its schema shape.
**Generalizable rule**: before renaming ANY field in a shared config file, check whether a validator
resolves it against the filesystem (path existence, key-presence-in-file) versus treating it as an
opaque string/label — only the latter is safe to rename ahead of the phase that does the actual move.

## Phase 4: renaming a file leaves repo-wide inbound links dangling, same as Phase 1

Confirmed the same class of bug documented in the Phase 1 entry above, this time for a `git mv`
(not just a content rename): after `git mv plans/ideas/baseerah-first-deploy.md
plans/ideas/beaver-nest-first-deploy.md`, `md links validate` found 2 more repo-wide inbound links
(`apps/README.md`, `plans/in-progress/beaver-nest-rebrand/brd.md`) beyond the one already caught in
Phase 3 (`docs/reference/system-architecture/deployment.md`, reverted there pending this phase).
Fixed all three with a targeted path-only sed (not a full prose sweep of the referencing files,
since e.g. `apps/README.md` still legitimately says `baseerah-fe`/`baseerah-be` elsewhere until
Phases 8-11). **Reconfirms the Phase 1 rule**: every phase with a `git mv` (Phases 6, 8, 9, 10, 11, 12) must re-run `git grep -l "<old-path>"` across the whole repo, not just its own file set, before
pushing.

## Phase 2: BSD `xargs` on macOS has no `-a` flag

The plan's own reference commands use `xargs -a <file> -I{} ...` to feed a captured citation-file
list into a revert command. This is GNU-xargs syntax; BSD `xargs` (macOS, this dev machine) has no
`-a` option and errors immediately (`xargs: invalid option -- a`). Portable equivalent:
`< <file> xargs -I{} ...` (redirect stdin instead of `-a`). Every later phase's citation-revert step
that copies this exact command needs this substitution when executing on macOS.
