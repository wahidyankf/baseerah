# Business Requirements Document — Salary Savings Calculator

## Business Rationale

`ayokoding-www` serves developers and tech learners, many of whom weigh relocation, remote-work, or
job-offer decisions. A salary-savings calculator turns abstract numbers into concrete, comparable
signals across three angles: _what does it actually cost to live here?_, _how much do I keep after
tax, here vs. elsewhere?_, and _what role would I need to reach a savings goal?_ It is sticky,
shareable, and reinforces the site's practical, career-oriented brand.

The tool models the **real** picture rather than a rule-of-thumb: a full per-category expense
breakdown (housing, food, transport, utilities, healthcare, childcare, lifestyle), net take-home after
a country's federal effective tax plus sub-national tax for federal countries (US/CA/CH), and a
separate one-time relocation budget split into sunk costs and a liquidity reserve. It reports **two**
savings figures (after essentials, and after lifestyle). This credibility — every number modeled,
confidence-tiered, and snapshot-dated — is what differentiates it from generic "salary minus a
living-cost number" calculators.

It also answers the question in reverse — _"to save at least this much net, what is the lowest
**software-engineering** role I'd need, and where?"_ — mapping a savings bar onto a canonical
software-engineering career ladder (IC + management tracks), with role salaries modeled as a per-role
× country **p25 / median / p75** distribution and a typical non-salary comp shown for context. The
ladder is reordered so qualifying roles sit above the minimum and non-qualifying roles below it. This
is a **novel** framing: research found no existing public tool offering a "savings target → minimum
role + city worldwide" lookup, so it is differentiating as well as career-relevant.

It is also a low-risk first **interactive tool** for the site — pure client-side, static data, no
backend — that proves out a reusable `tools/` pattern for future calculators.

### Intended use — salary negotiation and relocation

The tool exists to support **two real-life decisions**, and its scope is sized to do both well
without over-building:

- **Salary negotiation** — a user reads their role's **p25 / median / p75** salary distribution per
  country plus the typical **non-salary comp** (RSU/equity + bonus) → **total compensation**, so they
  can benchmark a concrete offer against the market and set a defensible target. The percentile band
  (not a single number) is what makes the benchmark credible in a negotiation.
- **Relocation evaluation** — a user reads **net-of-tax take-home**, the full per-category expense
  composition, the **two savings figures** (after essentials, after lifestyle), and the one-time
  relocation budget (sunk costs + a liquidity reserve) **per city**, so they compare destinations on
  what they would actually keep, not on gross salary.

Both flows are reachable from the existing three tabs: the Savings and Minimum-role tabs surface
percentile context + total compensation for negotiation; the Cost-of-living, Savings, and
single-city detail views surface net, expenses, savings, and relocation for relocation. No new tab is
introduced.

## Affected Roles

- **Visitor / tech worker** — primary user; reads the cost-of-living breakdown, enters a gross salary
  to compare net savings across cities.
- **Relocation / remote-work planner** — uses the cost-of-living tab (incl. the one-time relocation
  budget, the Region → Country → City cascading filters, and the single-city detail drill-down) and the
  savings tab to shortlist destinations.
- **Career planner / job seeker** — uses the minimum-role tab to see what seniority a savings goal
  implies and where it is most reachable, reading the role × country salary distribution.
- **Content / site owner** — gains an engaging, low-maintenance feature; owns dataset accuracy.
- **web-researcher agent** — sources the per-category expenses (incl. childcare medians),
  per-country federal effective tax bands + per-city sub-national rates (US/CA/CH) + each country's
  healthcare funding model, the split one-time relocation components (incl. key money), the
  **authoritative FX snapshot (ISO-4217 → USD per 1 unit) stored in `fx.ts`**, and the
  software-engineering role taxonomy + the **role × country gross-salary distribution (p25 / median /
  p75) and typical non-salary comp (RSU/equity + bonus)** (all with confidence tiers + snapshot dates).
- **swe-typescript-dev / swe-ui-maker agents** — implement calculation core and UI.

## Business Impact

### Pain Points

Developers and tech learners on `ayokoding-www` currently have no tool to compare real take-home
savings across tech-hub cities. Existing generic "cost of living" sites provide a single aggregate
number — no per-category breakdown, no net-of-tax figure, no savings residual. This forces users to
do their own mental arithmetic across multiple sources, producing unreliable estimates that can
misdirect a relocation or salary-negotiation decision.

The reverse question — _"what is the lowest software-engineering role I'd need to reach a savings
goal?"_ — has no equivalent public tool. Users must manually cross-reference salary surveys against
living-cost data across dozens of cities and roles, a prohibitive task that most abandon. This is a
concrete gap this feature fills with a novel, differentiating framing.

### Expected Benefits

- **Stickiness and shareability** — the tool answers specific, high-stakes questions (relocation and
  negotiation) rather than generic ones; deep-linkable city views (`?tab=cost&city=<id>`) make
  results shareable, driving return visits and social referrals.
- **Brand reinforcement** — a credible, modeled tool (per-category expenses, confidence-tiered,
  snapshot-dated, net-of-tax) reinforces `ayokoding-www`'s practical, career-oriented positioning
  rather than diluting it with rule-of-thumb data.
- **Reusable `tools/` pattern** — the feature is a low-risk first interactive tool (pure
  client-side, static datasets, no backend) that proves out a `tools/` route pattern and a shared
  `Table` primitive for future calculators, reducing the marginal cost of the next tool.
- **Content owner low maintenance burden** — static datasets with a curated refresh cycle mean no
  infra to maintain; the dataset owner updates a TypeScript file and pushes, no pipeline required.

## Goals & Success Metrics

- **G1**: Visitors can read a full per-city monthly expense breakdown (housing, food, transport,
  utilities, healthcare, childcare, lifestyle) with essentials subtotal, total, a separate one-time
  relocation sunk-cost line, and a separately labelled liquidity reserve, in both en and id; the
  healthcare funding scheme is always shown.
- **G2**: For a gross salary, the tool shows **net take-home after federal + sub-national tax** and
  the resulting **two** monthly savings figures (after essentials, and after lifestyle, each amount +
  %) across tech-hub cities, sortable.
- **G2b**: Cost basis adjustable by household (single/married + counts of pre-school and school-age
  children), area (city center vs rural), and — for households with school-age children — public vs
  private school (median cost).
- **G2c**: Given a savings **baseline** (own salary, a reference city + role, or a raw target), the
  tool names the **minimum software-engineering role** worldwide (IC + management) that reaches at
  least as much net **essential savings** (lifestyle excluded; computed from the role × country
  **median** salary) in absolute terms, with savings shown in USD, local, and a user-chosen display
  currency, the best city + its country named, the p25/median/p75 distribution shown, and the ladder
  reordered so qualifying roles sit above the marked minimum and non-qualifying roles below it. The
  shared Region → Country → City filters scope the candidate cities.
- **G3**: Tool is fully client-side and deterministic (static datasets), no new infra or API keys.
- **G4**: Calculation core has dedicated unit tests; page meets WCAG AA and is responsive.

Success signals: feature ships behind `/[locale]/tools/cost-of-living-calculator` in both locales with all three
tabs; calc, tax, and reverse-lookup modules' test coverage meets the app threshold; fe-e2e smoke test
passes in CI.

## Constraints

- Static curated datasets only; values are estimates with a recorded snapshot date and a visible
  "estimates only" disclaimer. The model is a **net-of-tax, per-category expense composition** — no
  budgeting heuristics (no 50/30/20, no percent-of-take-home). Household and area adjustments use a
  shared **OECD-modified** equivalence basis (sub-linear housing/utilities, near-per-capita
  food/healthcare/childcare), not per-city data; childcare and school costs are per-city medians;
  relocation is a one-time per-city estimate split into sunk costs and a liquidity reserve.
- **Tax accuracy** — net take-home uses a **simplified banded effective rate**: a per-country
  **federal** rate plus a per-city **sub-national** rate for federal countries (US states, Canada
  provinces, Switzerland cantons), not a full progressive bracket engine. Real effective rates vary
  with deductions, filing status, benefits-in-kind, and social-contribution caps, so each band carries
  a confidence tier and the UI discloses the simplification. Full brackets, equity/bonus, and
  per-individual tax are out of scope.
- **Richer dataset** — each city now stores six expense categories, a relocation block, and a country
  FK (with per-country tax bands) rather than a single living-cost number; the curation burden is
  higher, so `web-researcher` sourcing with per-cell confidence tiers is mandatory and uneven data
  is flagged rather than fabricated.
- **FX is stored in-repo and single-sourced** — currency conversion rates live in `fx.ts` (an
  ISO-4217 → USD-per-unit table + `fxSnapshotDate`), sourced by `web-researcher`. Every
  conversion in the app (local → USD, USD → chosen display currency) reads from this one table, and
  each city's `fxToUsd` is derived from it via the city's `currency` rather than hand-entered.
- The **role-salary matrix** (`roles.ts`) is likewise static and `web-researcher`-sourced; public
  salary data is uneven outside the US, so each cell carries a confidence tier (`high` | `moderate` |
  `proxy`) and lower-confidence rows are flagged in the UI. The role ladder is a synthesised
  industry-consensus taxonomy (no standards body publishes one), documented in `tech-docs.md`.
- **Israeli cities are deliberately excluded** from the dataset (explicit product constraint). This is
  a country-level choice about the state of Israel and its political stance, **not** a choice about any
  ethnic, racial, or religious group. The exclusion targets the country and its political stance only.
- Must follow existing `[locale]` routing and i18n; no new third-party dependencies. UI reuses the
  shared `web-ui` kit; the one missing primitive (a `Table` shared by all three tabs) is added to
  `libs/web-ui` rather than hand-rolled in the app.
- Bilingual parity (en/id) is mandatory.

## Risks & Mitigations

| Risk                                                                      | Impact | Mitigation                                                                                                                                                                                                                                                       |
| ------------------------------------------------------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Tax accuracy** — banded effective rate oversimplifies real tax          | Med    | Federal + sub-national (US/CA/CH) per-band confidence tiers; "simplified effective tax, not a bracket calculation; excludes filing status/deductions/benefits-in-kind/contribution caps" disclaimer; full brackets/equity/per-individual explicitly out of scope |
| **Category-data unevenness** — per-category public data is patchy         | Med    | Per-cell confidence tiers; `proxy` cells derived from documented regional multipliers, never fabricated; lower-confidence cells flagged in the UI                                                                                                                |
| **Nominal-FX vs PPP confusion** — USD savings misread as purchasing power | Med    | Explicit disclaimer that USD uses a nominal FX snapshot, not PPP; PPP comparison out of scope                                                                                                                                                                    |
| **Healthcare double-counting** — premium counted in tax and expense       | Med    | `healthcare` models out-of-pocket only for tax-funded/mixed countries; funding scheme always shown as a badge; `healthcareModelType` + `compulsoryInsurance.note` document which side the premium sits on                                                        |
| **Relocation estimate variance** — deposit/key-money/cushion are ranges   | Low    | v1 picks a documented midpoint per component; sunk costs vs liquidity reserve shown separately; confidence tier + disclaimer that relocation is an informational one-time estimate                                                                               |
| FX figures drift or mislead                                               | Med    | Single authoritative FX table in `fx.ts` (ISO-4217 → USD per unit) used for every conversion; `fxSnapshotDate` + "estimates only" disclaimer; per-city `fxToUsd` derived from `fx.ts` so no rate is hand-entered twice                                           |
| Scope creep (full brackets, live FX, charts)                              | Med    | Explicitly deferred in PRD out-of-scope; iterate later                                                                                                                                                                                                           |
| First interactive page diverges from patterns                             | Low    | Reuse Tailwind + i18n conventions; calc logic isolated and tested                                                                                                                                                                                                |
| Negative savings (expenses > net) confuses UI                             | Low    | Define and test the deficit case; show negative clearly                                                                                                                                                                                                          |
| OECD-modified multipliers + childcare/school medians oversimplify         | Med    | Disclaimer names each approximation; OECD-modified basis documented; indicative values sourced in Phase 1; per-city overrides deferred                                                                                                                           |
| Role-salary data uneven / stale outside US tech hubs                      | Med    | `web-researcher`-sourced per role × country as a p25/median/p75 distribution with per-cell confidence tier; low-confidence rows flagged; snapshot date shown; proxy cells derived from documented regional multipliers, never fabricated                         |
| Role salary modeled at national (country) level, not per city             | Med    | Documented simplification: cities inherit their country's role-salary distribution; per-cell confidence + the displayed p25/median/p75 band + an explicit "salary is national-level" disclaimer; per-city salary overrides deferred                              |
| Non-salary comp (RSU/equity/bonus) misread as part of savings             | Low    | Shown only as an informational total-comp column with a clear note; never folded into net or either savings figure; "RSU/equity/bonus modeling into savings" kept in Out of Scope                                                                                |
| Single linear "minimum" ordering across IC + mgmt tracks ambiguous        | Low    | Rank by resulting absolute savings (primary, on the median salary); documented seniority ordering only as display tiebreaker; ladder reordered into qualifying / below-minimum groups — `tech-docs.md` records the rule                                          |

## Out of Scope

Live data APIs (cost-of-living, FX, salary, **or tax**); **full progressive tax-bracket engines**;
**social-contribution caps**; **benefits-in-kind**; **pension / retirement contribution modeling**;
**clothing / personal-care as separate categories**; **PPP-adjusted (real purchasing-power)
comparison**; **equity / RSU / bonus modeling into savings** (the typical non-salary comp is displayed
as context only); **per-city role-salary granularity** (salary is per role × country; cities inherit
it); **deduction optimization**; **per-individual tax situations**; savings goals;
persistence/sharing/export; per-city non-default currencies; company-specific salary breakdowns;
per-person career-progression modeling; and any Israeli city. These are candidates for later iterations.
