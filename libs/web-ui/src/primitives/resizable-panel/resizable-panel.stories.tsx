import type { Meta, StoryObj } from "@storybook/react-vite";

import { ResizablePanel } from "./resizable-panel";

const meta: Meta<typeof ResizablePanel> = {
  title: "Primitives/ResizablePanel",
  component: ResizablePanel,
  tags: ["autodocs"],
};

export default meta;

type Story = StoryObj<typeof ResizablePanel>;

export const Default: Story = {
  render: () => (
    <div className="border-border flex h-64 border">
      <ResizablePanel storageKey="storybook-resizable-panel-default" defaultWidth={250} viewportPx={1000}>
        <div className="h-full p-4">
          <h4 className="mb-2 text-sm font-medium">Navigation</h4>
          <ul className="text-muted-foreground space-y-1 text-sm">
            <li>Getting started</li>
            <li>Guides</li>
            <li>API reference</li>
          </ul>
        </div>
      </ResizablePanel>
      <div className="flex-1 p-4">
        <p className="text-muted-foreground text-sm">
          Drag the handle on the right edge of the panel, or focus it and press ArrowLeft / ArrowRight.
        </p>
      </div>
    </div>
  ),
};

export const NarrowContentOverflow: Story = {
  name: "Narrow Content (Overflow)",
  render: () => (
    <div className="border-border flex h-64 border">
      <ResizablePanel storageKey="storybook-resizable-panel-overflow" defaultWidth={150} viewportPx={1000}>
        <div className="h-full overflow-x-auto p-4">
          <h4 className="mb-2 min-w-max text-sm font-medium">Navigation</h4>
          <ul className="text-muted-foreground min-w-max space-y-1 text-sm">
            <li>A very long label that overflows the narrow panel width</li>
            <li>Another overflowing label to demonstrate horizontal scroll</li>
          </ul>
        </div>
      </ResizablePanel>
      <div className="flex-1 p-4">
        <p className="text-muted-foreground text-sm">
          The panel starts at its minimum width; long labels scroll horizontally instead of clipping.
        </p>
      </div>
    </div>
  ),
};
