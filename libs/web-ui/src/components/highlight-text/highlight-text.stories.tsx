import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { HighlightText } from "./highlight-text";

const meta: Meta<typeof HighlightText> = {
  title: "Composites/HighlightText",
  component: HighlightText,
  tags: ["autodocs"],
  argTypes: {
    text: { control: "text" },
    searchTerm: { control: "text" },
    highlightClassName: { control: "text" },
  },
  args: {
    text: "The quick brown fox jumps over the lazy dog",
    searchTerm: "fox",
  },
};

export default meta;

type Story = StoryObj<typeof HighlightText>;

export const Default: Story = {};

export const MultipleMatches: Story = {
  name: "Multiple Matches",
  args: {
    text: "React components are great. React makes UI development easier.",
    searchTerm: "React",
  },
};

export const NoMatch: Story = {
  name: "No Match",
  args: {
    text: "The quick brown fox jumps over the lazy dog",
    searchTerm: "cat",
  },
};

export const EmptySearch: Story = {
  name: "Empty Search Term",
  args: {
    text: "The quick brown fox jumps over the lazy dog",
    searchTerm: "",
  },
};

export const CustomHighlightClass: Story = {
  name: "Custom Highlight Class",
  args: {
    text: "Search for important keywords in this text",
    searchTerm: "important",
    highlightClassName: "bg-blue-200 text-blue-900 rounded px-0.5",
  },
};
