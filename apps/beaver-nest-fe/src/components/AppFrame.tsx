import type { ReactNode } from "react";
import { AppHeader } from "@open-sharia-enterprise/web-ui";

export interface AppFrameProps {
  children: ReactNode;
}

// Shared chrome for every rendered surface of this app (landing, not-found, error), so a
// visitor who lands on a degraded/unmatched route still sees the same branded header/footer
// landmarks and design tokens as the happy path — see Rule-15 findings EWT-001/UWT-004/DWT-003.
export function AppFrame({ children }: AppFrameProps) {
  return (
    <div className="flex min-h-screen flex-col">
      <div className="border-border text-primary border-b">
        <AppHeader title="BeaverNest" titleHref="/" />
      </div>
      <main className="flex flex-1 flex-col items-center justify-center gap-5 px-6 py-12 text-center">{children}</main>
      <footer className="border-border bg-secondary text-muted-foreground border-t px-8 py-4 text-center text-sm">
        &copy; BeaverNest
      </footer>
    </div>
  );
}
