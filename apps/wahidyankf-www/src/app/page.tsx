import { HomeContent } from "@/features/home/shell/HomeContent";

export default async function Home({ searchParams }: { searchParams: Promise<{ search?: string }> }) {
  const { search } = await searchParams;
  return <HomeContent initialSearchTerm={search ?? ""} />;
}
