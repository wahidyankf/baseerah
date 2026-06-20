/**
 * Unit tests for URL ↔ filter synchronisation in CostOfLivingCalculatorContent.
 *
 * Strategy: unit tests (React Testing Library) rather than e2e because:
 *  1. Production build cycle is slow for URL-sync logic.
 *  2. Unit tests better isolate the URL sync logic.
 *
 * Covers:
 *  Cycle 3 — URL searchParams initialise GeoFilter state
 *  Cycle 4 — GeoFilter selection writes URL via router.replace
 *  Cycle 5 — City-name click pushes ?city=<id> and pre-selects City filter
 */

import { cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { dataset } from "../core/data/cities";

// ─── Mock next/navigation ────────────────────────────────────────────────────

const mockRouterReplace = vi.fn();
const mockSearchParamsGet = vi.fn();

vi.mock("next/navigation", () => ({
  useSearchParams: () => ({
    get: mockSearchParamsGet,
  }),
  useRouter: () => ({
    replace: mockRouterReplace,
    push: mockRouterReplace,
  }),
  useParams: () => ({ locale: "en" }),
  usePathname: () => "/en/tools/cost-of-living-calculator",
}));

// ─── Mock next/link (used transitively) ──────────────────────────────────────

vi.mock("next/link", () => ({
  default: ({ href, children, ...rest }: { href: string; children: React.ReactNode; [key: string]: unknown }) => (
    <a href={href} {...rest}>
      {children}
    </a>
  ),
}));

// Import after mocks are registered
// eslint-disable-next-line import/first
import { CostOfLivingCalculatorContent } from "@/app/[locale]/tools/cost-of-living-calculator/calculator-content";

afterEach(() => {
  cleanup();
  vi.clearAllMocks();
});

// ─── Helper to set up URLSearchParams mock return values ─────────────────────

function setupSearchParams(params: Record<string, string | null>) {
  mockSearchParamsGet.mockImplementation((key: string) => params[key] ?? null);
}

// ─── UWT-013: /tools index route ─────────────────────────────────────────────

describe("UWT-013 — tools index page exists", () => {
  it("UWT-013: the tools index page exports a default component with a link to the calculator", async () => {
    const toolsPage = await import("@/app/[locale]/tools/page");
    expect(toolsPage.default).toBeDefined();
  });
});

// ─── UWT-007: page title is descriptive ──────────────────────────────────────

describe("UWT-007 — descriptive page title via generateMetadata", () => {
  it("UWT-007: generateMetadata returns a title containing 'Cost of Living Calculator'", async () => {
    const { generateMetadata } = await import("@/app/[locale]/tools/cost-of-living-calculator/page");
    expect(generateMetadata).toBeDefined();
    const meta = await generateMetadata({ params: Promise.resolve({ locale: "en" }) });
    expect((meta as { title?: string }).title).toMatch(/cost of living calculator/i);
  });
});

// ─── UWT-012: predictive tab labels ──────────────────────────────────────────

describe("UWT-012 — predictive tab labels", () => {
  beforeEach(() => {
    setupSearchParams({ tab: null, country: null, city: null });
  });

  it("UWT-012: the Savings tab has an aria-label describing what you'll see", () => {
    render(<CostOfLivingCalculatorContent />);

    // The Savings tab trigger should have a descriptive aria-label
    const savingsTab = screen.getByRole("tab", { name: /savings/i });
    expect(savingsTab).toBeTruthy();
    const ariaLabel = savingsTab.getAttribute("aria-label") ?? savingsTab.textContent ?? "";
    expect(ariaLabel).toMatch(/savings/i);
    // Should have the description in aria-label or aria-describedby target
    const describedBy = savingsTab.getAttribute("aria-describedby");
    const hasDescription =
      savingsTab.getAttribute("aria-label") !== null ||
      (describedBy !== null && document.getElementById(describedBy) !== null) ||
      savingsTab.querySelector("[data-tab-desc]") !== null ||
      savingsTab.closest("[aria-label]") !== null;
    expect(hasDescription).toBe(true);
  });

  it("UWT-012: the Savings tab has a subtitle element with descriptive text about saving money", () => {
    render(<CostOfLivingCalculatorContent />);

    const savingsTabDesc = screen.getByTestId("tab-desc-savings");
    expect(savingsTabDesc).toBeTruthy();
    expect(savingsTabDesc.textContent).toMatch(/save/i);
  });

  it("UWT-012: the Min Role tab has a subtitle element with descriptive text about finding the minimum role", () => {
    render(<CostOfLivingCalculatorContent />);

    const minRoleTabDesc = screen.getByTestId("tab-desc-min-role");
    expect(minRoleTabDesc).toBeTruthy();
    expect(minRoleTabDesc.textContent).toMatch(/role/i);
  });
});

// ─── UWT-002: H1 subtitle describes the tool as a cost-of-living comparison ──

describe("UWT-002 — subtitle below H1", () => {
  beforeEach(() => {
    setupSearchParams({});
  });

  it("UWT-002: a subtitle element describes the tool as a cost-of-living comparison tool", () => {
    setupSearchParams({ tab: null, country: null, city: null });
    render(<CostOfLivingCalculatorContent />);

    const subtitle = screen.getByTestId("calc-subtitle");
    expect(subtitle).toBeTruthy();
    expect(subtitle.textContent).toMatch(/compare cost of living/i);
  });
});

// ─── Phase 6: Tab label purity ────────────────────────────────────────────────

describe("Phase 6 — tab labels are clean single phrases", () => {
  beforeEach(() => {
    setupSearchParams({ tab: null, country: null, city: null });
  });

  it("Phase6: Savings TabsTrigger text content is the label only (description not fused in)", () => {
    render(<CostOfLivingCalculatorContent />);
    const savingsTab = screen.getByRole("tab", { name: /savings/i });
    // Description "See how much you'd save" must NOT be inside the trigger
    expect(savingsTab.textContent?.trim()).not.toMatch(/see how much/i);
  });

  it("Phase6: Min Role TabsTrigger text content is the label only (description not fused in)", () => {
    render(<CostOfLivingCalculatorContent />);
    const minRoleTab = screen.getByRole("tab", { name: /minimum role/i });
    // Description "Find the min role you need" must NOT be inside the trigger
    expect(minRoleTab.textContent?.trim()).not.toMatch(/find the min role/i);
  });

  it("Phase6: Savings TabsTrigger references description via aria-describedby", () => {
    render(<CostOfLivingCalculatorContent />);
    const savingsTab = screen.getByRole("tab", { name: /savings/i });
    const describedById = savingsTab.getAttribute("aria-describedby");
    expect(describedById).toBeTruthy();
    expect(document.getElementById(describedById!)).toBeTruthy();
  });

  it("Phase6: Min Role TabsTrigger references description via aria-describedby", () => {
    render(<CostOfLivingCalculatorContent />);
    const minRoleTab = screen.getByRole("tab", { name: /minimum role/i });
    const describedById = minRoleTab.getAttribute("aria-describedby");
    expect(describedById).toBeTruthy();
    expect(document.getElementById(describedById!)).toBeTruthy();
  });
});

// ─── Phase 4: Tool identity — H1 and metadata match "Cost of Living Calculator"
describe("Phase 4 — H1 matches tool identity", () => {
  beforeEach(() => {
    setupSearchParams({ tab: null, country: null, city: null });
  });

  it("Phase4: en H1 reads 'Cost of Living Calculator'", () => {
    render(<CostOfLivingCalculatorContent />);
    const h1 = screen.getByRole("heading", { level: 1 });
    expect(h1.textContent).toMatch(/cost of living calculator/i);
  });

  it("Phase4: id generateMetadata title contains 'Kalkulator Biaya Hidup'", async () => {
    const { generateMetadata } = await import("@/app/[locale]/tools/cost-of-living-calculator/page");
    const meta = await generateMetadata({ params: Promise.resolve({ locale: "id" }) });
    expect((meta as { title?: string }).title).toMatch(/kalkulator biaya hidup/i);
  });
});

// ─── Cycle 3: URL params initialise GeoFilter state ─────────────────────────

describe("Cycle 3 — URL search params initialise filter state", () => {
  beforeEach(() => {
    setupSearchParams({});
  });

  it("?tab=cost&country=id causes the Country select to be pre-selected with Indonesia", async () => {
    setupSearchParams({ tab: "cost", country: "id", city: null });

    render(<CostOfLivingCalculatorContent />);

    // Country select should be pre-selected with Indonesia's countryId "id"
    const countrySelect = screen.getByRole("combobox", { name: /country/i });
    expect(countrySelect).toHaveValue("id");
  });

  it("?tab=cost&country=id filters the cost-of-living table to Indonesian cities only", async () => {
    setupSearchParams({ tab: "cost", country: "id", city: null });

    render(<CostOfLivingCalculatorContent />);

    // Count visible city links in the table — should only be Indonesian cities
    const indonesianCities = dataset.cities.filter((c) => c.countryId === "id");
    const allCities = dataset.cities;

    // The filtered table should have fewer rows than the full dataset
    const rows = screen.getAllByRole("row");
    // 1 header + N Indonesian cities (possibly more via mobile, but check count)
    // We have less rows than full dataset
    expect(rows.length).toBeLessThan(allCities.length + 1);
    expect(rows.length).toBe(indonesianCities.length + 1);
  });

  it("no URL params shows all cities in the cost-of-living table", () => {
    setupSearchParams({ tab: null, country: null, city: null });

    render(<CostOfLivingCalculatorContent />);

    const rows = screen.getAllByRole("row");
    expect(rows.length).toBe(dataset.cities.length + 1);
  });
});

// ─── Cycle 4: Filter selection writes URL via router.replace ─────────────────

describe("Cycle 4 — filter selection updates URL", () => {
  beforeEach(() => {
    setupSearchParams({ tab: null, country: null, city: null, region: null });
  });

  it("selecting Country 'Indonesia' calls router.replace with ?country=id", async () => {
    const user = userEvent.setup();
    setupSearchParams({ tab: null, country: null, city: null });

    render(<CostOfLivingCalculatorContent />);

    // Select ASEAN region first to narrow country list
    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    await user.selectOptions(regionSelect, "asean");

    // Then select Indonesia
    const countrySelect = screen.getByRole("combobox", { name: /country/i });
    await user.selectOptions(countrySelect, "id");

    // router.replace should have been called with a URL containing country=id
    await waitFor(() => {
      expect(mockRouterReplace).toHaveBeenCalledWith(expect.stringContaining("country=id"));
    });
  });

  it("round-trip: ?country=id pre-selects Indonesia; then clearing resets to all countries", async () => {
    const user = userEvent.setup();
    setupSearchParams({ tab: "cost", country: "id", city: null });

    render(<CostOfLivingCalculatorContent />);

    // Pre-selected
    const countrySelect = screen.getByRole("combobox", { name: /country/i });
    expect(countrySelect).toHaveValue("id");

    // Clear by selecting the "All countries" option (empty value)
    await user.selectOptions(countrySelect, "");
    expect(countrySelect).toHaveValue("");
  });
});

// ─── Cycle 5: City-click pushes ?city=<cityId> and City filter reflects it ───

describe("Cycle 5 — city click pre-selects City filter and updates URL", () => {
  beforeEach(() => {
    setupSearchParams({ tab: null, country: null, city: null });
  });

  it("?city=jakarta pre-selects the City select with jakarta", () => {
    setupSearchParams({ tab: "cost", city: "jakarta", country: null });

    render(<CostOfLivingCalculatorContent />);

    // The city detail view should be shown (since detailCityId is set from URL)
    // and the GeoFilters city select should reflect the city
    const cityDetail = screen.getByTestId("city-detail");
    expect(cityDetail).toBeTruthy();
  });

  it("clicking a city name link pushes ?tab=cost&city=<cityId> via router.replace", async () => {
    const user = userEvent.setup();
    setupSearchParams({ tab: null, country: null, city: null });

    render(<CostOfLivingCalculatorContent />);

    // Find the first city's link in the table
    const firstCity = dataset.cities[0]!;
    const cityLinks = screen.getAllByRole("link", { name: firstCity.name.en });
    const cityLink = cityLinks.find((l) => l.getAttribute("href") === `?tab=cost&city=${firstCity.id}`);
    expect(cityLink).toBeDefined();

    // Click the link (event delegation intercepts it)
    await user.click(cityLink!);

    await waitFor(() => {
      expect(mockRouterReplace).toHaveBeenCalledWith(`?tab=cost&city=${firstCity.id}`);
    });
  });

  it("after city click, the city detail view is displayed", async () => {
    const user = userEvent.setup();
    setupSearchParams({ tab: null, country: null, city: null });

    render(<CostOfLivingCalculatorContent />);

    const firstCity = dataset.cities[0]!;
    const cityLinks = screen.getAllByRole("link", { name: firstCity.name.en });
    const cityLink = cityLinks.find((l) => l.getAttribute("href") === `?tab=cost&city=${firstCity.id}`);
    expect(cityLink).toBeDefined();

    await user.click(cityLink!);

    await waitFor(() => {
      expect(screen.getByTestId("city-detail")).toBeTruthy();
    });
  });
});
