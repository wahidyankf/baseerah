import Link from "next/link";
import { ChevronRight } from "lucide-react";
import { contentUrl } from "@/features/content/core/content-url";
import type { Locale } from "@/features/i18n/core/config";

interface BreadcrumbProps {
  locale: string;
  slug: string;
  segments: { label: string; slug: string }[];
  // When true, the final (current-page) segment is rendered as a non-link
  // aria-current="page" crumb instead of being dropped. Callers that already
  // surface the current page in an <h1> leave this absent (default behaviour).
  showCurrent?: boolean;
  // When true, segment hrefs are built via contentUrl (→ /c/ namespace).
  // Default false for backward compatibility with non-content breadcrumbs.
  contentHrefs?: boolean;
}

function hrefFor(locale: string, slug: string, useContentUrl: boolean): string {
  if (!slug) return `/${locale}`;
  return useContentUrl ? contentUrl(locale as Locale, slug) : `/${locale}/${slug}`;
}

export function Breadcrumb({ locale, segments, showCurrent = false, contentHrefs = false }: BreadcrumbProps) {
  // Default: exclude the last segment — the current page title is shown in the h1.
  // showCurrent: keep every segment; render the last one as a non-link crumb.
  const visibleSegments = showCurrent ? segments : segments.slice(0, -1);
  if (visibleSegments.length === 0) return null;

  const lastIndex = visibleSegments.length - 1;

  return (
    <nav aria-label="Breadcrumb" className="mb-4 text-sm text-muted-foreground">
      <ol className="flex flex-wrap items-center gap-1">
        {visibleSegments.map((segment, i) => {
          const isCurrent = showCurrent && i === lastIndex;
          return (
            <li key={segment.slug} className="flex items-center gap-1">
              {i > 0 && <ChevronRight className="h-3 w-3 shrink-0" />}
              {isCurrent ? (
                <span aria-current="page" className="font-medium text-foreground">
                  {segment.label}
                </span>
              ) : (
                <Link href={hrefFor(locale, segment.slug, contentHrefs)} className="hover:text-foreground">
                  {segment.label}
                </Link>
              )}
            </li>
          );
        })}
      </ol>
    </nav>
  );
}
