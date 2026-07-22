# Business Requirements — Skills Paths: Accounting

> **Programme decisions** — the `R*` rules and `A*` amendments cited below are defined locally in
> [tech-docs.md §Programme decisions](./tech-docs.md#programme-decisions) (folded from the retired
> shared programme file).

## Business Goal

Open a **second subject domain** on ayokoding.com, as **two** paths. Today the entire 121-course
library is software engineering; this plan ships the platform's first non-software-engineering
content, `/en/learn/paths/skills/conventional-accounting` and
`/en/learn/paths/skills/sharia-accounting` (A10), as immediately-effective arcs for the reader who
has to build or reason about financial systems and has no accounting background.

The goal is not "teach accounting". Textbooks and certifications already do that, for accountants.
The goal is **accounting for people who build systems**: the reader leaves able to design a chart of
accounts as a schema, produce a balancing ledger, recognise the mistakes that still balance, and
architect — never build — the ledger system itself. A reader who continues into the Sharia path
additionally leaves able to model murabaha, ijara, mudaraba, musharaka, zakah and sukuk correctly
rather than as conventional instruments with different labels.

Three business consequences follow, and all three are load-bearing:

1. **It unblocks ERP.** `ayokoding-learning-path-07-skills-erp` cannot deliver its record-to-report
   capability without a balanced ledger to post into. Accounting first is a dependency fact, not a
   preference.
2. **It is the platform's proof that the path machinery is subject-agnostic — twice over.** Everything
   built by plans 01–03 was designed around one subject and one path per subject. Two
   accounting paths sharing nineteen courses through one manifest schema is the first evidence the
   machinery generalises to _shared_ course reuse across paths in the same category, not merely to a
   second subject.
3. **It teaches the domain to founding depth without teaching construction.** A6 forbids any
   capstone that builds a system. The corpus instead ends each path with an architecture course —
   what the reader needs to know to found an implementation, stopping short of asking them to write
   one.

## Why two paths (A10) and why accounting lands before ERP

**Two paths, not one.** A single `skills/accounting` path serving both a conventional-only reader and
a Sharia-compliant-systems reader forces a bad trade: pad the conventional reader with material they
never use, or bury the Sharia depth as an optional tail most readers never reach. A10 splits the
corpus instead — `conventional-accounting` is nineteen courses and complete in itself;
`sharia-accounting` is the same nineteen plus five Sharia-specific courses, twenty-four total, and is
never merely "the first path with extra units bolted on." **Both paths teach all the basics.** The
nineteen shared courses are authored exactly once and referenced from both manifests by ID (A11) —
the schema already supports this (see [tech-docs §Licensing and IP Compliance](./tech-docs.md) and
[tech-docs §Two manifests, nineteen shared courses](./tech-docs.md#two-manifests-nineteen-shared-courses-a10--a11)
for the cited schema rulings), so this is zero-marginal-cost reuse, not a duplicated corpus.

**Accounting before ERP is forced by dependency direction, not chosen for convenience:**

- **ERP depends on Accounting one-directionally. There is no cycle.** Nothing in either accounting
  path needs any ERP course.
- The hard edge first bites at ERP's **record-to-report capability** (subledger→GL posting is
  meaningless without a balanced ledger) — see
  [§The 06→07 dependency edge](./README.md#the-0607-dependency-edge-stage-granularity-not-course-numbers)
  for why this is now named by capability, not by an ERP course number.
- ERP's early courses and this plan's shared Stage 1 (#1–#3) are parallel-authorable; the hard
  convergence only bites at ERP's own later stage.

Folding accounting and ERP into one plan was rejected for the same reasons the original single-path
plan rejected it: two corpora whose only coupling is a set of one-directional prerequisite edges do
not belong in one plan or one PR stream.

## Why twenty-four courses, in nineteen-plus-five (A9)

**The count is curriculum judgment, not a sourced fact** [Judgment call], labelled as such in every
document that states it, including the catalog table. What changed from the original twenty-course
single-path design, and why each change is principled rather than arbitrary:

- **Two capstones deleted (A6).** `capstone-build-a-general-ledger-system` and
  `capstone-sharia-compliant-ledger` asked the reader to build a system. Neither the domain knowledge
  those courses carried, nor the linked cross-domain prerequisite each declared, is lost — both
  survive in the two new **architecture** courses, `general-ledger-system-architecture` and
  `sharia-ledger-system-architecture`, which teach the same subledger-to-GL, posting-rule, and
  document-state-machine material **without** the build instruction.
- **Two courses added to the shared conventional spine, both closing a genuine domain gap the
  original twenty never covered**: `journal-entries-and-posting-mechanics` (posting rules, batch
  posting, reversing entries, suspense accounts — the systems-level mechanics that "three courses to
  a balancing ledger" implies but the original catalog never made explicit as its own subject) and
  `multi-currency-accounting-and-fx-translation` (FX translation methods, realised versus unrealised
  gain/loss, cumulative translation adjustment — a real, commonly-needed domain area the original
  twenty omitted entirely despite naming consolidation, which cannot be taught honestly without it).
- **Three courses added to the Sharia stage**, expanding it from two courses to five now that it is a
  full path rather than a four-course tail on a twenty-course single path:
  `zakah-computation-and-reporting-for-systems` (AAOIFI FAS 9 is `[Verified]` in the seeding research
  yet the original corpus never taught Zakah — a real gap for a body of standards this corpus already
  cites), `sukuk-and-islamic-capital-markets-accounting` (AAOIFI FAS 32–34 are `[Verified]` and cover
  Ijarah through sukuk-holder reporting, also never taught), and `sharia-ledger-system-architecture`
  (the non-building replacement for the deleted Sharia capstone).

Net: nineteen shared courses (was sixteen after removing the conventional capstone and gaining two)
plus five Sharia-specific courses (was two after removing the Sharia capstone and gaining three) —
twenty-four authored bodies, expanding past the original twenty as A9 requires, with every addition
traceable to either a domain gap the research already evidenced (Zakah, Sukuk) or a structural
requirement of the split itself (the two architecture courses).

**The shape that survives unchanged**: three courses to first payoff, the ramp slowing deliberately
after that point because the domain's failures stop being loud, and the Sharia depth sitting at the
end because applying conventional models to Sharia contracts before the conventional model is solid
is the exact silent mistake this corpus exists to prevent. A10 changes _how many paths_ carry this
shape; it does not change the shape.

## Business Impact

**Pain points addressed**

- **The platform teaches one subject.** Unchanged from the original rationale: a reader who needs a
  domain other than software engineering has nothing here.
- **The existing accounting material is the wrong artefact for the wrong reader**, and now for two
  wrong readers. `business/accounting.md` is a 34 KB single page for small-business owners, with no
  Sharia treatment at all.
- **ERP is blocked with no path forward.**
- **Sharia-compliant financial systems have no learning surface anywhere on the platform**, despite
  being the parent project's entire reason for existing, and the gap is now deeper than the original
  plan recognised: Zakah and Sukuk accounting — both directly evidenced by verified AAOIFI standards —
  had no course at all, even in the original twenty-course design.

**Expected benefits** (qualitative reasoning; no fabricated metrics)

- **Two subject-domain paths**, proving the shared-library-plus-manifest model generalises to
  **shared course reuse across sibling paths**, not merely to a second subject.
- **ERP unblocked at the same early stage as before** — the stage-signal contract's shape is
  unchanged; only its ERP-side vocabulary moved from course numbers to capability names, which is a
  robustness improvement, not a delay.
- **A materially stronger position on Sharia accounting** than the original plan: five dedicated
  courses instead of two, covering the standards landscape, contract modelling, Zakah, Sukuk, and
  ledger architecture — the difference between a resource that touches Sharia accounting and one that
  actually equips a builder for Bahrain, Indonesia or Malaysia.
- **A defensible licensing posture stated up front (A8)**, rather than discovered as a risk during
  authoring — see [Business Risks and Mitigations](#business-risks-and-mitigations) below.
- **A reusable ramp pattern that now also demonstrates path-termination**: `conventional-accounting`
  is the first path in the programme whose manifest is designed to **stop growing** at a stated point
  rather than growing to a single terminal size, which is a pattern every future two-tier skills
  subject can reuse.

## Affected Roles

Solo-maintainer repo — no sign-off ceremony. The maintainer wears:

- **Content strategist** — owns both ramps, their shared boundary, and what is deliberately omitted
  from each.
- **Domain researcher** — owns the verification debt: resolving OI-2 against a primary source before
  any doctrinal riba claim is written as fact, and confirming OI-1/OI-3's resolution status is
  re-verified rather than assumed at authoring time.
- **Licensing steward** — owns the clean-room posture (A8): every standard is restated in original
  words, every chart of accounts is originally authored, and no copyleft reference-implementation
  code is ever pasted into a course.
- **Content author** — authors 24 syllabus specs and 24 course bodies via the ayokoding maker agents.
- **Frontend engineer** — authors two YAML manifests the `course-paths` feature loads and validates at
  build time.
- **Content reviewer** — validates the bodies and both landings via the ayokoding checkers.

Consuming agents: `apps-ayokoding-www-by-example-maker` and `apps-ayokoding-www-annotated-concept-maker`
(the two formats this corpus uses), their matching checkers and fixers,
`apps-ayokoding-www-general-maker` (landing prose), `apps-ayokoding-www-facts-checker`,
`apps-ayokoding-www-link-checker`, `web-researcher` (the verification debt, every external claim, and
the post-authoring syllabus verification pass), `apps-ayokoding-www-deployer`, and the three
live-site testers `web-exploratory-tester`, `web-usability-tester`, `web-design-tester` for the
Rule-15 retest [Repo-grounded — each verified present under `.claude/agents/`].

## Business-Level Success Metrics

Every metric is an **observable check**, not a projected number. Each is falsifiable in both
directions — the "before" value is stated so a vacuous pass is impossible.

- **Two skills manifests published, at their respective full compositions** (observable):
  `manifests/skills/conventional-accounting.yaml` holds exactly 19 IDs;
  `manifests/skills/sharia-accounting.yaml` holds exactly 24 IDs, the last five of which are the
  Sharia-specific courses and the first nineteen of which are the **same** IDs as the conventional
  manifest, byte-for-byte. Before Phase 2 neither file exists; after Phase 2 both hold 3; after
  Phase 3, both hold 19 and the conventional manifest is **done growing**; after Phase 5, the Sharia
  manifest holds 24.
- **Twenty-four course bundles resolve** (observable): each of the 24 course IDs resolves to a
  directory under `content/en/learn/courses/`. Before Phase 2 all 24 are missing. **Asserted by ID,
  never by a global directory count** — plan 04 authors concurrently.
- **Both 2-segment `pathId`s resolve end-to-end** (observable):
  `/en/learn/paths/skills/conventional-accounting` and `/en/learn/paths/skills/sharia-accounting`
  each render their ordered course list, and `?path=` propagates through prev/next and the
  breadcrumb for both.
- **No course body is duplicated** (observable): `<COURSES>` contains exactly 24 course directories
  once Phase 5 completes — never 43 (19 shared + 19 duplicated-into-Sharia + 5 Sharia-specific) — and
  the shared 19 IDs are byte-identical strings across both manifest files.
- **The ramp is visible to a reader on both landings** (observable): each landing states its own
  path's boundaries, in prose a reader meets before the course list.
- **No prerequisite is walked that should be linked** (observable): neither manifest's `courseOrder`
  contains `sql-essentials` or `backend-essentials`, while the courses that need them declare them in
  frontmatter.
- **The silent-failure requirement is met** (observable): every shared course from #4 onward and
  every Sharia-specific course carries an explicit "what still balances while being wrong" section.
- **Zero laundered verification claims** (observable): at the Phase 4 gate, **no `[Needs Verification]`
  marker is unaccounted for** — every one still standing is named, with a reason, in
  `verification-log.md`'s `## Carried residuals` register. Literal zero markers is deliberately **not**
  the target: some residuals (OI-2's doctrinal basis, the PPSAK ratification date, Fineract's
  Islamic-finance suitability) are permanent until a primary source exists, and `A4` requires them to
  stay marked rather than be quietly upgraded to fact.
- **Three jurisdictional models, not one** (observable): every Sharia-specific course that discusses
  standards names AAOIFI, PSAK Syariah **and** MFRS-plus-BNM, and none describes AAOIFI as "the"
  standard.
- **No relicensed content** (observable): a reading audit at the Phase 6 gate finds zero verbatim
  reproduction of standards text, proprietary chart-of-accounts structure, or copyleft
  reference-implementation code (A8).
- **Stage signals emitted at stage, not course-number, granularity** (observable): each recorded
  signal names an ERP capability, never an ERP course number.
- **No regressions** (observable): `ayokoding-www:build`, the affected test tiers,
  `specs:behavior:coverage`, heading-hierarchy, markdownlint, and link validation all pass.

## Business-Scope Non-Goals

- **Any ERP content.** The full ERP corpus, its manifest(s), and its landing(s) belong to
  `ayokoding-learning-path-07-skills-erp`.
- **Any structural `_index.md` under `paths/`.** Including **both** `paths/skills/_index.md` — plan
  01's (A3). This plan creates only its own two path-landing bundles.
- **Re-authoring or editing any existing library course.** `sql-essentials` and `backend-essentials`
  are linked, never forked.
- **Accountancy certification coverage.** Not a CPA/ACCA/CA syllabus.
- **Tax jurisdiction depth.** `payroll-and-tax-accounting-essentials` covers ledger mechanics, not
  any country's tax code.
- **Corporate finance.** Valuation and capital structure stay out.
- **An Indonesian mirror of either path.** `id/belajar/` holds zero courses and zero paths.
- **A second skills arc.** Every skills path is `immediately-effective` (R8).
- **Building a system, anywhere in the corpus (A6).** No capstone, no "implement X" exercise, no
  scaffolded codebase.
- **Reproducing any standards text, proprietary chart-of-accounts structure, or copyleft
  reference-implementation code (A8).**

## Business Risks and Mitigations

| Risk                                                                                                                                                                                      | Mitigation                                                                                                                                                                                                                                                                 |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **A course teaches a plausible, silently wrong model.**                                                                                                                                   | Every course from #4 onward, in either path, carries a mandatory "what still balances while being wrong" section, grep-verifiable at its authoring step; the ramp deliberately slows after #3.                                                                             |
| **A course reproduces licensed standards text or a proprietary chart-of-accounts structure (A8).**                                                                                        | The eleven safe-authoring rules in [tech-docs §Licensing and IP Compliance](./tech-docs.md#licensing-and-ip-compliance-a8) bind every course; every chart of accounts is originally authored (no public-domain template exists); a reading audit runs at the Phase 6 gate. |
| **AAOIFI is presented as "the" Sharia accounting standard.**                                                                                                                              | Three jurisdictional models are a stated content invariant across every Sharia-specific course that discusses standards, re-asserted at the Phase 5 gate.                                                                                                                  |
| **An `[Unverified]` research claim is restated as fact.**                                                                                                                                 | Phase 4 gates the entire Sharia stage; every external claim carries a confidence marker; `apps-ayokoding-www-facts-checker` runs on every body.                                                                                                                            |
| **A shared course is accidentally duplicated instead of referenced (A11).**                                                                                                               | The Phase 6 ownership sweep asserts exactly 24 directories under `<COURSES>`, never 43; both manifests' first-19 entries are asserted byte-identical.                                                                                                                      |
| **The `conventional-accounting` manifest keeps growing past its intended terminus**, silently becoming a fork of the Sharia manifest.                                                     | The Phase 3 gate asserts `conventional-accounting.yaml` is **untouched** from Phase 3 onward — a falsifiable clause checked again at Phase 5, 6, 8 and 10.                                                                                                                 |
| **"Interleave" (A11) is read as mid-ramp alternation**, scattering Sharia content through the conventional spine and reintroducing the exact silent mistake the corpus exists to prevent. | Resolved explicitly as a design decision ([tech-docs DD-601](./tech-docs.md#design-decisions)): the array composition is shared-then-Sharia; the silent-failure pedagogical ordering is preserved.                                                                         |
| **Course-number-keyed stage signals go stale** the moment either plan renumbers.                                                                                                          | The stage-signal contract now names ERP capabilities, never ERP course numbers (see [README §The 06→07 dependency edge](./README.md#the-0607-dependency-edge-stage-granularity-not-course-numbers)).                                                                       |
| **Manifest ships truncated and is never grown.**                                                                                                                                          | Falsifiable before/after deferred-ID checks at every publication and growth step; terminal gates assert the full 19 (conventional) or 24 (Sharia) IDs present.                                                                                                             |
| **Scope creep into ERP.**                                                                                                                                                                 | Each affected body states its scope boundary explicitly against the ERP course it abuts; the boundary statement is a grep-checkable acceptance clause.                                                                                                                     |
| **The linked prerequisites get walked.**                                                                                                                                                  | Both manifests are asserted to contain neither `sql-essentials` nor `backend-essentials`, and the corresponding frontmatter is asserted to declare them.                                                                                                                   |
| **Cross-plan file collision** on a shared manifest directory or the skills structural index.                                                                                              | Ownership is scoped to exactly two manifest **data** files per plan, each with its own co-located unit test; neither skills plan creates any `_index.md`.                                                                                                                  |
| **Either landing reads as a table of contents.**                                                                                                                                          | Both landing content contracts require the arc statement and ramp boundaries before the course list; verified by the Rule-15 usability tester.                                                                                                                             |
