import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { Sheet, SheetTrigger, SheetContent, SheetHeader, SheetTitle } from "./sheet";

describe("Sheet primitive", () => {
  it("mounts trigger in the DOM", () => {
    render(
      <Sheet>
        <SheetTrigger>Open</SheetTrigger>
        <SheetContent>
          <SheetHeader>
            <SheetTitle>Sheet Title</SheetTitle>
          </SheetHeader>
        </SheetContent>
      </Sheet>,
    );
    expect(screen.getByText("Open")).toBeTruthy();
  });
});
