import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { Card, CardHeader, CardTitle, CardContent } from "./card";

describe("Card primitive", () => {
  it("mounts in the DOM", () => {
    render(
      <Card>
        <CardHeader>
          <CardTitle>Card Title</CardTitle>
        </CardHeader>
        <CardContent>Card body content</CardContent>
      </Card>,
    );
    expect(screen.getByText("Card Title")).toBeTruthy();
  });
});
