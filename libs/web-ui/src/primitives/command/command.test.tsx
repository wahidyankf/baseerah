import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { Command, CommandInput, CommandList, CommandItem } from "./command";

describe("Command primitive", () => {
  it("mounts in the DOM", () => {
    render(
      <Command>
        <CommandInput placeholder="Search..." />
        <CommandList>
          <CommandItem>Option 1</CommandItem>
        </CommandList>
      </Command>,
    );
    expect(screen.getByPlaceholderText("Search...")).toBeTruthy();
  });
});
