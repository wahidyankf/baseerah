import { render } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { ScrollArea } from "./scroll-area";

describe("ScrollArea primitive", () => {
  it("mounts in the DOM", () => {
    const { container } = render(<ScrollArea>Scrollable content</ScrollArea>);
    expect(container.querySelector("[data-slot='scroll-area']")).toBeTruthy();
  });
});
