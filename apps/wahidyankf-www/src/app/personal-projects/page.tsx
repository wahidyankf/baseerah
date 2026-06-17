import { PersonalProjectsContent } from "@/features/personal-projects/shell/PersonalProjectsContent";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Personal Projects | Wahidyan Kresna Fridayoka",
  description:
    "Open-source and personal projects by Wahidyan Kresna Fridayoka, including OSE, AyoKoding, OrganicLever, and more.",
};

export default async function Projects({ searchParams }: { searchParams: Promise<{ search?: string }> }) {
  const { search } = await searchParams;
  return <PersonalProjectsContent initialSearchTerm={search ?? ""} />;
}
