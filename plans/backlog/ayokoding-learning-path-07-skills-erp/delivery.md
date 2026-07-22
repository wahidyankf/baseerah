# Delivery Checklist — Skills Paths: Enterprise Resource Planning

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
> Git-mechanical steps (worktree create/remove, branch, commit, push, merge) are `[AI]`.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (safe-to-stop state + resume command). Each gate covers the phase's
> **content/data correctness** (checkers, tests, build) and its **integration** (draft PR opened,
> 3-cycle PR-Review, CI green, `[AI]` merge, `ayokoding-www` deployed). A phase is not complete until
> every gate check is green.

Four standing constraints govern every step below.

> **Cross-plan source of truth**: the ERP catalog — course ids, formats, prerequisite edges, ramp
> order — is settled in
> [tech-docs.md §The ERP catalog](./tech-docs.md#the-erp-catalog-29-courses-settled). Transcribe it;
> do not re-derive it. The syllabus module/topic content is settled in
> [`syllabus/courses/`](./syllabus/README.md). Transcribe it into course bodies; do not re-derive it.
>
> **The category ownership invariant (binding)**: this plan owns `<CONVMAN>`, `<SHARMAN>`,
> `<CONVLANDING>`, `<SHARLANDING>`, the twenty-nine ERP course bundles, and `<SYL>`/`<SYLPATHS>`. It
> **never** writes an accounting file, a careers manifest, a component, a design asset, or a
> structural `_index.md`. A step here that authors accounting material is a boundary violation and is
> equally forbidden in the other direction.
>
> **Verification hygiene (A4/A12)**: the ERP research is almost entirely `[Unverified]`. No claim
> marked `[Unverified]` or `[Needs Verification]` may be written as fact. Syllabus confirmation
> (Phase 1.2a) is coverage-only and never reorders a syllabus's structure — see
> [tech-docs.md §Syllabus confirmation order](./tech-docs.md#syllabus-confirmation-order-a12).
>
> **Id-shape rule (schema-owner ruling, DD-21)**: each path id is the **full** string
> (`skills/conventional-erp` or `skills/sharia-erp`) — no separate `category` field, and **nothing
> keys on segment count**. Every URL/id match below is a **full-string literal** (`grep -F -q`) rather
> than a segment-shaped regex. Course ids carry no category prefix.

## Worktree

Worktree path: `worktrees/ayokoding-learning-path-07-skills-erp/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree ayokoding-learning-path-07-skills-erp
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before deleting
the worktree after the plan is archived and pushed.

Every phase branches from the **latest `origin/main`** inside this one shared worktree
(`git fetch origin && git checkout main && git pull && git checkout -b ayokoding-learning-path-07-skills-erp/<phase-slug>`),
authors its work there, commits, pushes that branch, and opens **its own draft PR**.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Delivery Mode: worktree-to-pr

Each phase works in the shared worktree on its **own branch**, opens a **draft PR** against `main`,
runs the **PR-Review Maker→Fixer Cycle** (`pr-review-maker` / `pr-review-fixer`, 3 sequential CI-gated
cycles), flips the PR to ready, and `[AI]` **merges it once all quality gates are green** — then
`[AI]` **deploys `ayokoding-www` to `prod-ayokoding-www` after every merge** (this plan ships to
ayokoding.com). This plan declares **no** `[HUMAN]` merge gate. See
[Plans Organization Convention §Delivery Mode](../../../repo-governance/conventions/structure/plans.md#delivery-mode)
and the [PR Review Quality Gate workflow](../../../repo-governance/workflows/pr/pr-review-quality-gate.md).

**Per-Phase Integration Protocol** (each phase's gate lists these as must-pass):

1. [AI] Sync the shared worktree to latest `origin/main` and branch:
   `git fetch origin && git checkout main && git pull && git checkout -b ayokoding-learning-path-07-skills-erp/<phase-slug>`.
2. [AI] Stage only this phase's paths (`git add <explicit paths>` — never `git add -A`), commit
   thematically (Conventional Commits, imperative, no period), push the branch, open a **draft PR**
   against `main` (`gh pr create --draft --base main ...`) — CI runs on the PR.
3. [AI] Run the **PR-Review Maker→Fixer Cycle** (3 sequential CI-gated cycles), resolve every finding,
   then `gh pr ready`.
4. [AI] **Merge** once all quality gates are green (typecheck, lint, `test:quick`, `test:unit`,
   `test:e2e` where affected, `specs:behavior:coverage`, CI, the 3-cycle review).
5. [AI] Dispatch `apps-ayokoding-www-deployer` to deploy `ayokoding-www` to `prod-ayokoding-www` — a
   content/data-only plan still deploys, since the manifests and course bundles are reachable
   behavior. Verify the deploy via `curl -sf https://ayokoding.com/en/learn/paths/skills/conventional-erp | grep -qi "conventional"`
   (after Phase 2) or the equivalent `sharia-erp` URL (after Phase 4).

## Shell constants (reused across phases)

```bash
COURSES="apps/ayokoding-www/content/en/learn/courses/"
PATHS="apps/ayokoding-www/content/en/learn/paths/"
MANIFESTS="apps/ayokoding-www/src/features/course-paths/manifests/"
CONVMAN="${MANIFESTS}skills/conventional-erp.yaml"
SHARMAN="${MANIFESTS}skills/sharia-erp.yaml"
CONVLANDING="${PATHS}skills/conventional-erp/_index.md"
SHARLANDING="${PATHS}skills/sharia-erp/_index.md"
SYL="plans/backlog/ayokoding-learning-path-07-skills-erp/syllabus/courses/"

# Stage A — 15 ids, no accounting precondition
ERP_STAGE_A=(
  erp-foundations-and-history erp-conceptual-data-model erp-module-map-and-architecture
  erp-document-lifecycle-and-state-machines erp-posting-rules-and-account-determination
  erp-subledger-to-gl-architecture erp-fiscal-calendar-and-period-close
  erp-numbering-sequences-and-uom-conversion erp-audit-trail-and-change-tracking
  procure-to-pay-systems order-to-cash-systems erp-procurement-and-fulfillment-exceptions
  erp-bom-and-routing-architecture erp-extension-and-customization erp-integration-patterns
)

# Stage B — 11 ids, gated on ACCT_GATE_B
ERP_STAGE_B=(
  record-to-report-systems inventory-and-warehouse-management erp-inventory-costing-methods
  erp-inventory-integrity-and-concurrency production-planning-and-mrp demand-and-supply-planning
  erp-availability-and-reservations human-capital-management-and-hire-to-retire
  multi-company-and-multi-currency-erp erp-security-and-controls erp-analytics-and-reporting
)

# Stage C — 3 ids, sharia-erp only, gated on ACCT_GATE_C
ERP_STAGE_C=(
  sharia-compliant-erp-design islamic-contract-based-transaction-flows
  zakat-and-sharia-compliance-modules
)

ERP_ALL=("${ERP_STAGE_A[@]}" "${ERP_STAGE_B[@]}" "${ERP_STAGE_C[@]}")

# Accounting gates — mechanical test -d checks against ayokoding-learning-path-06-skills-accounting's
# own course bundles on origin/main. See tech-docs.md's cross-plan coordination-risk note: these ids
# are as named in plan 06's own in-flight rewrite as of 2026-07-22 and must be re-verified before use.
ACCT_GATE_B=(
  financial-statements-and-close-cycle inventory-and-cogs-accounting
  payroll-and-tax-accounting-essentials consolidation-and-multi-entity-accounting
  audit-controls-and-compliance
)
ACCT_GATE_C=(
  islamic-contract-modeling-for-systems sharia-accounting-and-aaoifi-standards
)

# No id in ERP_ALL is a substring of another, and none collides with an accounting or
# existing-library course id — verified at Phase 0.
```

## Phase 0: Environment Setup

- [ ] [AI] Install dependencies: `npm install`.
- [ ] [AI] Run doctor to verify tooling: `npm run doctor -- --fix`.
- [ ] [AI] Verify dev server starts: `nx dev ayokoding-www`.
- [ ] [AI] Verify existing tests pass before making changes:
      `nx run ayokoding-www:test:quick`.
- [ ] [AI] Verify no id collision: for each id in `ERP_ALL`, confirm it is not a substring of any
      other id in `ERP_ALL` and does not already exist under `<COURSES>` —
      `for id in "${ERP_ALL[@]}"; do test -d "${COURSES}${id}" && echo "COLLISION: $id"; done | grep -q . && echo FAIL || echo PASS`
      — acceptance: prints `PASS`.
- [ ] [AI] Verify no id collides with an accounting id: `comm -12 <(printf '%s\n' "${ERP_ALL[@]}" | sort) <(printf '%s\n' "${ACCT_GATE_B[@]}" "${ACCT_GATE_C[@]}" | sort)` — acceptance: empty output.

### Phase 0 Gate

- [ ] [AI] All four checks above pass; `nx run ayokoding-www:test:quick` is green on a clean tree.

> **Pause Safety**: no plan file yet modified. Safe to stop. To resume: re-run
> `nx run ayokoding-www:test:quick`.

## Phase 1: Syllabus Authoring and Verification

Per [tech-docs.md §Syllabus layer](./tech-docs.md#syllabus-layer--custody-and-shape-dd-31), authoring
precedes confirmation, and confirmation is coverage-only (`A12`).

### 1.1 — Author all 29 syllabus specs (already drafted; this step verifies, not re-authors)

- [x] [AI] `<SYL>README.md` and all 29 `<SYL><id>.md` files exist, each with the required section set
      (header block, Scope note ending `License-aware (DD-15)`, Why this exists, Prerequisites,
      Accuracy notes, Concepts, Worked examples, Synthesis exercise, Read more, In which paths).
      Verify: `for id in "${ERP_ALL[@]}"; do test -f "${SYL}${id}.md" || echo "MISSING: $id"; done | grep -q . && echo FAIL || echo PASS` —
      acceptance: prints `PASS`.
- [x] [AI] `syllabus/paths/manifest-skills-conventional-erp.md` and
      `syllabus/paths/manifest-skills-sharia-erp.md` exist and together enumerate all 29 ids exactly
      once each (26 shared + 3 Sharia-exclusive) — verify:
      `grep -c '^[0-9]\+\.`' plans/backlog/ayokoding-learning-path-07-skills-erp/syllabus/paths/manifest-skills-sharia-erp.md`—
acceptance: the count of numbered`courseOrder` lines is 29.

### 1.2 — The A4 verification pass before any spec asserts a fact

- [ ] [AI] For every syllabus's "Accuracy notes" section, confirm every `[Verified]` claim traces to
      the domain-research grounding file or a fetched primary source, and every `[Unverified]` /
      `[Needs Verification]` claim is **not** restated as fact elsewhere in the same file — verify:
      `grep -L '\[Verified\]\|\[Unverified\]\|\[Needs Verification\]\|\[Judgment call\]' "${SYL}"*.md` returns
      no ERP-domain-specific-claim file lacking any marker (a file with zero markers because it makes
      no ERP-domain-specific claim is acceptable; flag any file whose Accuracy notes section is
      empty).
- [ ] [AI] Re-verify the two open items named in `tech-docs.md` before Phase 4 begins (they gate
      Stage C, not Stage A/B): the PSAK-numbering question in
      `sharia-compliant-erp-design.md`, and the AAOIFI/PSAK/MASB jurisdictional-model table. Dispatch
      `web-researcher` against AAOIFI's and IAI's own published standards indexes; update the syllabus
      files' Accuracy notes with the verified answer or an explicit `[Needs Verification]` carry-
      forward — never silently drop the marker.

### 1.2a — `web-researcher` confirmation pass (`A12`)

> Coverage-only. Never reorders a syllabus's modules or adopts a curriculum's sequence.

- [ ] [AI] Dispatch `web-researcher` once per syllabus (29 dispatches, or batched by module-family
      where the underlying topic overlaps) asking exactly: "does APICS/ASCM's CPIM or CSCP topic
      outline (for planning/operations content) or the named open-source system's own published
      module structure (for architecture/module-map content, nominative reference only) suggest a
      topic this syllabus's module list omits, or include a topic the field does not recognise?" —
      never "how should these modules be ordered".
- [ ] [AI] For each finding returned, add the missing topic to the relevant module in the syllabus
      file, in this plan's own words, citing the confirming body nominatively (e.g. "corroborated
      against ASCM's CPIM topic outline") — never quoting or reproducing the outline's own text.
- [ ] [AI] Resolve every `[Needs Verification]` tag left in a syllabus's Concepts/module list: either
      confirm and relabel `[Verified]`/`[Repo-grounded]`, or leave `[Needs Verification]` explicitly
      if the pass could not resolve it — never silently drop the tag.

### Phase 1 Gate

- [ ] [AI] `npm run lint:md` is green on all `syllabus/**` files.
- [ ] [AI] Every syllabus file's Accuracy notes section reflects the Phase 1.2/1.2a pass results (no
      file still reads "has not yet run for this course" after this phase completes).
- [ ] [AI] **Integration**: draft PR opened for `syllabus/**` changes only, 3-cycle PR-Review complete,
      CI green, `[AI]` merge, no deploy needed (plan-folder-only change, not a build input).

> **Pause Safety**: `syllabus/` is fully authored and confirmed; no `<COURSES>` or manifest file yet
> exists. Safe to stop. To resume: `test -f plans/backlog/ayokoding-learning-path-07-skills-erp/syllabus/courses/zakat-and-sharia-compliance-modules.md && echo READY`.

## Phase 2: Stage A — Foundations and Architecture

15 courses, no accounting precondition — fully concurrent with `ayokoding-learning-path-06-skills-accounting`.

### 2.1 — Author all 15 Stage A course bodies (maker-checker-fixer, per format)

For each `id` in `ERP_STAGE_A`, following the seven-step NEW-course authoring convention (DD-17:
accuracy pre-verify → skeleton → learning track → drilling track → checkers → fixers → re-verify),
transcribing the module/topic content from `<SYL>${id}.md`:

- [ ] [AI] Accuracy pre-verify: re-check every `[Verified]`/`[Unverified]` claim in
      `<SYL>${id}.md`'s Accuracy notes is current (no drift since Phase 1.2a).
- [ ] [AI] Skeleton: create `<COURSES>${id}/_index.md` with frontmatter (`title`, `format`,
      `prerequisites: [...]` transcribed verbatim from the catalog table in `tech-docs.md`) and the
      section scaffold.
- [ ] [AI] Learning track: dispatch `apps-ayokoding-www-annotated-concept-maker` (Annotated-concept
      ids) or `apps-ayokoding-www-by-example-maker` (By Example ids) to author the concept
      explanations, transcribing every `co-NN` from the syllabus.
- [ ] [AI] Drilling track: for By Example ids, author the worked examples transcribed from the
      syllabus's `ex-NN` list (prose worked scenarios, never runnable code standing up a system —
      A6). For Annotated-concept ids, author the equivalent worked-scenario drills.
- [ ] [AI] Checkers: dispatch `apps-ayokoding-www-annotated-concept-checker` or
      `apps-ayokoding-www-by-example-checker` plus `apps-ayokoding-www-facts-checker` and
      `apps-ayokoding-www-link-checker`.
- [ ] [AI] Fixers: dispatch `apps-ayokoding-www-general-fixer`-family agents for every finding.
- [ ] [AI] Re-verify: `test -d "${COURSES}${id}" && test -f "${COURSES}${id}/_index.md" && echo PASS` —
      acceptance: prints `PASS` for every id in `ERP_STAGE_A`.

### 2.2 — TDD: publish both manifests at 15 ids

- [ ] [AI] **RED** — Write `apps/ayokoding-www/src/features/course-paths/manifests/skills/erp-manifests.unit.test.ts`
      asserting `<CONVMAN>` and `<SHARMAN>` each parse against the `PathManifest` zod schema, each has
      `pathId` equal to `skills/conventional-erp` / `skills/sharia-erp` respectively, `arc:
immediately-effective`, and `courseOrder` containing exactly the 15 `ERP_STAGE_A` ids in order —
      run `nx run ayokoding-www:test:unit -- erp-manifests` and verify it **fails** (files do not
      exist yet).
- [ ] [AI] **GREEN** — Create `<CONVMAN>` and `<SHARMAN>` (both identical at this stage — 15 ids,
      transcribed from `syllabus/paths/manifest-skills-conventional-erp.md` Stage A section) — run
      `nx run ayokoding-www:test:unit -- erp-manifests` and verify it **passes**.
- [ ] [AI] **REFACTOR** — Run `checkManifestIntegrity` and `checkPrerequisiteConsistency` (from
      `ayokoding-learning-path-02-schema-and-prerequisite-dag`'s `course-paths` core) against both
      manifests — verify both return zero violations.

### 2.3 — Create both path landings and populate cards

- [ ] [AI] Create `<CONVLANDING>` and `<SHARLANDING>` with the content spec from
      [tech-docs.md §Landing content requirements](./tech-docs.md#landing-content-requirements-what-plan-03-cannot-infer)
      (the Dangerous-N ramp table, the L-2 runway justification, and — for `<SHARLANDING>` — the L-5
      "covers all the basics" statement) — using the design system components
      `ayokoding-learning-path-03-navigation-ui` already ships; author **content only**, no new
      component.
- [ ] [AI] Populate two cards each in `<PATHS>_index.md` and `<PATHS>skills/_index.md` (four
      insertions total) — edit only, these files already exist (A3).
- [ ] [AI] Populate 15 rows in `<COURSES>_index.md` — edit only, file already exists (A3).

### 2.4 — Gherkin coverage for Stage A

- [ ] [AI] Add scenarios to `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/course-paths/skills-erp-paths.feature`
      covering: both landings render, both manifests validate, the Dangerous-1 boundary appears
      correctly on both landings, and `<SHARLANDING>`'s "covers all the basics" statement is present.
- [ ] [AI] Author matching step definitions at
      `apps/ayokoding-www-fe-e2e/src/steps/skills-erp-paths.steps.ts`.
- [ ] [AI] `nx run ayokoding-www:specs:behavior:coverage` reports 100% coverage for the new feature
      file.

### Phase 2 Gate

- [ ] [AI] `nx run ayokoding-www:test:unit` green (erp-manifests suite).
- [ ] [AI] `nx run ayokoding-www:test:e2e` green for the new feature file.
- [ ] [AI] `nx run ayokoding-www:typecheck`, `:lint`, `:test:quick` all green.
- [ ] [AI] `for id in "${ERP_STAGE_A[@]}"; do test -d "${COURSES}${id}" || echo "MISSING: $id"; done | grep -q . && echo FAIL || echo PASS` prints `PASS`.
- [ ] [AI] **Integration**: draft PR opened, 3-cycle PR-Review complete, CI green, `[AI]` merge,
      `ayokoding-www` deployed, post-deploy curl check (Per-Phase Integration Protocol step 5) passes
      for both `<CONVLANDING>` and `<SHARLANDING>`.

> **Pause Safety**: both manifests exist at 15 ids; both landings render; Dangerous 1 is live for
> both paths. Safe to stop — a reader visiting either path today gets a coherent, if smaller,
> experience. To resume: `curl -sf https://ayokoding.com/en/learn/paths/skills/conventional-erp | grep -q "Dangerous"`.

## Phase 3: Stage B — Conventional Enterprise Depth

11 courses, gated on `ACCT_GATE_B` resolving on `origin/main`. `conventional-erp` reaches its terminal
26-id state at the end of this phase.

### 3.0 — Gate check (mechanical, independent of plan 06's own delivery tracking)

- [ ] [AI] `for id in "${ACCT_GATE_B[@]}"; do git -C worktrees/ayokoding-learning-path-07-skills-erp fetch origin main -q; git -C worktrees/ayokoding-learning-path-07-skills-erp show "origin/main:${COURSES}${id}/_index.md" >/dev/null 2>&1 || echo "WAITING: $id"; done | grep -q . && echo WAIT || echo READY` —
      if `WAIT`, poll every 2 minutes (per CI-monitoring convention's cadence) rather than
      tight-looping; do not begin 3.1 until `READY`.

### 3.1 — Author all 11 Stage B course bodies

Repeat the 2.1 seven-step cycle for each `id` in `ERP_STAGE_B`, transcribing from `<SYL>${id}.md`.
Two of these ids (`erp-security-and-controls`, `erp-analytics-and-reporting`) additionally require the
scope-boundary self-check worked example (DD-10) to be present and reviewed by
`apps-ayokoding-www-facts-checker` for accuracy against the stated boundary claim.

- [ ] [AI] `for id in "${ERP_STAGE_B[@]}"; do test -d "${COURSES}${id}" || echo "MISSING: $id"; done | grep -q . && echo FAIL || echo PASS` prints `PASS`.

### 3.2 — TDD: grow both manifests to 26 ids

- [ ] [AI] **RED** — Extend `erp-manifests.unit.test.ts` asserting both `<CONVMAN>` and `<SHARMAN>`
      each contain all 26 shared ids (Stage A's 15 plus Stage B's 11, at the insertion positions in
      [tech-docs.md §courseOrder arrays](./tech-docs.md#courseorder-arrays-at-each-growth-boundary)),
      with every Stage A id's relative order unchanged — run the suite and verify it **fails**.
- [ ] [AI] **GREEN** — Grow `<CONVMAN>` and `<SHARMAN>` to 26 ids each — run the suite and verify it
      **passes**.
- [ ] [AI] **REFACTOR** — Re-run `checkManifestIntegrity`/`checkPrerequisiteConsistency`; verify zero
      violations, including the hard edge (`record-to-report-systems` requiring
      `financial-statements-and-close-cycle` to exist under `<COURSES>` on `origin/main`).

### 3.3 — Deferral-check assertion (both directions)

- [ ] [AI] Confirm the **before** half of the falsifiable check recorded in Phase 2: re-run
      `grep -F -q 'record-to-report-systems' <(git show HEAD~1:"${CONVMAN}")` against the pre-growth
      commit and verify it **fails** (the id was genuinely absent before this phase).
- [ ] [AI] Confirm the **after** half: `grep -F -q 'record-to-report-systems' "${CONVMAN}"` **passes**.

### 3.4 — Landing update: Dangerous 2 and Dangerous 3 boundaries

- [ ] [AI] Update `<CONVLANDING>` and `<SHARLANDING>` content to show the Dangerous 2 boundary
      (course 16) and, for `<CONVLANDING>`, the terminal Dangerous 3 boundary (course 26, "ENDS
      HERE").
- [ ] [AI] Populate 11 more rows in `<COURSES>_index.md` (26 total).

### 3.5 — Gherkin coverage for Stage B

- [ ] [AI] Extend `skills-erp-paths.feature` with scenarios for the Dangerous 2/3 boundaries and the
      `conventional-erp`-ends-here statement; extend step definitions accordingly.

### Phase 3 Gate

- [ ] [AI] All Phase 2 Gate checks re-run and still green.
- [ ] [AI] `<CONVMAN>` has exactly 26 `courseOrder` entries; `<SHARMAN>` has exactly 26 (Stage C not
      yet grown) — `yq '.courseOrder | length' "${CONVMAN}"` prints `26`.
- [ ] [AI] **Integration**: draft PR opened, 3-cycle PR-Review complete, CI green, `[AI]` merge,
      `ayokoding-www` deployed, post-deploy curl check confirms `<CONVLANDING>` shows "ENDS HERE".

> **Pause Safety**: `conventional-erp` is terminal (26/26); `sharia-erp` is mid-growth (26/29). Safe
> to stop — `conventional-erp` readers get the complete path today. To resume:
> `yq '.courseOrder | length' worktrees/ayokoding-learning-path-07-skills-erp/${SHARMAN}` and confirm
> it reads `26`.

## Phase 4: Stage C — Sharia-Compliant Design

3 courses, `sharia-erp` only, gated on `ACCT_GATE_C` resolving on `origin/main`.

### 4.0 — Gate check

- [ ] [AI] `for id in "${ACCT_GATE_C[@]}"; do git -C worktrees/ayokoding-learning-path-07-skills-erp fetch origin main -q; git -C worktrees/ayokoding-learning-path-07-skills-erp show "origin/main:${COURSES}${id}/_index.md" >/dev/null 2>&1 || echo "WAITING: $id"; done | grep -q . && echo WAIT || echo READY` —
      poll every 2 minutes if `WAIT`.
- [ ] [AI] Complete Phase 1.2's deferred re-verification of the PSAK-numbering and jurisdictional-model
      open items before authoring begins, if not already resolved.

### 4.1 — Author all 3 Stage C course bodies

Repeat the 2.1 seven-step cycle for each `id` in `ERP_STAGE_C`, transcribing from `<SYL>${id}.md`.
Every claim in the jurisdictional-model table carries its A4 marker into the course body verbatim
(never restated as settled fact if still `[Unverified]`).

- [ ] [AI] `for id in "${ERP_STAGE_C[@]}"; do test -d "${COURSES}${id}" || echo "MISSING: $id"; done | grep -q . && echo FAIL || echo PASS` prints `PASS`.

### 4.2 — TDD: grow `<SHARMAN>` to 29 ids

- [ ] [AI] **RED** — Extend `erp-manifests.unit.test.ts` asserting `<SHARMAN>` contains all 29 ids at
      the positions in
      [tech-docs.md §courseOrder arrays](./tech-docs.md#courseorder-arrays-at-each-growth-boundary)
      (the 3 Sharia-exclusive ids inserted immediately after `multi-company-and-multi-currency-erp`
      and before `erp-security-and-controls`), and that `<CONVMAN>` is **unaffected** (still 26,
      unchanged) — run the suite and verify it **fails**.
- [ ] [AI] **GREEN** — Grow `<SHARMAN>` to 29 ids — run the suite and verify it **passes**.
- [ ] [AI] **REFACTOR** — Re-run integrity checks on `<SHARMAN>` only; verify zero violations.

### 4.3 — Deferral-check assertion (both directions)

- [ ] [AI] Before/after check for `zakat-and-sharia-compliance-modules`, mirroring 3.3's pattern
      against `<SHARMAN>`.

### 4.4 — Landing update: Dangerous 4 boundary

- [ ] [AI] Update `<SHARLANDING>` to show the terminal Dangerous 4 boundary (course 29, "ENDS HERE").
- [ ] [AI] Populate the final 3 rows in `<COURSES>_index.md` (29 total).

### 4.5 — Gherkin coverage for Stage C

- [ ] [AI] Extend `skills-erp-paths.feature` with the Dangerous 4 scenario and the `sharia-erp`-ends-
      here statement; extend step definitions.

### Phase 4 Gate

- [ ] [AI] All Phase 3 Gate checks re-run and still green; `<CONVMAN>` unchanged at 26.
- [ ] [AI] `yq '.courseOrder | length' "${SHARMAN}"` prints `29`.
- [ ] [AI] **Integration**: draft PR opened, 3-cycle PR-Review complete, CI green, `[AI]` merge,
      `ayokoding-www` deployed, post-deploy curl check confirms `<SHARLANDING>` shows "ENDS HERE".

> **Pause Safety**: both paths are terminal (26/26 and 29/29). The full corpus is live. Safe to stop.
> To resume: `yq '.courseOrder | length' ${SHARMAN}` reads `29` and
> `curl -sf https://ayokoding.com/en/learn/paths/skills/sharia-erp | grep -qi "covers all the basics"`.

## Phase 5: Cross-Path Integrity and Spec Coverage Verification

- [ ] [AI] Run `checkManifestIntegrity` and `checkPrerequisiteConsistency` against **both** final
      manifests together — verify zero violations, and specifically verify no shared course id's body
      differs in content depending on which manifest referenced it (A11 — one body, two references):
      `diff <(git log --follow --format=%H -- "${COURSES}erp-foundations-and-history/_index.md" | wc -l) <(echo 1)` is not the right check; instead assert **no duplicate file exists**:
      `find "${COURSES}" -maxdepth 1 -name 'erp-*' -o -name '*-to-*-systems' -o -name '*-erp*' | sort | uniq -d` — acceptance: empty output (no id appears as more than one directory).
- [ ] [AI] `nx run ayokoding-www:specs:behavior:coverage` reports 100% for `skills-erp-paths.feature`.
- [ ] [AI] `nx run ayokoding-www:test:unit` and `:test:e2e` both green for the full corpus.

### Phase 5 Gate

- [ ] [AI] All checks above pass. **Integration**: draft PR (if any residual changes), 3-cycle
      PR-Review, CI green, `[AI]` merge.

> **Pause Safety**: the full corpus is integrity-verified. Safe to stop. To resume: re-run
> `nx run ayokoding-www:specs:behavior:coverage`.

## Phase 6: Section and App Verification

Grep-checkable licensing and trademark acceptance clauses (A8) — each clause fails when violated,
never passes vacuously.

- [ ] [AI] **No vendor name in any course id, path id, or product name**:
      `grep -riE 'sap|oracle|netsuite|erpnext|odoo' <(printf '%s\n' "${ERP_ALL[@]}" skills/conventional-erp skills/sharia-erp)` —
      acceptance: **empty output** (a non-empty match is a trademark-rule violation and fails this
      clause).
- [ ] [AI] **No verbatim standards-text reproduction**: for each of the AAOIFI FAS numbers named in
      the grounding file (FAS 3, 4, 7, 9, 10, 28, 32, 33, 34), confirm no course body contains a
      100+-character verbatim span matching AAOIFI's own published standard text — this requires a
      `web-researcher`-assisted diff against the official AAOIFI standard for any course quoting a
      FAS number; **acceptance**: for every quoted FAS number, the confirming dispatch reports "no
      verbatim match found", or the offending span is rewritten before this clause is marked
      complete.
- [ ] [AI] **No screenshot of proprietary software**: `find apps/ayokoding-www/content/en/learn/courses -path '*erp*' -o -path '*sharia*' | xargs -I{} find {} -iname '*.png' -o -iname '*.jpg' 2>/dev/null` —
      acceptance: **empty output** (this corpus ships no binary image assets at all, per its
      no-net-new-screen exemption; any match fails this clause and must be investigated).
- [ ] [AI] **No chart of accounts lifted from a reference implementation**: manual review confirms
      every worked example's dataset in every By-Example course under `ERP_ALL` uses an
      originally-authored account/item/customer/vendor naming scheme, cross-checked against no known
      reference implementation's public sample-data set names (Odoo demo data, ERPNext demo data) —
      `apps-ayokoding-www-facts-checker` performs this check per course; **acceptance**: checker
      reports zero matches for every course.
- [ ] [AI] **Every syllabus's Scope note ends with the inherited licence tag**:
      `for id in "${ERP_ALL[@]}"; do grep -qF 'License-aware (DD-15)' "${SYL}${id}.md" || echo "MISSING TAG: $id"; done | grep -q . && echo FAIL || echo PASS` —
      acceptance: prints `PASS`.

### Phase 6 Gate

- [ ] [AI] All five clauses above pass (not vacuously — each has a real failure mode that was
      checked, not merely a command that could never fail).

> **Pause Safety**: licensing/trademark posture is verified across the full corpus. Safe to stop. To
> resume: re-run the five clauses above.

## Phase 7: Manual UI Retest (Rule 15)

Per [tech-docs.md §R9 gate posture](./tech-docs.md#r9-gate-posture-declared-explicitly), this plan is
UI-gate-exempt; the three-tester retest is the mandatory non-vacuous substitute.

- [ ] [AI] Dispatch `web-exploratory-tester` (spec-aware) against both live landings
      (`/en/learn/paths/skills/conventional-erp`, `/en/learn/paths/skills/sharia-erp`) in `delivery`
      mode — verify zero CRITICAL/HIGH findings.
- [ ] [AI] Dispatch `web-usability-tester` (spec-blind) against both landings — verify zero
      CRITICAL/HIGH findings.
- [ ] [AI] Dispatch `web-design-tester` (design-aware) against both landings — verify zero
      CRITICAL/HIGH findings, and specifically confirm the Dangerous-N ramp table renders
      legibly and the color-blind-friendly palette (inherited from plan 03's design system) is
      preserved.

### Phase 7 Gate

- [ ] [AI] All three testers report zero CRITICAL/HIGH findings, or every finding is fixed and
      re-verified.

> **Pause Safety**: both landings are manually retested and clean. Safe to stop. To resume: re-dispatch
> the three testers.

## Phase 8: Full-Corpus Integration Verification

- [ ] [AI] `nx run ayokoding-www:build` succeeds with both manifests and all 29 course bundles
      present.
- [ ] [AI] `nx affected -t build,test:quick,lint --base=main` is green for `ayokoding-www`.
- [ ] [AI] End-to-end path-walk: navigate `/en/learn/paths/skills/conventional-erp`, step through
      prev/next across all 26 courses via Playwright MCP, verify no broken link and no console error;
      repeat for `/en/learn/paths/skills/sharia-erp` across all 29.

### Phase 8 Gate

- [ ] [AI] Build succeeds; affected checks green; both path-walks complete with zero errors.
      **Integration**: final draft PR (if any residual changes) merged, `ayokoding-www` deployed.

> **Pause Safety**: the full corpus builds and both paths are walkable end to end. Safe to stop. To
> resume: re-run the Playwright path-walk.

## Phase 9: Knowledge Capture

> _Triage every surviving `learnings.md` entry before archival. See the
> [Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md)._

- [ ] [AI] Apply the litmus test to every `learnings.md` entry — keep only if a durable surface would
      catch this automatically next time; discard the rest with a one-line reason.
- [ ] [AI] Apply the secret/sensitivity gate — sanitize or discard any entry naming a real credential
      or private hostname.
- [ ] [AI] Apply the repo-relevance gate — infra-private content (none expected in this plan) stays
      out of `ose-public`.
- [ ] [AI] Route each surviving learning to exactly one durable home per the open-ended routing
      matrix; a code-homed learning is filed as a separate `plans/backlog/<slug>/` plan, never landed
      inline.
- [ ] [AI] If no generalizable learning surfaced, record `No generalizable learnings — <reason>` in
      `learnings.md`.

### Phase 9 Gate

- [ ] [AI] Every `learnings.md` entry is terminal (routed, filed as backlog, discarded with reason),
      or the explicit "none" escape is recorded.
- [ ] [AI] No code-homed learning landed inline in this plan's own commits/PR.

> **Pause Safety**: `learnings.md` is fully triaged. Safe to stop. To resume: re-read `learnings.md`
> and confirm every entry is terminal.

## Phase 10: Plan Archival

- [ ] [AI] `git mv plans/backlog/ayokoding-learning-path-07-skills-erp plans/done/$(date +%Y-%m-%d)__ayokoding-learning-path-07-skills-erp`.
- [ ] [AI] Update `plans/backlog/README.md` and `plans/backlog/ayokoding-learning-path-programme.md`
      to remove this plan's backlog entry and reflect its completed status.
- [ ] [AI] Commit and push the archival move in the same PR as any final residual change; merge.

### Phase 10 Gate

- [ ] [AI] The plan folder exists under `plans/done/` with the date prefix; no reference to it remains
      under `plans/backlog/`.

> **Pause Safety**: the plan is archived. Terminal state — no further resume needed.

## File impact and rollback

See [tech-docs.md §File impact](./tech-docs.md#file-impact) and
[§Rollback](./tech-docs.md#rollback) — this delivery checklist implements exactly that file set,
phase by phase, with no step outside it.
