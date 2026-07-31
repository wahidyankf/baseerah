import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Baseerah",
  description: "Baseerah — insight, wawasan",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
