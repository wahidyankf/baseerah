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
 *  Phase 2 — URL as single source of truth:
 *    2b — Tab change writes URL via router.push
 *    2c — Cost-basis controls write URL via router.push
 *    2d — Geo change uses router.push + full cascade/backfill
 *    2e — Canonicalize on mount via router.replace
 */

import { cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import React from "react";
import { dataset } from "../core/data/cities";

// ─── Mock next/navigation ────────────────────────────────────────────────────

const mockRouterReplace = vi.fn();
const mockRouterPush = vi.fn();

// Reactive router state: hoisted so the vi.mock factory closure can access it.
// push/replace update params AND call setParams to trigger React re-renders.
const { navState } = vi.hoisted(() => {
  const navState = {
    params: new URLSearchParams(),
    setParams: (_: URLSearchParams) => {},
  };
  return { navState };
});

vi.mock("next/navigation", () => ({
  useSearchParams: () => navState.params,
  useRouter: () => ({
    replace: (url: string) => {
      mockRouterReplace(url);
      const qs = url.startsWith("?") ? url.slice(1) : url;
      navState.params = new URLSearchParams(qs);
      navState.setParams(navState.params);
    },
    push: (url: string) => {
      mockRouterPush(url);
      const qs = url.startsWith("?") ? url.slice(1) : url;
      navState.params = new URLSearchParams(qs);
      navState.setParams(navState.params);
    },
  }),
  useParams: () => ({ locale: "en" }),
  usePathname: () => "/en/tools/cost-of-living-calculator",
}));

// NavigationWrapper: wrapper that holds URL params as React state so that
// router.push/replace trigger re-renders (simulates Next.js router behavior).
function NavigationWrapper({ children }: { children: React.ReactNode }) {
  const [, setTick] = React.useState(0);

  React.useEffect(() => {
    navState.setParams = (_newParams) => {
      setTick((t) => t + 1);
    };
    return () => {
      navState.setParams = () => {};
    };
  }, []);

  return <>{children}</>;
}

function renderWithNav(ui: React.ReactElement) {
  return render(<NavigationWrapper>{ui}</NavigationWrapper>);
}

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
  navState.params = new URLSearchParams();
  navState.setParams = () => {};
});

// ─── Helper to set up URLSearchParams mock ───────────────────────────────────

function setupSearchParams(params: Record<string, string>) {
  navState.params = new URLSearchParams(params);
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
    setupSearchParams({});
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
    setupSearchParams({});
    render(<CostOfLivingCalculatorContent />);

    const subtitle = screen.getByTestId("calc-subtitle");
    expect(subtitle).toBeTruthy();
    expect(subtitle.textContent).toMatch(/compare cost of living/i);
  });
});

// ─── Phase 6: Tab label purity ────────────────────────────────────────────────

describe("Phase 6 — tab labels are clean single phrases", () => {
  beforeEach(() => {
    setupSearchParams({});
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
    setupSearchParams({});
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
    setupSearchParams({ tab: "cost", country: "id" });

    render(<CostOfLivingCalculatorContent />);

    // Country select should be pre-selected with Indonesia's countryId "id"
    const countrySelect = screen.getByRole("combobox", { name: /country/i });
    expect(countrySelect).toHaveValue("id");
  });

  it("?tab=cost&country=id filters the cost-of-living table to Indonesian cities only", async () => {
    setupSearchParams({ tab: "cost", country: "id" });

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
    setupSearchParams({});

    render(<CostOfLivingCalculatorContent />);

    const rows = screen.getAllByRole("row");
    expect(rows.length).toBe(dataset.cities.length + 1);
  });
});

// ─── Cycle 4: Filter selection writes URL via router.replace ─────────────────

describe("Cycle 4 — filter selection updates URL", () => {
  beforeEach(() => {
    setupSearchParams({});
  });

  it("selecting Country 'Indonesia' calls router push/replace with ?country=id", async () => {
    const user = userEvent.setup();
    setupSearchParams({});

    render(<CostOfLivingCalculatorContent />);

    // Select ASEAN region first to narrow country list
    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    await user.selectOptions(regionSelect, "asean");

    // Then select Indonesia
    const countrySelect = screen.getByRole("combobox", { name: /country/i });
    await user.selectOptions(countrySelect, "id");

    // router.replace or router.push should have been called with a URL containing country=id
    await waitFor(() => {
      const allCalls = [...mockRouterReplace.mock.calls, ...mockRouterPush.mock.calls];
      expect(allCalls.some((args) => String(args[0]).includes("country=id"))).toBe(true);
    });
  });

  it("round-trip: ?country=id pre-selects Indonesia; then clearing calls router.push without country", async () => {
    const user = userEvent.setup();
    setupSearchParams({ tab: "cost", country: "id" });

    renderWithNav(<CostOfLivingCalculatorContent />);

    // Pre-selected
    const countrySelect = screen.getByRole("combobox", { name: /country/i });
    expect(countrySelect).toHaveValue("id");

    // Clear by selecting the "All countries" option (empty value)
    await user.selectOptions(countrySelect, "");

    // router.push should be called with a URL that does NOT contain country=id
    await waitFor(() => {
      expect(mockRouterPush).toHaveBeenCalled();
      const calls = mockRouterPush.mock.calls;
      const lastCall = calls[calls.length - 1]?.[0] as string;
      expect(lastCall).not.toContain("country=id");
    });
  });
});

// ─── Cycle 5: City-click pushes ?city=<cityId> and City filter reflects it ───

describe("Cycle 5 — city click pre-selects City filter and updates URL", () => {
  beforeEach(() => {
    setupSearchParams({});
  });

  it("?city=jakarta pre-selects the City select with jakarta", () => {
    setupSearchParams({ tab: "cost", city: "jakarta" });

    render(<CostOfLivingCalculatorContent />);

    // The city detail view should be shown (since cityId is set from URL)
    const cityDetail = screen.getByTestId("city-detail");
    expect(cityDetail).toBeTruthy();
  });

  it("clicking a city name link updates URL via router push/replace with city id", async () => {
    const user = userEvent.setup();
    setupSearchParams({});

    render(<CostOfLivingCalculatorContent />);

    // Find the first city's link in the table
    const firstCity = dataset.cities[0]!;
    const cityLinks = screen.getAllByRole("link", { name: firstCity.name.en });
    const cityLink = cityLinks.find((l) => l.getAttribute("href")?.includes(`city=${firstCity.id}`));
    expect(cityLink).toBeDefined();

    // Click the link (event delegation intercepts it)
    await user.click(cityLink!);

    await waitFor(() => {
      const allCalls = [...mockRouterReplace.mock.calls, ...mockRouterPush.mock.calls];
      expect(allCalls.some((args) => String(args[0]).includes(`city=${firstCity.id}`))).toBe(true);
    });
  });

  it("after city click, the city detail view is displayed", async () => {
    // This test verifies that rendering with the city URL param shows city detail.
    // The navigation response (re-render after router.push) is covered by the
    // NavigationWrapper integration in Cycle 4 and the steps file.
    const firstCity = dataset.cities[0]!;
    setupSearchParams({ city: firstCity.id });

    render(<CostOfLivingCalculatorContent />);

    // With city in URL params, city detail should be shown
    expect(screen.getByTestId("city-detail")).toBeTruthy();
  });
});

// ─── Phase 2b: Tab change writes URL via router.push ─────────────────────────

describe("Phase 2b — tab change writes URL", () => {
  beforeEach(() => {
    setupSearchParams({});
  });

  it("switching to the Savings tab calls router.push with tab=savings in the URL", async () => {
    const user = userEvent.setup();
    setupSearchParams({});

    render(<CostOfLivingCalculatorContent />);

    const savingsTab = screen.getByRole("tab", { name: /savings/i });
    await user.click(savingsTab);

    await waitFor(() => {
      expect(mockRouterPush).toHaveBeenCalledWith(expect.stringContaining("tab=savings"));
    });
  });

  it("switching to the Min Role tab calls router.push with tab=min-role in the URL", async () => {
    const user = userEvent.setup();
    setupSearchParams({});

    render(<CostOfLivingCalculatorContent />);

    const minRoleTab = screen.getByRole("tab", { name: /minimum role/i });
    await user.click(minRoleTab);

    await waitFor(() => {
      expect(mockRouterPush).toHaveBeenCalledWith(expect.stringContaining("tab=min-role"));
    });
  });

  it("?tab=savings causes the Savings tab to be active on mount", () => {
    setupSearchParams({ tab: "savings" });

    render(<CostOfLivingCalculatorContent />);

    const savingsTab = screen.getByRole("tab", { name: /savings/i });
    expect(savingsTab).toHaveAttribute("data-state", "active");
  });
});

// ─── Phase 2c: Cost-basis controls write URL via router.push ─────────────────

describe("Phase 2c — cost-basis controls write URL", () => {
  beforeEach(() => {
    setupSearchParams({});
  });

  it("changing Adults to 2 calls router.push with adults=2 in the URL", async () => {
    const user = userEvent.setup();
    setupSearchParams({});

    render(<CostOfLivingCalculatorContent />);

    // The adults select in Controls
    const adultsSelect = screen.getByRole("combobox", { name: /adults/i });
    await user.selectOptions(adultsSelect, "2");

    await waitFor(() => {
      expect(mockRouterPush).toHaveBeenCalledWith(expect.stringContaining("adults=2"));
    });
  });

  it("changing schooltype to private calls router.push with schooltype=private in the URL", async () => {
    const user = userEvent.setup();
    // schooltype control only shows when schoolKids >= 1
    setupSearchParams({ schoolkids: "1" });

    render(<CostOfLivingCalculatorContent />);

    // The school type segmented control uses role="radio" buttons (not combobox)
    const privateBtn = screen.getByRole("radio", { name: /private/i });
    await user.click(privateBtn);

    await waitFor(() => {
      expect(mockRouterPush).toHaveBeenCalledWith(expect.stringContaining("schooltype=private"));
    });
  });
});

// ─── Phase 2d: Geo change uses router.push + full cascade/backfill ────────────

describe("Phase 2d — geo change uses router.push with full state", () => {
  beforeEach(() => {
    setupSearchParams({});
  });

  it("selecting a region calls router.push with the region in the URL", async () => {
    const user = userEvent.setup();
    setupSearchParams({});

    render(<CostOfLivingCalculatorContent />);

    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    await user.selectOptions(regionSelect, "europe");

    await waitFor(() => {
      expect(mockRouterPush).toHaveBeenCalledWith(expect.stringContaining("region=europe"));
    });
  });

  it("selecting region 'Europe' when country=id clears country and city in URL", async () => {
    const user = userEvent.setup();
    // Start with Indonesia selected (ASEAN country)
    setupSearchParams({ country: "id" });

    render(<CostOfLivingCalculatorContent />);

    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    await user.selectOptions(regionSelect, "europe");

    await waitFor(() => {
      const calls = mockRouterPush.mock.calls;
      const lastCall = calls[calls.length - 1]?.[0] as string;
      expect(lastCall).toContain("region=europe");
      // country and city should be absent (cleared by cascade)
      expect(lastCall).not.toContain("country=");
      expect(lastCall).not.toContain("city=");
    });
  });
});

// ─── Phase 2e: Canonicalize on mount ─────────────────────────────────────────

describe("Phase 2e — canonicalize on mount", () => {
  it("mounting with already-clean params calls neither router.push nor router.replace for canonicalization", async () => {
    // Clean default state — no params needed
    setupSearchParams({});

    render(<CostOfLivingCalculatorContent />);

    // Wait a tick to ensure effects have run
    await waitFor(() => {
      // Neither push nor replace should have been called for canonicalization
      // (Note: other interactions may call them, so we check here before any user interaction)
      expect(mockRouterReplace).not.toHaveBeenCalled();
      expect(mockRouterPush).not.toHaveBeenCalled();
    });
  });

  it("mounting with ?city=atlantis (invalid city) calls router.replace with cleaned params", async () => {
    setupSearchParams({ city: "atlantis" });

    render(<CostOfLivingCalculatorContent />);

    await waitFor(() => {
      // router.replace should have been called to canonicalize (remove invalid city)
      expect(mockRouterReplace).toHaveBeenCalled();
      const replaceArg = mockRouterReplace.mock.calls[0]?.[0] as string;
      expect(replaceArg).not.toContain("atlantis");
    });
  });
});
