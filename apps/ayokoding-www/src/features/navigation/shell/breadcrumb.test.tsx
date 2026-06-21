import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

// Mock next/link as a simple <a> tag.
vi.mock("next/link", () => ({
  default: ({ href, children, ...rest }: { href: string; children: React.ReactNode; [key: string]: unknown }) => (
    <a href={href} {...rest}>
      {children}
    </a>
  ),
}));

// Import after mocks are registered.
// eslint-disable-next-line import/first
import { Breadcrumb } from "./breadcrumb";

afterEach(cleanup);

const segments = [
  { label: "Home", slug: "" },
  { label: "Tools", slug: "tools" },
  { label: "Cost of Living Calculator", slug: "tools/cost-of-living-calculator" },
];

describe("Breadcrumb", () => {
  it("by default excludes the last (current-page) segment", () => {
    render(<Breadcrumb locale="en" slug="tools/cost-of-living-calculator" segments={segments} />);
    // Final segment is not rendered when showCurrent is absent (legacy behaviour).
    expect(screen.queryByText("Cost of Living Calculator")).toBeNull();
    expect(screen.getByRole("link", { name: "Home" })).toBeTruthy();
    expect(screen.getByRole("link", { name: "Tools" })).toBeTruthy();
  });

  it("with showCurrent renders the final segment as a non-link aria-current='page' crumb", () => {
    render(<Breadcrumb locale="en" slug="tools/cost-of-living-calculator" segments={segments} showCurrent />);
    const current = screen.getByText("Cost of Living Calculator");
    expect(current.getAttribute("aria-current")).toBe("page");
    // The current crumb must not be a link.
    expect(current.closest("a")).toBeNull();
    // Ancestor segments remain links.
    expect(screen.getByRole("link", { name: "Home" }).getAttribute("href")).toBe("/en");
    expect(screen.getByRole("link", { name: "Tools" }).getAttribute("href")).toBe("/en/tools");
  });

  it("with showCurrent uses ChevronRight separators, never a literal '/'", () => {
    const { container } = render(
      <Breadcrumb locale="en" slug="tools/cost-of-living-calculator" segments={segments} showCurrent />,
    );
    // lucide-react ChevronRight renders an <svg>; one separator between each of 3 crumbs.
    expect(container.querySelectorAll("svg").length).toBe(2);
    expect(container.textContent).not.toContain("/");
  });
});

const contentSegments = [
  { label: "Learn", slug: "learn" },
  { label: "Software Engineering", slug: "learn/software-engineering" },
  { label: "Data Structures", slug: "learn/software-engineering/data-structures" },
];

describe("Breadcrumb with contentHrefs", () => {
  it("when contentHrefs=true emits /c/ prefixed hrefs for content ancestor segments", () => {
    render(
      <Breadcrumb
        locale="en"
        slug="learn/software-engineering/data-structures"
        segments={contentSegments}
        contentHrefs
        showCurrent
      />,
    );
    // Ancestor links point into the /c/ namespace.
    expect(screen.getByRole("link", { name: "Learn" }).getAttribute("href")).toBe("/en/c/learn");
    expect(screen.getByRole("link", { name: "Software Engineering" }).getAttribute("href")).toBe(
      "/en/c/learn/software-engineering",
    );
    // Final segment is non-link aria-current.
    const current = screen.getByText("Data Structures");
    expect(current.getAttribute("aria-current")).toBe("page");
    expect(current.closest("a")).toBeNull();
  });

  it("when contentHrefs is absent still emits bare hrefs (backward compat)", () => {
    render(<Breadcrumb locale="en" slug="tools" segments={segments} showCurrent />);
    expect(screen.getByRole("link", { name: "Tools" }).getAttribute("href")).toBe("/en/tools");
  });
});
