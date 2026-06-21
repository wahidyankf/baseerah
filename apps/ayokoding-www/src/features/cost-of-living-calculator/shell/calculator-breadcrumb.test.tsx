import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

// Mock useLocale to return "en" for all tests
vi.mock("@/features/i18n/shell/use-locale", () => ({
  useLocale: () => "en",
}));

// Mock next/link as a simple <a> tag
vi.mock("next/link", () => ({
  default: ({ href, children, ...rest }: { href: string; children: React.ReactNode; [key: string]: unknown }) => (
    <a href={href} {...rest}>
      {children}
    </a>
  ),
}));

// Import after mocks are registered
// eslint-disable-next-line import/first
import { CalculatorBreadcrumb } from "./calculator-breadcrumb";

afterEach(cleanup);

// Gherkin (binds): "Breadcrumb nav provides Home / Tools / Calculator escape affordance"
describe("CalculatorBreadcrumb", () => {
  it("3a: renders a <nav> with aria-label='Breadcrumb'", () => {
    render(<CalculatorBreadcrumb />);
    const nav = screen.getByRole("navigation", { name: /breadcrumb/i });
    expect(nav).toBeTruthy();
  });

  it("3a: renders a 'Home' link pointing to /en when locale is en", () => {
    render(<CalculatorBreadcrumb />);
    const homeLink = screen.getByRole("link", { name: /home/i });
    expect(homeLink.getAttribute("href")).toBe("/en");
  });

  it("3a: renders a 'Tools' link pointing to /en/tools when locale is en", () => {
    render(<CalculatorBreadcrumb />);
    const toolsLink = screen.getByRole("link", { name: /tools/i });
    expect(toolsLink.getAttribute("href")).toBe("/en/tools");
  });

  it("3a: renders 'Calculator' as the current page item with aria-current='page'", () => {
    render(<CalculatorBreadcrumb />);
    const current = screen.getByText(/calculator/i);
    expect(current.getAttribute("aria-current")).toBe("page");
  });
});
