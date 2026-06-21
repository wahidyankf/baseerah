import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("next/link", () => ({
  default: ({ href, children, ...rest }: { href: string; children: React.ReactNode; [key: string]: unknown }) => (
    <a href={href} {...rest}>
      {children}
    </a>
  ),
}));

// eslint-disable-next-line import/first
import { SectionCard } from "./section-card";

afterEach(cleanup);

describe("SectionCard", () => {
  it("renders the section title as a link to its content URL", () => {
    render(<SectionCard href="/en/c/learn" title="Software Engineering" description="Languages, architecture." />);
    const link = screen.getByRole("link", { name: /Software Engineering/ });
    expect(link.getAttribute("href")).toBe("/en/c/learn");
  });

  it("renders the description blurb", () => {
    render(<SectionCard href="/en/c/learn" title="Software Engineering" description="Languages, architecture." />);
    expect(screen.getByText("Languages, architecture.")).toBeTruthy();
  });

  it("reuses the shared Card token surface (rounded border)", () => {
    const { container } = render(<SectionCard href="/en/c/learn" title="Learn" description="x" />);
    // Card primitive applies the rounded-xl border bg-card token surface.
    expect(container.querySelector('[data-slot="card"]')).not.toBeNull();
  });
});
