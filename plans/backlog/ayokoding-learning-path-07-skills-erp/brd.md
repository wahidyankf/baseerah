# Business Requirements Document — Skills Paths: Enterprise Resource Planning

> **Programme decisions** — the `R*` rules and `A*` amendments cited below are defined in
> [ayokoding-learning-path-programme.md](../ayokoding-learning-path-programme.md).

## Business Goal

Give `ayokoding.com` readers a domain-depth ERP learning surface — enough to found a conventional or
Sharia-compliant ERP implementation, informed enough to evaluate one critically — without the site
ever teaching installation, vendor selection, or system construction, and without exposing the
platform to copyright or trademark risk from the standards bodies, open-source projects, and
commercial curricula the domain touches.

## Business Context

`ayokoding.com`'s `skills/` category currently ships zero ERP content. The domain is one of the two
`skills/` subjects this programme scopes (the other, accounting, is
[`ayokoding-learning-path-06-skills-accounting`](../ayokoding-learning-path-06-skills-accounting/brd.md)).
Two products ship from this plan: `skills/conventional-erp` and `skills/sharia-erp` — both real
products with independent value, not one product with an optional add-on.

## Business Impact

- **Two new content products**, each a complete, standalone learning path (`sharia-erp` covers all
  the basics; it is never gated behind `conventional-erp`).
- **Domain-depth positioning without operational liability**: because no course installs, configures,
  or stands up a live system (A6), and no course teaches vendor evaluation or selection (A7), the site
  makes no implicit claim of operational competence with any named commercial or open-source ERP
  product — reducing both user-expectation risk and vendor-relations risk.
- **Cross-domain reinforcement**: 10 existing software-engineering courses and, once
  `ayokoding-learning-path-06-skills-accounting` ships, 7 accounting courses gain a new downstream
  audience via this corpus's prerequisite edges.
- **Shared-corpus efficiency**: 26 of 29 course bodies serve both products; `A11`'s
  reference-by-id-never-duplicate architecture means the platform maintains one body per shared
  concept, not two — a direct cost avoidance versus a duplicated-content design.

## Affected Roles

- **Content authors** (AI agents in the maker-checker-fixer pipeline) — author 29 new course bodies
  and 30 syllabus files against a settled catalog and prerequisite graph.
- **Site readers** pursuing ERP domain literacy — two new personas, detailed in
  [prd.md](./prd.md#personas).
- **`ayokoding-learning-path-06-skills-accounting`** — the one cross-plan dependency; this plan's
  Stage B and Stage C gates block on that plan's conventional-accounting and sharia-accounting
  completion respectively.
- **`ayokoding-learning-path-03-navigation-ui`** — supplies every rendered component this plan's
  content appears on; this plan supplies content specifications only (see
  [tech-docs.md §Landing content requirements](./tech-docs.md#landing-content-requirements-what-plan-03-cannot-infer)).

## Risks

| Risk                                                                                                                | Likelihood | Impact | Mitigation                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------- | ---------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A course reproduces a copyrighted standard's text, a proprietary system's schema, or copyleft code                  | Medium     | High   | Eleven safe-authoring rules (A8); `apps-ayokoding-www-facts-checker` per-course review; Phase 6 grep-checkable acceptance clauses                             |
| A course reproduces a commercial curriculum's structure (ASCM/APICS CPIM/CSCP) via the syllabus-confirmation step   | Medium     | High   | `A12`'s binding order of operations — author first, confirm coverage only, never adopt structure; enforced procedurally at Phase 1.2a                         |
| A vendor name appears in a course title, path segment, or product name (trademark exposure)                         | Low        | Medium | Nominative-use rule; Phase 6 grep clause scanning every id for vendor-name substrings                                                                         |
| The accounting-side course ids this plan's Stage B/C gates cite change before plan 06 finalizes                     | Medium     | Medium | Mechanical `test -d` gates fail safely (wait, not silent wrong-authoring); coordination risk explicitly flagged in tech-docs.md, re-verified before Phase 3/4 |
| A syllabus's "Advanced topics"-style vague module title makes the corpus unverifiable                               | Low        | Low    | Module-title specificity rule enforced at authoring time; Phase 1.2a confirmation pass would surface any surviving vagueness as unconfirmable                 |
| A jurisdictional claim in the Sharia-exclusive courses (27-29) is stated as settled fact while still `[Unverified]` | Medium     | Medium | A4 verification-status carry-forward; explicit Phase 1.2 re-verification step gates Stage C authoring                                                         |

## Success Criteria

- Both path landings live at `/en/learn/paths/skills/conventional-erp` and
  `/en/learn/paths/skills/sharia-erp`, each rendering its full course count.
- Zero CRITICAL/HIGH findings from the Phase 7 three-tester manual retest.
- All five Phase 6 licensing/trademark acceptance clauses pass.
- No accounting file, careers manifest, component, design asset, or structural `_index.md` modified by
  this plan (ownership-invariant check, verified at every phase gate).
