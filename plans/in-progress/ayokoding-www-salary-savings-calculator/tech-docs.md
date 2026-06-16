# Technical Documentation — Salary Savings Calculator

## Architecture Overview

A self-contained, **client-side rendered (CSR)** feature in `apps/ayokoding-www`. The page is a
`'use client'` component; all state, input handling, and computation run in the browser. No
server-side rendering of results, no backend, no tRPC procedure, no network at runtime. Three layers:

1. **Data** — a static, hand-curated dataset of tech-hub cities (`cities.ts`).
2. **Calculation core** — pure functions (no React, no I/O) that compute savings; fully unit-tested.
3. **Presentation** — a `'use client'` interactive page under `/[locale]/tools/salary-savings`,
   plus small components for the two modes; consumes the calc core and i18n strings.

```mermaid
flowchart TD
    A[cities.ts dataset] --> B[calc core pure fns]
    C[user input: salary, mode, city] --> B
    B --> D[page + mode components]
    E[i18n strings en/id] --> D
    D --> F[rendered calculator UI]
```

## Proposed File Layout

Exact paths confirmed against the app during Phase 1; the shape:

```
apps/ayokoding-www/src/
  app/[locale]/tools/salary-savings/
    page.tsx                      # 'use client' page; mode toggle + state
  features/salary-savings/        # feature module (co-located)
    data/cities.ts                # static dataset + snapshot date
    data/cities.test.ts           # dataset invariants (no Israel, fields present)
    calc.ts                       # pure calculation functions
    calc.test.ts                  # unit tests for calc
    components/compare-table.tsx  # "Compare all" mode
    components/single-city.tsx    # "Single city" mode
    components/*.test.tsx          # component tests
    strings.ts                    # en/id UI strings (or wired into existing i18n)
```

If the app convention prefers `src/contexts/<name>/` over `src/features/`, follow the existing
pattern discovered in Phase 0 rather than introducing a new top-level folder.

## Data Model

```ts
type City = {
  id: string; // stable slug, e.g. "lisbon"
  name: { en: string; id: string };
  country: { en: string; id: string };
  currency: string; // ISO 4217, e.g. "EUR"
  costOfLivingLocal: number; // est. monthly single-person cost, local currency
  fxToUsd: number; // USD value of 1 local-currency unit (snapshot)
  region: "asean" | "japan" | "europe" | "nordics" | "americas" | "mena" | "asia" | "oceania" | "africa";
};

type Dataset = {
  snapshotDate: string; // ISO date, e.g. "2026-06-16"
  cities: City[]; // tech hubs worldwide; NO Israeli cities
};
```

Goal: cover **as many tech-hub cities worldwide as we reasonably can** (static, breadth-first),
excluding Israel. **ASEAN, Japan, broader Europe, and the Nordics must each be represented** (in
addition to the Americas, Middle East, South/East Asia, Oceania, and Africa). Seed set grouped by
region (extend freely in Phase 1):

- **ASEAN**: Singapore, Bangkok, Ho Chi Minh City, Hanoi, Kuala Lumpur, Penang, Jakarta, Bandung,
  Manila, Cebu, Phnom Penh, Vientiane, Yangon, Bandar Seri Begawan.
- **Japan**: Tokyo, Osaka, Fukuoka.
- **Europe (non-Nordic)**: London, Berlin, Munich, Amsterdam, Lisbon, Porto, Dublin, Paris, Zurich,
  Geneva, Vienna, Tallinn, Warsaw, Kraków, Prague, Barcelona, Madrid, Milan.
- **Nordics**: Stockholm, Gothenburg, Copenhagen, Helsinki, Oslo, Reykjavík.
- **Americas**: San Francisco, New York, Seattle, Austin, Boston, Toronto, Vancouver, Mexico City,
  São Paulo, Buenos Aires, Santiago.
- **Middle East / South & East Asia / Oceania / Africa**: Dubai, Bengaluru, Hyderabad, Seoul,
  Taipei, Shenzhen, Sydney, Melbourne, Nairobi, Lagos, Cairo.

More cities are welcome as long as each has credible cost-of-living + FX estimates.

## Calculation Core (pure)

```ts
// All amounts monthly. Salary is GROSS (pre-tax); taxes/deductions are NOT modelled in v1.
// fxToUsd = USD per 1 local unit.
costUsd(city): number                     // city.costOfLivingLocal * city.fxToUsd
compareRow(city, salaryUsd): {            // "Compare all" mode
  savingsUsd, savingsLocal, savingsPct }
singleCity(city, salaryLocal): {          // "Single city" mode
  costLocal, savingsLocal, savingsPct }
sortBySavings(rows): rows                 // desc by savings
```

- Percentages and amounts may be negative (deficit) and must be tested for that case.
- Guard `salary <= 0` to avoid division-by-zero in percentage (return 0% or N/A, decided + tested).
- Functions are deterministic and side-effect-free → straightforward Vitest coverage.

## Presentation

- `page.tsx` is `'use client'`; holds mode (`compare` | `single`), salary input, selected city.
- Number/currency formatting via `Intl.NumberFormat` keyed on city `currency` and active locale.
- Mode toggle, sortable table, and single-city breakdown are plain React + Tailwind; reuse existing
  `ayokoding-www` UI/Tailwind conventions (no new dependency).
- Inputs are labeled; table is keyboard-navigable; "estimates only" disclaimer always visible.
- A prominent **"Data last updated: &lt;date&gt;"** label (localized, formatted from the dataset
  `snapshotDate` via `Intl.DateTimeFormat`) sits near the results so users always know the data's
  vintage. v1 has no runtime fetch, so this equals the static snapshot date; if a live source is
  added later, the same label surfaces the actual fetch/update timestamp.

## i18n

Follow the existing `ayokoding-www` i18n mechanism (`src/contexts/i18n/`). Add the calculator's UI
strings (headings, labels, mode names, disclaimer) for both `en` and `id`. City/country display
names live in the dataset (`name.en` / `name.id`) so data and UI strings stay separable.

## Testing Strategy

- **Unit (vitest)**: `calc.test.ts` covers each function incl. deficit and zero-salary edge cases;
  `cities.test.ts` asserts dataset invariants — every city has all fields, currencies are ISO codes,
  and **no Israeli city / `IL` currency is present**. Also assert region coverage — at least one
  city each from **ASEAN, Japan, Europe (non-Nordic), and the Nordics** (e.g. via a `region` field).
- **Component (vitest + Testing Library)**: render each mode, simulate input, assert rendered
  savings %, local amount, and locale strings.
- **E2E (ayokoding-www-fe-e2e, Playwright)**: one smoke test — load `/en/tools/salary-savings`,
  enter a salary, assert a populated table and a savings cell.
- Meet the app's existing coverage threshold (rhino-cli validator in `test:quick`).

## Design Decisions

- **Static dataset over live API** — deterministic, testable, no keys/flakiness; snapshot date +
  disclaimer communicate the trade-off. Live data deferred.
- **Client-only, no tRPC** — pure computation; avoids backend surface and keeps it cacheable/simple.
- **Calc isolated from React** — pure module enables exhaustive, fast unit tests independent of UI.
- **`fxToUsd` as USD-per-local** — single rate per city powers both modes; avoids a rate matrix.

## Risks / Open Questions

- Final cost-of-living/FX numbers need a credible source noted in `cities.ts` comments (curation
  task in Phase 1; figures are estimates).
- Confirm app's feature-folder convention (`features/` vs `contexts/`) in Phase 0 before scaffolding.
- Confirm whether disclaimers/snapshot belong in i18n strings or dataset — default: disclaimer text
  in i18n, snapshot date in dataset.

## Dependencies

None new. Uses Next.js 16 App Router, React 19, Tailwind 4, Vitest, and Playwright already present
in `ayokoding-www` / `ayokoding-www-fe-e2e`.
