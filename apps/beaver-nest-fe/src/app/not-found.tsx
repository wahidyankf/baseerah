import type { Metadata } from "next";
import Link from "next/link";
import { AppFrame } from "@/components/AppFrame";

export const metadata: Metadata = {
  title: "404 · BeaverNest",
};

export default function NotFound() {
  return (
    <AppFrame>
      <p className="text-2xl font-semibold">Page not found.</p>
      <Link href="/" className="text-primary underline underline-offset-4">
        Back to home
      </Link>
    </AppFrame>
  );
}
