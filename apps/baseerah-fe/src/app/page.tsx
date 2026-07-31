import { AppShell } from "@/components/AppShell";
import { fetchGreeting } from "@/lib/greeting-client";

// The greeting is fetched from baseerah-be on every request — it must never be
// statically prerendered at build time, when the backend is unreachable.
export const dynamic = "force-dynamic";

export default async function Page() {
  const greeting = await fetchGreeting();

  return <AppShell greeting={greeting} />;
}
