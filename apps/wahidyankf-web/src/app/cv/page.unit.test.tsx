import React from "react";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { CvContent } from "@/features/cv/CvContent";

// Mock declarations
const mockPush = vi.fn();
const mockReplace = vi.fn();

// Add this mock for window.scrollTo
vi.stubGlobal("scrollTo", vi.fn());

vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: mockPush,
    replace: mockReplace,
  }),
}));

vi.mock("@/features/app-shell/Navigation", () => ({
  Navigation: () => <div data-testid="navigation">Navigation</div>,
}));

vi.mock("@open-sharia-enterprise/web-ui", () => ({
  SearchComponent: ({
    searchTerm,
    setSearchTerm,
    placeholder,
  }: {
    searchTerm: string;
    setSearchTerm: (term: string) => void;
    placeholder: string;
  }) => (
    <input
      data-testid="search-component"
      value={searchTerm}
      onChange={(e) => setSearchTerm(e.target.value)}
      placeholder={placeholder}
    />
  ),
  HighlightText: ({ text }: { text: string }) => <span>{text}</span>,
}));

vi.mock("@/features/search/search", () => ({
  filterItems: vi.fn((items) => items),
}));

vi.mock("@/features/cv/data", () => ({
  cvData: [
    {
      type: "about",
      title: "About Me",
      details: ["Test about me"],
      links: {
        github: "https://github.com",
        linkedin: "https://linkedin.com",
        email: "test@example.com",
      },
    },
    {
      type: "work",
      title: "Software Engineer",
      organization: "Tech Company",
      period: "Jan 2020 - Present",
      details: ["Worked on various projects"],
      skills: ["React", "TypeScript"],
      programmingLanguages: ["JavaScript"],
      frameworks: ["Next.js"],
    },
    {
      type: "education",
      title: "Bachelor of Science in Computer Science",
      organization: "University of Example",
      period: "2015 - 2019",
      details: ["Graduated with honors"],
    },
  ],
  getTopSkillsLastFiveYears: () => [{ name: "React", duration: 60 }],
  getTopLanguagesLastFiveYears: () => [{ name: "JavaScript", duration: 60 }],
  getTopFrameworksLastFiveYears: () => [{ name: "Next.js", duration: 60 }],
  formatDuration: (duration: number) => `${duration} months`,
  parseDate: vi.fn((dateStr: string) => new Date(dateStr)),
  calculateDuration: vi.fn(() => 12),
  calculateTotalDuration: vi.fn(() => 12),
}));

describe("CV component", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(window.scrollTo).mockReset();
  });

  it("renders the main sections", () => {
    render(<CvContent initialSearchTerm="" scrollTop={false} />);
    expect(screen.getByText("Curriculum Vitae")).toBeInTheDocument();
    expect(screen.getByText("Highlights")).toBeInTheDocument();
  });

  it("renders the Navigation component", () => {
    render(<CvContent initialSearchTerm="" scrollTop={false} />);
    expect(screen.getByTestId("navigation")).toBeInTheDocument();
  });

  it("renders the SearchComponent", () => {
    render(<CvContent initialSearchTerm="" scrollTop={false} />);
    expect(screen.getByTestId("search-component")).toBeInTheDocument();
  });

  it("renders the about section", () => {
    render(<CvContent initialSearchTerm="" scrollTop={false} />);
    expect(screen.getByText("About Me")).toBeInTheDocument();
    expect(screen.getByText(/Test about me/)).toBeInTheDocument();
  });

  it("renders work experience", () => {
    render(<CvContent initialSearchTerm="" scrollTop={false} />);
    expect(screen.getByText("Work Experience")).toBeInTheDocument();
    const softwareElements = screen.getAllByText(/Software/);
    expect(softwareElements.length).toBeGreaterThan(0);

    const workExperienceSection = screen.getByText("Work Experience").closest("div");
    expect(workExperienceSection).toBeInTheDocument();
    expect(workExperienceSection?.textContent).toMatch(/\w+/);
  });

  it("renders skills, languages, and frameworks", () => {
    render(<CvContent initialSearchTerm="" scrollTop={false} />);
    expect(screen.getByText("Top Skills Used in The Last 5 Years")).toBeInTheDocument();
    const reactElements = screen.getAllByText(/React/);
    expect(reactElements.length).toBeGreaterThan(0);
  });

  it("updates search term when typing in the search component", () => {
    render(<CvContent initialSearchTerm="" scrollTop={false} />);
    const searchInput = screen.getByTestId("search-component") as HTMLInputElement;
    fireEvent.change(searchInput, { target: { value: "React" } });
    expect(searchInput.value).toBe("React");
  });

  it("filters content based on search term", async () => {
    render(<CvContent initialSearchTerm="" scrollTop={false} />);
    const searchInput = screen.getByTestId("search-component");
    fireEvent.change(searchInput, { target: { value: "Software" } });
    await waitFor(() => {
      expect(screen.getAllByText(/Software/).length).toBeGreaterThan(0);
    });
  });

  it("handles item click and updates search", () => {
    render(<CvContent initialSearchTerm="" scrollTop={false} />);
    const skillButtons = screen.getAllByText("React");
    fireEvent.click(skillButtons[0]);
    expect(mockPush).toHaveBeenCalled();
    expect(mockPush.mock.calls[0][0]).toMatch(/^\/cv\?search=React/);
    expect(window.scrollTo).toHaveBeenCalledWith(0, 0);
  });

  it("renders education entries with organization", () => {
    render(<CvContent initialSearchTerm="" scrollTop={false} />);

    expect(screen.getByText("Education")).toBeInTheDocument();
    expect(screen.getByText("Bachelor of Science in Computer Science")).toBeInTheDocument();
    expect(screen.getByText("University of Example")).toBeInTheDocument();
    expect(screen.getByText("2015 - 2019")).toBeInTheDocument();
    expect(screen.getByText("Graduated with honors")).toBeInTheDocument();
  });

  it("scrolls to top when scrollTop prop is true", () => {
    render(<CvContent initialSearchTerm="" scrollTop={true} />);
    expect(window.scrollTo).toHaveBeenCalledWith(0, 0);
  });
});
