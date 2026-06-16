# Business Requirements Document — Salary Savings Calculator

## Business Rationale

`ayokoding-www` serves developers and tech learners, many of whom weigh relocation, remote-work, or
job-offer decisions. A salary-savings calculator turns an abstract salary number into a concrete,
comparable signal: _how much can I actually keep, here vs. elsewhere?_ It is sticky, shareable, and
reinforces the site's practical, career-oriented brand.

It is also a low-risk first **interactive tool** for the site — pure client-side, static data, no
backend — that proves out a reusable `tools/` pattern for future calculators.

## Affected Roles

- **Visitor / tech worker** — primary user; inputs a salary, compares cities, reads savings.
- **Relocation / remote-work planner** — uses the comparison table to shortlist destinations.
- **Content / site owner** — gains an engaging, low-maintenance feature; owns dataset accuracy.
- **swe-typescript-dev / swe-ui-maker agents** — implement calculation core and UI.

## Goals & Success Metrics

- **G1**: Visitors can compute monthly savings for a salary across tech-hub cities in both en and id.
- **G2**: Savings shown as percentage **and** local-currency amount, per the request.
- **G2b**: Cost basis adjustable by household type (single → married + 3 kids), area (city center vs
  rural), and — for households with kids — public vs private school (median cost).
- **G3**: Tool is fully client-side and deterministic (static dataset), no new infra or API keys.
- **G4**: Calculation core has dedicated unit tests; page meets WCAG AA and is responsive.

Success signals: feature ships behind `/[locale]/tools/salary-savings` in both locales; calc module
test coverage meets the app threshold; fe-e2e smoke test passes in CI.

## Constraints

- Static curated dataset only; values are estimates with a recorded snapshot date and a visible
  "estimates only" disclaimer. Household and area adjustments use shared multiplier tables (not
  per-city data); school cost is a per-city public/private median.
- **Israeli cities are deliberately excluded** from the dataset (explicit product constraint). This
  is a country-level choice about the state of Israel and its political stance, **not** a choice
  about any ethnic, racial, or religious group. The exclusion targets the country and its political
  stance only.
- Must follow existing `[locale]` routing and i18n; no new third-party dependencies. UI reuses the
  shared `web-ui` kit; the one missing primitive (a `Table` for the comparison view) is added to
  `libs/web-ui` rather than hand-rolled in the app.
- Bilingual parity (en/id) is mandatory.

## Risks & Mitigations

| Risk                                                     | Impact | Mitigation                                                                                             |
| -------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------------ |
| Cost-of-living / FX figures drift or mislead             | Med    | Snapshot date + "estimates only" disclaimer; centralize in data                                        |
| Scope creep (taxes, live FX, charts)                     | Med    | Explicitly deferred in PRD out-of-scope; iterate later                                                 |
| First interactive page diverges from patterns            | Low    | Reuse Tailwind + i18n conventions; calc logic isolated and tested                                      |
| Negative savings (cost > salary) confuses UI             | Low    | Define and test the deficit case; show negative clearly                                                |
| Household/area multipliers + school medians oversimplify | Med    | Disclaimer names each approximation; indicative values sourced in Phase 1; per-city overrides deferred |

## Out of Scope

Live data APIs, tax/deduction modelling, savings goals, persistence/sharing/export, non-default
currencies per city, and any Israeli city. These are candidates for later iterations.
