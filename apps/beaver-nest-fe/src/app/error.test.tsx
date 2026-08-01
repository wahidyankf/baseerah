import { render, screen } from "@testing-library/react";
import { describe, expect, test, vi } from "vitest";
import ErrorBoundary from "./error";

describe("Branded error boundary", () => {
  test("reuses the app's header/footer landmarks and offers a retry action", () => {
    const reset = vi.fn();
    const { container } = render(<ErrorBoundary error={new Error("boom")} reset={reset} />);

    expect(container.querySelectorAll("h1")).toHaveLength(1);
    expect(screen.getByRole("heading", { level: 1 })).toHaveTextContent("BeaverNest");
    expect(container.querySelector("header")).not.toBeNull();
    expect(container.querySelector("main")).not.toBeNull();
    expect(container.querySelector("footer")).not.toBeNull();

    const retryButton = screen.getByRole("button", { name: /try again/i });
    retryButton.click();
    expect(reset).toHaveBeenCalledOnce();
  });
});
