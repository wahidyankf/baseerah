import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import ScrollToTop from "./scroll-to-top";

const meta: Meta<typeof ScrollToTop> = {
  title: "Composites/ScrollToTop",
  component: ScrollToTop,
  tags: ["autodocs"],
  argTypes: {
    threshold: { control: "number" },
    className: { control: "text" },
    buttonClassName: { control: "text" },
  },
  args: {
    threshold: 300,
  },
};

export default meta;

type Story = StoryObj<typeof ScrollToTop>;

export const Default: Story = {
  render: () => (
    <div className="relative h-96 overflow-y-auto rounded-md border p-4">
      <p className="mb-4 text-sm text-muted-foreground">
        The ScrollToTop button appears after the user scrolls past the threshold. In this story the button is always
        rendered for demonstration purposes.
      </p>
      {/* Render a visible placeholder since the scroll event won't fire in a story */}
      <button
        className="fixed right-4 bottom-20 z-50 rounded-full bg-yellow-400 p-2 text-gray-900 shadow-lg transition-colors duration-300 hover:bg-yellow-300"
        aria-label="Scroll to top"
        onClick={() => {}}
      >
        ↑
      </button>
    </div>
  ),
};

export const WithScrollableContent: Story = {
  name: "With Scrollable Content",
  render: () => (
    <div style={{ height: "200px", overflow: "auto", border: "1px solid #ccc", padding: "1rem" }}>
      <p className="text-sm text-muted-foreground">Scroll down to see the button appear.</p>
      <ScrollToTop threshold={50} />
      {Array.from({ length: 20 }, (_, i) => (
        <p key={i} className="my-2 text-sm">
          Content paragraph {i + 1}
        </p>
      ))}
    </div>
  ),
};
