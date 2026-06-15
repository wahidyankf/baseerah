import type { Meta, StoryObj } from "@storybook/nextjs-vite";
import { useState } from "react";

import { SearchComponent } from "./search-component";

const meta: Meta<typeof SearchComponent> = {
  title: "Composites/SearchComponent",
  component: SearchComponent,
  tags: ["autodocs"],
};

export default meta;

type Story = StoryObj<typeof SearchComponent>;

function SearchWrapper({ placeholder = "Search..." }: { placeholder?: string }) {
  const [searchTerm, setSearchTerm] = useState("");
  return (
    <div className="w-80">
      <SearchComponent
        searchTerm={searchTerm}
        setSearchTerm={setSearchTerm}
        updateURL={(term) => {
          // no-op in story context
          void term;
        }}
        placeholder={placeholder}
      />
      {searchTerm && (
        <p className="mt-2 text-sm text-muted-foreground">
          Searching for: <strong>{searchTerm}</strong>
        </p>
      )}
    </div>
  );
}

export const Default: Story = {
  render: () => <SearchWrapper placeholder="Search..." />,
};

export const WithPrefilledTerm: Story = {
  name: "With Pre-filled Term",
  render: () => {
    const [searchTerm, setSearchTerm] = useState("react");
    return (
      <div className="w-80">
        <SearchComponent
          searchTerm={searchTerm}
          setSearchTerm={setSearchTerm}
          updateURL={() => {}}
          placeholder="Search topics..."
        />
      </div>
    );
  },
};

export const CustomPlaceholder: Story = {
  name: "Custom Placeholder",
  render: () => <SearchWrapper placeholder="Search articles, topics, tags..." />,
};
