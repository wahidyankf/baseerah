import { notFound } from "next/navigation";
import type { Metadata } from "next";
import { serverCaller } from "@/lib/trpc/server";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";
import { contentUrl } from "@/features/content/core/content-url";
import { slugFromSegments } from "@/features/content/core/slug";
import { Breadcrumb } from "@/features/navigation/shell/breadcrumb";
import { TableOfContents } from "@/features/navigation/shell/toc";
import { PrevNext } from "@/features/navigation/shell/prev-next";
import { MarkdownRenderer } from "@/features/content/shell/markdown-renderer";
import { TRPCError } from "@trpc/server";
import { createTRPCContext } from "@/features/app-shell/shell/trpc-init";

export const dynamicParams = false;

export async function generateStaticParams({ params }: { params: { locale: string } }) {
  const { contentService } = createTRPCContext();
  const index = await contentService.getIndex();
  const slugs: { slug: string[] }[] = [];

  // Enumerate ALL content slugs for this locale. The slug captured under /c/ is
  // already the bare content slug (no leading "c/" to strip).
  for (const [key, meta] of index.contentMap) {
    if (!key.startsWith(`${params.locale}:`)) continue;
    if (meta.slug === "") continue;
    slugs.push({ slug: meta.slug.split("/") });
  }

  return slugs;
}

interface Props {
  params: Promise<{ locale: string; slug: string[] }>;
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const { locale, slug } = await params;
  const slugStr = slugFromSegments(slug);

  try {
    const page = await serverCaller.content.getBySlug({
      locale: locale as Locale,
      slug: slugStr,
    });

    return {
      title: page.title,
      description: page.description ?? undefined,
      alternates: {
        canonical: contentUrl(locale as Locale, slugStr),
        languages: {
          en: contentUrl("en", slugStr),
          "x-default": contentUrl("en", slugStr),
        },
      },
      openGraph: {
        title: page.title,
        description: page.description ?? undefined,
        type: "article",
        locale: locale === "id" ? "id_ID" : "en_US",
      },
    };
  } catch {
    return { title: "Not Found" };
  }
}

export default async function ContentPage({ params }: Props) {
  const { locale, slug } = await params;
  const slugStr = slugFromSegments(slug);

  let page;
  try {
    page = await serverCaller.content.getBySlug({
      locale: locale as Locale,
      slug: slugStr,
    });
  } catch (err) {
    if (err instanceof TRPCError && err.code === "NOT_FOUND") {
      notFound();
    }
    throw err;
  }

  const breadcrumbSegments = buildBreadcrumbs(slugStr, page.title);

  return (
    <>
      <article className="min-w-0 flex-1 px-6 py-8 lg:px-8">
        <Breadcrumb locale={locale} slug={slugStr} segments={breadcrumbSegments} contentHrefs />

        <h1 className="mb-6 text-4xl font-extrabold tracking-tight">{page.title}</h1>

        <MarkdownRenderer html={page.html} locale={locale} />

        {page.date && (
          <p className="mt-8 text-sm text-muted-foreground">
            {t(locale as Locale, "lastUpdated")}{" "}
            {new Date(page.date).toLocaleDateString(locale === "id" ? "id-ID" : "en-US", {
              year: "numeric",
              month: "long",
              day: "numeric",
            })}
          </p>
        )}

        <PrevNext locale={locale} prev={page.prev} next={page.next} />
      </article>

      <aside className="hidden w-[200px] shrink-0 xl:block">
        <div className="sticky top-20 p-4">
          <TableOfContents headings={page.headings} label={t(locale as Locale, "onThisPage")} />
        </div>
      </aside>
    </>
  );
}

function buildBreadcrumbs(slug: string, currentTitle: string): { label: string; slug: string }[] {
  const parts = slug.split("/");
  const segments: { label: string; slug: string }[] = [];

  for (let i = 0; i < parts.length - 1; i++) {
    const part = parts[i];
    if (!part) continue;
    segments.push({
      label: part.charAt(0).toUpperCase() + part.slice(1).replace(/-/g, " "),
      slug: parts.slice(0, i + 1).join("/"),
    });
  }

  segments.push({ label: currentTitle, slug });
  return segments;
}
