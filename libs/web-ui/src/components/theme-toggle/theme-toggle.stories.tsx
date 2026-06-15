import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import ThemeToggle from "./theme-toggle";

const meta: Meta<typeof ThemeToggle> = {
  title: "Composites/ThemeToggle",
  component: ThemeToggle,
  tags: ["autodocs"],
  argTypes: {
    className: { control: "text" },
  },
};

export default meta;

type Story = StoryObj<typeof ThemeToggle>;

export const Default: Story = {};

export const CustomClassName: Story = {
  name: "Custom Class Name",
  args: {
    className: "rounded-md border border-border bg-background p-2 text-foreground transition-colors hover:bg-accent",
  },
};

export const InHeader: Story = {
  name: "In Header Context",
  render: () => (
    <header className="flex items-center justify-between border-b px-4 py-2">
      <span className="font-semibold">My App</span>
      <ThemeToggle />
    </header>
  ),
};
