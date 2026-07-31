import { render, screen } from "@testing-library/react";
import { describe, expect, test } from "vitest";
import NotFound from "./not-found";

describe("Branded 404 page", () => {
  test("reuses the app's header/footer landmarks and offers a way back home", () => {
    const { container } = render(<NotFound />);

    expect(container.querySelectorAll("h1")).toHaveLength(1);
    expect(screen.getByRole("heading", { level: 1 })).toHaveTextContent("Baseerah");
    expect(container.querySelector("header")).not.toBeNull();
    expect(container.querySelector("main")).not.toBeNull();
    expect(container.querySelector("footer")).not.toBeNull();

    const homeLink = screen.getByRole("link", { name: /back to home/i });
    expect(homeLink).toHaveAttribute("href", "/");
  });
});
