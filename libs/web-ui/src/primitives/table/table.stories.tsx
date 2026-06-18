import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableCaption } from "./table";

const meta: Meta<typeof Table> = {
  title: "Primitives/Table",
  component: Table,
  tags: ["autodocs"],
};

export default meta;

type Story = StoryObj<typeof Table>;

export const Default: Story = {
  render: () => (
    <Table>
      <TableCaption>Software-engineering roles (IC + management)</TableCaption>
      <TableHeader>
        <TableRow>
          <TableHead>Country</TableHead>
          <TableHead>City</TableHead>
          <TableHead className="text-right">Essential savings (USD)</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow>
          <TableCell>Singapore</TableCell>
          <TableCell>Singapore</TableCell>
          <TableCell className="text-right">$4,200</TableCell>
        </TableRow>
        <TableRow>
          <TableCell>Japan</TableCell>
          <TableCell>Tokyo</TableCell>
          <TableCell className="text-right">$3,100</TableCell>
        </TableRow>
      </TableBody>
    </Table>
  ),
};

export const Empty: Story = {
  render: () => (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Country</TableHead>
          <TableHead>City</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow>
          <TableCell colSpan={2} className="text-center text-muted-foreground">
            No cities match the selected filters.
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  ),
};
