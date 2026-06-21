import { serverCaller } from "@/lib/trpc/server";
import { SidebarTree } from "./sidebar-tree";
import type { TreeNode } from "@/features/content/core/types";

interface SidebarProps {
  locale: string;
}

export async function Sidebar({ locale }: SidebarProps) {
  const tree = (await serverCaller.content.getTree({
    locale: locale as "en" | "id",
  })) as TreeNode[];

  // Skip the root locale node (e.g., "English Content") and show its section children only.
  // Loose pages (About, Terms, Tools) are non-sections and belong in the footer, not the sidebar.
  const rootNode = tree.find((n) => n.slug === "");
  const sidebarNodes = (rootNode ? rootNode.children : tree).filter((n) => n.isSection);

  return (
    <nav aria-label="Sidebar navigation">
      <SidebarTree nodes={sidebarNodes} locale={locale} />
    </nav>
  );
}
