import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { SkipLink } from "./skip-link";

afterEach(cleanup);

// Gherkin (binds): "Skip-to-content link is translated in the ID locale"
describe("SkipLink", () => {
  it("renders Indonesian skip-to-content text for locale=id", () => {
    render(<SkipLink locale="id" />);
    expect(screen.getByRole("link", { name: "Langsung ke konten" })).toBeTruthy();
  });

  it("renders English skip-to-content text for locale=en", () => {
    render(<SkipLink locale="en" />);
    expect(screen.getByRole("link", { name: "Skip to content" })).toBeTruthy();
  });
});
