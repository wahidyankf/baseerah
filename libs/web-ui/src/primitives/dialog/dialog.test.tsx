import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { Dialog, DialogTrigger, DialogContent, DialogTitle } from "./dialog";

describe("Dialog primitive", () => {
  it("mounts trigger in the DOM", () => {
    render(
      <Dialog>
        <DialogTrigger>Open Dialog</DialogTrigger>
        <DialogContent>
          <DialogTitle>Dialog Title</DialogTitle>
        </DialogContent>
      </Dialog>,
    );
    expect(screen.getByText("Open Dialog")).toBeTruthy();
  });
});
