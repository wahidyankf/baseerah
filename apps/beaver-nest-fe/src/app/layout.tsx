import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "BeaverNest",
  description: "BeaverNest — a personal operating layer",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
