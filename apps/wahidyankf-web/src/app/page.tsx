import { HomeContent } from "@/features/home/HomeContent";

export default async function Home({ searchParams }: { searchParams: Promise<{ search?: string }> }) {
  const { search } = await searchParams;
  return <HomeContent initialSearchTerm={search ?? ""} />;
}
