import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { ScrollArea, ScrollBar } from "./scroll-area";
import { Separator } from "../separator/separator";

const meta: Meta<typeof ScrollArea> = {
  title: "Primitives/ScrollArea",
  component: ScrollArea,
  tags: ["autodocs"],
};

export default meta;

type Story = StoryObj<typeof ScrollArea>;

const tags = [
  "v1.2.0",
  "v1.1.0",
  "v1.0.0",
  "v0.9.0",
  "v0.8.0",
  "v0.7.0",
  "v0.6.0",
  "v0.5.0",
  "v0.4.0",
  "v0.3.0",
  "v0.2.0",
  "v0.1.0",
];

export const Default: Story = {
  render: () => (
    <ScrollArea className="h-72 w-48 rounded-md border">
      <div className="p-4">
        <h4 className="mb-4 text-sm leading-none font-medium">Tags</h4>
        {tags.map((tag) => (
          <div key={tag}>
            <div className="text-sm">{tag}</div>
            <Separator className="my-2" />
          </div>
        ))}
      </div>
    </ScrollArea>
  ),
};

export const Horizontal: Story = {
  render: () => (
    <ScrollArea className="w-96 rounded-md border whitespace-nowrap">
      <div className="flex w-max space-x-4 p-4">
        {Array.from({ length: 20 }, (_, i) => (
          <div key={i} className="w-32 shrink-0 rounded-md border p-4 text-sm">
            Item {i + 1}
          </div>
        ))}
      </div>
      <ScrollBar orientation="horizontal" />
    </ScrollArea>
  ),
};
