import type { MetadataRoute } from "next";
import { createTRPCContext } from "@/features/app-shell/shell/trpc-init";
import { contentUrl } from "@/features/content/core/content-url";
import type { Locale } from "@/features/i18n/core/config";

export default async function sitemap(): Promise<MetadataRoute.Sitemap> {
  const { contentService } = createTRPCContext();
  const index = await contentService.getIndex();
  const entries: MetadataRoute.Sitemap = [];

  for (const [, meta] of index.contentMap) {
    const path = contentUrl(meta.locale as Locale, meta.slug);
    entries.push({
      url: `https://ayokoding.com${path}`,
      lastModified: meta.date ?? new Date(),
      changeFrequency: "weekly",
      priority: meta.isSection ? 0.8 : 0.6,
    });
  }

  return entries;
}
