import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { HomeContent } from "@/features/home/shell/HomeContent";
import { filterItems } from "@/features/search/core/search";

// Mock the next/navigation module
const mockPush = vi.fn();
vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

// Mock the components imported from other files
vi.mock("@/features/app-shell/shell/Navigation", () => ({
  Navigation: () => <div data-testid="navigation">Navigation</div>,
}));

vi.mock("@open-sharia-enterprise/web-ui", () => ({
  SearchComponent: ({
    searchTerm,
    setSearchTerm,
    updateURL,
    placeholder,
  }: {
    searchTerm: string;
    setSearchTerm: (term: string) => void;
    updateURL: (term: string) => void;
    placeholder: string;
  }) => (
    <input
      data-testid="search-component"
      value={searchTerm}
      onChange={(e) => {
        setSearchTerm(e.target.value);
        updateURL(e.target.value);
      }}
      placeholder={placeholder}
    />
  ),
  HighlightText: ({ text }: { text: string }) => <span>{text}</span>,
}));

// Mock the filterItems function
vi.mock("@/features/search/core/search", () => ({
  filterItems: vi.fn((items) => items),
}));

// Mock the data and utility functions
vi.mock("@/features/cv/core/data", () => ({
  cvData: [
    {
      type: "about",
      details: ["Test about me"],
      links: {
        github: "https://github.com",
        linkedin: "https://linkedin.com",
        email: "test@example.com",
      },
    },
  ],
  getTopSkillsLastFiveYears: () => [
    { name: "Software Engineering", duration: 60 },
    { name: "Web Development", duration: 55 },
    { name: "React", duration: 50 },
  ],
  getTopLanguagesLastFiveYears: () => [
    { name: "JavaScript", duration: 60 },
    { name: "TypeScript", duration: 50 },
  ],
  getTopFrameworksLastFiveYears: () => [
    { name: "React", duration: 60 },
    { name: "Next.js", duration: 45 },
  ],
  formatDuration: (duration: number) => `${duration} months`,
}));

describe("Home component", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders the main sections", () => {
    render(<HomeContent initialSearchTerm="" />);
    expect(screen.getByText("Welcome to My Portfolio")).toBeInTheDocument();
    expect(screen.getByText("About Me")).toBeInTheDocument();
    expect(screen.getByText("Skills & Expertise")).toBeInTheDocument();
    expect(screen.getByText("Quick Links")).toBeInTheDocument();
    expect(screen.getByText("Connect With Me")).toBeInTheDocument();
  });

  it("renders the Navigation component", () => {
    render(<HomeContent initialSearchTerm="" />);
    expect(screen.getByTestId("navigation")).toBeInTheDocument();
  });

  it("renders the SearchComponent", () => {
    render(<HomeContent initialSearchTerm="" />);
    expect(screen.getByTestId("search-component")).toBeInTheDocument();
  });

  it("renders the about me section", () => {
    render(<HomeContent initialSearchTerm="" />);
    expect(screen.getByText("Test about me")).toBeInTheDocument();
  });

  it("renders skills, languages, and frameworks", () => {
    render(<HomeContent initialSearchTerm="" />);
    expect(screen.getByText("Software Engineering")).toBeInTheDocument();
    expect(screen.getByText("JavaScript")).toBeInTheDocument();
    expect(screen.getAllByText("React")).toHaveLength(2);
  });

  it("renders quick links", () => {
    render(<HomeContent initialSearchTerm="" />);
    expect(screen.getByText("View My CV")).toBeInTheDocument();
    expect(screen.getByText("Browse My Personal Projects")).toBeInTheDocument();
  });

  it("renders connect with me links", () => {
    render(<HomeContent initialSearchTerm="" />);
    expect(screen.getByText("Github")).toBeInTheDocument();
    expect(screen.getByText("Linkedin")).toBeInTheDocument();
    expect(screen.getByText("Email")).toBeInTheDocument();
  });

  it("updates search term when typing in the search component", () => {
    render(<HomeContent initialSearchTerm="" />);
    const searchInput = screen.getByTestId("search-component") as HTMLInputElement;
    fireEvent.change(searchInput, { target: { value: "React" } });
    expect(searchInput.value).toBe("React");
  });

  it("filters content based on search term", () => {
    render(<HomeContent initialSearchTerm="" />);
    const searchInput = screen.getByTestId("search-component");
    fireEvent.change(searchInput, { target: { value: "React" } });

    expect(filterItems).toHaveBeenCalled();
  });

  it("handles item click and navigates to CV page", () => {
    render(<HomeContent initialSearchTerm="" />);
    const skillButtons = screen.getAllByText("React");
    fireEvent.click(skillButtons[0]);
    expect(mockPush).toHaveBeenCalledWith("/cv?search=React&scrollTop=true");
  });

  it("handles language button click and navigates to CV page", () => {
    render(<HomeContent initialSearchTerm="" />);
    const languageButton = screen.getByText("JavaScript");
    fireEvent.click(languageButton);
    expect(mockPush).toHaveBeenCalledWith("/cv?search=JavaScript&scrollTop=true");
  });

  it("handles framework button click and navigates to CV page", () => {
    render(<HomeContent initialSearchTerm="" />);
    const frameworkButton = screen.getByText("Next.js");
    fireEvent.click(frameworkButton);
    expect(mockPush).toHaveBeenCalledWith("/cv?search=Next.js&scrollTop=true");
  });

  it("updates URL when typing in search", () => {
    render(<HomeContent initialSearchTerm="" />);
    const searchInput = screen.getByTestId("search-component") as HTMLInputElement;
    fireEvent.change(searchInput, { target: { value: "TypeScript" } });
    expect(mockPush).toHaveBeenCalledWith("/?search=TypeScript", { scroll: false });
  });

  it("handles TypeScript skill pill click navigates with scrollTop", () => {
    render(<HomeContent initialSearchTerm="" />);
    const languageButton = screen.getByText("TypeScript");
    fireEvent.click(languageButton);
    expect(mockPush).toHaveBeenCalledWith("/cv?search=TypeScript&scrollTop=true");
  });
});
