import Link from "next/link";
import { ArrowRight } from "lucide-react";
import { Card, CardDescription, CardHeader, CardTitle } from "@open-sharia-enterprise/web-ui";

interface SectionCardProps {
  href: string;
  title: string;
  description: string;
  /** Optional footer line (e.g. "12 topics"). */
  meta?: string;
}

/**
 * A browse-grid section card. Composition over the shared `Card` token surface
 * (rounded border + `bg-card`) with an `bg-accent` hover affordance — no
 * net-new design primitive. The whole card is a single link to `href`.
 */
export function SectionCard({ href, title, description, meta }: SectionCardProps) {
  return (
    <Link href={href} className="group block focus-visible:outline-none">
      <Card className="h-full transition-colors group-focus-visible:ring-2 group-focus-visible:ring-ring hover:bg-accent">
        <CardHeader>
          <CardTitle className="text-lg">{title}</CardTitle>
          <CardDescription>{description}</CardDescription>
          {meta && (
            <span className="mt-2 inline-flex items-center gap-1 text-sm font-medium text-primary">
              {meta}
              <ArrowRight className="h-3.5 w-3.5" />
            </span>
          )}
        </CardHeader>
      </Card>
    </Link>
  );
}
