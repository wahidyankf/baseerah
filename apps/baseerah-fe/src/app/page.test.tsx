import { render, screen } from "@testing-library/react";
import { describe, expect, test, vi } from "vitest";

vi.mock("@/lib/greeting-client", () => ({
  fetchGreeting: vi.fn().mockResolvedValue("Hello from Baseerah"),
}));

describe("Backend hello world", () => {
  test("The landing page names the product and shows the backend greeting", async () => {
    const { default: Page } = await import("./page");
    render(await Page());

    expect(screen.getByRole("heading", { level: 1 })).toHaveTextContent("Baseerah");
    expect(screen.getByText("Hello from Baseerah")).toBeInTheDocument();
  });

  test("The landing page meets the baseline accessibility bar", async () => {
    const { default: Page } = await import("./page");
    const { container } = render(await Page());

    expect(container.querySelectorAll("h1")).toHaveLength(1);
    expect(container.querySelector("header")).not.toBeNull();
    expect(container.querySelector("main")).not.toBeNull();
    expect(container.querySelector("footer")).not.toBeNull();

    const arabic = screen.getByText("بصيرة");
    expect(arabic).toHaveAttribute("lang", "ar");
    expect(arabic).toHaveAttribute("dir", "rtl");
  });

  test("tells a first-time visitor what Baseerah is, glosses the multilingual chip, and offers a way to learn more", async () => {
    const { default: Page } = await import("./page");
    render(await Page());

    expect(screen.getByText(/Baseerah is a personal operating layer/i)).toBeInTheDocument();

    const chip = screen.getByText("بصيرة").closest("div");
    expect(chip).toHaveAttribute("title", "insight (English) · wawasan (Indonesian) · بصيرة (Arabic)");

    const githubLink = screen.getByRole("link", { name: /view on github/i });
    expect(githubLink).toHaveAttribute("href", "https://github.com/wahidyankf/baseerah");
  });
});
