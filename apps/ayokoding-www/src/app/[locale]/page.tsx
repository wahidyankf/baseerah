import { serverCaller } from "@/lib/trpc/server";
import type { Locale } from "@/features/i18n/core/config";
import type { TreeNode } from "@/features/content/core/types";
import { t } from "@/features/i18n/core/translations";
import { LANDING_SECTION_OVERRIDES, mergeLandingSections } from "@/features/content/core/landing-sections";
import { Landing } from "@/features/app-shell/shell/landing";

interface Props {
  params: Promise<{ locale: string }>;
}

export default async function LocaleHomePage({ params }: Props) {
  const { locale } = await params;
  const typedLocale = locale as Locale;

  const tree = (await serverCaller.content.getTree({
    locale: typedLocale,
  })) as TreeNode[];

  // The tree root may be a synthetic node (slug ""). The landing section cards
  // derive from its section-typed children (e.g. "learn"/"rants" for `en`,
  // "belajar"/"celoteh"/"konten-video" for `id`). Fall back to the root's
  // direct children when the tree is flat (no synthetic root node).
  const rootNode = tree.find((node) => node.slug === "");
  const topLevelSections = (rootNode ? rootNode.children : tree).filter((node) => node.isSection);

  const sections = mergeLandingSections(
    topLevelSections,
    LANDING_SECTION_OVERRIDES[typedLocale],
    t(typedLocale, "sectionBlurbFallback"),
  );

  return <Landing locale={typedLocale} sections={sections} />;
}
