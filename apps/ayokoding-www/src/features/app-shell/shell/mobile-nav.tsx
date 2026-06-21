"use client";

import Link from "next/link";
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@open-sharia-enterprise/web-ui/primitives";
import { SidebarTree } from "@/features/navigation/shell/sidebar-tree";
import { useEffect, useState } from "react";
import type { TreeNode } from "@/features/content/core/types";
import { trpcClient } from "@/lib/trpc/client";
import { t } from "@/features/i18n/core/translations";
import type { Locale } from "@/features/i18n/core/config";
import { PRIMARY_NAV_LINKS } from "@/features/app-shell/core/nav-links";

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
          <p className="px-1 text-xs font-semibold tracking-wide text-muted-foreground uppercase">Menu</p>
          <ul className="mt-2 mb-4 space-y-1">
            {PRIMARY_NAV_LINKS.map((link) => (
              <li key={link.labelKey}>
                <Link
                  href={link.hrefFor(locale as Locale)}
                  onClick={() => onOpenChange(false)}
                  className="block rounded-md px-3 py-2 text-sm font-medium hover:bg-accent hover:text-accent-foreground"
                >
                  {t(locale as Locale, link.labelKey)}
                </Link>
              </li>
            ))}
          </ul>
          <SidebarTree nodes={tree} locale={locale} />
        </nav>
      </SheetContent>
    </Sheet>
  );
}
