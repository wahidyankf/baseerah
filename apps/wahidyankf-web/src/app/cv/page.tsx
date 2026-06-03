import { CvContent } from "@/features/cv/CvContent";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "CV | Wahidyan Kresna Fridayoka",
  description:
    "Full curriculum vitae of Wahidyan Kresna Fridayoka — work experience, skills, education, and certifications.",
};

export default async function CV({ searchParams }: { searchParams: Promise<{ search?: string; scrollTop?: string }> }) {
  const { search, scrollTop } = await searchParams;
  return <CvContent initialSearchTerm={search ?? ""} scrollTop={scrollTop === "true"} />;
}
