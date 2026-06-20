"use client";

import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@open-sharia-enterprise/web-ui/primitives";
import { SidebarTree } from "@/features/navigation/shell/sidebar-tree";
import { useEffect, useState } from "react";
import type { TreeNode } from "@/features/content/core/types";
import { trpcClient } from "@/lib/trpc/client";

interface MobileNavProps {
  locale: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function MobileNav({ locale, open, onOpenChange }: MobileNavProps) {
  const [tree, setTree] = useState<TreeNode[]>([]);

  useEffect(() => {
    if (open && tree.length === 0) {
      trpcClient.content.getTree.query({ locale: locale as "en" | "id" }).then((data) => {
        const raw = data as TreeNode[];
        // Skip the root locale node (e.g., "English Content") — mirror desktop Sidebar behaviour
        const rootNode = raw.find((n) => n.slug === "");
        setTree(rootNode ? rootNode.children : raw);
      });
    }
  }, [open, locale, tree.length]);

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent side="left" className="w-[280px] overflow-y-auto p-4">
        <SheetHeader>
          <SheetTitle className="text-left text-lg font-bold">AyoKoding</SheetTitle>
        </SheetHeader>
        <nav className="mt-4" aria-label="Mobile navigation">
          <SidebarTree nodes={tree} locale={locale} />
        </nav>
      </SheetContent>
    </Sheet>
  );
}
