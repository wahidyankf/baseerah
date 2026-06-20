import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { DEFAULT_LOCALE, isValidLocale } from "@/features/i18n/core/config";

export function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;

  // Skip API routes, static files, and Next.js internals
  if (
    pathname.startsWith("/api/") ||
    pathname.startsWith("/_next/") ||
    pathname.startsWith("/favicon") ||
    pathname === "/robots.txt" ||
    pathname === "/sitemap.xml" ||
    pathname === "/feed.xml"
  ) {
    return NextResponse.next();
  }

  // Redirect root to default locale
  if (pathname === "/") {
    return NextResponse.redirect(new URL(`/${DEFAULT_LOCALE}`, request.url));
  }

  // Extract first segment as potential locale
  const segments = pathname.split("/").filter(Boolean);
  const firstSegment = segments[0];

  // If no valid locale prefix, redirect to default locale
  if (firstSegment && !isValidLocale(firstSegment)) {
    // Don't redirect for static assets
    if (pathname.includes(".")) {
      return NextResponse.next();
    }
  }

  // Forward pathname so the root layout can resolve the locale for lang="…".
  const response = NextResponse.next();
  response.headers.set("x-pathname", pathname);
  return response;
}

export const config = {
  matcher: ["/((?!_next/static|_next/image|favicon.ico|favicon.png).*)"],
};
