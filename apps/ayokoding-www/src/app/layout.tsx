import type { Metadata } from "next";
import { GoogleAnalytics } from "@next/third-parties/google";
import { headers } from "next/headers";
import "katex/dist/katex.min.css";
import "./globals.css";
import { htmlLang } from "@/features/i18n/core/html-lang";
import { isValidLocale } from "@/features/i18n/core/config";

export const metadata: Metadata = {
  title: {
    default: "AyoKoding",
    template: "%s | AyoKoding",
  },
  description:
    "Bilingual educational platform for software engineering - helping the Indonesian tech community learn and grow",
  metadataBase: new URL("https://ayokoding.com"),
};

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const headersList = await headers();
  const pathname = headersList.get("x-pathname") ?? headersList.get("x-url") ?? "";
  const firstSegment = pathname.split("/").filter(Boolean)[0] ?? "";
  const locale = isValidLocale(firstSegment) ? firstSegment : "en";

  return (
    <html lang={htmlLang(locale)} suppressHydrationWarning>
      <body className="min-h-screen antialiased">
        {children}
        <GoogleAnalytics gaId="G-1NHDR7S3GV" />
      </body>
    </html>
  );
}
