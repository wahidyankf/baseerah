import Link from "next/link";

export default function ToolsIndexPage() {
  return (
    <main className="mx-auto max-w-6xl space-y-4 px-4 py-6">
      <h1 className="text-2xl font-bold tracking-tight">Tools</h1>
      <ul className="space-y-2">
        <li>
          <Link href="./tools/cost-of-living-calculator" className="text-primary underline">
            Cost of Living Calculator
          </Link>
        </li>
      </ul>
    </main>
  );
}
