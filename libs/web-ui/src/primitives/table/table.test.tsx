import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableCaption } from "./table";

describe("Table primitive", () => {
  it("renders a <table> element with role table", () => {
    render(
      <Table>
        <TableBody>
          <TableRow>
            <TableCell>data</TableCell>
          </TableRow>
        </TableBody>
      </Table>,
    );
    expect(screen.getByRole("table")).toBeTruthy();
  });

  it("renders TableHeader as <thead>", () => {
    const { container } = render(
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Col</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow>
            <TableCell>data</TableCell>
          </TableRow>
        </TableBody>
      </Table>,
    );
    expect(container.querySelector("thead")).toBeTruthy();
  });

  it("renders TableBody as <tbody>", () => {
    const { container } = render(
      <Table>
        <TableBody>
          <TableRow>
            <TableCell>data</TableCell>
          </TableRow>
        </TableBody>
      </Table>,
    );
    expect(container.querySelector("tbody")).toBeTruthy();
  });

  it("renders TableRow as <tr>", () => {
    const { container } = render(
      <Table>
        <TableBody>
          <TableRow>
            <TableCell>data</TableCell>
          </TableRow>
        </TableBody>
      </Table>,
    );
    expect(container.querySelector("tr")).toBeTruthy();
  });

  it("renders TableHead as <th> with columnheader role", () => {
    render(
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>City</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow>
            <TableCell>data</TableCell>
          </TableRow>
        </TableBody>
      </Table>,
    );
    expect(screen.getByRole("columnheader", { name: "City" })).toBeTruthy();
  });

  it("renders TableCell as <td> with cell role", () => {
    render(
      <Table>
        <TableBody>
          <TableRow>
            <TableCell>Singapore</TableCell>
          </TableRow>
        </TableBody>
      </Table>,
    );
    expect(screen.getByRole("cell", { name: "Singapore" })).toBeTruthy();
  });

  it("renders TableCaption as <caption>", () => {
    render(
      <Table>
        <TableCaption>Software-engineering roles</TableCaption>
        <TableBody>
          <TableRow>
            <TableCell>data</TableCell>
          </TableRow>
        </TableBody>
      </Table>,
    );
    expect(screen.getByText("Software-engineering roles")).toBeTruthy();
  });

  it("accepts className overrides on each sub-component", () => {
    const { container } = render(
      <Table className="custom-table">
        <TableHeader className="custom-header">
          <TableRow className="custom-row">
            <TableHead className="custom-head">H</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody className="custom-body">
          <TableRow>
            <TableCell className="custom-cell">D</TableCell>
          </TableRow>
        </TableBody>
      </Table>,
    );
    expect(container.querySelector(".custom-table")).toBeTruthy();
    expect(container.querySelector(".custom-header")).toBeTruthy();
    expect(container.querySelector(".custom-head")).toBeTruthy();
    expect(container.querySelector(".custom-body")).toBeTruthy();
    expect(container.querySelector(".custom-cell")).toBeTruthy();
  });
});
